// === File: simulate/src/store/db.rs ===
/*
    Description: Implementação do store de simulações com backend Supabase/PostgREST.
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
// Import the core data types from the crate root.  These types are
// defined in `lib.rs` and re-exported via `crate::models` for
// backwards‑compatibility.
use crate::SimulationResult;
use crate::SimulateError;
use db::Db;

/// Singleton client instance.  A runtime handle is used to synchronously
/// initialize the database connection on first use.  Subsequent
/// operations reuse the same connection behind a mutex for
/// thread‑safety.
static DB: Lazy<Mutex<Db>> = Lazy::new(|| {
    let rt = tokio::runtime::Handle::current();
    let db = rt.block_on(async { Db::new().unwrap() });
    Mutex::new(db)
});

/// Persist a simulation result into the Supabase/PostgREST backend.  The
/// payload is constructed from the fields of `SimulationResult`,
/// converting complex types into serializable forms.
pub async fn save_simulation_result(res: &SimulationResult) -> Result<(), SimulateError> {
    let payload = serde_json::json!({
        "id":        res.id,
        "entity_id": res.entity_id,
        "round":     res.round,
        "timestamp": res.timestamp.to_rfc3339(),
        "metrics":   res.metrics,
        "events":    res.events,
        "status":    format!("{:?}", res.status),
    });
    let db = DB.lock().await;
    db.insert("simulations", &payload)
        .await
        .map_err(|e| SimulateError::Internal(e.to_string()))
}

/// Retrieve up to `limit` results for a given entity.  Results are
/// ordered by descending round number so that the most recent results
/// appear first.  The filter string uses the PostgREST syntax.
pub async fn get_results_by_entity(entity_id: &str, limit: usize) -> Result<Vec<SimulationResult>, SimulateError> {
    let db = DB.lock().await;
    let filter = format!("entity_id=eq.{}&order=round.desc&limit={}", entity_id, limit);
    let rows: Vec<SimulationResult> = db
        .select("simulations", Some(&filter))
        .await
        .map_err(|e| SimulateError::Internal(e.to_string()))?;
    Ok(rows)
}

/// Delete all results for a given entity.  The underlying `Db` API does
/// not currently expose a delete operation, so this function returns
/// `Ok(())` without performing any action.
pub async fn clear_results_by_entity(_entity_id: &str) -> Result<(), SimulateError> {
    // TODO: when delete support is added to `Db`, implement this.
    Ok(())
}

/// List all simulation results available in the persistent store.
#[allow(dead_code)]
pub async fn list_results() -> Vec<SimulationResult> {
    let db = DB.lock().await;
    db.select("simulations", None).await.unwrap_or_default()
}