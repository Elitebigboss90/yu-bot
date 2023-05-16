use std::sync::Arc;

use actix_web::{web, Responder, get, HttpResponse};
use log::info;
use crate::{handlers::{self, receive_webhook}, database::GameDatabase};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(test_handler))
            .route("/acceptMessage", web::post().to(receive_webhook))
    );
}

async fn test_handler(game_database: web::Data<Arc<GameDatabase>>) -> impl Responder {
    game_database.test_database();
    info!("Test handler called");
    HttpResponse::Ok().body("Test successful")
}