// === File: simulate/src/engine.rs ===
/*
    Description: Motor de simulação para entidades LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-08
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use crate::{SimulateError, SimulationResult, SimulationStatus};
use crate::scenarios::Scenario;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Modos possíveis de simulação
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimulationMode {
    /// Simulação determinística (resultados reproduzíveis)
    Deterministic,
    
    /// Simulação aleatória
    Random,
    
    /// Simulação controlada por cenário
    Scenario,
}

/// Configuração do motor de simulação
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Semente aleatória para simulações reproduzíveis
    pub random_seed: Option<u64>,
    
    /// Modo de simulação
    pub mode: SimulationMode,
}

/// Motor de simulação para entidades LogLine
pub struct SimulationEngine {
    /// Gerador de números aleatórios
    rng: StdRng,
    
    /// Modo de simulação
    mode: SimulationMode,
}

impl SimulationEngine {
    /// Cria uma nova instância do motor de simulação
    pub fn new(config: EngineConfig) -> Self {
        // Inicializa o gerador de números aleatórios
        let rng = match config.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        Self {
            rng,
            mode: config.mode,
        }
    }
    
    /// Simula uma rodada para uma entidade
    pub async fn simulate_round(
        &mut self,
        entity_id: &str,
        scenario: &Scenario,
        round: usize,
    ) -> Result<SimulationResult, SimulateError> {
        // Cria um resultado de simulação com ID e timestamp
        let mut result = SimulationResult {
            id: Uuid::new_v4(),
            entity_id: entity_id.to_string(),
            round,
            timestamp: Utc::now(),
            metrics: HashMap::new(),
            events: Vec::new(),
            status: SimulationStatus::InProgress,
        };
        
        // Executa a simulação conforme o modo configurado
        match self.mode {
            SimulationMode::Deterministic => {
                self.run_deterministic_simulation(&mut result, scenario);
            }
            SimulationMode::Random => {
                self.run_random_simulation(&mut result);
            }
            SimulationMode::Scenario => {
                self.run_scenario_simulation(&mut result, scenario);
            }
        }
        
        // Finaliza o resultado
        result.status = SimulationStatus::Success;
        
        Ok(result)
    }
    
    /// Executa uma simulação determinística
    fn run_deterministic_simulation(&mut self, result: &mut SimulationResult, scenario: &Scenario) {
        // Aplica as métricas do cenário deterministicamente
        for (key, range) in &scenario.metrics {
            let value = (range.0 + range.1) / 2.0;
            result.metrics.insert(key.clone(), value);
        }
        
        // Adiciona eventos conforme o cenário
        for event in &scenario.events {
            if event.probability >= 1.0 {
                result.events.push(format!("{}:entity={}", event.name, result.entity_id));
            }
        }
    }
    
    /// Executa uma simulação aleatória
    fn run_random_simulation(&mut self, result: &mut SimulationResult) {
        // Gera métricas aleatórias
        result.metrics.insert("success_rate".to_string(), self.rng.gen_range(0.5..1.0));
        result.metrics.insert("processing_time".to_string(), self.rng.gen_range(10.0..500.0));
        result.metrics.insert("resource_usage".to_string(), self.rng.gen_range(0.1..0.9));
        
        // Gera eventos aleatórios
        if self.rng.gen_bool(0.7) {
            result.events.push(format!("process_start:entity={}", result.entity_id));
        }
        if self.rng.gen_bool(0.5) {
            result.events.push(format!("validation_complete:entity={}", result.entity_id));
        }
        if self.rng.gen_bool(0.3) {
            result.events.push(format!("resource_allocation:entity={}", result.entity_id));
        }
    }
    
    /// Executa uma simulação controlada por cenário
    fn run_scenario_simulation(&mut self, result: &mut SimulationResult, scenario: &Scenario) {
        // Aplica métricas do cenário com variações aleatórias
        for (key, (min, max)) in &scenario.metrics {
            let value = self.rng.gen_range(*min..*max);
            result.metrics.insert(key.clone(), value);
        }
        
        // Adiciona eventos conforme probabilidade
        for event in &scenario.events {
            if self.rng.gen_bool(event.probability) {
                result.events.push(format!("{}:entity={}", event.name, result.entity_id));
            }
        }
        
        // Aplica modificadores do cenário
        for modifier in &scenario.modifiers {
            if let Some(value) = result.metrics.get_mut(&modifier.metric) {
                *value *= modifier.factor;
            }
        }
    }
}