// === File: logline_api/src/main.rs ===
/*
    Description: Servidor API integrado para o LogLine Motor.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

mod api;
mod error;
mod config;
mod metrics;
mod health;

use api::setup_routes;
use config::ApiConfig;
use streaming::EventBroker;
use plugin_manager::PluginManager;
use std::sync::Arc;
use log::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializa o logger
    setup_logging();
    
    // Carrega a configuração
    let config = config::load_config().expect("Falha ao carregar configuração");
    
    info!("Iniciando LogLine Motor API v{}", core::VERSION);
    info!("Configuração carregada: {:?}", config);
    
    // Inicializa o runtime do LogLine
    info!("Inicializando runtime...");
    runtime::init().await?;
    
    // Inicializa o broker de eventos
    info!("Inicializando broker de eventos...");
    let broker = Arc::new(streaming::init_streaming(Some(config.stream_buffer_size)));
    
    // Inicializa o gerenciador de plugins
    info!("Inicializando gerenciador de plugins...");
    let plugin_dir = std::path::PathBuf::from(&config.plugin_directory);
    let mut plugin_manager = PluginManager::new(&plugin_dir).await?;
    
    // Inicia o monitoramento de plugins se estiver habilitado
    if config.enable_plugins {
        info!("Iniciando monitoramento de plugins...");
        match plugin_manager.watch_plugins().await {
            Ok(_) => info!("Monitoramento de plugins iniciado"),
            Err(e) => warn!("Falha ao iniciar monitoramento de plugins: {}", e),
        }
    }
    
    // Inicializa métricas se habilitado
    if config.enable_metrics {
        info!("Inicializando sistema de métricas...");
        metrics::init_metrics(&config)?;
    }
    
    // Configura as rotas da API
    info!("Configurando rotas da API...");
    let routes = setup_routes(broker, Arc::new(plugin_manager), &config).await?;
    
    // Inicia o servidor HTTP
    let addr = config.bind_address.parse()?;
    info!("Iniciando servidor HTTP em {}...", addr);
    warp::serve(routes).run(addr).await;
    
    // Nunca deve chegar aqui, mas por precaução
    runtime::shutdown().await?;
    
    Ok(())
}

// Configura o logging
fn setup_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,logline_api=debug,runtime=debug");
    }
    env_logger::init();
}