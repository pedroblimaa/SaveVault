use super::config::GAMES_DB_TABLE;

pub fn get_games_table() -> String {
    let id = &get_id_query();
    let name = "name TEXT";
    let exe_path = "exe_path TEXT UNIQUE";
    let img = "img_url TEXT";
    let save_path = "save_path TEXT";

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({}, {}, {}, {}, {})",
        GAMES_DB_TABLE, id, name, exe_path, img, save_path
    );

    query.to_string()
}


fn get_id_query() -> String {
    "id INTEGER PRIMARY KEY AUTOINCREMENT".to_string()
}