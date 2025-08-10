// === File: ideas/tests/ideas_tests.rs ===
/*
    Description: Testes para a biblioteca de ideias LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use ideas::{create_idea, get_idea, list_ideas, update_idea, add_tags, remove_idea};

#[test]
fn test_create_and_get_idea() {
    // Gera um ID único para evitar conflitos entre testes
    let id = format!("idea-test-{}", uuid::Uuid::new_v4());
    let text = "Esta é uma ideia de teste";
    
    // Cria a ideia
    let create_result = create_idea(&id, text);
    assert!(create_result.is_ok(), "Falha ao criar ideia");
    
    // Busca a ideia criada
    let get_result = get_idea(&id);
    assert!(get_result.is_ok(), "Falha ao buscar ideia");
    
    if let Ok((idea_id, idea_text, _created_at)) = get_result {
        assert_eq!(idea_id, id, "ID da ideia não corresponde");
        assert_eq!(idea_text, text, "Texto da ideia não corresponde");
    }
}

#[test]
fn test_create_duplicate_idea() {
    let id = format!("idea-duplicate-{}", uuid::Uuid::new_v4());
    let text = "Teste de duplicação";
    
    // Primeira criação deve ter sucesso
    let result1 = create_idea(&id, text);
    assert!(result1.is_ok(), "Primeira criação deveria ter sucesso");
    
    // Segunda criação com mesmo ID deve falhar
    let result2 = create_idea(&id, text);
    assert!(result2.is_err(), "Segunda criação com mesmo ID deveria falhar");
    
    if let Err(e) = result2 {
        assert!(e.to_string().contains("já existe"), 
                "Mensagem de erro não indica duplicação");
    }
}

#[test]
fn test_list_ideas() {
    // Gera um ID único
    let id = format!("idea-list-{}", uuid::Uuid::new_v4());
    let text = "Teste de listagem";
    
    // Cria a ideia
    create_idea(&id, text).unwrap();
    
    // Lista as ideias
    let list_result = list_ideas();
    assert!(list_result.is_ok(), "Falha ao listar ideias");
    
    if let Ok(ideas) = list_result {
        assert!(ideas.contains(&id), "Ideia criada não está na lista");
    }
}

#[test]
fn test_update_idea() {
    let id = format!("idea-update-{}", uuid::Uuid::new_v4());
    let original_text = "Texto original";
    let updated_text = "Texto atualizado";
    
    // Cria a ideia
    create_idea(&id, original_text).unwrap();
    
    // Atualiza a ideia
    let update_result = update_idea(&id, updated_text);
    assert!(update_result.is_ok(), "Falha ao atualizar ideia");
    
    // Verifica se o texto foi atualizado
    let get_result = get_idea(&id);
    
    if let Ok((_, idea_text, _)) = get_result {
        assert_eq!(idea_text, updated_text, "Texto da ideia não foi atualizado");
    }
}

#[test]
fn test_remove_idea() {
    let id = format!("idea-remove-{}", uuid::Uuid::new_v4());
    let text = "Ideia para remoção";
    
    // Cria a ideia
    create_idea(&id, text).unwrap();
    
    // Remove a ideia
    let remove_result = remove_idea(&id);
    assert!(remove_result.is_ok(), "Falha ao remover ideia");
    
    // Verifica que a ideia não existe mais
    let get_result = get_idea(&id);
    assert!(get_result.is_err(), "A ideia deveria ter sido removida");
}