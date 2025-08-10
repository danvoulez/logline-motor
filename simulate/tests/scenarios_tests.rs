// === File: simulate/tests/scenarios_tests.rs ===
/*
    Description: Testes para diferentes cenários de simulação.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use simulate::{
    run_simulation_with_config, SimulationConfig, SimulationMode,
    evaluate_simulation, SimulationResult, SimulationStatus
};
use registry::register_entity;

#[tokio::test]
async fn test_deterministic_simulation() {
    // Registra uma entidade de teste
    let entity_id = "sim-test-deterministic";
    let _ = register_entity(entity_id, "TEST").await;
    
    // Configura uma simulação determinística com seed fixo
    let config = SimulationConfig {
        mode: SimulationMode::Deterministic,
        rounds: 5,
        random_seed: Some(12345), // Seed fixo para resultados reproduzíveis
        metrics: vec![
            "success_rate".to_string(),
            "processing_time".to_string(),
        ],
        scenario: Some("normal".to_string()),
    };
    
    // Executa a simulação determinística
    let result1 = run_simulation_with_config(entity_id, config.clone()).await;
    assert!(result1.is_ok(), "A primeira simulação determinística falhou");
    
    // Executa a mesma simulação novamente
    let result2 = run_simulation_with_config(entity_id, config).await;
    assert!(result2.is_ok(), "A segunda simulação determinística falhou");
    
    // Com o mesmo seed e configuração determinística, os resultados devem ser idênticos
    let sim_results1 = result1.unwrap();
    let sim_results2 = result2.unwrap();
    
    assert_eq!(sim_results1.len(), sim_results2.len(), "Número de resultados difere");
    
    // Verifica se as métricas são idênticas em cada rodada
    for (r1, r2) in sim_results1.iter().zip(sim_results2.iter()) {
        assert_eq!(r1.metrics.get("success_rate"), r2.metrics.get("success_rate"), 
                   "Taxa de sucesso difere na rodada {}", r1.round);
        assert_eq!(r1.metrics.get("processing_time"), r2.metrics.get("processing_time"), 
                   "Tempo de processamento difere na rodada {}", r1.round);
    }
}

#[tokio::test]
async fn test_random_simulation_different_seeds() {
    // Registra uma entidade de teste
    let entity_id = "sim-test-random";
    let _ = register_entity(entity_id, "TEST").await;
    
    // Configura uma simulação com seed 1
    let config1 = SimulationConfig {
        mode: SimulationMode::Random,
        rounds: 5,
        random_seed: Some(12345),
        metrics: vec![],
        scenario: None,
    };
    
    // Configura uma simulação com seed 2
    let config2 = SimulationConfig {
        mode: SimulationMode::Random,
        rounds: 5,
        random_seed: Some(54321), // Seed diferente
        metrics: vec![],
        scenario: None,
    };
    
    // Executa as simulações
    let result1 = run_simulation_with_config(entity_id, config1).await;
    let result2 = run_simulation_with_config(entity_id, config2).await;
    
    assert!(result1.is_ok() && result2.is_ok(), "Falha ao executar simulações aleatórias");
    
    let sim_results1 = result1.unwrap();
    let sim_results2 = result2.unwrap();
    
    // Verifica se pelo menos algumas métricas são diferentes
    // (é estatisticamente improvável que todas sejam iguais com seeds diferentes)
    let mut found_different_values = false;
    for (r1, r2) in sim_results1.iter().zip(sim_results2.iter()) {
        if r1.metrics.get("success_rate") != r2.metrics.get("success_rate") {
            found_different_values = true;
            break;
        }
    }
    
    assert!(found_different_values, "Seeds diferentes deveriam produzir resultados diferentes");
}

#[tokio::test]
async fn test_scenario_based_simulation() {
    // Registra uma entidade de teste
    let entity_id = "sim-test-scenario";
    let _ = register_entity(entity_id, "TEST").await;
    
    // Configura uma simulação com cenário de alta performance
    let config = SimulationConfig {
        mode: SimulationMode::Scenario,
        rounds: 5,
        random_seed: None,
        metrics: vec![],
        scenario: Some("high_performance".to_string()),
    };
    
    // Executa a simulação
    let result = run_simulation_with_config(entity_id, config).await;
    assert!(result.is_ok(), "Falha ao executar simulação baseada em cenário");
    
    let sim_results = result.unwrap();
    
    // Verifica características do cenário de alta performance
    for result in &sim_results {
        // O cenário "high_performance" deve ter success_rate entre 0.9 e 1.0
        if let Some(success_rate) = result.metrics.get("success_rate") {
            assert!(*success_rate >= 0.9 && *success_rate <= 1.0, 
                    "Success rate fora do esperado para cenário high_performance: {}", success_rate);
        }
        
        // O cenário deve ter tempo de processamento baixo
        if let Some(processing_time) = result.metrics.get("processing_time") {
            assert!(*processing_time <= 50.0, 
                    "Tempo de processamento fora do esperado para cenário high_performance: {}", 
                    processing_time);
        }
    }
}

#[tokio::test]
async fn test_evaluation_function() {
    // Cria resultados de simulação de teste
    let results = vec![
        SimulationResult {
            id: uuid::Uuid::new_v4(),
            entity_id: "eval-test".to_string(),
            round: 1,
            timestamp: chrono::Utc::now(),
            metrics: {
                let mut m = std::collections::HashMap::new();
                m.insert("success_rate".to_string(), 0.8);
                m
            },
            events: vec![],
            status: SimulationStatus::Success,
        },
        SimulationResult {
            id: uuid::Uuid::new_v4(),
            entity_id: "eval-test".to_string(),
            round: 2,
            timestamp: chrono::Utc::now(),
            metrics: {
                let mut m = std::collections::HashMap::new();
                m.insert("success_rate".to_string(), 0.6);
                m
            },
            events: vec![],
            status: SimulationStatus::Success,
        },
    ];
    
    // Avalia os resultados
    let score = evaluate_simulation(&results);
    
    // A média das taxas de sucesso (0.8 + 0.6) / 2 = 0.7
    assert_eq!(score, 0.7, "Avaliação incorreta dos resultados");
    
    // Teste com lista vazia
    let empty_score = evaluate_simulation(&[]);
    assert_eq!(empty_score, 0.0, "Avaliação de lista vazia deveria ser 0.0");
}