// === File: datatrack/src/store/mem.rs ===
/*
    Description: Implementação do store de spans em memória.
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
use std::collections::VecDeque;
use crate::models::{Span, SpanFilter};
use crate::error::DataTrackError;

static MEM_SPANS: Lazy<Mutex<VecDeque<Span>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

pub async fn record_span(span: Span) -> Result<(), DataTrackError> {
    let mut q = MEM_SPANS.lock().await;
    q.push_back(span);
    Ok(())
}

pub async fn fetch_spans(filter: SpanFilter) -> Result<Vec<Span>, DataTrackError> {
    let q = MEM_SPANS.lock().await;
    let mut results: Vec<Span> = q.iter()
        .filter(|s| filter.kind.as_ref().map_or(true, |k| &s.kind == k))
        .cloned()
        .collect();

    if let Some(limit) = filter.limit {
        results.truncate(limit);
    }
    
    Ok(results)
}