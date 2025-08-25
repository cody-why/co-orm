#![allow(unused)]
fn main() {}

#[cfg(feature = "mysql")]
pub async fn get_pool() -> sqlx::Result<sqlx::MySqlPool> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::mysql::MySqlPoolOptions::new().connect(&db_url).await
}

#[cfg(feature = "postgres")]
pub async fn get_pool() -> sqlx::Result<sqlx::PgPool> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new().connect(&db_url).await
}

#[cfg(feature = "sqlite")]
pub async fn get_pool() -> sqlx::Result<sqlx::SqlitePool> {
    sqlx::sqlite::SqlitePool::connect("sqlite://:memory:").await
}
