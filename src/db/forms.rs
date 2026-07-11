use sqlx::SqlitePool;

use crate::models::Form;

/// Partial update payload. `None` means "leave unchanged".
/// For `webhook_url`: `Some(None)` clears it, `Some(Some(url))` sets it.
pub struct FormPatch {
    pub data: Option<String>,
    pub webhook_url: Option<Option<String>>,
    pub is_active: Option<bool>,
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, data, is_active, client_id, external_id,
                admin_token, submit_token, webhook_url, created_at, updated_at
         FROM forms WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_client_and_external(
    pool: &SqlitePool,
    client_id: i64,
    external_id: &str,
) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, data, is_active, client_id, external_id,
                admin_token, submit_token, webhook_url, created_at, updated_at
         FROM forms WHERE client_id = ? AND external_id = ?",
    )
    .bind(client_id)
    .bind(external_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_admin_token(pool: &SqlitePool, token: &str) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, data, is_active, client_id, external_id,
                admin_token, submit_token, webhook_url, created_at, updated_at
         FROM forms WHERE admin_token = ?",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_submit_token(pool: &SqlitePool, token: &str) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, data, is_active, client_id, external_id,
                admin_token, submit_token, webhook_url, created_at, updated_at
         FROM forms WHERE submit_token = ? AND is_active = TRUE",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn list_by_client(pool: &SqlitePool, client_id: i64) -> sqlx::Result<Vec<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, data, is_active, client_id, external_id,
                admin_token, submit_token, webhook_url, created_at, updated_at
         FROM forms WHERE client_id = ? ORDER BY id",
    )
    .bind(client_id)
    .fetch_all(pool)
    .await
}

/// Create or replace a form (client PUT). Preserves admin_token/submit_token on conflict.
pub async fn upsert(
    pool: &SqlitePool,
    client_id: i64,
    external_id: &str,
    data: &str,
    admin_token: &str,
    submit_token: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<Form> {
    // INSERT OR IGNORE preserves existing tokens on conflict
    sqlx::query(
        "INSERT OR IGNORE INTO forms
         (client_id, external_id, data, admin_token, submit_token, webhook_url)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(client_id)
    .bind(external_id)
    .bind(data)
    .bind(admin_token)
    .bind(submit_token)
    .bind(webhook_url)
    .execute(pool)
    .await?;

    sqlx::query(
        "UPDATE forms SET data = ?, webhook_url = ?, updated_at = datetime('now')
         WHERE client_id = ? AND external_id = ?",
    )
    .bind(data)
    .bind(webhook_url)
    .bind(client_id)
    .bind(external_id)
    .execute(pool)
    .await?;

    find_by_client_and_external(pool, client_id, external_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

/// Full replacement of data + webhook_url (admin PUT). Tokens are never changed.
pub async fn replace(
    pool: &SqlitePool,
    id: i64,
    data: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<Form> {
    sqlx::query(
        "UPDATE forms SET data = ?, webhook_url = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(data)
    .bind(webhook_url)
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)
}

/// Partial update (PATCH). Only provided fields are changed.
pub async fn patch(pool: &SqlitePool, id: i64, p: FormPatch) -> sqlx::Result<Form> {
    let current = find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)?;

    let data = p.data.as_deref().unwrap_or(&current.data);
    let webhook_url: Option<&str> = match &p.webhook_url {
        None => current.webhook_url.as_deref(),
        Some(v) => v.as_deref(),
    };
    let is_active = p.is_active.unwrap_or(current.is_active);

    sqlx::query(
        "UPDATE forms SET data = ?, webhook_url = ?, is_active = ?, updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(data)
    .bind(webhook_url)
    .bind(is_active)
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)
}
