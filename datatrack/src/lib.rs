// === File: datatrack/src/lib.rs ===
/*
    Description: Ponto de entrada público do crate datatrack.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

pub mod models;
pub mod error;
pub mod store;

pub use models::{Span, SpanFilter};
pub use error::DataTrackError;
pub use store::{record_span, fetch_spans};