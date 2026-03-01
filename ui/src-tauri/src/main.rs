// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use monban_service::util::init_logger;

fn main() {
    init_logger();
    monban_tauri::run();
}
