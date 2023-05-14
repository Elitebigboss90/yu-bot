mod routes;
mod handlers;
mod models;
mod utils;

use actix_web::{HttpServer, App};
use actix_rt;
use env_logger::Env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    HttpServer::new(|| {
        App::new().configure(routes::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
