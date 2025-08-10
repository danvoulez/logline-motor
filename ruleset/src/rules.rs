// === File: ruleset/src/rules.rs ===
/*
    Description: Definição de regras e vereditos para o motor de regras do LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use serde::{Serialize, Deserialize};

/// Possíveis vereditos da avaliação de uma regra sobre uma entidade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Verdict {
    /// A entidade está em conformidade com a regra.
    Accepted,
    
    /// A entidade não está em conformidade com a regra.
    Rejected,
}

/// Regras que podem ser aplicadas a entidades LogLine.
#[derive(Debug, Clone)]
pub enum Rule {
    /// Sempre aceita a entidade, independente do seu conteúdo.
    AlwaysAccept,
    
    /// Sempre rejeita a entidade, independente do seu conteúdo.
    AlwaysReject,
    
    /// Verifica se um campo específico contém um padrão.
    ContentCheck {
        /// O campo a ser verificado no conteúdo da entidade.
        field: String,
        
        /// O padrão a ser buscado no campo especificado.
        pattern: String,
    },
}

impl Rule {
    /// Avalia a regra contra o conteúdo de uma entidade.
    pub fn evaluate(&self, content: &str) -> Verdict {
        match self {
            Rule::AlwaysAccept => Verdict::Accepted,
            Rule::AlwaysReject => Verdict::Rejected,
            Rule::ContentCheck { field: _, pattern } => {
                // Na implementação real, field seria usado para navegar em um objeto estruturado
                // Aqui estamos simplificando e apenas verificando se o pattern existe no content
                if content.contains(pattern) {
                    Verdict::Accepted
                } else {
                    Verdict::Rejected
                }
            }
        }
    }
}