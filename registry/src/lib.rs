// === File: registry/src/lib.rs ===
/*
    Description: Registro central de entidades com suporte a diferentes tipos de dados para o LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::collections::HashMap;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Estrutura de dados para as entidades registradas
struct Entity {
    /// Identificador lógico da entidade (informado pelo usuário)
    logical_id: String,
    /// Tipo da entidade (CONTRACT, IDEA, etc.)
    entity_type: String,
    /// Quando a entidade foi registrada
    created_at: DateTime<Utc>,
}

/// Armazena entidades em memória.
/// Em uma implementação de produção, esse estado estaria em um banco de dados.
static REGISTRY: Lazy<RwLock<HashMap<String, Entity>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Registra uma nova entidade.
/// 
/// # Parâmetros
/// 
/// * `logical_id` - ID lógico da entidade (informado pelo usuário)
/// * `entity_type` - Tipo da entidade (ex: CONTRACT, IDEA)
/// 
/// # Retorna
/// 
/// O ID lógico da entidade se o registro for bem-sucedido, ou um erro se falhar.
pub async fn register_entity(logical_id: &str, entity_type: &str) -> Result<String, Box<dyn Error>> {
    let entity = Entity {
        logical_id: logical_id.to_string(),
        entity_type: entity_type.to_string(),
        created_at: Utc::now(),
    };
    
    let mut registry = REGISTRY.write().await;
    registry.insert(logical_id.to_string(), entity);
    
    Ok(logical_id.to_string())
}

/// Busca uma entidade pelo seu ID.
/// 
/// # Parâmetros
/// 
/// * `id` - ID lógico da entidade a buscar
/// 
/// # Retorna
/// 
/// Um tuple (logical_id, entity_type) se a entidade for encontrada, ou um erro se não for.
pub async fn fetch_entity(id: &str) -> Result<(String, String), Box<dyn Error>> {
    let registry = REGISTRY.read().await;
    
    match registry.get(id) {
        Some(entity) => Ok((entity.logical_id.clone(), entity.entity_type.clone())),
        None => Err(format!("Entidade não encontrada: {}", id).into()),
    }
}

/// Lista todas as entidades do tipo especificado.
/// 
/// # Parâmetros
/// 
/// * `entity_type` - Tipo de entidade a filtrar
/// 
/// # Retorna
/// 
/// Um vetor de IDs lógicos das entidades do tipo especificado.
pub async fn list_entities_by_type(entity_type: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let registry = REGISTRY.read().await;
    
    let ids: Vec<String> = registry
        .iter()
        .filter(|(_, entity)| entity.entity_type == entity_type)
        .map(|(id, _)| id.clone())
        .collect();
    
    Ok(ids)
}

/// Remove uma entidade do registro.
/// 
/// # Parâmetros
/// 
/// * `id` - ID lógico da entidade a remover
/// 
/// # Retorna
/// 
/// `true` se a entidade foi removida, `false` se não foi encontrada.
pub async fn remove_entity(id: &str) -> Result<bool, Box<dyn Error>> {
    let mut registry = REGISTRY.write().await;
    
    Ok(registry.remove(id).is_some())
}