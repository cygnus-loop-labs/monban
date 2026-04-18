use std::collections::HashMap;

use monban_core::{Deck, WordCategory};
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
pub struct DeckStats {
    pub deck_id: u64,
    pub name: String,
    #[serde(default)]
    pub fullname: String,
    pub new_count: u32,
    pub learn_count: u32,
    pub review_count: u32,
    pub total_in_deck: u32,
}

#[derive(Debug, Deserialize)]
pub struct NoteField {
    pub value: String,
    pub order: u32,
}

#[derive(Debug, Deserialize)]
pub struct NoteInfo {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    pub profile: String,
    pub tags: Vec<String>,
    pub fields: HashMap<String, NoteField>,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub cards: Vec<u64>,
}

pub struct NoteModel {
    pub name: String,
    pub fields: Vec<String>,
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

    pub async fn get_decks(&self) -> Result<Vec<DeckStats>, ConnectError> {
        let response: HashMap<String, u64> = self.api_request("deckNamesAndIds", json!({})).await?;

        let deck_names: Vec<&str> = response
            .iter()
            .filter(|(_, id)| **id != 1)
            .map(|(name, _)| name.as_str())
            .collect();

        let name_lookup: HashMap<String, &str> = response
            .iter()
            .map(|(name, &id)| (id.to_string(), name.as_str()))
            .collect();

        let decks: HashMap<String, DeckStats> = self
            .api_request("getDeckStats", json!({ "decks": deck_names }))
            .await?;

        Ok(decks
            .into_iter()
            .map(|(name, mut deck)| {
                deck.fullname = name_lookup[&name].to_string();
                deck
            })
            .collect())
    }

    pub async fn get_deck(
        &self,
        name: &str,
        word: &str,
        reading: &str,
        meaning: &str,
    ) -> Result<Deck, ConnectError> {
        let mut deck = Deck::new(name);

        let query = format!("deck:\"{name}\"");

        tracing::info!("Fetch deck: {:?}", query);

        let notes_id: Vec<u64> = self
            .api_request("findNotes", json!({"query": query}))
            .await?;

        let notes: Vec<NoteInfo> = self
            .api_request("notesInfo", json!({"notes": notes_id}))
            .await?;

        let short_name = name.split("::").last().unwrap();

        for note in notes {
            let word = note.fields[word].value.clone();
            let reading = note.fields[reading].value.clone();
            let meaning = note.fields[meaning].value.clone();

            let deck_entry = deck.add_word(word, reading, meaning, WordCategory::Unknown);
            deck_entry.tag(short_name.to_string());
        }

        Ok(deck)
    }

    pub async fn get_model(&self, name: &str) -> Result<NoteModel, ConnectError> {
        let details: Vec<String> = self
            .api_request("modelFieldNames", json!({ "modelName": name }))
            .await?;

        let model = NoteModel {
            name: name.to_string(),
            fields: details,
        };

        Ok(model)
    }

    pub async fn get_models(&self) -> Result<Vec<NoteModel>, ConnectError> {
        let mut result = vec![];

        let models: Vec<String> = self.api_request("modelNames", json!({})).await?;

        tracing::info!("Models: {:?}", models);

        for model in &models {
            let details: Vec<String> = self
                .api_request("modelFieldNames", json!({ "modelName": model }))
                .await?;

            tracing::info!("Model details: {}: {:?}", model, details);

            let note_model = NoteModel {
                name: model.to_string(),
                fields: details,
            };

            result.push(note_model);
        }

        Ok(result)
    }
}
