pub const CLOUD_LOCATION_DB_TABLE: &str = "cloud_folder_location";

pub fn get_cloud_table() -> String {
    let id = &get_id_query();
    let cloud_path = "cloud_path TEXT";

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({}, {})",
        CLOUD_LOCATION_DB_TABLE, id, cloud_path
    );

    query.to_string()
}

fn get_id_query() -> String {
    "id INTEGER PRIMARY KEY AUTOINCREMENT".to_string()
}
