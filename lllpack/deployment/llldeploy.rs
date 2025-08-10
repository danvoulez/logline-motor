//! CLI institucional simplificado para orquestração de deployment
//! 
//! Um wrapper leve para o sistema de deployment institucional do LogLine.

use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::sync::Arc;

use clap::{App, Arg, SubCommand};
use anyhow::{Result, Context, anyhow};
use serde_json::Value;
use tokio::runtime::Runtime;
use uuid::Uuid;
use chrono::Utc;
use log::{info, error, warn};

use crate::deployment::{
    DeploymentOrchestrator, 
    DeploymentPlan,
    DeploymentRequest,
    ApiConfig,
    DeploymentStatus,
    RollbackRequest,
    ResourceSpec,
};

/// Versão da CLI
const VERSION: &str = "1.0.0";

/// Função principal do CLI
pub fn main() -> Result<()> {
    // Configura logger
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info")
    );
    
    info!("llldeploy v{} - CLI Institucional para Deployment", VERSION);
    
    // Cria runtime assíncrono
    let runtime = Runtime::new().context("Falha ao inicializar runtime Tokio")?;
    
    // Processa argumentos via clap
    let matches = App::new("llldeploy")
        .version(VERSION)
        .author("VOULEZVOUS.TV")
        .about("CLI Institucional para Deployment do LogLine")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Arquivo de configuração alternativo")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Modo verbose")
                .takes_value(false)
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Inicia um novo deployment")
                .arg(
                    Arg::with_name("plan")
                        .short("p")
                        .long("plan")
                        .value_name("FILE")
                        .help("Arquivo JSON com plano de deployment")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("environment")
                        .short("e")
                        .long("environment")
                        .value_name("ENV")
                        .help("Ambiente alvo (production, staging, etc)")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("Verifica status de deployment")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("DEPLOYMENT_ID")
                        .help("ID do deployment")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("watch")
                        .short("w")
                        .long("watch")
                        .help("Acompanha o status em tempo real")
                        .takes_value(false)
                )
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lista deployments")
                .arg(
                    Arg::with_name("limit")
                        .short("l")
                        .long("limit")
                        .value_name("N")
                        .help("Limite de deployments a exibir")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("environment")
                        .short("e")
                        .long("environment")
                        .value_name("ENV")
                        .help("Filtrar por ambiente")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("rollback")
                .about("Inicia rollback de deployment")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("DEPLOYMENT_ID")
                        .help("ID do deployment")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("reason")
                        .short("r")
                        .long("reason")
                        .value_name("REASON")
                        .help("Motivo do rollback")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Força rollback mesmo em estado não recomendado")
                        .takes_value(false)
                )
        )
        .subcommand(
            SubCommand::with_name("cancel")
                .about("Cancela um deployment em andamento")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("DEPLOYMENT_ID")
                        .help("ID do deployment")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("Inicia servidor HTTP da API de deployment")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("PORT")
                        .help("Porta para o servidor HTTP")
                        .default_value("8080")
                        .takes_value(true)
                )
        )
        .get_matches();
    
    // Carrega configuração
    let config_path = matches.value_of("config").unwrap_or("config/deployment.json");
    
    // Executa comando correspondente
    let result = match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            let plan_path = deploy_matches.value_of("plan").unwrap();
            let environment = deploy_matches.value_of("environment");
            
            runtime.block_on(handle_deploy(config_path, plan_path, environment))
        },
        ("status", Some(status_matches)) => {
            let deployment_id = status_matches.value_of("id").unwrap();
            let watch = status_matches.is_present("watch");
            
            runtime.block_on(handle_status(config_path, deployment_id, watch))
        },
        ("list", Some(list_matches)) => {
            let limit = list_matches.value_of("limit")
                .map(|s| s.parse::<usize>().unwrap_or(10))
                .unwrap_or(10);
            let environment = list_matches.value_of("environment");
            
            runtime.block_on(handle_list(config_path, limit, environment))
        },
        ("rollback", Some(rollback_matches)) => {
            let deployment_id = rollback_matches.value_of("id").unwrap();
            let reason = rollback_matches.value_of("reason").unwrap();
            let force = rollback_matches.is_present("force");
            
            runtime.block_on(handle_rollback(config_path, deployment_id, reason, force))
        },
        ("cancel", Some(cancel_matches)) => {
            let deployment_id = cancel_matches.value_of("id").unwrap();
            
            runtime.block_on(handle_cancel(config_path, deployment_id))
        },
        ("server", Some(server_matches)) => {
            let port = server_matches.value_of("port")
                .unwrap_or("8080")
                .parse::<u16>()
                .unwrap_or(8080);
                
            runtime.block_on(handle_server(config_path, port))
        },
        _ => {
            println!("Comando não reconhecido. Use 'llldeploy --help' para ajuda.");
            Ok(())
        }
    };
    
    // Trata erros
    if let Err(e) = result {
        error!("Erro: {}", e);
        eprintln!("Erro: {}", e);
        exit(1);
    }
    
    Ok(())
}

