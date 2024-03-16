use actix_web::{middleware::Logger, HttpServer};
use rust_rest_api::core::{app::app, config::Config};

#[cfg(test)]
mod tests;

// Your existing main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Config::init();
    let server = HttpServer::new(move || {
        app()
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?;
    server.run().await
}
