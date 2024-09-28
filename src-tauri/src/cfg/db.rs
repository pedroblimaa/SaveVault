use rusqlite::Connection;

const DB_PATH: &str = "game_database.db";

pub fn create_db() {
    let conn = Connection::open(DB_PATH).expect("Failed to connect to the database");

    conn.execute(&get_table_query(), [])
        .expect("Failed to create table");
}

fn get_table_query() -> String {
    let id = "id INTEGER PRIMARY KEY AUTOINCREMENT";
    let ex_name = "ex_name TEXT UNIQUE";
    let name = "name TEXT";
    let save_path = "save_path TEXT";

    let query = format!(
        "CREATE TABLE IF NOT EXISTS games ({}, {}, {}, {})",
        id, ex_name, name, save_path
    );

    query.to_string()
}
