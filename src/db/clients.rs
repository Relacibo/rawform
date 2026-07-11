use sqlx::SqlitePool;

use crate::models::Client;

pub async fn find_by_name(pool: &SqlitePool, name: &str) -> sqlx::Result<Option<Client>> {
    sqlx::query_as::<_, Client>(
        "SELECT id, name, api_key_hash, is_active, created_at, updated_at
         FROM clients WHERE name = ? AND is_active = TRUE",
    )
    .bind(name)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Client>> {
    sqlx::query_as::<_, Client>(
        "SELECT id, name, api_key_hash, is_active, created_at, updated_at
         FROM clients WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list(pool: &SqlitePool) -> sqlx::Result<Vec<Client>> {
    sqlx::query_as::<_, Client>(
        "SELECT id, name, api_key_hash, is_active, created_at, updated_at
         FROM clients ORDER BY id",
    )
    .fetch_all(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, name: &str, api_key_hash: &str) -> sqlx::Result<Client> {
    sqlx::query("INSERT INTO clients (name, api_key_hash) VALUES (?, ?)")
        .bind(name)
        .bind(api_key_hash)
        .execute(pool)
        .await?;

    sqlx::query_as::<_, Client>(
        "SELECT id, name, api_key_hash, is_active, created_at, updated_at
         FROM clients WHERE name = ?",
    )
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn set_active(pool: &SqlitePool, id: i64, is_active: bool) -> sqlx::Result<()> {
    sqlx::query("UPDATE clients SET is_active = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(is_active)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Replace the stored API key hash (key rotation).
pub async fn set_api_key_hash(pool: &SqlitePool, id: i64, api_key_hash: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE clients SET api_key_hash = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(api_key_hash)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
