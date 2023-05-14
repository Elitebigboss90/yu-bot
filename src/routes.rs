use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::receive_webhook);
}