// === File: core/src/types.rs ===
/*
    Description: Tipos compartilhados entre os módulos do LogLine Motor.
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

/// Representa uma entidade rastreada pelo sistema LogLine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Identificador único da entidade
    pub id: String,
    /// Tipo de entidade (CONTRATO, IDEIA, etc)
    pub entity_type: String,
    /// Quando a entidade foi criada
    pub created_at: DateTime<Utc>,
    /// Metadados opcionais da entidade
    pub metadata: Option<EntityMetadata>,
}

/// Metadados opcionais que podem ser associados a uma entidade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMetadata {
    /// Quem criou a entidade
    pub creator: Option<String>,
    /// Tags associadas à entidade
    pub tags: Vec<String>,
    /// Versão da entidade
    pub version: Option<String>,
    /// ID de uma entidade relacionada
    pub related_to: Option<String>,
    /// Campos customizados
    pub custom_fields: Option<serde_json::Value>,
}

/// Resultado de uma operação LogLine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Identificador único da operação
    pub operation_id: Uuid,
    /// Status da operação
    pub status: OperationStatus,
    /// Timestamp de conclusão
    pub timestamp: DateTime<Utc>,
    /// Mensagem de resultado
    pub message: String,
    /// Dados adicionais do resultado
    pub data: Option<serde_json::Value>,
}

/// Status possíveis para uma operação
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationStatus {
    /// Operação completada com sucesso
    Success,
    /// Operação falhou
    Failure,
    /// Operação cancelada
    Cancelled,
    /// Operação em progresso
    InProgress,
}

/// Configuração global do sistema LogLine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Modo de execução (development, production)
    pub execution_mode: String,
    /// Nível de logging
    pub log_level: String,
    /// Se deve usar o backend de armazenamento em memória
    pub use_memory_store: bool,
    /// Diretório para carregar plugins
    pub plugin_directory: Option<String>,
    /// Intervalo de refresh dos plugins (em segundos)
    pub plugin_refresh_interval: Option<u64>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            execution_mode: "development".to_string(),
            log_level: "info".to_string(),
            use_memory_store: true,
            plugin_directory: Some("./plugins".to_string()),
            plugin_refresh_interval: Some(30),
        }
    }
}