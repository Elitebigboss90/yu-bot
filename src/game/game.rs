use std::sync::Arc;

use actix_web::web::Data;
use rand::Rng;
use mongodb::{bson::{Bson, doc, oid::ObjectId, from_document, to_bson}};
use anyhow::{anyhow, Result};

use crate::{database::GameDatabase, models::{User, Weapon, Armor}};

pub struct Game {
    pub database: Data<Arc<GameDatabase>>,
}

impl Game {
    pub fn new(database: Data<Arc<GameDatabase>>) -> Self {
        Self { database }
    }

    pub async fn get_random_weapon(&self) -> Result<Weapon> {
        let random_document = self.database.get_random_document("weapons").await?;

        if let Ok(weapon) = from_document::<Weapon>(random_document.as_document().unwrap().clone()) {
            Ok(weapon)
        } else {
            Err(anyhow!("Failed to convert Bson to Weapon."))
        }
    }

    pub async fn get_random_armor(&self) -> Result<Armor> {
        let random_document = self.database.get_random_document("armors").await?;

        if let Ok(armor) = from_document::<Armor>(random_document.as_document().unwrap().clone()) {
            Ok(armor)
        } else {
            Err(anyhow!("Failed to convert Bson to Armor."))
        }
    }

    pub async fn register(&self, user_id: String, user_name: String) -> Result<(), anyhow::Error> {
        // Check if the user exists
        if let Some(_) = self.database.find_user(&user_id).await? {
            return Err(anyhow!("User already exists."));
        }

        // Create a new user with default weapon and armor
        let user = User {
            id: ObjectId::new(),
            user_id,
            user_name,
            weapon: Weapon::default(), // Assuming default weapon and armor are defined
            armor: Armor::default(),
            wins: 0,
            losses: 0,
            total_games: 0,
        };

        self.database.register_user(user).await?;

        Ok(())
    }

    pub async fn pvp(&self, user_id: &str, opponent_id: &str) -> Result<()> {
        let mut user = match self.database.find_user(user_id).await? {
            Some(user) => user,
            None => return Err(anyhow::anyhow!("User does not exist.")),
        };
    
        let mut opponent = match self.database.find_user(opponent_id).await? {
            Some(opponent) => opponent,
            None => return Err(anyhow::anyhow!("Opponent does not exist.")),
        };
    
        // Calculate the power levels
        let user_power = user.weapon.power_level + user.armor.power_level;
        let opponent_power = opponent.weapon.power_level + opponent.armor.power_level;
    
        // Determine the winner
        let winning_rate = user_power as f64 / (user_power + opponent_power) as f64;
        let rng = rand::thread_rng().gen_range(0.0..1.0);
        let mut winner: User = if rng < winning_rate { user.clone() } else { opponent.clone() };
    
        println!("The winner is: {}", winner.user_name);
    
        let new_equipment = if rand::thread_rng().gen_bool(0.5) {
            // Select a random weapon from the database
            match self.get_random_weapon().await {
                Ok(weapon) => match to_bson(&weapon) {
                    Ok(bson) => Some(bson),
                    Err(_) => None,
                },
                Err(_) => None,
            }
        } else {
            // Select a random armor from the database
            match self.get_random_armor().await {
                Ok(armor) => match to_bson(&armor) {
                    Ok(bson) => Some(bson),
                    Err(_) => None,
                },
                Err(_) => None,
            }
        };

        if let Some(new_equipment) = new_equipment {
            match new_equipment {
                Bson::Document(document) => {
                    if let Ok(equipment) = from_document::<Weapon>(document.clone()) {
                        winner.weapon = equipment;
                    } else if let Ok(equipment) = from_document::<Armor>(document) {
                        winner.armor = equipment;
                    }
                }
                _ => {}
            }
        }

        if winner.user_id == user.user_id {
            user.wins += 1;
            user.weapon = winner.weapon;
            user.armor = winner.armor;
            opponent.losses += 1;
        } else {
            user.losses += 1;
            opponent.wins += 1;
            opponent.weapon = winner.weapon;
            opponent.armor = winner.armor;
        }
        user.total_games += 1;
        opponent.total_games += 1;

        
        // Update the users in the database
        self.database.update_user(&user).await?;
        self.database.update_user(&opponent).await?;
    
        Ok(())
    }
    


    // Similar method for PvE...
}
