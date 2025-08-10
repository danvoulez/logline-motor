// === File: llm_trainer/src/error.rs ===
/*
    Description: Enum de erros customizados para o crate llm_trainer.
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
pub enum LLMTrainerError {
    #[error("Erro ao deserializar a configuração de treino: {0}")]
    ConfigParse(#[from] serde_json::Error),

    #[error("Erro no datatrack: {0}")]
    DataTrack(#[from] datatrack::DataTrackError),

    #[error("Erro na mineração de diamantes: {0}")]
    DiamondError(#[from] diamondminer::DiamondError),

    #[error("Erro na indexação de vetores: {0}")]
    VectorError(#[from] vectorindex::VectorIndexError),

    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Erro interno do trainer: {0}")]
    Other(String),
}