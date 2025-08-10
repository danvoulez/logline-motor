// === File: diamondminer/src/error.rs ===
/*
    Description: Enum de erros customizados para o crate diamondminer.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiamondError {
    #[error("A lista de spans para mineração não pode estar vazia.")]
    EmptyInput,
}