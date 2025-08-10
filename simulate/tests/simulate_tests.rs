// === File: simulate/tests/simulate_tests.rs ===
/*
    Description: Testes para o simulador de entidades LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use simulate::{run_simulation, evaluate_simulation};
use registry::register_entity;

#[tokio::test]
async fn test_simulation_runs_with_valid_entity() {
    // Registra uma entidade de teste
    let entity_id = "sim-test-entity";
    let _ = register_entity(entity_id, "TEST").await;
    
    // Executa a simulação
    let rounds = 5;
    let result = run_simulation(entity_id, rounds).await;
    
    assert!(result.is_ok(), "A simulação falhou com uma entidade válida");
    
    if let Ok(sim_results) = result {
        // Verifica se todas as rodadas foram executadas
        assert_eq!(sim_results.len(), rounds, "Número incorreto de rodadas");
        
        // Verifica se cada rodada tem métricas e eventos
        for (i, round_result) in sim_results.iter().enumerate() {
            assert_eq!(round_result.round, i + 1, "Número da rodada incorreto");
            assert!(!round_result.metrics.is_empty(), "As métricas não deveriam estar vazias");
            
            // Verifica métricas comuns
            assert!(round_result.metrics.contains_key("success_rate"), 
                    "A métrica success_rate deveria estar presente");
            assert!(round_result.metrics.contains_key("processing_time"), 
                    "A métrica processing_time deveria estar presente");
        }
        
        // Avalia a simulação
        let score = evaluate_simulation(&sim_results);
        assert!(score >= 0.0 && score <= 1.0, "O score de avaliação deve estar entre 0 e 1");
    }
}

#[tokio::test]
async fn test_simulation_fails_with_invalid_entity() {
    // Tenta simular uma entidade que não existe
    let entity_id = "non-existent-entity";
    let rounds = 3;
    let result = run_simulation(entity_id, rounds).await;
    
    assert!(result.is_err(), "A simulação deveria falhar com uma entidade inválida");
}

#[test]
fn test_evaluate_simulation_with_empty_results() {
    let results = vec![];
    let score = evaluate_simulation(&results);
    
    assert_eq!(score, 0.0, "Avaliação de resultados vazios deveria ser 0.0");
}