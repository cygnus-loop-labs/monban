use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter as _, State, command};

use monban_core::{Config, Lexicon};
use monban_service::{
    analysis::analyzer::Stats,
    commands::analyze::{cmd_analyze, cmd_stats},
    parsing::InputType,
};

pub struct AppState {
    config: Config,
    current_lexicon: Option<Lexicon>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            current_lexicon: None,
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

    let mut state = state.lock().map_err(|e| e.to_string())?;

    let lexicon = cmd_analyze(&state.config, input, ty, |p| {
        let _ = app.emit("progress", p).map_err(|e| e.to_string());
        tracing::info!("Progress: {}", p);
    })
    .map_err(|e| {
        tracing::error!(target: "Tauri", "Error parsing file: {}", e.to_string());
        e.to_string()
    })?;

    state.current_lexicon = Some(lexicon.clone());

    Ok(lexicon)
}

#[command]
pub async fn stats(
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Stats, String> {
    tracing::info!(target: "Tauri", "Invoke stats");

    let state = state.lock().map_err(|e| e.to_string())?;

    let stats = cmd_stats(&state.config, state.current_lexicon.as_ref(), |p| {
        let _ = app.emit("progress", p).map_err(|e| e.to_string());
        tracing::info!("Progress: {}", p);
    });

    Ok(stats)
}
