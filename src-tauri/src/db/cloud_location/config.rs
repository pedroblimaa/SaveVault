use std::sync::Mutex;

use rusqlite::Connection;

use super::queries;

const CLOUD_LOCATION_DB_PATH: &str = "cloud_location.db";
pub const CLOUD_LOCATION_DB_TABLE: &str = "cloud_folder_location";

pub struct DbPath {
    pub path: Mutex<Option<String>>,
}

pub type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn create_conn() -> DbResult<Connection> {
    let conn = Connection::open(CLOUD_LOCATION_DB_PATH)?;
    conn.execute(&queries::get_cloud_table(), [])?;

    Ok(conn)
}
