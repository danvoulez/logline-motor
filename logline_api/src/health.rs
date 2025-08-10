// === File: logline_api/src/health.rs ===
/*
    Description: Endpoints de health check para a API LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use warp::{Rejection, Reply};
use serde_json::json;

// Handler para verificação básica de saúde (/health)
pub async fn handle_health() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json!({
        "status": "UP",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": core::VERSION,
        "product": core::PRODUCT_NAME
    })))
}

// Handler para verificação de prontidão (/ready)
pub async fn handle_readiness() -> Result<impl Reply, Rejection> {
    // Em uma implementação real, verificaria todos os componentes
    let all_services_ready = true; // Simplifição para este exemplo
    
    let status = if all_services_ready { "READY" } else { "NOT_READY" };
    let status_code = if all_services_ready { 
        warp::http::StatusCode::OK 
    } else { 
        warp::http::StatusCode::SERVICE_UNAVAILABLE 
    };
    
    let response = warp::reply::json(&json!({
        "status": status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "parser": "UP",
            "runtime": "UP", 
            "database": "UP",
            "streaming": "UP"
        }
    }));
    
    Ok(warp::reply::with_status(response, status_code))
}

// Handler para verificação de integridade do banco de dados (/healthz/db)
pub async fn handle_db_health() -> Result<impl Reply, Rejection> {
    // Em uma implementação real, verificaria a conexão com o banco
    // Simulação: 95% do tempo responde OK
    let db_status_ok = rand::random::<f32>() > 0.05;
    
    let status = if db_status_ok { "UP" } else { "DOWN" };
    let status_code = if db_status_ok { 
        warp::http::StatusCode::OK 
    } else { 
        warp::http::StatusCode::SERVICE_UNAVAILABLE 
    };
    
    let response = warp::reply::json(&json!({
        "database": status,
        "latency_ms": if db_status_ok { 15 } else { 500 },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }));
    
    Ok(warp::reply::with_status(response, status_code))
}