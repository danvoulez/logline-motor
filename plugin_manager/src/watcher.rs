// === File: plugin_manager/src/watcher.rs ===
/*
    Description: Implementação de monitoramento de diretório para hot reload de plugins.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use crate::error::PluginError;
use notify::{Watcher, RecursiveMode, Event, EventKind, Config as NotifyConfig};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use futures::StreamExt;

/// Cria um watcher que monitora mudanças no diretório de plugins
pub async fn create_watcher<P: AsRef<Path>>(
    plugin_dir: P
) -> Result<Arc<RwLock<Box<dyn Watcher + Send + Sync>>>, PluginError> {
    let plugin_dir = plugin_dir.as_ref().to_path_buf();
    
    // Cria um canal para eventos
    let (tx, mut rx) = mpsc::channel(100);
    
    // Inicializa o watcher
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                if let Err(e) = tx.blocking_send(event) {
                    tracing::error!("Erro ao enviar evento de arquivo: {}", e);
                }
            }
            Err(e) => tracing::error!("Erro no watcher de arquivos: {}", e),
        }
    })
    .map_err(|e| PluginError::WatcherError(e.to_string()))?;
    
    // Começa a observar o diretório
    watcher.watch(&plugin_dir, RecursiveMode::Recursive)
        .map_err(|e| PluginError::WatcherError(e.to_string()))?;
    
    let watcher = Arc::new(RwLock::new(Box::new(watcher) as Box<dyn Watcher + Send + Sync>));
    let watcher_clone = watcher.clone();
    
    // Spawn de uma tarefa para processar eventos
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            process_watcher_event(event, &plugin_dir).await;
        }
    });
    
    tracing::info!("Watcher iniciado para o diretório {}", plugin_dir.display());
    Ok(watcher_clone)
}

/// Processa eventos do watcher de arquivos
async fn process_watcher_event(event: Event, plugin_dir: &PathBuf) {
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {
            for path in event.paths {
                if is_wasm_file(&path) {
                    tracing::info!("Arquivo WASM alterado/criado: {}", path.display());
                    // Em uma implementação completa, aqui você recarregaria o plugin
                    // Exemplo: plugin_manager.reload_plugin(&path).await
                }
            }
        }
        EventKind::Remove(_) => {
            for path in event.paths {
                if is_wasm_file(&path) {
                    if let Some(file_name) = path.file_stem() {
                        if let Some(name) = file_name.to_str() {
                            tracing::info!("Arquivo WASM removido: {} ({})", name, path.display());
                            // Em uma implementação completa, aqui você removeria o plugin do registro
                            // Exemplo: plugin_manager.unload_plugin(name).await
                        }
                    }
                }
            }
        }
        _ => {} // Ignore outros eventos
    }
}

/// Verifica se um arquivo é um módulo WebAssembly (.wasm)
fn is_wasm_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "wasm")
        .unwrap_or(false)
}