use std::env;

use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use dotenv::dotenv;

use crate::domain::constants::POSTGRESQL_DB_URI;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

const DEFAULT_DB_POOL_MIN: u32 = 2;
const DEFAULT_DB_POOL_MAX: u32 = 10;

pub fn db_pool() -> DBConn {
    dotenv().ok();
    let database_url = env::var(POSTGRESQL_DB_URI)
        .expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .min_idle(Some(DEFAULT_DB_POOL_MIN))
        .max_size(DEFAULT_DB_POOL_MAX)
        .build(manager)
        .expect("Failed to create pool");

    pool
}

pub fn run_migrations() -> Result<(), diesel_migrations::MigrationError> {
    // Get a connection from the pool safely
    let mut conn = db_pool().get().expect("Failed to get connection from pool");

    println!("Running migrations...");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    
    Ok(())
}