// === File: runtime/tests/runtime_tests.rs ===
/*
    Description: Testes de unidade para o runtime do LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use runtime::{process_command, init, timeline, events::EventKind};
use tokio::time::timeout;
use std::time::Duration;
use anyhow::Result;

#[tokio::test]
async fn test_runtime_initialization() -> Result<()> {
    // Limpa a timeline antes do teste
    timeline::clear_timeline().await?;
    
    // Inicializa o runtime
    init().await?;
    
    // Verifica se o evento de inicialização foi registrado
    let events = timeline::list_events().await?;
    assert!(!events.is_empty(), "A timeline deve conter pelo menos um evento");
    
    let lifecycle_events = timeline::find_events_by_kind("RuntimeLifecycle").await?;
    assert_eq!(lifecycle_events.len(), 1, "Deve haver exatamente um evento de ciclo de vida");
    
    if let EventKind::RuntimeLifecycle { status } = &lifecycle_events[0].kind {
        assert_eq!(status, "initialized", "O status deve ser 'initialized'");
    } else {
        panic!("Evento de tipo incorreto");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_process_command_idea() -> Result<()> {
    // Limpa a timeline
    timeline::clear_timeline().await?;
    
    // Processa um comando para criar uma ideia
    let cmd = "DEFINE IDEA id001 \"Minha ideia de teste\"";
    let result = process_command(cmd).await?;
    
    assert!(result.contains("Ideia registrada"), "A resposta deve indicar que a ideia foi registrada");
    
    // Verifica se o evento foi registrado
    let events = timeline::find_events_by_kind("IdeaRegistered").await?;
    assert_eq!(events.len(), 1, "Deve haver exatamente um evento de ideia registrada");
    
    if let EventKind::IdeaRegistered { id } = &events[0].kind {
        assert_eq!(id, "id001", "O ID da ideia deve ser 'id001'");
    } else {
        panic!("Evento de tipo incorreto");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_process_command_simulation() -> Result<()> {
    // Limpa a timeline
    timeline::clear_timeline().await?;
    
    // Processa um comando para simular uma entidade
    let cmd = "SIMULATE ENTITY test_entity 5";
    
    // Adiciona um timeout para garantir que não trave
    let result = timeout(Duration::from_secs(5), process_command(cmd)).await??;
    
    assert!(result.contains("Simulação"), "A resposta deve indicar que a simulação foi concluída");
    assert!(result.contains("5 rodadas"), "A resposta deve mencionar 5 rodadas");
    
    // Verifica se o evento foi registrado
    let events = timeline::find_events_by_kind("SimulationCompleted").await?;
    assert_eq!(events.len(), 1, "Deve haver exatamente um evento de simulação concluída");
    
    if let EventKind::SimulationCompleted { id, rounds } = &events[0].kind {
        assert_eq!(id, "test_entity", "O ID da entidade deve ser 'test_entity'");
        assert_eq!(*rounds, 5, "O número de rodadas deve ser 5");
    } else {
        panic!("Evento de tipo incorreto");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_process_command_syntax_error() -> Result<()> {
    // Limpa a timeline
    timeline::clear_timeline().await?;
    
    // Processa um comando com erro de sintaxe
    let cmd = "INVALID COMMAND";
    let result = process_command(cmd).await;
    
    assert!(result.is_err(), "O processamento deve falhar com erro de sintaxe");
    
    // Verifica se o evento de erro foi registrado
    let events = timeline::find_events_by_kind("ErrorOccurred").await?;
    assert_eq!(events.len(), 1, "Deve haver exatamente um evento de erro");
    
    if let EventKind::ErrorOccurred { context, .. } = &events[0].kind {
        assert_eq!(context, "parsing", "O contexto do erro deve ser 'parsing'");
    } else {
        panic!("Evento de tipo incorreto");
    }
    
    Ok(())
}