// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tauri::State;

mod db;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_cloud_folder(state: State<Arc<Mutex<Connection>>>, folder: &str) -> Result<(), String> {
    let conn = state.lock().map_err(|_| "Failed to lock connection").expect("Failed to lock connection");
    let result = db::db_manager::set_cloud_folder(conn, folder);

    Ok(result)
}

fn main() {
    let conn = Arc::new(Mutex::new(db::config::create_db().unwrap()));

    tauri::Builder::default()
        .manage(conn)
        .invoke_handler(tauri::generate_handler![greet, set_cloud_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
