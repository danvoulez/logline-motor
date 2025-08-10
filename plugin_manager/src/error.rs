// === File: plugin_manager/src/error.rs ===
/*
    Description: Definições de erros do gerenciador de plugins.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin não encontrado: {0}")]
    PluginNotFound(String),
    
    #[error("Função do plugin não encontrada: {0}")]
    FunctionNotFound(String),
    
    #[error("Erro ao compilar WASM: {0}")]
    WasmCompileError(String),
    
    #[error("Erro de importação WASM: {0}")]
    WasmImportError(String),
    
    #[error("Erro ao instanciar WASM: {0}")]
    WasmInstantiationError(String),
    
    #[error("Erro ao executar função WASM: {0}")]
    WasmExecutionError(String),
    
    #[error("Erro de configuração WASI: {0}")]
    WasiError(String),
    
    #[error("Erro ao extrair metadados do plugin: {0}")]
    MetadataError(String),
    
    #[error("Formato inválido do arquivo de plugin: {0}")]
    InvalidPluginFile(String),
    
    #[error("Erro de serialização/deserialização: {0}")]
    SerializationError(String),
    
    #[error("Erro ao observar mudanças no diretório de plugins: {0}")]
    WatcherError(String),
    
    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Erro interno do gerenciador de plugins: {0}")]
    Internal(String),
}