use monban_core::Config;

use crate::integration::anki::{AnkiClient, AnkiDeck, ConnectError};

pub struct AnkiDeckLoader {}

impl AnkiDeckLoader {
    pub async fn list_decks(config: &Config) -> Result<Vec<AnkiDeck>, ConnectError> {
        let url = &config.anki.url;

        let client = AnkiClient::new(url);

        tracing::info!("Status={:?}", client.get_status().await);

        client.get_decks().await
    }
}
