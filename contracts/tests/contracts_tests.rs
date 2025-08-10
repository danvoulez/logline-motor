// === File: contracts/tests/contracts_tests.rs ===
/*
    Description: Testes para a biblioteca de contratos LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use contracts::{create_contract, get_contract, list_contracts, update_contract};

#[test]
fn test_create_and_get_contract() {
    // Gera um ID único para evitar conflitos entre testes
    let id = format!("contract-test-{}", uuid::Uuid::new_v4());
    let clauses = vec![
        "Cláusula 1: Termos gerais".to_string(),
        "Cláusula 2: Condições específicas".to_string(),
    ];
    
    // Cria o contrato
    let create_result = create_contract(&id, &clauses);
    assert!(create_result.is_ok(), "Falha ao criar contrato");
    
    // Busca o contrato criado
    let get_result = get_contract(&id);
    assert!(get_result.is_ok(), "Falha ao buscar contrato");
    
    if let Ok((contract_id, contract_clauses, _created_at)) = get_result {
        assert_eq!(contract_id, id, "ID do contrato não corresponde");
        assert_eq!(contract_clauses.len(), 2, "Número incorreto de cláusulas");
        assert_eq!(contract_clauses[0], clauses[0], "Cláusula 1 não corresponde");
        assert_eq!(contract_clauses[1], clauses[1], "Cláusula 2 não corresponde");
    }
}

#[test]
fn test_create_duplicate_contract() {
    let id = format!("contract-duplicate-{}", uuid::Uuid::new_v4());
    let clauses = vec!["Teste de duplicação".to_string()];
    
    // Primeira criação deve ter sucesso
    let result1 = create_contract(&id, &clauses);
    assert!(result1.is_ok(), "Primeira criação deveria ter sucesso");
    
    // Segunda criação com mesmo ID deve falhar
    let result2 = create_contract(&id, &clauses);
    assert!(result2.is_err(), "Segunda criação com mesmo ID deveria falhar");
    
    if let Err(e) = result2 {
        assert!(e.to_string().contains("já existe"), 
                "Mensagem de erro não indica duplicação");
    }
}

#[test]
fn test_list_contracts() {
    // Gera um ID único
    let id = format!("contract-list-{}", uuid::Uuid::new_v4());
    let clauses = vec!["Teste de listagem".to_string()];
    
    // Cria o contrato
    create_contract(&id, &clauses).unwrap();
    
    // Lista os contratos
    let list_result = list_contracts();
    assert!(list_result.is_ok(), "Falha ao listar contratos");
    
    if let Ok(contracts) = list_result {
        assert!(contracts.contains(&id), "Contrato criado não está na lista");
    }
}

#[test]
fn test_update_contract() {
    let id = format!("contract-update-{}", uuid::Uuid::new_v4());
    let clauses_original = vec!["Cláusula original".to_string()];
    
    // Cria o contrato original
    create_contract(&id, &clauses_original).unwrap();
    
    // Novas cláusulas
    let clauses_updated = vec![
        "Cláusula atualizada 1".to_string(),
        "Cláusula atualizada 2".to_string(),
    ];
    
    // Atualiza o contrato
    let update_result = update_contract(&id, &clauses_updated);
    assert!(update_result.is_ok(), "Falha ao atualizar contrato");
    
    // Verifica se o contrato foi atualizado
    let get_result = get_contract(&id);
    assert!(get_result.is_ok(), "Falha ao buscar contrato atualizado");
    
    if let Ok((_, contract_clauses, _)) = get_result {
        assert_eq!(contract_clauses.len(), 2, "Número incorreto de cláusulas após atualização");
        assert_eq!(contract_clauses[0], clauses_updated[0], "Cláusula 1 não foi atualizada");
        assert_eq!(contract_clauses[1], clauses_updated[1], "Cláusula 2 não foi atualizada");
    }
}