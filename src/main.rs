use actix_web::{middleware, App, HttpServer};
use rust_rest_api::v1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .configure(v1::router::config_app)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}