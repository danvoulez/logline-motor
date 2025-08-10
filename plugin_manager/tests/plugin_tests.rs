// === File: plugin_manager/tests/plugin_tests.rs ===
/*
    Description: Testes para o gerenciador de plugins.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use plugin_manager::{PluginManager, PluginError};
use std::path::PathBuf;
use tokio::fs;
use anyhow::Result;

// Utilitário para criar um diretório de teste temporário
async fn setup_test_dir() -> Result<PathBuf> {
    let test_dir = PathBuf::from("./target/plugin_test");
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).await?;
    }
    fs::create_dir_all(&test_dir).await?;
    Ok(test_dir)
}

// Limpa o diretório de teste
async fn cleanup_test_dir(dir: &PathBuf) -> Result<()> {
    if dir.exists() {
        fs::remove_dir_all(dir).await?;
    }
    Ok(())
}

#[tokio::test]
async fn test_plugin_manager_initialization() -> Result<()> {
    let test_dir = setup_test_dir().await?;
    
    // Testa a inicialização do gerenciador de plugins
    let plugin_manager = PluginManager::new(&test_dir).await;
    assert!(plugin_manager.is_ok(), "O gerenciador de plugins deveria inicializar corretamente");
    
    cleanup_test_dir(&test_dir).await?;
    Ok(())
}

#[tokio::test]
async fn test_watch_plugins_directory() -> Result<()> {
    let test_dir = setup_test_dir().await?;
    
    // Inicializa o gerenciador de plugins
    let mut plugin_manager = PluginManager::new(&test_dir).await?;
    
    // Inicia o monitoramento
    let watch_result = plugin_manager.watch_plugins().await;
    assert!(watch_result.is_ok(), "O monitoramento deveria iniciar corretamente");
    
    cleanup_test_dir(&test_dir).await?;
    Ok(())
}

// Nota: Testes mais completos precisariam de arquivos WASM reais para testar carregamento e invocação
// Este é apenas um conjunto básico de testes de inicialização