// === File: contracts/src/lib.rs ===
/*
    Description: Implementação da API pública para gestão de contratos LogLine.
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

// Definição da estrutura de contratos
pub struct Contract {
    pub id: String,
    pub clauses: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Um simples store em memória para os contratos
// Em uma implementação real, isso seria persistido em um banco de dados
static CONTRACT_STORE: Lazy<Mutex<HashMap<String, Contract>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Cria um novo contrato com as cláusulas especificadas.
///
/// # Parâmetros
///
/// * `id` - Identificador único do contrato
/// * `clauses` - Lista de cláusulas do contrato
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na criação do contrato.
pub fn create_contract(id: &str, clauses: &[String]) -> Result<(), Box<dyn Error>> {
    let now = Utc::now();
    
    let contract = Contract {
        id: id.to_string(),
        clauses: clauses.to_vec(),
        created_at: now,
        updated_at: now,
    };
    
    let mut store = CONTRACT_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de contratos")?;
    
    if store.contains_key(id) {
        return Err(format!("Contrato com ID '{}' já existe", id).into());
    }
    
    store.insert(id.to_string(), contract);
    Ok(())
}

/// Busca um contrato pelo seu ID.
///
/// # Parâmetros
///
/// * `id` - Identificador do contrato
///
/// # Retorna
///
/// Um `Result` contendo informações do contrato ou erro caso não seja encontrado.
pub fn get_contract(id: &str) -> Result<(String, Vec<String>, DateTime<Utc>), Box<dyn Error>> {
    let store = CONTRACT_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de contratos")?;
    
    if let Some(contract) = store.get(id) {
        Ok((contract.id.clone(), contract.clauses.clone(), contract.created_at))
    } else {
        Err(format!("Contrato com ID '{}' não encontrado", id).into())
    }
}

/// Lista todos os contratos disponíveis.
///
/// # Retorna
///
/// Um `Result` contendo os IDs de todos os contratos ou erro.
pub fn list_contracts() -> Result<Vec<String>, Box<dyn Error>> {
    let store = CONTRACT_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de contratos")?;
    
    Ok(store.keys().cloned().collect())
}

/// Atualiza as cláusulas de um contrato existente.
///
/// # Parâmetros
///
/// * `id` - Identificador do contrato
/// * `clauses` - Novas cláusulas para o contrato
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha na atualização.
pub fn update_contract(id: &str, clauses: &[String]) -> Result<(), Box<dyn Error>> {
    let mut store = CONTRACT_STORE.lock().map_err(|_| "Falha ao obter acesso ao armazenamento de contratos")?;
    
    if let Some(contract) = store.get_mut(id) {
        contract.clauses = clauses.to_vec();
        contract.updated_at = Utc::now();
        Ok(())
    } else {
        Err(format!("Contrato com ID '{}' não encontrado", id).into())
    }
}