/// Manipula comando de deploy
async fn handle_deploy(
    config_path: &str,
    plan_path: &str,
    environment_override: Option<&str>,
) -> Result<()> {
    info!("Iniciando deployment a partir de {}", plan_path);
    
    // Lê o arquivo de plano
    let plan_content = fs::read_to_string(plan_path)
        .with_context(|| format!("Falha ao ler arquivo de plano: {}", plan_path))?;
        
    // Parse como JSON
    let mut plan_json: DeploymentRequest = serde_json::from_str(&plan_content)
        .with_context(|| "Falha ao parsear JSON do plano")?;
        
    // Sobrescreve ambiente se especificado
    if let Some(env) = environment_override {
        info!("Sobrescrevendo ambiente para: {}", env);
        plan_json.environment = env.to_string();
    }
    
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    // Converte para plano interno
    let deployment_id = Uuid::new_v4().to_string();
    let deployment_plan = DeploymentPlan::new(
        deployment_id.clone(),
        plan_json.name.clone(),
        plan_json.version.clone(),
        plan_json.environment.clone(),
        plan_json.strategy.parse().unwrap_or_default(),
        plan_json.created_by.clone().unwrap_or_else(|| "llldeploy-cli".to_string()),
    );
    
    // Inicia deployment
    info!("Iniciando deployment ID: {}", deployment_id);
    orchestrator.start_deployment(deployment_plan).await?;
    
    println!("Deployment iniciado com sucesso!");
    println!("ID: {}", deployment_id);
    println!("Aplicação: {} v{}", plan_json.name, plan_json.version);
    println!("Ambiente: {}", plan_json.environment);
    println!("\nVerifique o status com: llldeploy status -i {}", deployment_id);
    
    Ok(())
}

