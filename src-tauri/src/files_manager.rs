use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use winres::IconResource;
use base64;

use crate::models::game::Game;

pub fn move_folder_items(from_path: &str, to_path: &str) {
    let entries = fs::read_dir(from_path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let file_name = path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy();
        let new_path = format!("{}/{}", to_path, file_name);
        fs::rename(path, new_path).expect("Failed to move file");
    }
}

pub fn get_game_info(path: &str) -> Game {
    let path = Path::new(path);
    let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or("");
    let name = file_name.trim_end_matches(".exe").to_string();

    let icon = IconResource::from_path(path)
        .and_then(|mut icon| icon.largest_icon())
        .map(|icon| icon.to_png_bytes())
        .unwrap_or_default();

    let img64 = format!("data:image/png;base64,{}", base64::encode(&icon));

    Game {
        id: 0, // Assuming id is set elsewhere or auto-incremented
        name,
        exe_path: path.to_str().unwrap().to_string(),
        img: img64,
    }
}
