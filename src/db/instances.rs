use sqlx::{AssertSqlSafe, SqlitePool};

use crate::models::InstanceView;

pub struct InstancePatch {
    pub definition_id: Option<i64>,
    pub webhook_url: Option<Option<String>>,
    pub is_active: Option<bool>,
}

macro_rules! join_query {
    ($where:literal) => {
        sqlx::query_as::<_, InstanceView>(AssertSqlSafe(concat!(
            "SELECT i.id, i.client_id, i.external_id, i.definition_id,
                    i.admin_token, i.submit_token, i.webhook_url, i.is_active,
                    i.created_at, i.updated_at, d.data
             FROM forms i
             JOIN form_definitions d ON i.definition_id = d.id ",
            $where
        )))
    };
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<InstanceView>> {
    join_query!("WHERE i.id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_client_and_external(
    pool: &SqlitePool,
    client_id: i64,
    external_id: &str,
) -> sqlx::Result<Option<InstanceView>> {
    join_query!("WHERE i.client_id = ? AND i.external_id = ?")
        .bind(client_id)
        .bind(external_id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_client_name_and_external(
    pool: &SqlitePool,
    client_name: &str,
    external_id: &str,
) -> sqlx::Result<Option<InstanceView>> {
    join_query!(
        "JOIN clients c ON i.client_id = c.id
         WHERE c.name = ? AND i.external_id = ? AND i.is_active = TRUE AND c.is_active = TRUE"
    )
    .bind(client_name)
    .bind(external_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_admin_token(
    pool: &SqlitePool,
    token: &str,
) -> sqlx::Result<Option<InstanceView>> {
    join_query!("WHERE i.admin_token = ?")
        .bind(token)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_submit_token(
    pool: &SqlitePool,
    token: &str,
) -> sqlx::Result<Option<InstanceView>> {
    join_query!("WHERE i.submit_token = ? AND i.is_active = TRUE")
        .bind(token)
        .fetch_optional(pool)
        .await
}

/// Create or replace a form (client PUT). Preserves admin/submit tokens on conflict.
pub async fn upsert(
    pool: &SqlitePool,
    client_id: i64,
    external_id: &str,
    definition_id: i64,
    admin_token: &str,
    submit_token: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<InstanceView> {
    sqlx::query(
        "INSERT OR IGNORE INTO forms
         (client_id, external_id, definition_id, admin_token, submit_token, webhook_url)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(client_id).bind(external_id).bind(definition_id)
    .bind(admin_token).bind(submit_token).bind(webhook_url)
    .execute(pool)
    .await?;

    sqlx::query(
        "UPDATE forms SET definition_id = ?, webhook_url = ?, updated_at = datetime('now')
         WHERE client_id = ? AND external_id = ?",
    )
    .bind(definition_id).bind(webhook_url)
    .bind(client_id).bind(external_id)
    .execute(pool)
    .await?;

    find_by_client_and_external(pool, client_id, external_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

/// Partial update. None fields are left unchanged.
pub async fn patch(pool: &SqlitePool, id: i64, p: InstancePatch) -> sqlx::Result<InstanceView> {
    let current = find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)?;

    let definition_id = p.definition_id.unwrap_or(current.definition_id);
    let webhook_url: Option<&str> = match &p.webhook_url {
        None => current.webhook_url.as_deref(),
        Some(v) => v.as_deref(),
    };
    let is_active = p.is_active.unwrap_or(current.is_active);

    sqlx::query(
        "UPDATE forms SET definition_id = ?, webhook_url = ?, is_active = ?,
         updated_at = datetime('now') WHERE id = ?",
    )
    .bind(definition_id).bind(webhook_url).bind(is_active).bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)
}

/// Returns true if deleted, false if not found / wrong client.
pub async fn delete(pool: &SqlitePool, id: i64, client_id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM forms WHERE id = ? AND client_id = ?")
        .bind(id).bind(client_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
