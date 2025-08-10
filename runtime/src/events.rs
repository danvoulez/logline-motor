// === File: runtime/src/events.rs ===
/*
    Description: Define o sistema de eventos usado pelo runtime para propagar alterações de estado.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ruleset::rules::Verdict;

/// Tipo de evento produzido pelo runtime
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EventKind {
    /// Evento de ciclo de vida do runtime
    RuntimeLifecycle {
        status: String,
    },
    /// Comando imperativo executado
    ImperativeExecuted {
        kind: String,
    },
    /// Ideia registrada
    IdeaRegistered {
        id: String,
    },
    /// Contrato registrado
    ContractRegistered {
        id: String,
    },
    /// Veredicto de uma regra do ruleset
    RuleVerdict {
        rule: String,
        verdict: Verdict,
    },
    /// Orquestração iniciada
    OrchestrationStarted {
        mode: String,
        concurrency: usize,
    },
    /// Orquestração concluída
    OrchestrationCompleted {
        mode: String,
        concurrency: usize,
        duration_ms: u64,
    },
    /// Simulação executada
    SimulationCompleted {
        id: String,
        rounds: usize,
    },
    /// Erro ocorrido
    ErrorOccurred {
        context: String,
        message: String,
    },
}

/// Evento com metadados
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    /// Identificador único do evento
    pub id: Uuid,
    /// Quando o evento ocorreu
    pub timestamp: DateTime<Utc>,
    /// Tipo do evento com dados específicos
    pub kind: EventKind,
}

impl Event {
    /// Cria um novo evento com timestamp atual e ID gerado
    pub fn new(kind: EventKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            kind,
        }
    }
}