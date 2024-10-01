// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

mod db;

// Struct to hold the local database connection
struct DbConnection {
    conn: Arc<Mutex<Connection>>, // Your rusqlite connection (or whatever DB you're using)
}

// Struct to hold the cloud database connection
struct CloudFolderDbConnection {
    conn: Arc<Mutex<Connection>>, // Could also be another rusqlite connection
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_cloud_folder(state: State<'_, CloudFolderDbConnection>, app_handle: AppHandle, path: &str) {
    let cloud_folder_conn = state.conn.lock().expect("Failed to lock connection");

    db::db_manager::set_cloud_folder(cloud_folder_conn, path);

    let conn = Arc::new(Mutex::new(
        db::config::create_db(path).expect("Failed to create cloud db"),
    ));
    app_handle.manage(DbConnection { conn });
}

fn main() {
    let conn = Arc::new(Mutex::new(db::config::create_cloud_folder_db().unwrap()));

    tauri::Builder::default()
        .manage(CloudFolderDbConnection { conn })
        .invoke_handler(tauri::generate_handler![greet, set_cloud_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
