// === File: simulate/src/lib.rs ===
/*
    Description: Simulador de entidades com suporte avançado para o sistema LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

mod store;
mod engine;
mod scenarios;

// Expose the models and error modules to external crates.  Without these
// public declarations, `crate::models` and `crate::error` cannot be
// resolved in child modules such as `store::mod`.
pub mod models;
pub mod error;

use thiserror::Error;
// rand::Rng import removed because this file does not use random number generation directly
use registry::fetch_entity;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
// Removed unused SimulationStore import; storage functions are provided directly by store.rs
use uuid::Uuid;

pub use engine::{SimulationEngine, EngineConfig, SimulationMode};
pub use scenarios::{Scenario, ScenarioConfig, ScenarioOutcome};

/// Possíveis erros durante a simulação
#[derive(Error, Debug)]
pub enum SimulateError {
    #[error("Entidade {0} não encontrada")]
    EntityNotFound(String),
    
    #[error("Erro do registro: {0}")]
    RegistryError(String),
    
    #[error("Erro na simulação: {0}")]
    SimulationError(String),
    
    #[error("Cenário {0} não encontrado")]
    ScenarioNotFound(String),
    
    #[error("Erro de armazenamento: {0}")]
    StorageError(String),
    
    #[error("Erro de configuração: {0}")]
    ConfigError(String),
    
    #[error("Erro interno: {0}")]
    Internal(String),
}

/// Resultado de uma rodada de simulação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// ID único do resultado
    pub id: Uuid,
    
    /// ID da entidade simulada
    pub entity_id: String,
    
    /// Número da rodada de simulação
    pub round: usize,
    
    /// Timestamp da execução
    pub timestamp: DateTime<Utc>,
    
    /// Métricas coletadas durante a simulação
    pub metrics: HashMap<String, f64>,
    
    /// Eventos ocorridos durante a simulação
    pub events: Vec<String>,
    
    /// Status da simulação
    pub status: SimulationStatus,
}

/// Status possíveis de uma simulação
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimulationStatus {
    /// Simulação concluída com sucesso
    Success,
    
    /// Simulação falhou
    Failed,
    
    /// Simulação foi interrompida
    Interrupted,
    
    /// Simulação em progresso
    InProgress,
}

/// Configuração para uma simulação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Modo de simulação (determinístico, aleatório, etc)
    pub mode: SimulationMode,
    
    /// Número de rodadas
    pub rounds: usize,
    
    /// Semente aleatória (para simulações reproduzíveis)
    pub random_seed: Option<u64>,
    
    /// Métricas a coletar
    pub metrics: Vec<String>,
    
    /// Cenário a utilizar
    pub scenario: Option<String>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            mode: SimulationMode::Random,
            rounds: 10,
            random_seed: None,
            metrics: vec![
                "success_rate".to_string(),
                "processing_time".to_string(),
                "resource_usage".to_string(),
            ],
            scenario: None,
        }
    }
}

/// Executa uma simulação para uma entidade com configuração padrão.
///
/// # Parâmetros
///
/// * `entity_id` - ID da entidade a ser simulada
/// * `rounds` - Número de rodadas de simulação a executar
///
/// # Retorna
///
/// Um `Result` contendo os resultados da simulação ou um erro
pub async fn run_simulation(entity_id: &str, rounds: usize) -> Result<Vec<SimulationResult>, SimulateError> {
    let config = SimulationConfig {
        rounds,
        ..Default::default()
    };
    
    run_simulation_with_config(entity_id, config).await
}

/// Executa uma simulação para uma entidade com configuração customizada.
///
/// # Parâmetros
///
/// * `entity_id` - ID da entidade a ser simulada
/// * `config` - Configuração da simulação
///
/// # Retorna
///
/// Um `Result` contendo os resultados da simulação ou um erro
pub async fn run_simulation_with_config(
    entity_id: &str, 
    config: SimulationConfig
) -> Result<Vec<SimulationResult>, SimulateError> {
    // Verifica se a entidade existe no registro
    let (_id, entity_type) = fetch_entity(entity_id).await
        .map_err(|e| SimulateError::RegistryError(e.to_string()))?;
    
    // Configura o motor de simulação
    let engine_config = EngineConfig {
        random_seed: config.random_seed,
        mode: config.mode.clone(),
    };
    
    // Create a mutable simulation engine so that simulate_round can borrow &mut self
    let mut engine = SimulationEngine::new(engine_config);
    
    // Configura o cenário, se fornecido
    let scenario = if let Some(scenario_name) = &config.scenario {
        scenarios::load_scenario(scenario_name)
            .ok_or_else(|| SimulateError::ScenarioNotFound(scenario_name.clone()))?
    } else {
        // Escolhe um cenário padrão com base no tipo de entidade
        scenarios::default_scenario_for_type(&entity_type)
    };
    
    let mut results = Vec::with_capacity(config.rounds);
    
    // Executa as rodadas de simulação
    for round in 0..config.rounds {
        // Executa uma rodada de simulação
        let result = engine.simulate_round(entity_id, &scenario, round + 1).await?;
        
        // Armazena o resultado
        store::save_simulation_result(&result).await
            .map_err(|e| SimulateError::StorageError(e.to_string()))?;
        
        results.push(result);
        
        // Simula um pequeno delay entre rodadas
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    
    Ok(results)
}

/// Avalia a performance de uma simulação com base nos seus resultados.
///
/// # Parâmetros
///
/// * `results` - Resultados de uma simulação
///
/// # Retorna
///
/// Um score de performance entre 0.0 e 1.0
pub fn evaluate_simulation(results: &[SimulationResult]) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    
    let mut total_success_rate = 0.0;
    let mut count = 0;
    
    for result in results {
        if let Some(&success_rate) = result.metrics.get("success_rate") {
            total_success_rate += success_rate;
            count += 1;
        }
    }
    
    if count == 0 {
        return 0.0;
    }
    
    total_success_rate / count as f64
}

/// Busca resultados de simulação para uma entidade específica.
///
/// # Parâmetros
///
/// * `entity_id` - ID da entidade
/// * `limit` - Número máximo de resultados a retornar
///
/// # Retorna
///
/// Um `Result` contendo os resultados da simulação ou um erro
pub async fn get_simulation_results(entity_id: &str, limit: usize) -> Result<Vec<SimulationResult>, SimulateError> {
    store::get_results_by_entity(entity_id, limit).await
        .map_err(|e| SimulateError::StorageError(e.to_string()))
}

/// Limpa todos os resultados de simulação para uma entidade.
///
/// # Parâmetros
///
/// * `entity_id` - ID da entidade
///
/// # Retorna
///
/// Um `Result` indicando sucesso ou falha
pub async fn clear_simulation_results(entity_id: &str) -> Result<(), SimulateError> {
    store::clear_results_by_entity(entity_id).await
        .map_err(|e| SimulateError::StorageError(e.to_string()))
}