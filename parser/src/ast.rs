// === File: parser/src/ast.rs ===
/*
    Description: AST (Árvore de Sintaxe Abstrata) para a linguagem LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::fmt;

/// Um comando LogLine completo.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Um comando imperativo que executa uma ação.
    Imperative(Imperative),
}

/// Um comando imperativo LogLine.
#[derive(Debug, PartialEq)]
pub struct Imperative {
    /// Tipo específico do comando imperativo.
    pub kind: ImperativeKind,
}

impl fmt::Display for Imperative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ImperativeKind::DefineContract { id, clauses } => {
                write!(f, "DEFINE CONTRACT {} {}", id, clauses.join(", "))
            }
            ImperativeKind::DefineIdea { id, text } => {
                write!(f, "DEFINE IDEA {} \"{}\"", id, text)
            }
            ImperativeKind::SimulateEntity { id, rounds } => {
                write!(f, "SIMULATE ENTITY {} {}", id, rounds)
            }
            ImperativeKind::Orchestrate { mode } => {
                write!(f, "ORCHESTRATE {}", mode)
            }
            ImperativeKind::InvokeRuleset { entity_id, ruleset_id } => {
                write!(f, "INVOKE RULESET {} ON {}", ruleset_id, entity_id)
            }
        }
    }
}

/// Tipos específicos de comandos imperativos LogLine.
#[derive(Debug, PartialEq)]
pub enum ImperativeKind {
    /// Define um contrato LogLine.
    DefineContract {
        /// Identificador do contrato.
        id: String,
        /// Lista de cláusulas do contrato.
        clauses: Vec<String>,
    },
    
    /// Define uma ideia LogLine.
    DefineIdea {
        /// Identificador da ideia.
        id: String,
        /// Texto descritivo da ideia.
        text: String,
    },
    
    /// Executa uma simulação sobre uma entidade.
    SimulateEntity {
        /// Identificador da entidade a ser simulada.
        id: String,
        /// Número de rodadas de simulação.
        rounds: usize,
    },
    
    /// Inicia uma orquestração com o modo especificado.
    Orchestrate {
        /// Modo de orquestração (sequencial, paralelo, etc).
        mode: String,
    },
    
    /// Invoca um conjunto de regras sobre uma entidade.
    InvokeRuleset {
        /// Identificador da entidade alvo.
        entity_id: String,
        /// Identificador do conjunto de regras.
        ruleset_id: String,
    },
}