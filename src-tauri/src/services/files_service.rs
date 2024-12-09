use std::path::Path;
use std::{fs, io};

use pelite::pe::{Pe, PeFile};

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
    let name = get_name(path).unwrap();

    println!("Name: {}", name);

    Game {
        id: 0,
        name,
        exe_path: path.to_str().unwrap().to_string(),
        img_path: "".to_string(),
    }
}

pub fn folder_already_used(path: &str) -> bool {
    let games_db_path = Path::new(path).join(GAMES_DB);

    games_db_path.exists()
}


// TODO - Add option for when there's no ProductName or Product Description 
pub fn get_name(path: &Path) -> io::Result<String> {
    let data = fs::read(path)?;
    let pe = PeFile::from_bytes(&data).unwrap();
    let resources = pe.resources().unwrap();
    let version_info = resources.version_info().unwrap();

    let lang = version_info.translation().first().unwrap();
    
    // TODO - Check why Dragon Age is coming with Name in Japanese
    println!("File Info: {:#?}", version_info.file_info());
    let name = version_info
        .value(*lang, "ProductName")
        .unwrap_or("BootstrapPackagedGame".to_string())
        .to_string();

    if name == "BootstrapPackagedGame" {
        return Ok(get_parent_folder_name(path));
    }

    Ok(name)
}

fn get_parent_folder_name(path: &Path) -> String {
    let path = path.parent().unwrap().to_str().unwrap().to_string();
    let name = path.split("\\").last().unwrap().to_string();

    name
}
