// TODO: Change names for <folder>_config.rs

use rusqlite::Connection;

use super::queries;

pub const GAMES_DB: &str = "save_vault.db";
pub const GAMES_DB_TABLE: &str = "games";

pub type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn create_conn(path: &str) -> DbResult<Connection> {
    let final_path = format!("{}\\{}", path, GAMES_DB);
    let conn = Connection::open(final_path)?;
    conn.execute(&queries::get_games_table(), [])?;

    Ok(conn)
}
