// === File: llm_trainer/tests/llm_trainer_tests.rs ===
/*
    Description: Testes de integração para o crate llm_trainer.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use llm_trainer::*;
use datatrack::{record_span, Span};
use uuid::Uuid;
use chrono::Utc;
use std::fs;

#[tokio::test]
async fn test_train_llm_pipeline_runs_without_error() {
    // Este teste deve ser executado com o feature flag `mem-store` para
    // que os módulos subjacentes não dependam de um DB real.
    
    // 1. Preparar dados iniciais no datatrack
    let parent_id = Uuid::new_v4();
    record_span(Span {
        id: parent_id,
        parent: None,
        kind: "root".into(),
        payload: "start".into(),
        timestamp: Utc::now()
    }).await.unwrap();
    
    for _ in 0..5 {
        record_span(Span {
            id: Uuid::new_v4(),
            parent: Some(parent_id),
            kind: "child".into(),
            payload: "data".into(),
            timestamp: Utc::now()
        }).await.unwrap();
    }

    // 2. Executar o pipeline de treino
    let config = r#"{"threshold": 3, "index": "test_index"}"#;
    let result = train_llm("test-model", config).await;
    
    // 3. Verificar
    assert!(result.is_ok(), "O pipeline de treino não deve falhar.");
}

#[tokio::test]
async fn test_eval_llm_returns_stub_metrics() {
    let metrics = eval_llm("test-model", "validation-set").await.unwrap();
    assert_eq!(metrics.accuracy, 0.0);
    assert_eq!(metrics.loss, 0.0);
}

#[tokio::test]
async fn test_export_checkpoint_creates_file() {
    let dir = "./tmp_checkpoints_test";
    // Limpa o diretório de teste antes de começar
    let _ = fs::remove_dir_all(dir);
    
    export_checkpoint("my-test-model", dir).await.unwrap();
    
    let entries = fs::read_dir(dir).unwrap();
    assert_eq!(entries.count(), 1, "Deveria haver um arquivo de checkpoint no diretório.");
    
    // Limpa o diretório após o teste
    fs::remove_dir_all(dir).unwrap();
}