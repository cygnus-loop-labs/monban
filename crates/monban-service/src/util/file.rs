use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, ErrorKind, Read},
    path::{Path, PathBuf},
};

use flate2::bufread::GzDecoder;

use crate::parsing::ParseError;

pub fn get_data_dir() -> Result<PathBuf, ParseError> {
    if cfg!(debug_assertions) {
        Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or(ParseError::FileNotFound("".to_string()))?
            .parent()
            .ok_or(ParseError::FileNotFound("".to_string()))?
            .join("data"))
    } else {
        Ok(dirs::data_dir()
            .ok_or(ParseError::FileNotFound("".to_string()))?
            .join("monban"))
    }
}

pub fn load_data_file(path: impl AsRef<Path>) -> Result<String, ParseError> {
    let root_dir = get_data_dir()?;
    load_file(root_dir, path)
}

pub fn load_file(root_dir: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<String, ParseError> {
    let root_dir = root_dir.as_ref();
    let path = path.as_ref();

    let path = if path.is_absolute() {
        tracing::debug!(target: "Parser", "Load absolute: {:?}", path);
        path.to_path_buf()
    } else {
        tracing::debug!(target: "Parser", "Load relative to data dir: {:?}/{:?}", root_dir, path);
        root_dir.join(path)
    };

    let file = File::open(&path)
        .inspect_err(|e| tracing::error!(target: "Parser", "Error reading file: {}", e.to_string()))
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => ParseError::FileNotFound(path.to_string_lossy().to_string()),
            _ => ParseError::InvalidFileFormat(path.to_string_lossy().to_string()),
        })?;

    tracing::info!(target: "Parser", "Load file: {:?}", path);

    let mut s = String::new();

    if path.extension() == Some(OsStr::new("gz")) {
        GzDecoder::new(BufReader::new(file)).read_to_string(&mut s)
    } else {
        BufReader::new(file).read_to_string(&mut s)
    }
    .map_err(|_| ParseError::InvalidFileFormat(path.to_string_lossy().to_string()))?;

    Ok(s)
}
