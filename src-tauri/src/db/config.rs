use rusqlite::Connection;

const CLOUD_FOLDER_DB_PATH: &str = "cloud_folder_database.db";
pub const CLOUD_FOLDER_DB_TABLE: &str = "cloud_folder";
pub const FAILED_QUERY_MESSAGE: &str = "Failed to execute query.";
pub const FOLDER_DB: &str = "save_vault.db";

pub type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn create_cloud_folder_db() -> DbResult<Connection> {
    let conn = Connection::open(CLOUD_FOLDER_DB_PATH)?;
    conn.execute(&get_cloud_table_query(), [])?;

    Ok(conn)
}

pub fn create_db(path: &str) -> DbResult<Connection> {
    let final_path = format!("{}\\{}", path, FOLDER_DB);
    let conn = Connection::open(final_path)?;
    conn.execute(&get_games_table_query(), [])?;

    Ok(conn)
}

fn get_cloud_table_query() -> String {
    let id = &get_id_query();
    let cloud_path = "cloud_path TEXT";

    let query = format!(
        "CREATE TABLE IF NOT EXISTS cloud_folder ({}, {})",
        id, cloud_path
    );

    query.to_string()
}

fn get_id_query() -> String {
    "id INTEGER PRIMARY KEY AUTOINCREMENT".to_string()
}

fn get_games_table_query() -> String {
    let id = &get_id_query();
    let exe_name = "exe_name TEXT UNIQUE";
    let name = "name TEXT";
    let save_path = "save_path TEXT";

    let query = format!(
        "CREATE TABLE IF NOT EXISTS games ({}, {}, {}, {})",
        id, exe_name, name, save_path
    );

    query.to_string()
}
