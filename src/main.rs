#[macro_use]
extern crate lazy_static;

mod routes;
mod handlers;
mod models;
mod utils;
mod constants;
mod database;
mod game;

use std::sync::{Arc};

use actix_web::{HttpServer, App, web};
use actix_rt;
use database::GameDatabase;
use env_logger::Env;
use log::info;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let game_database = Arc::new(GameDatabase::new().await?);
    info!("game_database inited");
    // Initialize weapons and armors
    game_database.initialize_weapons_and_armors().await?;
    info!("initialize_weapons_and_armors inited");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_database.clone()))  // again, clone the Arc
            .configure(routes::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}