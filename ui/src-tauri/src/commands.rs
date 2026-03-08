use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter as _, State, command};

use monban_core::{Config, Lexicon, Word};
use monban_service::{
    commands::analyze::{cmd_analyze, cmd_get_blacklist},
    parsing::InputType,
};

pub struct AppState {
    config: Config,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }
}

#[command]
pub async fn analyze(
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
    input: String,
) -> Result<Lexicon, String> {
    tracing::info!(target: "Tauri", "Invoke analyze");

    let ty = InputType::Txt;

    let state = state.lock().map_err(|e| e.to_string())?;

    let lexicon = cmd_analyze(&state.config, input, ty, |p| {
        let _ = app.emit("progress", p).map_err(|e| e.to_string());
        tracing::info!("Progress: {}", p);
    })
    .map_err(|e| {
        tracing::error!(target: "Tauri", "Error parsing file: {}", e.to_string());
        e.to_string()
    })?;

    Ok(lexicon)
}

#[command]
pub async fn get_blacklist(
    _app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<Word>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;

    cmd_get_blacklist(&state.config).map_err(|e| {
        tracing::error!(target: "Tauri", "Error parsing file: {}", e.to_string());
        e.to_string()
    })
}
