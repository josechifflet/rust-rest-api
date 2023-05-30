#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let DATABASE_URL = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let JWT_PRIVATE_KEY =
            std::env::var("JWT_PRIVATE_KEY").expect("JWT_PRIVATE_KEY must be set");
        let JWT_PUBLIC_KEY = std::env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY must be set");
        let JWT_EXPIRED_IN = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");

        Config {
            DATABASE_URL,
            JWT_PRIVATE_KEY,
            JWT_PUBLIC_KEY,
            JWT_EXPIRED_IN,
        }
    }
}
