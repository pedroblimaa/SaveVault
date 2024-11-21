use base64;
use ico::IconDir;
use pelite::pe::Pe;
use pelite::pe64::PeFile;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::Path;

use crate::db::games::config::GAMES_DB;
use crate::models::game::Game;

pub fn move_folder_items(from_path: &str, to_path: &str) {
    let entries = fs::read_dir(from_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        let new_path = format!("{}/{}", to_path, file_name);
        fs::rename(path, new_path).unwrap();
    }
}

pub fn get_game_info(path_str: &str) -> Game {
    let path = Path::new(path_str);
    let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or("");
    let name = file_name.trim_end_matches(".exe").to_string();

    // TODO - FIX getting icon, currently is coming empty
    let icon = File::open(path)
        .ok()
        .and_then(|file| IconDir::read(file).ok())
        .and_then(|icon_dir| icon_dir.entries().get(0).cloned())
        .map(|icon_image| icon_image.data().to_vec())
        .unwrap_or_default();

    let img64 = format!("data:image/png;base64,{}", base64::encode(&icon));

    &get_exe_icon(path_str.to_string());

    Game {
        id: 0, // Assuming id is set elsewhere or auto-incremented
        name,
        exe_path: path.to_str().unwrap().to_string(),
        img: img64,
    }
}

pub fn folder_already_used(path: &str) -> bool {
    let games_db_path = Path::new(path).join(GAMES_DB);

    games_db_path.exists()
}

fn get_exe_icon(path: String) {
    let data = fs::read(path).unwrap();
    let pe = PeFile::from_bytes(&data).unwrap();

    // TODO - FIX

    let resources = pe.resources().unwrap();
    println!("Resources: {:#?}", resources.to_string())
}
