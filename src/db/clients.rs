use crate::{
    db::{DbPool, select_timestamp_columns},
    models::Client,
};

pub async fn find_by_name(pool: &DbPool, name: &str) -> sqlx::Result<Option<Client>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT id, name, api_key_hash, is_active, {ts}
         FROM clients WHERE name = $1 AND is_active = TRUE"
        )
    } else {
        format!(
            "SELECT id, name, api_key_hash, is_active, {ts}
         FROM clients WHERE name = ? AND is_active = TRUE"
        )
    };
    sqlx::query_as::<_, Client>(sqlx::AssertSqlSafe(query))
        .bind(name)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &DbPool, id: i64) -> sqlx::Result<Option<Client>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!("SELECT id, name, api_key_hash, is_active, {ts} FROM clients WHERE id = $1")
    } else {
        format!("SELECT id, name, api_key_hash, is_active, {ts} FROM clients WHERE id = ?")
    };
    sqlx::query_as::<_, Client>(sqlx::AssertSqlSafe(query))
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &DbPool) -> sqlx::Result<Vec<Client>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = format!(
        "SELECT id, name, api_key_hash, is_active, {ts}
     FROM clients ORDER BY id"
    );
    sqlx::query_as::<_, Client>(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await
}

pub async fn insert(pool: &DbPool, name: &str, api_key_hash: &str) -> sqlx::Result<Client> {
    let query = if cfg!(feature = "postgres") {
        "INSERT INTO clients (name, api_key_hash) VALUES ($1, $2)"
    } else {
        "INSERT INTO clients (name, api_key_hash) VALUES (?, ?)"
    };
    sqlx::query(query)
        .bind(name)
        .bind(api_key_hash)
        .execute(pool)
        .await?;

    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!("SELECT id, name, api_key_hash, is_active, {ts} FROM clients WHERE name = $1")
    } else {
        format!("SELECT id, name, api_key_hash, is_active, {ts} FROM clients WHERE name = ?")
    };
    sqlx::query_as::<_, Client>(sqlx::AssertSqlSafe(query))
        .bind(name)
        .fetch_one(pool)
        .await
}

pub async fn set_active(pool: &DbPool, id: i64, is_active: bool) -> sqlx::Result<()> {
    let query = if cfg!(feature = "postgres") {
        "UPDATE clients SET is_active = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    } else {
        "UPDATE clients SET is_active = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    };
    sqlx::query(query)
        .bind(is_active)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Replace the stored API key hash (key rotation).
pub async fn set_api_key_hash(pool: &DbPool, id: i64, api_key_hash: &str) -> sqlx::Result<()> {
    let query = if cfg!(feature = "postgres") {
        "UPDATE clients SET api_key_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    } else {
        "UPDATE clients SET api_key_hash = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    };
    sqlx::query(query)
        .bind(api_key_hash)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
