use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::db::cloud_location::config::DbPath;
use crate::services::{files_service, game_service};
use crate::{db, models::game::Game};

#[tauri::command]
pub fn set_cloud_location(app_handle: AppHandle, path: &str, move_foler: bool) {
    game_service::set_cloud_location(path, move_foler);

    let mutex_path = Mutex::new(path.to_string());
    app_handle.manage(DbPath { path: mutex_path });
}

#[tauri::command]
pub fn get_cloud_location() -> String {
    db::db_manager::select_cloud_location()
}

#[tauri::command]
pub fn add_game(state: State<'_, DbPath>, path: &str) -> Game {
    db::db_manager::add_game(&state.path.lock().unwrap(), path)
}

#[tauri::command]
pub fn is_cloud_location_empty() -> bool {
    game_service::is_cloud_location_empty()
}

#[tauri::command]
pub fn folder_already_used(path: &str) -> bool {
    files_service::folder_already_used(path)
}
