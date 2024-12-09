use crate::models::game::Game;
use crate::services::files_service;

use super::cloud_location::config::CLOUD_LOCATION_DB_TABLE;
use super::games::config::GAMES_DB_TABLE;
use super::{cloud_location, games};

pub fn set_cloud_folder(folder: &str) {
    let conn = cloud_location::config::create_conn().unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {}", CLOUD_LOCATION_DB_TABLE))
        .unwrap();

    let result: String = stmt.query_row([], |row| row.get(1)).unwrap();

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
            img_path: game.img_path,
        };
    }

    let game = files_service::get_game_info(path);
    conn.execute(
        "INSERT INTO games (name, exe_path, img_url) VALUES (?1, ?2, ?3)",
        &[&game.name, &game.exe_path, &game.img_path],
    )
    .unwrap();

    let added_game = get_last_game(db_path).unwrap();

    added_game
}

pub fn get_game(db_path: &str, game_path: &str) -> Option<Game> {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn
        .prepare(&format!(
            "SELECT * FROM  {} WHERE exe_path = ?",
            GAMES_DB_TABLE
        ))
        .unwrap();

    let result: Result<Game, rusqlite::Error> = stmt.query_row([game_path], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            exe_path: row.get(2)?,
            img_path: row.get(3)?,
        })
    });

    result.ok()
}

pub fn get_game_by_id(db_path: &str, id: i32) -> Option<Game> {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {} WHERE id = ?", GAMES_DB_TABLE))
        .unwrap();

    let result: Result<Game, rusqlite::Error> = stmt.query_row([id], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            exe_path: row.get(2)?,
            img_path: row.get(3)?,
        })
    });

    result.ok()
}

pub fn get_last_game(db_path: &str) -> Option<Game> {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM  {} ORDER BY id DESC LIMIT 1", GAMES_DB_TABLE))
        .unwrap();

    let result: Result<Game, rusqlite::Error> = stmt.query_row([], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            exe_path: row.get(2)?,
            img_path: row.get(3)?,
        })
    });

    result.ok()
}

pub fn get_all_games(db_path: &str) -> Vec<Game> {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn.prepare(&games::queries::get_games_query()).unwrap();

    let result: Vec<Game> = stmt
        .query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                exe_path: row.get(2)?,
                img_path: row.get(3)?,
            })
        })
        .unwrap()
        .filter_map(|row| row.ok())
        .collect();

    result
}

pub fn update_game_metadata(db_path: &str, id: i32, url: &str) {
    let conn = games::config::create_conn(db_path).unwrap();

    let mut stmt = conn.prepare(&games::queries::update_game_img()).unwrap();

    stmt.execute([url, &id.to_string()]).unwrap();
}
