pub mod clients;
pub mod definitions;
pub mod instances;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

pub async fn connect(database_url: &str) -> anyhow::Result<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(&mut *conn)
                    .await?;
                Ok(())
            })
        })
        .connect_with(opts)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(include_str!("../../migrations/001_create_clients.sql"))
        .execute(pool)
        .await?;
    sqlx::query(include_str!(
        "../../migrations/002_create_form_definitions.sql"
    ))
    .execute(pool)
    .await?;
    sqlx::query(include_str!("../../migrations/003_create_forms.sql"))
        .execute(pool)
        .await?;
    Ok(())
}
