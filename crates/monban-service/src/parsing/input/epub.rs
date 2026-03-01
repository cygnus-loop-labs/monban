use std::path::Path;

use epub::doc::{DocError, EpubDoc};
use scraper::Html;

use crate::parsing::{InputType, ParseError};

pub struct EpubTextLoader;

impl EpubTextLoader {
    pub fn load(file: impl AsRef<Path>) -> Result<Vec<String>, ParseError> {
        let mut result = vec![];

        tracing::info!(target: "epub", "Loading epub: {:?}", file.as_ref());

        let mut doc = EpubDoc::new(&file)
            .inspect_err(
                |e| tracing::error!(target: "epub", "Error loading epub: {}", e.to_string()),
            )
            .map_err(|e| match e {
                DocError::IOError(_) => {
                    ParseError::FileNotFound(file.as_ref().to_string_lossy().to_string())
                }
                _ => ParseError::InvalidFileType(InputType::Epub),
            })?;

        tracing::info!(target: "epub", "Chapter count: {}", doc.get_num_chapters());

        for i in 0..doc.get_num_chapters() {
            doc.set_current_chapter(i);
            if let Some(content) = doc.get_current_str() {
                tracing::debug!(target: "epub", "Adding content: {}", &content.1);
                result.push(Self::strip_html(&content.0));
            }
        }

        Ok(result)
    }

    fn strip_html(html: &str) -> String {
        Html::parse_document(html).root_element().text().collect()
    }
}
