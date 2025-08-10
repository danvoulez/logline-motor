// === File: ruleset/src/lib.rs ===
/*
    Description: Motor de regras para avaliação de entidades LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

pub mod rules;

use rules::{Rule, Verdict};
use std::error::Error;

/// Aplica um conjunto de regras identificado a uma entidade.
/// 
/// # Parâmetros
/// 
/// * `ruleset_id` - Identificador do conjunto de regras a ser aplicado
/// * `entity_id` - Identificador da entidade a ser avaliada
/// 
/// # Retorna
/// 
/// Um `Result` contendo o veredicto da aplicação do ruleset ou erro.
pub async fn apply_ruleset(ruleset_id: &str, entity_id: &str) -> Result<Verdict, Box<dyn Error>> {
    // Em uma implementação real, as regras viriam de um banco de dados ou configuração
    let rule = match ruleset_id {
        "always-accept" => Rule::AlwaysAccept,
        "always-reject" => Rule::AlwaysReject,
        "basic-check" => Rule::ContentCheck {
            field: "text".to_string(),
            pattern: "important".to_string(),
        },
        _ => Rule::AlwaysAccept, // Fallback
    };
    
    // Buscar entidade e avaliar a regra
    // Em uma implementação real, o conteúdo da entidade seria buscado
    // em um repositório de persistência
    let entity_content = fetch_entity_content(entity_id).await?;
    
    Ok(rule.evaluate(&entity_content))
}

/// Função simulada que busca o conteúdo de uma entidade.
/// Em uma implementação real, isso seria uma consulta ao banco de dados.
async fn fetch_entity_content(_entity_id: &str) -> Result<String, Box<dyn Error>> {
    // Simulando uma entidade para o teste
    Ok("This is a test entity with important content".to_string())
}