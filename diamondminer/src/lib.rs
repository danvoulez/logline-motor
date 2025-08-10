// === File: diamondminer/src/lib.rs ===
/*
    Description: Lógica principal para mineração de spans diamante com processamento paralelo via Rayon.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

pub mod error;

use datatrack::Span;
use rayon::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::error::DiamondError;

/// Extrai spans "diamante" (nós com ≥ threshold filhos) de forma paralela.
///
/// A mineração é baseada na centralidade de um span, medida pelo número de outros
/// spans que o referenciam como `parent`.
///
/// # Parâmetros
/// - `spans`: Um slice de todos os spans a serem analisados.
/// - `threshold`: O número mínimo de filhos que um span deve ter para ser considerado um "diamante".
///
/// # Retorno
/// Um `Result` contendo um `Vec<Span>` com os spans diamante encontrados, ou um `DiamondError`.
pub fn mine_diamonds(spans: &[Span], threshold: usize) -> Result<Vec<Span>, DiamondError> {
    if spans.is_empty() {
        return Err(DiamondError::EmptyInput);
    }

    // 1. Contagem paralela de filhos para cada `parent_id`.
    //    O padrão fold-reduce é altamente eficiente para agregações em paralelo.
    let child_counts: HashMap<Uuid, usize> = spans
        .par_iter()
        .filter_map(|s| s.parent)
        .fold(
            HashMap::new,
            |mut acc, parent_id| {
                *acc.entry(parent_id).or_insert(0) += 1;
                acc
            },
        )
        .reduce(
            HashMap::new,
            |mut acc, local_map| {
                for (key, count) in local_map {
                    *acc.entry(key).or_insert(0) += count;
                }
                acc
            },
        );

    // 2. Filtragem paralela dos spans que atendem ao critério de diamante.
    let diamonds: Vec<Span> = spans
        .par_iter()
        .filter(|span| {
            child_counts
                .get(&span.id)
                .map_or(false, |count| *count >= threshold)
        })
        .cloned()
        .collect();

    Ok(diamonds)
}