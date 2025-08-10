// === File: logline_api/src/config.rs ===
/*
    Description: Configuração para a API LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::env;

// Configuração da API
#[derive(Debug, Clone)]
pub struct ApiConfig {
    // Configurações de servidor
    pub bind_address: String,
    pub api_prefix: String,
    pub cors_allowed_origin: Option<String>,
    
    // Configurações de plugins
    pub enable_plugins: bool,
    pub plugin_directory: String,
    pub plugin_refresh_interval: u64,
    
    // Configurações de streaming
    pub stream_buffer_size: usize,
    
    // Configurações de métricas
    pub enable_metrics: bool,
    pub metrics_port: u16,
}

// Carrega a configuração do ambiente
pub fn load_config() -> Result<ApiConfig, String> {
    let bind_address = env::var("LOGLINE_BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    
    let api_prefix = env::var("LOGLINE_API_PREFIX")
        .unwrap_or_else(|_| "api".to_string());
    
    let cors_allowed_origin = env::var("LOGLINE_CORS_ALLOWED_ORIGIN").ok();
    
    let enable_plugins = env::var("LOGLINE_ENABLE_PLUGINS")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(true);
    
    let plugin_directory = env::var("LOGLINE_PLUGIN_DIRECTORY")
        .unwrap_or_else(|_| "./plugins".to_string());
    
    let plugin_refresh_interval = env::var("LOGLINE_PLUGIN_REFRESH_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30);
    
    let stream_buffer_size = env::var("LOGLINE_STREAM_BUFFER_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);
    
    let enable_metrics = env::var("LOGLINE_ENABLE_METRICS")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(true);
    
    let metrics_port = env::var("LOGLINE_METRICS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(9090);
    
    Ok(ApiConfig {
        bind_address,
        api_prefix,
        cors_allowed_origin,
        enable_plugins,
        plugin_directory,
        plugin_refresh_interval,
        stream_buffer_size,
        enable_metrics,
        metrics_port,
    })
}