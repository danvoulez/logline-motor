// === File: logline_api/src/error.rs ===
/*
    Description: Definições de erros para a API LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use serde::{Serialize, Deserialize};
use thiserror::Error;
use warp::reject::Reject;

// Erros da API
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Erro ao processar comando: {0}")]
    CommandProcessingError(String),
    
    #[error("Formato de comando inválido: {0}")]
    InvalidCommandFormat(String),
    
    #[error("Erro interno: {0}")]
    InternalError(String),
}

// Implementa Reject para ApiError
impl Reject for ApiError {}

// Modelo de resposta de erro
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub timestamp: String,
}