// === File: registry/tests/registry_tests.rs ===
/*
    Description: Testes para o registro central de entidades do LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use registry::{register_entity, fetch_entity, list_entities_by_type, remove_entity};

#[tokio::test]
async fn test_register_and_fetch_entity() {
    // Registra uma entidade de teste
    let id = "test-entity-1";
    let entity_type = "TEST";
    
    let result = register_entity(id, entity_type).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), id);
    
    // Busca a entidade registrada
    let entity = fetch_entity(id).await;
    assert!(entity.is_ok());
    
    let (fetched_id, fetched_type) = entity.unwrap();
    assert_eq!(fetched_id, id);
    assert_eq!(fetched_type, entity_type);
}

#[tokio::test]
async fn test_list_entities_by_type() {
    // Registra múltiplas entidades de diferentes tipos
    register_entity("list-test-1", "TYPE_A").await.unwrap();
    register_entity("list-test-2", "TYPE_B").await.unwrap();
    register_entity("list-test-3", "TYPE_A").await.unwrap();
    
    // Lista entidades do TYPE_A
    let type_a_entities = list_entities_by_type("TYPE_A").await.unwrap();
    
    // Verifica se todas as entidades TYPE_A foram encontradas
    assert!(type_a_entities.contains(&"list-test-1".to_string()));
    assert!(type_a_entities.contains(&"list-test-3".to_string()));
    assert_eq!(type_a_entities.len(), 2);
    
    // Lista entidades do TYPE_B
    let type_b_entities = list_entities_by_type("TYPE_B").await.unwrap();
    
    // Verifica se todas as entidades TYPE_B foram encontradas
    assert!(type_b_entities.contains(&"list-test-2".to_string()));
    assert_eq!(type_b_entities.len(), 1);
}

#[tokio::test]
async fn test_remove_entity() {
    // Registra uma entidade para remoção
    let id = "remove-test";
    register_entity(id, "TEMPORARY").await.unwrap();
    
    // Verifica se a entidade existe
    let exists = fetch_entity(id).await.is_ok();
    assert!(exists, "A entidade deveria existir antes da remoção");
    
    // Remove a entidade
    let removed = remove_entity(id).await.unwrap();
    assert!(removed, "A remoção deveria ser bem-sucedida");
    
    // Verifica se a entidade foi removida
    let not_exists = fetch_entity(id).await.is_err();
    assert!(not_exists, "A entidade não deveria existir após a remoção");
    
    // Tenta remover novamente (deveria falhar)
    let not_removed = !remove_entity(id).await.unwrap();
    assert!(not_removed, "A segunda tentativa de remoção deveria falhar");
}