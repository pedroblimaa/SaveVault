use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::db::cloud_location::config::DbPath;
use crate::services::game_service;
use crate::{db, models::game::Game};

#[tauri::command]
pub fn set_cloud_location(app_handle: AppHandle, path: &str) {
    game_service::set_cloud_location(path);

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
