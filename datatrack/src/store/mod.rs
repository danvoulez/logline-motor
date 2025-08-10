// === File: datatrack/src/store/mod.rs ===
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

use crate::models::{Span, SpanFilter};
use crate::error::DataTrackError;

/// Grava um span no backend configurado.
pub async fn record_span(span: Span) -> Result<(), DataTrackError> {
    backend::record_span(span).await
}

/// Consulta spans segundo um filtro no backend configurado.
pub async fn fetch_spans(filter: SpanFilter) -> Result<Vec<Span>, DataTrackError> {
    backend::fetch_spans(filter).await
}