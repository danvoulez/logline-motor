// === File: datatrack/src/models.rs ===
/*
    Description: Modelos de dados para o datatrack, incluindo Span e SpanFilter.
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

/// Um span genérico, a unidade fundamental de registro no LogLine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    pub id:        Uuid,
    pub parent:    Option<Uuid>,
    pub kind:      String,
    pub payload:   String,
    pub timestamp: DateTime<Utc>,
}

/// Filtro para consulta de spans.
#[derive(Debug, Clone, Default)]
pub struct SpanFilter {
    /// Se fornecido, retorna apenas spans deste `kind`.
    pub kind: Option<String>,
    /// Limita ao número máximo de spans retornados.
    pub limit: Option<usize>,
}