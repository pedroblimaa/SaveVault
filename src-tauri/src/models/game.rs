use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub img_path: String,
    pub exe_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: Option<String>,
    pub url: Option<String>,
}