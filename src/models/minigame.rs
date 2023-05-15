use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")] 
    pub id: ObjectId,
    pub user_id: String,
    pub user_name: String,
    pub weapon: Weapon,
    pub armor: Armor,
    pub wins: i32,
    pub losses: i32,
    pub total_games: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weapon {
    pub id: ObjectId,
    pub name: String,
    pub power_level: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Armor {
    pub id: ObjectId,
    pub name: String,
    pub power_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Boss {
    pub id: ObjectId,
    pub name: String,
    pub health: i32,
    pub power_level: i32,
}

impl User {
    pub fn win_rate(&self) -> f64 {
        if self.total_games == 0 {
            return 0.0;
        }
        self.wins as f64 / self.total_games as f64
    }
}

impl Weapon {
    pub fn default() -> Self {
        Self {
            id: ObjectId::new(),
            name: String::from("Default Weapon"),
            power_level: 100,
        }
    }
}

impl Armor {
    pub fn default() -> Self {
        Self {
            id: ObjectId::new(),
            name: String::from("Default Armor"),
            power_level: 100,
        }
    }
}