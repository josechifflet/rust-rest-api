use actix_web::web;
use crate::v1::todo::router::todo_config;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(todo_config());
}