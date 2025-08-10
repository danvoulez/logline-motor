// === File: diamondminer/tests/diamondminer_tests.rs ===
/*
    Description: Testes de unidade para o crate diamondminer.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use diamondminer::{mine_diamonds, error::DiamondError};
use datatrack::Span;
use uuid::Uuid;
use chrono::Utc;

// Helper para criar spans de teste de forma concisa.
fn make_span(id_suffix: u8, parent_suffix: Option<u8>) -> Span {
    let id = Uuid::from_u128(id_suffix as u128);
    let parent = parent_suffix.map(|p| Uuid::from_u128(p as u128));
    Span {
        id,
        parent,
        kind: "test_kind".into(),
        payload: String::new(),
        timestamp: Utc::now(),
    }
}

#[test]
fn test_empty_input_returns_error() {
    let result = mine_diamonds(&[], 1);
    assert!(matches!(result, Err(DiamondError::EmptyInput)));
}

#[test]
fn test_no_diamonds_found_when_below_threshold() {
    let spans = vec![
        make_span(1, None),      // Parent 1
        make_span(2, Some(1)),   // Child of 1
        make_span(3, Some(1)),   // Child of 1
    ];
    // Threshold é 3, mas o pai só tem 2 filhos.
    let diamonds = mine_diamonds(&spans, 3).unwrap();
    assert!(diamonds.is_empty());
}

#[test]
fn test_finds_single_diamond_span() {
    let spans = vec![
        make_span(1, None),      // Diamante
        make_span(2, Some(1)),
        make_span(3, Some(1)),
        make_span(4, Some(1)),
        make_span(5, None),      // Outro pai, mas com menos filhos
        make_span(6, Some(5)),
    ];
    // Threshold é 3, span 1 tem 3 filhos.
    let diamonds = mine_diamonds(&spans, 3).unwrap();
    assert_eq!(diamonds.len(), 1);
    assert_eq!(diamonds[0].id, Uuid::from_u128(1));
}

#[test]
fn test_finds_multiple_diamonds_and_ignores_others() {
    let spans = vec![
        make_span(1, None),      // Diamante 1 (2 filhos)
        make_span(10, Some(1)),
        make_span(11, Some(1)),
        make_span(2, None),      // Não é diamante (1 filho)
        make_span(20, Some(2)),
        make_span(3, None),      // Diamante 2 (3 filhos)
        make_span(30, Some(3)),
        make_span(31, Some(3)),
        make_span(32, Some(3)),
    ];
    // Threshold é 2. Apenas os spans 1 e 3 devem ser retornados.
    let mut diamonds = mine_diamonds(&spans, 2).unwrap();
    assert_eq!(diamonds.len(), 2);
    // Ordena para garantir consistência no teste
    diamonds.sort_by_key(|d| d.id);
    assert_eq!(diamonds[0].id, Uuid::from_u128(1));
    assert_eq!(diamonds[1].id, Uuid::from_u128(3));
}