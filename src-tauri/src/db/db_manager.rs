use crate::files_manager;

use crate::models::game::Game;

use super::cloud_location::config::CLOUD_LOCATION_DB_TABLE;
use super::games::config::GAMES_DB_TABLE;
use super::{cloud_location, games};

pub fn set_cloud_folder(folder: &str) {
    let conn = cloud_location::config::create_conn().unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {}", CLOUD_LOCATION_DB_TABLE))
        .unwrap();

    let result: String = stmt.query_row([], |row| row.get(1)).unwrap_or_default();

    let update_query = if !result.is_empty() {
        format!(
            "UPDATE {} SET cloud_path = \"{}\" WHERE id = 1",
            CLOUD_LOCATION_DB_TABLE, folder
        )
    } else {
        format!(
            "INSERT INTO {} (cloud_path) VALUES (\"{}\")",
            CLOUD_LOCATION_DB_TABLE, folder
        )
    };

    conn.execute(&update_query, []).unwrap();
}

pub fn select_cloud_location() -> String {
    let conn = cloud_location::config::create_conn().unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {}", CLOUD_LOCATION_DB_TABLE))
        .unwrap();

    let result: String = stmt
        .query_row([], |row| row.get(1))
        .unwrap_or("".to_string());

    result
}

pub fn add_game(db_path: &str, path: &str) -> Game {
    let conn = games::config::create_conn(db_path).unwrap();

    if let Some(game) = get_game(db_path, path) {
        return Game {
            id: 0,
            name: game.name,
            exe_path: game.exe_path,
            img: game.img,
        };
    }

    let game = files_manager::get_game_info(path);
    conn.execute(
        "INSERT INTO games (name, exe_path, img_url) VALUES (?1, ?2, ?3)",
        &[&game.name, &game.exe_path, &game.img],
    )
    .unwrap();

    println!("Game added: {:?}", game);

    game
}

pub fn get_game(db_path: &str, game_path: &str) -> Option<Game> {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {} WHERE exe_path = ?", GAMES_DB_TABLE))
        .unwrap();

    let result: Result<Game, rusqlite::Error> = stmt.query_row([game_path], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            exe_path: row.get(2)?,
            img: row.get(3)?,
        })
    });

    result.ok()
}
