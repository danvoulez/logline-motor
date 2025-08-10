// === File: llm_trainer/src/models.rs ===
/*
    Description: Modelo de dados para métricas de avaliação de LLM.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

/// Representa as métricas de performance de um modelo após avaliação.
#[derive(Debug, Clone, PartialEq)]
pub struct EvalMetrics {
    pub accuracy: f64,
    pub loss:     f64,
}