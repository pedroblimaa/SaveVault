use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

use crate::db::{
    self,
    config::{CloudFolderDbConnection, DbConnection},
};
use crate::files_manager;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn set_cloud_folder(
    state: State<'_, CloudFolderDbConnection>,
    app_handle: AppHandle,
    path: &str,
) {
    let cloud_folder_conn = state.conn.lock().expect("Failed to lock connection");
    let cloud_folder = db::db_manager::select_cloud_folder(&cloud_folder_conn);
    if !cloud_folder.is_empty() {
        files_manager::move_folder_items(&cloud_folder, path);
    }

    db::db_manager::set_cloud_folder(&cloud_folder_conn, path);

    let conn = Arc::new(Mutex::new(
        db::config::create_conn(path).expect("Failed to create cloud db"),
    ));
    app_handle.manage(DbConnection { conn });
}

#[tauri::command]
pub fn get_cloud_folder(state: State<'_, CloudFolderDbConnection>) -> String {
    let cloud_folder_conn = state.conn.lock().expect("Failed to lock connection");

    db::db_manager::select_cloud_folder(&cloud_folder_conn)
}

#[tauri::command]
pub fn add_game(state: State<'_, DbConnection>, path: &str) -> Game {
    let conn = state.conn.lock().expect("Failed to lock connection");
    db::db_manager::add_game(&conn, path)
}
