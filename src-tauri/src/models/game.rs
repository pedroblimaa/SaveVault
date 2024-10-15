use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub img: String,
    pub exe_path: String,
}