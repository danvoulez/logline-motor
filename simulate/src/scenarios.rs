// === File: simulate/src/scenarios.rs ===
/*
    Description: Cenários de simulação para entidades LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Evento que pode ocorrer durante uma simulação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEvent {
    /// Nome do evento
    pub name: String,
    
    /// Probabilidade do evento ocorrer (0.0 a 1.0)
    pub probability: f64,
    
    /// Dados associados ao evento (opcional)
    pub data: Option<serde_json::Value>,
}

/// Modificador que afeta métricas durante uma simulação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioModifier {
    /// Métrica a ser modificada
    pub metric: String,
    
    /// Fator de multiplicação
    pub factor: f64,
    
    /// Condição para aplicação do modificador (opcional)
    pub condition: Option<String>,
}

/// Resultado possível de um cenário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOutcome {
    /// Nome do resultado
    pub name: String,
    
    /// Condição para este resultado ocorrer
    pub condition: String,
    
    /// Probabilidade base deste resultado (0.0 a 1.0)
    pub probability: f64,
}

/// Configuração de um cenário de simulação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConfig {
    /// Nome do cenário
    pub name: String,
    
    /// Descrição do cenário
    pub description: String,
    
    /// Métricas do cenário (nome, (min, max))
    pub metrics: HashMap<String, (f64, f64)>,
    
    /// Eventos do cenário
    pub events: Vec<ScenarioEvent>,
    
    /// Modificadores do cenário
    pub modifiers: Vec<ScenarioModifier>,
    
    /// Resultados possíveis do cenário
    pub outcomes: Vec<ScenarioOutcome>,
}

/// Cenário de simulação para entidades
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Configuração do cenário
    pub config: ScenarioConfig,
    
    /// Métricas do cenário (nome, (min, max))
    pub metrics: HashMap<String, (f64, f64)>,
    
    /// Eventos do cenário
    pub events: Vec<ScenarioEvent>,
    
    /// Modificadores do cenário
    pub modifiers: Vec<ScenarioModifier>,
}

impl Scenario {
    /// Cria um novo cenário a partir de uma configuração
    pub fn new(config: ScenarioConfig) -> Self {
        Self {
            metrics: config.metrics.clone(),
            events: config.events.clone(),
            modifiers: config.modifiers.clone(),
            config,
        }
    }
}

/// Biblioteca de cenários pré-definidos
static SCENARIOS: once_cell::sync::Lazy<HashMap<&str, ScenarioConfig>> = once_cell::sync::Lazy::new(|| {
    let mut m = HashMap::new();
    
    // Cenário: Alta Performance
    let high_performance = ScenarioConfig {
        name: "high_performance".to_string(),
        description: "Cenário de alta performance com recursos abundantes".to_string(),
        metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("success_rate".to_string(), (0.9, 1.0));
            metrics.insert("processing_time".to_string(), (5.0, 50.0));
            metrics.insert("resource_usage".to_string(), (0.1, 0.4));
            metrics
        },
        events: vec![
            ScenarioEvent {
                name: "process_start".to_string(),
                probability: 1.0,
                data: None,
            },
            ScenarioEvent {
                name: "optimization_applied".to_string(),
                probability: 0.8,
                data: None,
            },
            ScenarioEvent {
                name: "process_complete".to_string(),
                probability: 0.95,
                data: None,
            },
        ],
        modifiers: vec![
            ScenarioModifier {
                metric: "processing_time".to_string(),
                factor: 0.8,
                condition: Some("event:optimization_applied".to_string()),
            },
        ],
        outcomes: vec![
            ScenarioOutcome {
                name: "optimal".to_string(),
                condition: "metrics.success_rate > 0.95".to_string(),
                probability: 0.7,
            },
            ScenarioOutcome {
                name: "good".to_string(),
                condition: "metrics.success_rate > 0.9".to_string(),
                probability: 0.25,
            },
            ScenarioOutcome {
                name: "acceptable".to_string(),
                condition: "true".to_string(),
                probability: 0.05,
            },
        ],
    };
    m.insert("high_performance", high_performance);
    
    // Cenário: Baixa Performance
    let low_performance = ScenarioConfig {
        name: "low_performance".to_string(),
        description: "Cenário de baixa performance com recursos escassos".to_string(),
        metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("success_rate".to_string(), (0.4, 0.7));
            metrics.insert("processing_time".to_string(), (200.0, 800.0));
            metrics.insert("resource_usage".to_string(), (0.7, 0.95));
            metrics
        },
        events: vec![
            ScenarioEvent {
                name: "process_start".to_string(),
                probability: 0.9,
                data: None,
            },
            ScenarioEvent {
                name: "resource_exhausted".to_string(),
                probability: 0.6,
                data: None,
            },
            ScenarioEvent {
                name: "process_timeout".to_string(),
                probability: 0.4,
                data: None,
            },
            ScenarioEvent {
                name: "process_complete".to_string(),
                probability: 0.6,
                data: None,
            },
        ],
        modifiers: vec![
            ScenarioModifier {
                metric: "success_rate".to_string(),
                factor: 0.5,
                condition: Some("event:resource_exhausted".to_string()),
            },
            ScenarioModifier {
                metric: "processing_time".to_string(),
                factor: 1.5,
                condition: Some("metrics.resource_usage > 0.85".to_string()),
            },
        ],
        outcomes: vec![
            ScenarioOutcome {
                name: "failure".to_string(),
                condition: "metrics.success_rate < 0.5".to_string(),
                probability: 0.6,
            },
            ScenarioOutcome {
                name: "partial_success".to_string(),
                condition: "metrics.success_rate >= 0.5".to_string(),
                probability: 0.3,
            },
            ScenarioOutcome {
                name: "success".to_string(),
                condition: "metrics.success_rate >= 0.7".to_string(),
                probability: 0.1,
            },
        ],
    };
    m.insert("low_performance", low_performance);
    
    // Cenário: Normal
    let normal = ScenarioConfig {
        name: "normal".to_string(),
        description: "Cenário normal com comportamento médio".to_string(),
        metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("success_rate".to_string(), (0.7, 0.9));
            metrics.insert("processing_time".to_string(), (50.0, 200.0));
            metrics.insert("resource_usage".to_string(), (0.4, 0.7));
            metrics
        },
        events: vec![
            ScenarioEvent {
                name: "process_start".to_string(),
                probability: 1.0,
                data: None,
            },
            ScenarioEvent {
                name: "validation_complete".to_string(),
                probability: 0.8,
                data: None,
            },
            ScenarioEvent {
                name: "process_complete".to_string(),
                probability: 0.85,
                data: None,
            },
        ],
        modifiers: vec![],
        outcomes: vec![
            ScenarioOutcome {
                name: "success".to_string(),
                condition: "metrics.success_rate >= 0.8".to_string(),
                probability: 0.7,
            },
            ScenarioOutcome {
                name: "partial".to_string(),
                condition: "metrics.success_rate >= 0.7".to_string(),
                probability: 0.2,
            },
            ScenarioOutcome {
                name: "failure".to_string(),
                condition: "true".to_string(),
                probability: 0.1,
            },
        ],
    };
    m.insert("normal", normal);
    
    m
});

/// Carrega um cenário pelo nome
pub fn load_scenario(name: &str) -> Option<Scenario> {
    SCENARIOS.get(name).map(|config| Scenario::new(config.clone()))
}

/// Retorna um cenário padrão para um tipo de entidade
pub fn default_scenario_for_type(entity_type: &str) -> Scenario {
    match entity_type.to_uppercase().as_str() {
        "CONTRACT" => load_scenario("high_performance").unwrap_or_else(|| load_scenario("normal").unwrap()),
        "IDEA" => load_scenario("normal").unwrap(),
        _ => load_scenario("normal").unwrap(),
    }
}

/// Lista todos os cenários disponíveis
pub fn list_scenarios() -> Vec<String> {
    SCENARIOS.keys().map(|&k| k.to_string()).collect()
}