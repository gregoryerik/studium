use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Group {
    pub id: i32,
    pub collectibles: serde_json::Value,
    pub name: String
}



