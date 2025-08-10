// === File: plugin_manager/src/plugin.rs ===
/*
    Description: Implementação de plugins e suas funcionalidades.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use crate::error::PluginError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use wasmer::{Store, Instance, WasmPtr, Val, Value, Memory};
use tokio::sync::RwLock;

/// Metadados de um plugin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Nome do plugin
    pub name: String,
    
    /// Versão do plugin
    pub version: String,
    
    /// Autor do plugin
    pub author: String,
    
    /// Descrição do plugin
    pub description: String,
    
    /// Hooks disponíveis no plugin
    pub hooks: Vec<String>,
}

/// Tipos de hooks que podem ser implementados por plugins
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HookType {
    /// Hook executado antes do processamento de um comando
    PreCommand,
    
    /// Hook executado após o processamento de um comando
    PostCommand,
    
    /// Hook executado antes da validação de uma entidade
    PreValidation,
    
    /// Hook executado após a validação de uma entidade
    PostValidation,
    
    /// Hook executado durante simulações
    Simulation,
    
    /// Hook customizado
    Custom(String),
}

impl HookType {
    /// Converte uma string em um tipo de hook
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "precommand" => Self::PreCommand,
            "postcommand" => Self::PostCommand,
            "prevalidation" => Self::PreValidation,
            "postvalidation" => Self::PostValidation,
            "simulation" => Self::Simulation,
            _ => Self::Custom(s.to_string()),
        }
    }
    
    /// Converte um tipo de hook em string
    pub fn as_str(&self) -> String {
        match self {
            Self::PreCommand => "precommand".to_string(),
            Self::PostCommand => "postcommand".to_string(),
            Self::PreValidation => "prevalidation".to_string(),
            Self::PostValidation => "postvalidation".to_string(),
            Self::Simulation => "simulation".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }
}

/// Um plugin carregado
pub struct Plugin {
    /// Metadados do plugin
    pub metadata: PluginMetadata,
    
    /// ID único do plugin (nome do arquivo)
    pub id: String,
    
    /// Funções exportadas pelo plugin
    functions: HashMap<String, String>,
}

/// Extrai os metadados de um plugin a partir de sua instância WebAssembly
pub async fn load_plugin_metadata(
    store: &mut Store,
    instance: &Instance,
    id: String,
) -> Result<Plugin, PluginError> {
    // Tenta obter a função de metadados do plugin
    let get_metadata = instance
        .exports
        .get_function("get_metadata")
        .map_err(|_| PluginError::FunctionNotFound("get_metadata".to_string()))?;
    
    // Executa a função para obter os metadados
    let result = get_metadata.call(store, &[])
        .map_err(|e| PluginError::WasmExecutionError(e.to_string()))?;
    
    // O primeiro resultado deve ser um ponteiro para a string de metadados
    let ptr = if let Some(Value::I32(ptr)) = result.get(0) {
        *ptr as u32
    } else {
        return Err(PluginError::MetadataError("Retorno de metadados inválido".to_string()));
    };
    
    // O segundo resultado deve ser o tamanho da string
    let len = if let Some(Value::I32(len)) = result.get(1) {
        *len as u32
    } else {
        return Err(PluginError::MetadataError("Retorno de metadados inválido".to_string()));
    };
    
    // Obtém a memória do módulo
    let memory = instance
        .exports
        .get_memory("memory")
        .map_err(|_| PluginError::MetadataError("Memória não encontrada".to_string()))?;
    
    // Lê os bytes da memória
    let buffer = unsafe {
        let data = memory.data_ptr().add(ptr as usize);
        std::slice::from_raw_parts(data, len as usize)
    };
    
    // Converte os bytes para string e depois para objeto de metadados
    let metadata_str = std::str::from_utf8(buffer)
        .map_err(|e| PluginError::SerializationError(e.to_string()))?;
    
    let metadata: PluginMetadata = serde_json::from_str(metadata_str)
        .map_err(|e| PluginError::SerializationError(e.to_string()))?;
    
    // Mapeie as funções exportadas
    let mut functions = HashMap::new();
    for hook in &metadata.hooks {
        // Verifica se o hook existe como uma função exportada
        let hook_fn_name = format!("hook_{}", hook);
        if instance.exports.get_function(&hook_fn_name).is_ok() {
            functions.insert(hook.clone(), hook_fn_name);
        }
    }
    
    Ok(Plugin {
        metadata,
        id,
        functions,
    })
}

/// Invoca uma função de um plugin com o payload fornecido
pub async fn invoke_plugin_function(
    store_lock: &RwLock<Store>,
    plugin: &Plugin,
    hook: &str,
    payload: &str,
) -> Result<String, PluginError> {
    // Verificar se o plugin tem a função para o hook solicitado
    let function_name = plugin.functions.get(hook)
        .ok_or_else(|| PluginError::FunctionNotFound(format!("{} em {}", hook, plugin.id)))?;
    
    // Serializa o payload para JSON
    let payload_json = payload.to_string();
    
    // Adquire acesso à store
    let mut store = store_lock.write().await;
    
    // Obter referência à instância (reimplementar em código real com Store adequado)
    // Nota: Esta é uma simplificação; em um código real, você recriaria/obteria a instância
    let instance = create_or_get_instance(&mut store, &plugin.id)
        .map_err(|e| PluginError::Internal(e.to_string()))?;
    
    // Aloca memória para o payload no módulo WASM
    let alloc = instance.exports.get_function("alloc")
        .map_err(|_| PluginError::FunctionNotFound("alloc".to_string()))?;
    
    let payload_len = payload_json.len() as i32;
    let alloc_result = alloc.call(&mut store, &[Value::I32(payload_len)])
        .map_err(|e| PluginError::WasmExecutionError(e.to_string()))?;
    
    let ptr = if let Some(Value::I32(ptr)) = alloc_result.get(0) {
        *ptr as u32
    } else {
        return Err(PluginError::WasmExecutionError("Falha na alocação de memória".to_string()));
    };
    
    // Obtém a memória do módulo
    let memory = instance.exports.get_memory("memory")
        .map_err(|_| PluginError::WasmExecutionError("Memória não encontrada".to_string()))?;
    
    // Escreve o payload na memória
    unsafe {
        let data = memory.data_ptr().add(ptr as usize);
        std::ptr::copy_nonoverlapping(
            payload_json.as_ptr(),
            data,
            payload_json.len(),
        );
    }
    
    // Invoca a função do hook com o ponteiro para o payload
    let hook_fn = instance.exports.get_function(function_name)
        .map_err(|_| PluginError::FunctionNotFound(function_name.clone()))?;
    
    let result = hook_fn.call(&mut store, &[Value::I32(ptr as i32), Value::I32(payload_len)])
        .map_err(|e| PluginError::WasmExecutionError(e.to_string()))?;
    
    // O primeiro resultado deve ser um ponteiro para a string de resultado
    let result_ptr = if let Some(Value::I32(ptr)) = result.get(0) {
        *ptr as u32
    } else {
        return Err(PluginError::WasmExecutionError("Retorno de função inválido".to_string()));
    };
    
    // O segundo resultado deve ser o tamanho da string
    let result_len = if let Some(Value::I32(len)) = result.get(1) {
        *len as u32
    } else {
        return Err(PluginError::WasmExecutionError("Retorno de função inválido".to_string()));
    };
    
    // Lê o resultado da memória
    let result_buffer = unsafe {
        let data = memory.data_ptr().add(result_ptr as usize);
        std::slice::from_raw_parts(data, result_len as usize)
    };
    
    let result_str = std::str::from_utf8(result_buffer)
        .map_err(|e| PluginError::SerializationError(e.to_string()))?
        .to_string();
    
    // Libera a memória alocada (se o plugin tiver essa função)
    if let Ok(dealloc) = instance.exports.get_function("dealloc") {
        let _ = dealloc.call(&mut store, &[Value::I32(ptr as i32), Value::I32(payload_len)]);
        let _ = dealloc.call(&mut store, &[Value::I32(result_ptr as i32), Value::I32(result_len)]);
    }
    
    Ok(result_str)
}

// Esta função é uma simplificação; em um código real você manteria um cache de instâncias
fn create_or_get_instance(store: &mut Store, id: &str) -> Result<Instance, String> {
    // Em uma implementação real, você armazenaria e recuperaria instâncias de um cache
    Err("Not implemented in this example".to_string())
}