// === File: datatrack/src/store/db.rs ===
/*
    Description: Implementação do store de spans com backend Supabase/PostgREST.
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
use serde_json::json;
use uuid::Uuid;
use crate::models::{Span, SpanFilter};
use crate::error::DataTrackError;
use db::Db;
use chrono::Utc;

static DB: Lazy<Mutex<Db>> = Lazy::new(|| {
    let rt = tokio::runtime::Handle::current();
    let db = rt.block_on(async { Db::new().unwrap() });
    Mutex::new(db)
});

pub async fn record_span(span: Span) -> Result<(), DataTrackError> {
    let payload = json!({
        "id":        span.id,
        "parent":    span.parent,
        "kind":      span.kind,
        "payload":   span.payload,
        "timestamp": span.timestamp.to_rfc3339(),
    });
    let db = DB.lock().await;
    db.insert("spans", &payload).await?;
    Ok(())
}

pub async fn fetch_spans(filter: SpanFilter) -> Result<Vec<Span>, DataTrackError> {
    let mut conds = Vec::new();
    if let Some(kind) = filter.kind {
        conds.push(format!("kind=eq.{}", kind));
    }
    let predicate = if conds.is_empty() { None } else { Some(conds.join("&").as_str()) };

    let db = DB.lock().await;
    // Note: The generic select in the db crate might not handle limits directly.
    // This implementation fetches all and then truncates.
    // A production version might add limit support to the db::select method.
    let rows: Vec<Span> = db.select("spans", predicate).await?;
    
    let mut results = rows;
    if let Some(limit) = filter.limit {
        results.truncate(limit);
    }
    
    Ok(results)
}