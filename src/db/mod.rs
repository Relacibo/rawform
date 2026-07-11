pub mod clients;
pub mod forms;

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Enable only one database backend feature at a time.");
#[cfg(not(any(feature = "sqlite", feature = "postgres")))]
compile_error!("Enable one database backend feature: sqlite or postgres.");

#[cfg(feature = "sqlite")]
pub type DbPool = sqlx::SqlitePool;
#[cfg(feature = "postgres")]
pub type DbPool = sqlx::PgPool;

#[cfg(feature = "sqlite")]
pub type Db = sqlx::Sqlite;
#[cfg(feature = "postgres")]
pub type Db = sqlx::Postgres;

#[cfg(feature = "postgres")]
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[cfg(feature = "sqlite")]
pub async fn connect(database_url: &str) -> anyhow::Result<DbPool> {
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

#[cfg(feature = "postgres")]
pub async fn connect(database_url: &str) -> anyhow::Result<DbPool> {
    let opts = PgConnectOptions::from_str(database_url)?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    Ok(pool)
}

#[cfg(feature = "sqlite")]
pub async fn migrate(pool: &DbPool) -> anyhow::Result<()> {
    sqlx::query(include_str!("../../migrations/001_create_clients.sql"))
        .execute(pool)
        .await?;
    sqlx::query(include_str!("../../migrations/002_create_forms.sql"))
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(feature = "postgres")]
pub async fn migrate(pool: &DbPool) -> anyhow::Result<()> {
    sqlx::query(include_str!(
        "../../migrations/postgres/001_create_clients.sql"
    ))
    .execute(pool)
    .await?;
    sqlx::query(include_str!(
        "../../migrations/postgres/002_create_forms.sql"
    ))
    .execute(pool)
    .await?;
    Ok(())
}

pub fn select_timestamp_columns(created_at: &str, updated_at: &str) -> String {
    #[cfg(feature = "sqlite")]
    {
        format!("{created_at} AS created_at, {updated_at} AS updated_at")
    }

    #[cfg(feature = "postgres")]
    {
        format!("{created_at}::text AS created_at, {updated_at}::text AS updated_at")
    }
}
