// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use db::{cloud_location::config::DbPath, db_manager};
use tauri_plugin_dialog;

mod cmds;
mod db;
mod models;
mod services;

pub fn run() {
    let path = db_manager::select_cloud_location();
    let path_mutex = Mutex::new(path);

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(DbPath { path: path_mutex })
        .invoke_handler(tauri::generate_handler![
            cmds::set_cloud_location,
            cmds::get_cloud_location,
            cmds::add_game,
            cmds::is_cloud_location_empty,
            cmds::check_is_folder_already_used,
            cmds::get_games,
            cmds::set_game_metadata
        ]);

    builder.run(tauri::generate_context!()).unwrap();
}
