use monban_core::{Config, Deck};

use crate::integration::anki::{AnkiClient, ConnectError, DeckStats, NoteModel};

pub struct AnkiDeckLoader {
    client: AnkiClient,
}

impl AnkiDeckLoader {
    pub fn new(config: &Config) -> Self {
        let url = &config.anki.url;
        let client = AnkiClient::new(url);

        Self { client }
    }

    pub async fn load(
        name: String,
        config: &Config,
        word: &str,
        reading: &str,
        meaning: &str,
    ) -> Result<Deck, ConnectError> {
        let url = &config.anki.url;
        let client = AnkiClient::new(url);

        client.get_deck(&name, word, reading, meaning).await
    }

    pub async fn list_decks(&self) -> Result<Vec<DeckStats>, ConnectError> {
        self.client.get_decks().await
    }

    pub async fn get_deck(
        &self,
        deck: &str,
        word: &str,
        reading: &str,
        meaning: &str,
    ) -> Result<Deck, ConnectError> {
        self.client.get_deck(deck, word, reading, meaning).await
    }

    pub async fn get_models(&self) -> Result<Vec<NoteModel>, ConnectError> {
        self.client.get_models().await
    }
}
