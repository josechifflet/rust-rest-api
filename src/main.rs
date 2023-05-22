use actix_web::HttpServer;
use rust_rest_api::create_app::create_app;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start server...");

    let server = HttpServer::new(move || create_app()).bind(("127.0.0.1", 8080))?;

    println!("Server is running at port 8080");
    server.run().await
}
