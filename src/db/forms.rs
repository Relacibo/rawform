use sqlx::SqlitePool;

use crate::models::Form;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FormSummary {
    pub id: i64,
    pub client_id: i64,
    pub client_name: String,
    pub external_id: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct FormPatch {
    pub data: Option<String>,
    pub webhook_url: Option<Option<String>>,
    pub is_active: Option<bool>,
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, created_at, updated_at
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
        "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, created_at, updated_at
         FROM forms WHERE client_id = ? AND external_id = ?",
    )
    .bind(client_id)
    .bind(external_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_client_name_and_external(
    pool: &SqlitePool,
    client_name: &str,
    external_id: &str,
) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT f.id, f.client_id, f.external_id, f.data, f.admin_token, f.submit_token,
                f.webhook_url, f.is_active, f.created_at, f.updated_at
         FROM forms f
         JOIN clients c ON f.client_id = c.id
         WHERE c.name = ? AND f.external_id = ? AND f.is_active = TRUE AND c.is_active = TRUE",
    )
    .bind(client_name)
    .bind(external_id)
    .fetch_optional(pool)
    .await
}

pub async fn list(
    pool: &SqlitePool,
    client_name: Option<&str>,
    client_id: Option<i64>,
    name: Option<&str>,
) -> sqlx::Result<Vec<FormSummary>> {
    let mut qb = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
        "SELECT f.id, f.client_id, c.name AS client_name, f.external_id, f.is_active, f.created_at, f.updated_at \
         FROM forms f JOIN clients c ON f.client_id = c.id WHERE 1=1",
    );

    if let Some(client_name) = client_name {
        qb.push(" AND c.name = ").push_bind(client_name);
    }
    if let Some(client_id) = client_id {
        qb.push(" AND f.client_id = ").push_bind(client_id);
    }
    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        qb.push(" AND f.external_id LIKE ").push_bind(pattern);
    }

    qb.push(" ORDER BY c.name, f.external_id");

    qb.build_query_as::<FormSummary>().fetch_all(pool).await
}

pub async fn find_by_admin_token(pool: &SqlitePool, token: &str) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, created_at, updated_at
         FROM forms WHERE admin_token = ?",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_submit_token(pool: &SqlitePool, token: &str) -> sqlx::Result<Option<Form>> {
    sqlx::query_as::<_, Form>(
        "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, created_at, updated_at
         FROM forms WHERE submit_token = ? AND is_active = TRUE",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn upsert(
    pool: &SqlitePool,
    client_id: i64,
    external_id: &str,
    data: &str,
    admin_token: &str,
    submit_token: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<Form> {
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

pub async fn patch(pool: &SqlitePool, id: i64, p: FormPatch) -> sqlx::Result<Form> {
    let current = find_by_id(pool, id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

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

pub async fn delete(pool: &SqlitePool, id: i64, client_id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM forms WHERE id = ? AND client_id = ?")
        .bind(id)
        .bind(client_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