/// Manipula comando de status
async fn handle_status(
    config_path: &str,
    deployment_id: &str,
    watch: bool,
) -> Result<()> {
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    if !watch {
        // Busca status uma vez
        let status = orchestrator.get_deployment_status(deployment_id).await?;
        
        println!("Deployment ID: {}", deployment_id);
        println!("Status: {}", status.status.as_str());
        
        if let Some(phases) = status.phases {
            println!("\nFases:");
            for phase in phases {
                let status_symbol = match phase.status.as_str() {
                    "completed" => "✅",
                    "in_progress" => "🔄",
                    "failed" => "❌",
                    "pending" => "⏳",
                    _ => "  ",
                };
                
                println!("{} {} - {}", status_symbol, phase.name, phase.status);
                
                if let Some(details) = &phase.details {
                    println!("   └─ {}", details);
                }
                
                if let Some(error) = &phase.error {
                    println!("   ❗ {}", error);
                }
            }
        }
        
        if let Some(details) = status.details {
            println!("\nDetalhes: {}", details);
        }
    } else {
        // Modo watch - atualiza a cada segundo
        println!("Monitorando deployment {}...", deployment_id);
        println!("Pressione Ctrl+C para sair");
        
        let mut last_status = String::new();
        
        loop {
            let status = match orchestrator.get_deployment_status(deployment_id).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Erro ao obter status: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }
            };
            
            let current_status = status.status.as_str().to_string();
            
            if current_status != last_status {
                println!("{} - Status: {}", 
                    chrono::Local::now().format("%H:%M:%S"),
                    current_status);
                last_status = current_status;
            }
            
            if status.status.is_terminal() {
                println!("\nDeployment finalizado com status: {}", current_status);
                break;
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    
    Ok(())
}

/// Manipula comando de listar deployments
async fn handle_list(
    config_path: &str,
    limit: usize,
    environment: Option<&str>,
) -> Result<()> {
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    // Obtém lista de deployments
    let deployments = orchestrator.list_deployments().await?;
    
    // Filtra por ambiente se necessário
    let filtered = if let Some(env) = environment {
        deployments
            .into_iter()
            .filter(|d| d.environment == env)
            .collect::<Vec<_>>()
    } else {
        deployments
    };
    
    // Limita número de resultados
    let limited = filtered.into_iter().take(limit).collect::<Vec<_>>();
    
    if limited.is_empty() {
        println!("Nenhum deployment encontrado.");
        return Ok(());
    }
    
    println!("{:<36} {:<15} {:<10} {:<12} {:<10}", 
        "ID", "NOME", "VERSÃO", "AMBIENTE", "STATUS");
    println!("{}", "-".repeat(90));
    
    for deployment in limited {
        println!("{:<36} {:<15} {:<10} {:<12} {:<10}",
            deployment.id,
            deployment.name,
            deployment.version,
            deployment.environment,
            deployment.status);
    }
    
    Ok(())
}

/// Manipula comando de rollback
async fn handle_rollback(
    config_path: &str,
    deployment_id: &str,
    reason: &str,
    force: bool,
) -> Result<()> {
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    // Cria requisição de rollback
    let request = RollbackRequest {
        reason: reason.to_string(),
        target_version: None,
        options: Some(serde_json::json!({
            "force": force
        }).as_object().unwrap().clone()),
    };
    
    // Inicia rollback
    println!("Iniciando rollback do deployment {}...", deployment_id);
    orchestrator.start_rollback(deployment_id, request).await?;
    
    println!("Rollback iniciado com sucesso!");
    println!("\nVerifique o status com: llldeploy status -i {}", deployment_id);
    
    Ok(())
}

/// Manipula comando de cancelamento
async fn handle_cancel(
    config_path: &str,
    deployment_id: &str,
) -> Result<()> {
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    // Cancela deployment
    println!("Cancelando deployment {}...", deployment_id);
    orchestrator.cancel_deployment(deployment_id).await?;
    
    println!("Deployment cancelado com sucesso!");
    
    Ok(())
}

/// Manipula comando de servidor
async fn handle_server(
    config_path: &str,
    port: u16,
) -> Result<()> {
    // Inicializa orquestrador
    let orchestrator = crate::deployment::initialize(Some(config_path)).await?;
    
    // Configura servidor API
    let api_config = ApiConfig {
        port,
        host: "0.0.0.0".to_string(),
        enable_auth: true,
        auth_token: Some(Uuid::new_v4().to_string()),
        request_timeout_seconds: 60,
        ..Default::default()
    };
    
    println!("Iniciando servidor HTTP na porta {}", port);
    
    if let Some(token) = &api_config.auth_token {
        println!("Token de autenticação: {}", token);
        println!("Use este token como Bearer em requisições à API");
    }
    
    println!("\nEndpoints disponíveis:");
    println!("POST   /api/deploy           - Iniciar deployment");
    println!("GET    /api/deploy/:id       - Status de deployment");
    println!("POST   /api/deploy/:id/rollback - Iniciar rollback");
    println!("DELETE /api/deploy/:id       - Cancelar deployment");
    println!("GET    /api/deploy           - Listar deployments");
    
    // Inicia servidor HTTP
    crate::deployment::start_api_server(orchestrator, Some(api_config)).await?;
    
    Ok(())
}
