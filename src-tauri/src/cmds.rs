use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::db::cloud_location::config::DbPath;
use crate::services::{files_service, game_service, metadata_service};
use crate::{db, models::game::Game};

#[tauri::command]
pub fn set_cloud_location(app_handle: AppHandle, path: &str, override_folder: bool) {
    game_service::set_cloud_location(path, override_folder);

    let mutex_path = Mutex::new(path.to_string());
    app_handle.manage(DbPath { path: mutex_path });
}

#[tauri::command]
pub fn get_cloud_location() -> String {
    let cloud_location = db::db_manager::select_cloud_location();
    let path_exists = files_service::path_exists(&cloud_location);

    if !path_exists {
        db::db_manager::set_cloud_folder("");
        return "".to_string();
    }

    cloud_location
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
pub fn check_is_folder_already_used(path: &str) -> bool {
    files_service::is_folder_already_used(path)
}

#[tauri::command]
pub fn get_games(state: State<'_, DbPath>) -> Vec<Game> {
    db::db_manager::get_all_games(&state.path.lock().unwrap()).unwrap_or([].to_vec())
}

#[tauri::command]
pub async fn set_game_metadata(state: State<'_, DbPath>, id: i32, name: String) -> Result<(), String> {
    let token = metadata_service::get_token().await;
    let game_info = metadata_service::get_game_info(&name, &token).await;
    let url = game_info.url.clone();

    db::db_manager::update_game_metadata(&state.path.lock().unwrap(), id, url.unwrap().as_str());

    Ok(())
}
