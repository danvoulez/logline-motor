// === File: simulate/src/store/mem.rs ===
/*
    Description: Implementação do store de simulações em memória.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::collections::HashMap;
// Import the core data types from the crate root.  These types are
// defined in `lib.rs` and re-exported via `crate::models` for
// backwards‑compatibility.
use crate::SimulationResult;
use crate::SimulateError;

// In-memory store keyed by entity_id. Each entity_id maps to a list of
// `SimulationResult` objects.  We use a `Lazy` mutex-protected map
// so that the store is initialized on first use and is safe to access
// across async contexts.
static MEM: Lazy<Mutex<HashMap<String, Vec<SimulationResult>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Save a simulation result into the in-memory store.  Results are
/// grouped by `entity_id` so that they can be queried efficiently per
/// entity.  The result is cloned because the store owns its own
/// collection of results.
pub async fn save_simulation_result(res: &SimulationResult) -> Result<(), SimulateError> {
    let mut map = MEM.lock().await;
    map.entry(res.entity_id.clone()).or_default().push(res.clone());
    Ok(())
}

/// Retrieve up to `limit` simulation results for the given entity.  If the
/// number of stored results exceeds `limit`, only the most recent
/// `limit` results are returned.  Returns an `EntityNotFound` error if
/// there are no entries for the given entity.
pub async fn get_results_by_entity(entity_id: &str, limit: usize) -> Result<Vec<SimulationResult>, SimulateError> {
    let map = MEM.lock().await;
    if let Some(results) = map.get(entity_id) {
        let len = results.len();
        let start = if len > limit { len - limit } else { 0 };
        Ok(results[start..].to_vec())
    } else {
        Err(SimulateError::EntityNotFound(entity_id.to_string()))
    }
}

/// Remove all results for a given entity.  Always returns `Ok(())` even
/// if the entity was not present.
pub async fn clear_results_by_entity(entity_id: &str) -> Result<(), SimulateError> {
    let mut map = MEM.lock().await;
    map.remove(entity_id);
    Ok(())
}

/// List all simulation results across all entities.
#[allow(dead_code)]
pub async fn list_results() -> Vec<SimulationResult> {
    let map = MEM.lock().await;
    map.values().flat_map(|v| v.clone()).collect()
}