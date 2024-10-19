// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod db;
mod files_manager;
mod models;

fn main() {
    let builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        cmds::set_cloud_location,
        cmds::get_cloud_location,
        cmds::add_game
    ]);
    builder
        .run(tauri::generate_context!())
        .unwrap();
}
