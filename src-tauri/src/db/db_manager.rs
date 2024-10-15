use std::sync::MutexGuard;

use rusqlite::{Connection, OptionalExtension};

use crate::db::config::CLOUD_FOLDER_DB_TABLE;
use crate::files_manager;
use crate::models::game::Game;

use super::config::FAILED_QUERY_MESSAGE;

pub fn set_cloud_folder(connection: &MutexGuard<'_, Connection>, folder: &str) {
    let mut stmt = connection
        .prepare(&format!("SELECT * FROM  {}", CLOUD_FOLDER_DB_TABLE))
        .expect(FAILED_QUERY_MESSAGE);

    let result: Option<String> = stmt
        .query_row([], |row| row.get(1))
        .optional()
        .expect(FAILED_QUERY_MESSAGE);

    let update_query = if let Some(_value) = result {
        format!(
            "UPDATE {} SET cloud_path = \"{}\" WHERE id = 1",
            CLOUD_FOLDER_DB_TABLE, folder
        )
    } else {
        format!(
            "INSERT INTO {} (cloud_path) VALUES (\"{}\")",
            CLOUD_FOLDER_DB_TABLE, folder
        )
    };

    connection
        .execute(&update_query, [])
        .expect(FAILED_QUERY_MESSAGE);
}

pub fn select_cloud_folder(connection: &MutexGuard<'_, Connection>) -> String {
    let mut stmt = connection
        .prepare(&format!("SELECT * FROM  {}", CLOUD_FOLDER_DB_TABLE))
        .expect(FAILED_QUERY_MESSAGE);

    let result: Option<String> = stmt
        .query_row([], |row| row.get(1))
        .optional()
        .expect(FAILED_QUERY_MESSAGE);

    if let Some(path) = result {
        path
    } else {
        "".to_string()
    }
}

pub fn add_game(connection: &MutexGuard<'_, Connection>, path: &str) -> Game {
    let mut stmt = connection
        .prepare(&format!(
            "SELECT * FROM  {} WHERE path = ?",
            CLOUD_FOLDER_DB_TABLE
        ))
        .expect(FAILED_QUERY_MESSAGE);

    let result: Option<Game> = stmt
        .query_row([path], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                exe_path: row.get(2)?,
                img: row.get(3)?,
            })
        })
        .optional()
        .expect(FAILED_QUERY_MESSAGE);

    if let Some(_value) = result {
        Game {
            id: 0,
            name: _value.name,
            exe_path: _value.exe_path,
            img: _value.img,
        }
    } else {
        let game = files_manager::get_game_info(path);
        connection.execute(
            "INSERT INTO games (name, exe_path, img) VALUES (?1, ?2, ?3)",
            &[&game.name, &game.exe_path, &game.img],
        );
        game
    }
}
