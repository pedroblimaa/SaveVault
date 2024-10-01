use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, State};

use crate::db::{
    self,
    config::{CloudFolderDbConnection, DbConnection},
};

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

    db::db_manager::set_cloud_folder(cloud_folder_conn, path);

    let conn = Arc::new(Mutex::new(
        db::config::create_db(path).expect("Failed to create cloud db"),
    ));
    app_handle.manage(DbConnection { conn });
}

#[tauri::command]
pub fn get_cloud_folder(state: State<'_, CloudFolderDbConnection>) -> String {
    let cloud_folder_conn = state.conn.lock().expect("Failed to lock connection");

    db::db_manager::select_cloud_folder(cloud_folder_conn)
}