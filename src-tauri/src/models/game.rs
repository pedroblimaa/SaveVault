use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub img: String,
    pub exe_path: String,
}

impl Game {
    pub fn new(id: i64, name: String, img: String, exe_path: String) -> Self {
        Game {
            id,
            name,
            img,
            exe_path,
        }
    }
}