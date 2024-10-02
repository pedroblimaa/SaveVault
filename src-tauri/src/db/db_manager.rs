use std::sync::MutexGuard;

use rusqlite::{Connection, OptionalExtension};

use crate::db::config::CLOUD_FOLDER_DB_TABLE;

use super::config::FAILED_QUERY_MESSAGE;

pub fn set_cloud_folder(connection: MutexGuard<'_, Connection>, folder: &str) {
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

pub fn select_cloud_folder(connection: MutexGuard<'_, Connection>) -> String {
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
