use std::path::Path;

use epub::doc::EpubDoc;

pub struct EpubTextLoader;

impl EpubTextLoader {
    pub fn load(file: impl AsRef<Path>) -> Vec<String> {
        let mut result = vec![];

        tracing::info!(target = "epub", "Loading epub: {:?}", file.as_ref());

        let mut doc = EpubDoc::new(file).unwrap();

        tracing::info!(target = "epub", "Chapter count: {}", doc.get_num_chapters());

        for i in 0..doc.get_num_chapters() {
            doc.set_current_chapter(i);
            if let Some(content) = doc.get_current_str() {
                tracing::debug!("Adding content: {}", content.1);
                result.push(content.0);
            }
        }

        result
    }
}
