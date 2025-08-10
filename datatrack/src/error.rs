// === File: datatrack/src/error.rs ===
/*
    Description: Enum de erros customizados para o crate datatrack.
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
pub enum DataTrackError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UUID parse error: {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("DB error: {0}")]
    Db(#[from] db::DbError),

    #[error("Internal error: {0}")]
    Other(String),
}