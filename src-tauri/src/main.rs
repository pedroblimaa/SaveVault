// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use db::{cloud_location::config::DbPath, db_manager};

mod cmds;
mod db;
mod files_manager;
mod models;
mod services;

fn main() {
    let path = db_manager::select_cloud_location();
    let path_mutex = Mutex::new(path);

    let builder = tauri::Builder::default()
        .manage(DbPath { path: path_mutex })
        .invoke_handler(tauri::generate_handler![
            cmds::set_cloud_location,
            cmds::get_cloud_location,
            cmds::add_game
        ]);

    builder.run(tauri::generate_context!()).unwrap();
}
