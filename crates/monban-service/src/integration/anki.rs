use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Anki error: {0}")]
    Anki(String),
    #[error("AnkiConnect error: {0}")]
    AnkiConnect(#[from] reqwest::Error),
}

#[derive(Deserialize)]
struct AnkiResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct AnkiStatus {
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct DeckStats {
    pub deck_id: u64,
    pub name: String,
    pub new_count: u32,
    pub learn_count: u32,
    pub review_count: u32,
    pub total_in_deck: u32,
}

#[derive(Debug)]
pub struct AnkiDeck {
    pub id: u64,
    pub name: String,
    pub size: u32,
}

pub struct AnkiClient {
    client: Client,
    url: String,
}

impl AnkiClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
        }
    }

    async fn api_request<T: for<'a> Deserialize<'a>>(
        &self,
        action: &str,
        params: serde_json::Value,
    ) -> Result<T, ConnectError> {
        let response: AnkiResponse<T> = self
            .client
            .post(&self.url)
            .json(&json!({"action": action, "version": 6, "params": params}))
            .send()
            .await?
            .json()
            .await?;

        if let Some(e) = response.error {
            Err(ConnectError::Anki(e))
        } else {
            response.result.ok_or(ConnectError::ConnectionFailed)
        }
    }

    pub async fn get_status(&self) -> Result<String, ConnectError> {
        let response: AnkiStatus = self.client.post(&self.url).send().await?.json().await?;

        response.api_version.ok_or(ConnectError::ConnectionFailed)
    }

    pub async fn get_deck_names(&self) -> Result<Vec<String>, ConnectError> {
        self.api_request("deckNames", json!({})).await
    }

    pub async fn get_decks(&self) -> Result<Vec<AnkiDeck>, ConnectError> {
        let response: HashMap<String, u64> = self.api_request("deckNamesAndIds", json!({})).await?;

        let deck_names: Vec<&str> = response
            .iter()
            .filter(|(_, id)| **id != 1)
            .map(|(name, _)| name.as_str())
            .collect();

        let decks: HashMap<String, DeckStats> = self
            .api_request("getDeckStats", json!({ "decks": deck_names }))
            .await?;

        Ok(decks
            .into_values()
            .map(|stats| AnkiDeck {
                id: stats.deck_id,
                name: stats.name,
                size: stats.total_in_deck,
            })
            .collect())
    }
}
