use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::db::cloud_location::config::DbPath;
use crate::files_manager;
use crate::{db, models::game::Game};

#[tauri::command]
pub fn set_cloud_location(app_handle: AppHandle, path: &str) {
    let cloud_location = db::db_manager::select_cloud_location();

    if !cloud_location.is_empty() && cloud_location != path {
        files_manager::move_folder_items(&cloud_location, path);
    }

    db::db_manager::set_cloud_folder(path);

    let mutex_path = Mutex::new(Some(path.to_string()));
    app_handle.manage(DbPath { path: mutex_path });
}

#[tauri::command]
pub fn get_cloud_location() -> String {
    db::db_manager::select_cloud_location()
}

#[tauri::command]
pub fn add_game(state: State<'_, DbPath>, path: &str) -> Game {
    // TODO: Handle adding path to manager every time the app is started
    println!("Adding game: {:?}", path);
    db::db_manager::add_game(&state.path.lock().unwrap().as_ref().unwrap(), path)
}
