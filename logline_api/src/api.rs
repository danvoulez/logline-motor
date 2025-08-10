// === File: logline_api/src/api.rs ===
/*
    Description: Configuração de rotas e handlers API para o LogLine Motor.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use crate::config::ApiConfig;
use crate::error::{ApiError, ErrorResponse};
use crate::health;
use streaming::{EventBroker, Event};
use plugin_manager::PluginManager;
use runtime::{process_command, timeline};
use parser::ast::Command;
use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use std::convert::Infallible;
use log::{error};
use serde::{Serialize, Deserialize};

// Modelo para requisição DSL
#[derive(Debug, Serialize, Deserialize)]
struct DslRequest {
    command: String,
}

// Modelo para resposta DSL
#[derive(Debug, Serialize, Deserialize)]
struct DslResponse {
    result: String,
    events: Vec<serde_json::Value>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

// Configura todas as rotas da API
pub async fn setup_routes(
    broker: Arc<EventBroker>,
    plugin_manager: Arc<PluginManager>,
    config: &ApiConfig,
) -> Result<impl Filter<Extract = impl Reply> + Clone, ApiError> {
    // Rota base da API
    let api_base = warp::path(config.api_prefix.clone());
    
    // Rota para comandos DSL
    let dsl_route = api_base
        .and(warp::path("dsl"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_broker(broker.clone()))
        .and(with_plugin_manager(plugin_manager.clone()))
        .and_then(handle_dsl);
    
    // Rota para streaming de eventos
    let stream_route = api_base
        .and(warp::path("stream"))
        .and(warp::get())
        .and(with_broker(broker.clone()))
        .and_then(handle_stream);
    
    // Rota para verificação de saúde
    let health_route = api_base
        .and(warp::path("health"))
        .and(warp::get())
        .and_then(health::handle_health);
    
    // Rota para verificação de prontidão
    let readiness_route = api_base
        .and(warp::path("ready"))
        .and(warp::get())
        .and_then(health::handle_readiness);
    
    // Rota para verificação de integridade do banco de dados
    let db_health_route = api_base
        .and(warp::path("healthz"))
        .and(warp::path("db"))
        .and(warp::get())
        .and_then(health::handle_db_health);
    
    // Combina todas as rotas
    let routes = dsl_route
        .or(stream_route)
        .or(health_route)
        .or(readiness_route)
        .or(db_health_route)
        .recover(handle_rejection);
    
    // Adiciona CORS se configurado
    let routes = if let Some(allowed_origin) = &config.cors_allowed_origin {
        routes.with(warp::cors()
            .allow_origin(allowed_origin.as_str())
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
            .allow_headers(vec!["Content-Type", "Last-Event-ID"])
            .build())
    } else {
        routes
    };
    
    Ok(routes)
}

// Handler para a rota /dsl
async fn handle_dsl(
    request: DslRequest,
    broker: Arc<EventBroker>,
    plugin_manager: Arc<PluginManager>,
) -> Result<impl Reply, Rejection> {
    // Esta chamada de log em nível de debug foi removida para evitar excesso de ruído no runtime.
    // debug!("Recebido comando DSL: {}", request.command);
    
    // Processa o comando DSL
    let result = match process_command(&request.command).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erro ao processar comando DSL: {}", e);
            return Err(warp::reject::custom(ApiError::CommandProcessingError(e.to_string())));
        }
    };
    
    // Recupera os eventos gerados pelo comando
    let events = timeline::list_events().await
        .map_err(|e| warp::reject::custom(ApiError::InternalError(e.to_string())))?;
    
    // Converte os eventos para JSON
    let events_json: Vec<serde_json::Value> = events.iter()
        .map(|e| serde_json::to_value(e).unwrap_or(serde_json::Value::Null))
        .collect();
    
    // Cria a resposta
    let response = DslResponse {
        result,
        events: events_json,
        timestamp: chrono::Utc::now(),
    };
    
    // Publica o evento de comando executado no broker de streaming
    let event = Event::new("command_executed", serde_json::to_value(&response).unwrap())
        .with_channels(vec!["commands".to_string(), "default".to_string()]);
    
    // Não bloqueia a resposta se a publicação falhar
    if let Err(e) = broker.publish(event).await {
        error!("Erro ao publicar evento de comando: {}", e);
    }
    
    Ok(warp::reply::json(&response))
}

// Handler para a rota /stream
async fn handle_stream(broker: Arc<EventBroker>) -> Result<impl Reply, Rejection> {
    // Obtém o cabeçalho Last-Event-ID se presente
    let last_event_id = warp::header::optional::<String>("Last-Event-ID")
        .map(|id| id);
    
    // Obtém os parâmetros de query para canais
    let channels = warp::query::raw()
        .map(|q: String| {
            q.split('&')
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    if let (Some("channel"), Some(channel)) = (parts.next(), parts.next()) {
                        Some(channel.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        });
    
    // Se não houver canais especificados, usa o canal "default"
    let channels = if channels.is_empty() {
        Some(vec!["default".to_string()])
    } else {
        Some(channels)
    };
    
    // Cria um stream de eventos para o cliente
    let event_stream = broker.subscribe(channels, last_event_id).await;
    
    // Mapeia eventos para o formato SSE
    let sse_stream = event_stream.map(|event| {
        match event {
            Ok(event) => {
                let data = serde_json::to_string(&event.data).unwrap_or_default();
                format!(
                    "id: {}\nevent: {}\ndata: {}\n\n",
                    event.id,
                    event.event,
                    data
                )
            }
            Err(_) => "event: heartbeat\ndata: {}\n\n".to_string(),
        }
    });
    
    // Configura a resposta SSE
    let response = warp::http::Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("X-Accel-Buffering", "no")
        .body(warp::sse::reply(sse_stream))
        .unwrap();
    
    Ok(response)
}

// Handler para erros
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    
    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Recurso não encontrado".to_string();
    } else if let Some(api_error) = err.find::<ApiError>() {
        match api_error {
            ApiError::CommandProcessingError(e) => {
                code = warp::http::StatusCode::BAD_REQUEST;
                message = format!("Erro ao processar comando: {}", e);
            }
            ApiError::InvalidCommandFormat(e) => {
                code = warp::http::StatusCode::BAD_REQUEST;
                message = format!("Formato de comando inválido: {}", e);
            }
            ApiError::InternalError(e) => {
                code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
                message = format!("Erro interno do servidor: {}", e);
            }
        }
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = format!("Corpo da requisição inválido: {}", e);
    } else {
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Erro interno não especificado".to_string();
    }
    
    // Constrói a resposta de erro
    let json = warp::reply::json(&ErrorResponse {
        status: code.as_u16(),
        message,
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    
    Ok(warp::reply::with_status(json, code))
}

// Utilidades para injeção de dependências nos handlers
fn with_broker(
    broker: Arc<EventBroker>
) -> impl Filter<Extract = (Arc<EventBroker>,), Error = Infallible> + Clone {
    warp::any().map(move || broker.clone())
}

fn with_plugin_manager(
    plugin_manager: Arc<PluginManager>
) -> impl Filter<Extract = (Arc<PluginManager>,), Error = Infallible> + Clone {
    warp::any().map(move || plugin_manager.clone())
}