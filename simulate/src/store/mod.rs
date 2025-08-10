// === File: simulate/src/store/mod.rs ===
/*
    Description: Wrapper de store que alterna entre backend in-memory e Supabase.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

#[cfg(feature = "mem-store")]
mod mem;
#[cfg(feature = "mem-store")]
use mem as backend;

#[cfg(not(feature = "mem-store"))]
mod db;
#[cfg(not(feature = "mem-store"))]
use db as backend;

use crate::models::SimulationResult;
use crate::error::SimulateError;

/// Salva um resultado de simulação utilizando o backend configurado.
pub async fn save_simulation_result(res: &SimulationResult) -> Result<(), SimulateError> {
    backend::save_simulation_result(res).await
}

/// Busca resultados de simulação para uma entidade específica.
/// O parâmetro `limit` define o número máximo de registros a retornar.
pub async fn get_results_by_entity(entity_id: &str, limit: usize) -> Result<Vec<SimulationResult>, SimulateError> {
    backend::get_results_by_entity(entity_id, limit).await
}

/// Remove todos os resultados de simulação de uma entidade específica.
pub async fn clear_results_by_entity(entity_id: &str) -> Result<(), SimulateError> {
    backend::clear_results_by_entity(entity_id).await
}

/// Lista todos os resultados de simulação armazenados.
#[allow(dead_code)]
pub async fn list_results() -> Vec<SimulationResult> {
    backend::list_results().await
}