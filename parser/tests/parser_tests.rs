// === File: parser/tests/parser_tests.rs ===
/*
    Description: Testes de unidade para o parser da linguagem LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use parser::{parse, ast::{Command, Imperative, ImperativeKind}};

#[test]
fn test_parse_define_contract() {
    let input = "DEFINE CONTRACT contract-123 clause1, clause2";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input válido");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::DefineContract { id, clauses } })) = result {
        assert_eq!(id, "contract-123", "ID do contrato incorreto");
        assert_eq!(clauses.len(), 2, "Número incorreto de cláusulas");
        assert_eq!(clauses[0], "clause1", "Primeira cláusula incorreta");
        assert_eq!(clauses[1], "clause2", "Segunda cláusula incorreta");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}

#[test]
fn test_parse_define_idea() {
    let input = "DEFINE IDEA idea-456 \"Esta é uma ideia de teste\"";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input válido");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::DefineIdea { id, text } })) = result {
        assert_eq!(id, "idea-456", "ID da ideia incorreto");
        assert_eq!(text, "Esta é uma ideia de teste", "Texto da ideia incorreto");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}

#[test]
fn test_parse_simulate_entity() {
    let input = "SIMULATE ENTITY entity-789 10";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input válido");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::SimulateEntity { id, rounds } })) = result {
        assert_eq!(id, "entity-789", "ID da entidade incorreto");
        assert_eq!(rounds, 10, "Número de rodadas incorreto");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}

#[test]
fn test_parse_orchestrate() {
    let input = "ORCHESTRATE parallel";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input válido");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::Orchestrate { mode } })) = result {
        assert_eq!(mode, "parallel", "Modo de orquestração incorreto");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}

#[test]
fn test_parse_invoke_ruleset() {
    let input = "INVOKE RULESET basic-check ON entity-123";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input válido");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::InvokeRuleset { entity_id, ruleset_id } })) = result {
        assert_eq!(entity_id, "entity-123", "ID da entidade incorreto");
        assert_eq!(ruleset_id, "basic-check", "ID do ruleset incorreto");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}

#[test]
fn test_parse_error_for_invalid_input() {
    let input = "INVALID COMMAND";
    let result = parse(input);
    
    assert!(result.is_err(), "Parsing deveria falhar para input inválido");
}

#[test]
fn test_parse_with_whitespace() {
    let input = "  DEFINE  IDEA   my-idea    \"Com espaços\"  ";
    let result = parse(input);
    
    assert!(result.is_ok(), "Parsing falhou para input com espaços extras");
    
    if let Ok(Command::Imperative(Imperative { kind: ImperativeKind::DefineIdea { id, text } })) = result {
        assert_eq!(id, "my-idea", "ID da ideia incorreto");
        assert_eq!(text, "Com espaços", "Texto da ideia incorreto");
    } else {
        panic!("Tipo de comando incorreto após parsing");
    }
}