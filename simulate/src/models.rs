// === File: simulate/src/models.rs ===
/*
    Description: Modelo de dados para o resultado de uma simulação.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

// Re-export the core types defined in the `simulate` crate to avoid
// duplicating model definitions. The primary definitions live in
// `lib.rs`. Any module that needs these types can import them from
// `crate::models` to maintain a stable public API.

pub use crate::{
    SimulationResult,
    SimulationStatus,
    SimulationConfig,
    SimulateError,
};