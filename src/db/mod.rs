pub mod clients;
pub mod forms;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect(database_url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(include_str!("../../migrations/001_create_clients.sql"))
        .execute(pool)
        .await?;
    sqlx::query(include_str!("../../migrations/002_create_forms.sql"))
        .execute(pool)
        .await?;
    Ok(())
}
