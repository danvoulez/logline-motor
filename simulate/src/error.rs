// === File: simulate/src/error.rs ===
/*
    Description: Enum de erros customizados para o crate simulate.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

// Re-export the error type defined in the crate root.  This module is
// retained solely for backwards compatibility.  The primary
// `SimulateError` enum lives in `lib.rs`.

pub use crate::SimulateError;