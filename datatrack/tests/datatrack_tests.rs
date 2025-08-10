// === File: datatrack/tests/datatrack_tests.rs ===
/*
    Description: Testes de integração para o crate datatrack.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use datatrack::*;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_record_and_fetch() {
    // Este teste usará o backend configurado pelo feature flag `mem-store`
    // ou o backend de DB por padrão.

    let id = Uuid::new_v4();
    let span = Span {
        id,
        parent: None,
        kind: "test_kind".into(),
        payload: "hello world".into(),
        timestamp: Utc::now(),
    };
    record_span(span.clone()).await.unwrap();

    let filter = SpanFilter { 
        kind: Some("test_kind".into()), 
        limit: Some(1) 
    };
    let got = fetch_spans(filter).await.unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].id, span.id);
    assert_eq!(got[0].payload, span.payload);
}