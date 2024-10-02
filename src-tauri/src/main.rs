// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use db::config::CloudFolderDbConnection;

mod cmds;
mod db;

fn main() {
    let conn = Arc::new(Mutex::new(db::config::create_cloud_folder_db().unwrap()));

    let builder = tauri::Builder::default()
        .manage(CloudFolderDbConnection { conn })
        .invoke_handler(tauri::generate_handler![
            cmds::greet,
            cmds::set_cloud_folder,
            cmds::get_cloud_folder
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
