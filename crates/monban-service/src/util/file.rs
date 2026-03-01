use std::{fs, io::ErrorKind, path::Path};

use crate::parsing::ParseError;

pub fn load_file(path: impl AsRef<Path>) -> Result<String, ParseError> {
    let path = path.as_ref();

    let path = if path.is_absolute() {
        tracing::debug!(target: "Parser", "Load absolute: {:?}", path);
        path.to_path_buf()
    } else if cfg!(debug_assertions) {
        tracing::debug!(target: "Parser", "Load relative to workspace: {:?}", path);
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or(ParseError::FileNotFound(path.to_string_lossy().to_string()))?
            .parent()
            .ok_or(ParseError::FileNotFound(path.to_string_lossy().to_string()))?
            .join(path)
    } else {
        tracing::debug!(target: "Parser", "Load relative to bin: {:?}", path);
        std::env::current_exe()
            .map_err(|_| ParseError::FileNotFound(path.to_string_lossy().to_string()))?
            .join(path)
    };

    tracing::info!(target: "Parser", "Load file: {:?}", path);

    fs::read_to_string(&path)
        .inspect_err(|e| tracing::error!(target: "Parser", "Error reading file: {}", e.to_string()))
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => ParseError::FileNotFound(path.to_string_lossy().to_string()),
            _ => ParseError::InvalidFileFormat(path.to_string_lossy().to_string()),
        })
}
