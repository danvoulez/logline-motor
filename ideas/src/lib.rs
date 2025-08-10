// === File: ideas/src/lib.rs ===
/*
    Description: Biblioteca para gestão e manipulação de ideias LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::collections::HashMap;
use std::sync::Mutex;
use std::error::Error;
use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

// Definição da estrutura de ideia
pub struct Idea {
    pub id: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

// Um simples store em memória para as ideias
// Em uma implementação real, isso seria persistido em um banco de dados
static IDEA_STORE: Lazy<Mutex<HashMap<String, Idea>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Cria uma nova ideia.
///
/// # Parâmetros
///
/// * `id` - Identificador único da ideia
/// * `text` - Conteúdo textual da ideia
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na criação da ideia.
pub fn create_idea(id: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let now = Utc::now();
    
    let idea = Idea {
        id: id.to_string(),
        text: text.to_string(),
        created_at: now,
        updated_at: now,
        tags: Vec::new(),
    };
    
    let mut store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    if store.contains_key(id) {
        return Err(format!("Ideia com ID '{}' já existe", id).into());
    }
    
    store.insert(id.to_string(), idea);
    Ok(())
}

/// Busca uma ideia pelo seu ID.
///
/// # Parâmetros
///
/// * `id` - Identificador da ideia
///
/// # Retorna
///
/// Um `Result` contendo informações da ideia ou erro caso não seja encontrada.
pub fn get_idea(id: &str) -> Result<(String, String, DateTime<Utc>), Box<dyn Error>> {
    let store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    if let Some(idea) = store.get(id) {
        Ok((idea.id.clone(), idea.text.clone(), idea.created_at))
    } else {
        Err(format!("Ideia com ID '{}' não encontrada", id).into())
    }
}

/// Lista todas as ideias disponíveis.
///
/// # Retorna
///
/// Um `Result` contendo os IDs de todas as ideias ou erro.
pub fn list_ideas() -> Result<Vec<String>, Box<dyn Error>> {
    let store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    Ok(store.keys().cloned().collect())
}

/// Atualiza o texto de uma ideia existente.
///
/// # Parâmetros
///
/// * `id` - Identificador da ideia
/// * `text` - Novo texto para a ideia
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na atualização.
pub fn update_idea(id: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let mut store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    if let Some(idea) = store.get_mut(id) {
        idea.text = text.to_string();
        idea.updated_at = Utc::now();
        Ok(())
    } else {
        Err(format!("Ideia com ID '{}' não encontrada", id).into())
    }
}

/// Adiciona tags a uma ideia existente.
///
/// # Parâmetros
///
/// * `id` - Identificador da ideia
/// * `tags` - Lista de tags a serem adicionadas
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na operação.
pub fn add_tags(id: &str, tags: &[String]) -> Result<(), Box<dyn Error>> {
    let mut store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    if let Some(idea) = store.get_mut(id) {
        for tag in tags {
            if !idea.tags.contains(tag) {
                idea.tags.push(tag.clone());
            }
        }
        idea.updated_at = Utc::now();
        Ok(())
    } else {
        Err(format!("Ideia com ID '{}' não encontrada", id).into())
    }
}

/// Remove uma ideia pelo seu ID.
///
/// # Parâmetros
///
/// * `id` - Identificador da ideia a ser removida
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na remoção.
pub fn remove_idea(id: &str) -> Result<(), Box<dyn Error>> {
    let mut store = IDEA_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de ideias")?;
    
    if store.remove(id).is_some() {
        Ok(())
    } else {
        Err(format!("Ideia com ID '{}' não encontrada", id).into())
    }
}