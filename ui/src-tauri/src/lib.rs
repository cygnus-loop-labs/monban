mod commands;

use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::commands::{AppState, analyze};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze])
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = Arc::new(Mutex::new(AppState::new()));
            app.manage(state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
