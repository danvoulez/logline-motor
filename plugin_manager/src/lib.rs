// === File: plugin_manager/src/lib.rs ===
/*
    Description: Engine de plugins com suporte a WebAssembly para o LogLine Motor.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

mod error;
mod watcher;
mod plugin;

pub use error::PluginError;
pub use plugin::{Plugin, PluginMetadata, HookType};

use dashmap::DashMap;
use notify::{Watcher, RecursiveMode};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use wasmer::{Store, Module, Instance, Value, Function, FunctionEnv, Environment};
use wasmer_wasi::WasiState;
use anyhow::Result;

/// Armazenamento global de plugins carregados
static PLUGINS: Lazy<DashMap<String, Arc<Plugin>>> = Lazy::new(|| DashMap::new());

/// Estado do gerenciador de plugins
pub struct PluginManager {
    /// Diretório de plugins a monitorar
    plugin_dir: PathBuf,
    /// Store Wasmer para módulos WebAssembly
    store: RwLock<Store>,
    /// Watcher de arquivos para hot reload
    watcher: Option<Arc<RwLock<Box<dyn Watcher + Send + Sync>>>>,
}

impl PluginManager {
    /// Cria uma nova instância do gerenciador de plugins
    pub async fn new<P: AsRef<Path>>(plugin_dir: P) -> Result<Self, PluginError> {
        let plugin_dir = plugin_dir.as_ref().to_path_buf();
        
        // Cria o diretório de plugins se não existir
        if !plugin_dir.exists() {
            tokio::fs::create_dir_all(&plugin_dir).await?;
        }
        
        // Inicializa o store Wasmer
        let store = RwLock::new(Store::default());
        
        Ok(Self {
            plugin_dir,
            store,
            watcher: None,
        })
    }
    
    /// Inicia o monitoramento do diretório de plugins para atualizações
    pub async fn watch_plugins(&mut self) -> Result<(), PluginError> {
        let watcher = watcher::create_watcher(self.plugin_dir.clone()).await?;
        self.watcher = Some(watcher);
        
        // Carrega os plugins iniciais
        self.reload_all_plugins().await?;
        
        Ok(())
    }
    
    /// Recarrega todos os plugins do diretório
    pub async fn reload_all_plugins(&self) -> Result<usize, PluginError> {
        let mut loaded = 0;
        
        let mut entries = tokio::fs::read_dir(&self.plugin_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
                if let Ok(_) = self.load_plugin(&path).await {
                    loaded += 1;
                }
            }
        }
        
        tracing::info!("Carregados {} plugins do diretório {}", loaded, self.plugin_dir.display());
        Ok(loaded)
    }
    
    /// Carrega um único plugin a partir de um arquivo
    pub async fn load_plugin<P: AsRef<Path>>(&self, path: P) -> Result<Arc<Plugin>, PluginError> {
        let path = path.as_ref();
        let file_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginError::InvalidPluginFile(path.to_string_lossy().to_string()))?;
        
        let plugin_bytes = tokio::fs::read(path).await?;
        
        // Compila o módulo WebAssembly
        let mut store = self.store.write().await;
        let module = Module::new(&store, plugin_bytes)
            .map_err(|e| PluginError::WasmCompileError(e.to_string()))?;
        
        // Configura o ambiente WASI (WebAssembly System Interface)
        let wasi_env = WasiState::new("logline-plugin")
            .env("LOGLINE_VERSION", "1.0.0")
            .finalize()
            .map_err(|e| PluginError::WasiError(e.to_string()))?;
        
        let import_object = wasi_env.import_object(&mut store, &module)
            .map_err(|e| PluginError::WasmImportError(e.to_string()))?;
        
        // Instancia o módulo
        let instance = Instance::new(&mut store, &module, &import_object)
            .map_err(|e| PluginError::WasmInstantiationError(e.to_string()))?;
        
        // Extrai metadados do plugin
        let plugin = plugin::load_plugin_metadata(&mut store, &instance, file_name.to_string())
            .await
            .map_err(|e| PluginError::MetadataError(e.to_string()))?;
        
        let plugin = Arc::new(plugin);
        
        // Armazena o plugin no mapa global
        PLUGINS.insert(file_name.to_string(), plugin.clone());
        
        tracing::info!("Plugin carregado: {} ({})", plugin.metadata.name, plugin.metadata.version);
        Ok(plugin)
    }
    
    /// Invoca uma função de um plugin com os parâmetros fornecidos
    pub async fn invoke(&self, plugin_name: &str, function: &str, payload: &str) -> Result<String, PluginError> {
        let plugin = PLUGINS.get(plugin_name)
            .ok_or_else(|| PluginError::PluginNotFound(plugin_name.to_string()))?;
        
        let result = plugin::invoke_plugin_function(
            &self.store,
            &plugin, 
            function, 
            payload
        ).await?;
        
        Ok(result)
    }
    
    /// Lista todos os plugins carregados
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        PLUGINS.iter()
            .map(|entry| entry.value().metadata.clone())
            .collect()
    }
}

// Implementação de Drop para limpar recursos
impl Drop for PluginManager {
    fn drop(&mut self) {
        if let Some(_watcher) = &self.watcher {
            // A implementação atual é suficiente já que Arc e RwLock serão liberados automaticamente.
            // Utilizamos `info!` em vez de `debug!` para que esta mensagem apareça apenas em níveis de log
            // mais altos, evitando poluir a saída em ambientes de produção.
            tracing::info!("Finalizando watcher de plugins");
        }
    }
}