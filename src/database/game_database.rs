use mongodb::{options::ClientOptions, Client, Collection};
use mongodb::bson::{doc, Bson, to_bson, Document};
use std::env;


use crate::constants::{weapon_d2, armor_d2};
use crate::models::{User, Boss, Weapon, Armor};

pub struct GameDatabase {
    pub users: Collection<User>,
    pub bosses: Collection<Boss>,
    pub weapons: Collection<Weapon>,
    pub armors: Collection<Armor>,
    client: Client,
    database_name: String
}


impl GameDatabase {
    pub async fn new() -> mongodb::error::Result<Self> {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("mongodb://localhost:27017"));
        let client_options = ClientOptions::parse(&database_url).await?;
        let client = Client::with_options(client_options)?;
        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| String::from("yubot"));

        let users = client.database(&database_name).collection("users");
        let bosses = client.database(&database_name).collection("bosses");
        let weapons = client.database(&database_name).collection("weapons");
        let armors = client.database(&database_name).collection("armors");


        Ok(Self { users, bosses, weapons, armors, client, database_name })
    }

    pub async fn initialize_weapons_and_armors(&self) -> mongodb::error::Result<()> {
        // Delete all weapons and armors
        self.weapons.delete_many(doc! {}, None).await?;
        self.armors.delete_many(doc! {}, None).await?;
    
        // Insert the new weapons and armors
        let weapons_list = weapon_d2();
        let armors_list = armor_d2();

        self.add_armors(armors_list).await?;
        self.add_weapons(weapons_list).await?;
        
        Ok(())
    }

    pub async fn find_user(&self, user_id: &str) -> mongodb::error::Result<Option<User>> {
        let filter = doc! { "user_id": user_id };
        self.users.find_one(filter, None).await
    }

    pub async fn update_user(&self, user: &User) -> mongodb::error::Result<mongodb::results::UpdateResult> {
        let filter = doc! { "user_id": &user.user_id };
        let update = doc! {
            "$set": {
                "user_name": &user.user_name,
                "wins": user.wins,
                "losses": user.losses,
                "total_games": user.total_games,
                "weapon": {
                    "name": &user.weapon.name,
                    "power": user.weapon.power_level,
                },
                "armor": {
                    "name": &user.armor.name,
                    "power": user.armor.power_level,
                },
            }
        };
        self.users.update_one(filter, update, None).await
    }

    pub async fn add_weapons(&self, weapons: Vec<Weapon>) -> mongodb::error::Result<mongodb::results::InsertManyResult> {
        self.weapons.insert_many(weapons, None).await
    }

    pub async fn add_armors(&self, armors: Vec<Armor>) -> mongodb::error::Result<mongodb::results::InsertManyResult> {
        self.armors.insert_many(armors, None).await
    }

    pub async fn get_random_document(&self, aggregate: &str) -> mongodb::error::Result<Bson> {
        let command = doc! {
            "aggregate": aggregate,  // replace "weapons" with your actual collection name
            "pipeline": [
                {"$sample": {"size": 1}}
            ],
            "cursor": {}
        };
    
        let result = self.client.database(&self.database_name).run_command(command, None).await?;
    
        match result.get_document("cursor") {
            Ok(cursor) => match cursor.get_array("firstBatch") {
                Ok(batch) => match batch.get(0) {
                    Some(Bson::Document(doc)) => Ok(Bson::Document(doc.clone())),
                    _ => Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "No document found"))),
                },
                Err(_) => Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "No document found"))),
            },
            Err(_) => Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "No document found"))),
        }
    }

    pub async fn register_user(&self, user: User) -> Result<mongodb::results::InsertOneResult, anyhow::Error> {
        // Convert the User object into a MongoDB document    
        self.users.insert_one(&user, None).await
            .map_err(|err| anyhow::anyhow!("Failed to insert user: {}", err))
    }
}
