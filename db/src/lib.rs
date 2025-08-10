// === File: db/src/lib.rs ===
/*
    Description: Cliente Supabase/PostgREST para armazenamento persistente.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Config missing: {0}")]
    Config(String),
}

pub struct Db {
    pub client: Client,
    pub url:    String,
}

impl Db {
    pub fn new() -> Result<Self, DbError> {
        let url = std::env::var("PGRST_URL")
            .map_err(|_| DbError::Config("PGRST_URL".into()))?;
        let key = std::env::var("PGRST_KEY")
            .map_err(|_| DbError::Config("PGRST_KEY".into()))?;
        let client = Client::builder()
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("apikey", key.parse().unwrap());
                h.insert("Authorization", format!("Bearer {}", key).parse().unwrap());
                h
            })
            .build()?;
        Ok(Db { client, url })
    }

    /// Insere um registro JSON na tabela
    pub async fn insert<T: Serialize>(&self, table: &str, record: &T) -> Result<(), DbError> {
        let url = format!("{}/{}", self.url, table);
        self.client.post(&url).json(record).send().await?.error_for_status()?;
        Ok(())
    }

    /// Busca registros (opcional WHERE)
    pub async fn select<T: DeserializeOwned>(
        &self,
        table: &str,
        filter: Option<&str>
    ) -> Result<Vec<T>, DbError> {
        let url = format!("{}/{}", self.url, table);
        let mut req = self.client.get(&url);
        if let Some(f) = filter {
            // A simple approach, for complex queries consider a query builder
            let params: Vec<(&str, &str)> = f.split('&').filter_map(|part| part.split_once("=")).collect();
            req = req.query(&params);
        }
        let res = req.send().await?.error_for_status()?.json().await?;
        Ok(res)
    }

    /// Atualiza um registro por ID
    pub async fn update<T: Serialize>(&self, table: &str, id: &str, record: &T) -> Result<(), DbError> {
        let url = format!("{}/{}?id=eq.{}", self.url, table, id);
        self.client.patch(&url).json(record).send().await?.error_for_status()?;
        Ok(())
    }
}