use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}
