use crate::{
    db::{Db, DbPool, select_timestamp_columns},
    models::Form,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FormSummary {
    pub id: i64,
    pub client_id: i64,
    pub client_name: String,
    pub external_id: String,
    pub admin_token: String,
    pub submit_token: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct FormPatch {
    pub data: Option<String>,
    pub webhook_url: Option<Option<String>>,
    pub is_active: Option<bool>,
}

pub async fn find_by_id(pool: &DbPool, id: i64) -> sqlx::Result<Option<Form>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE id = $1"
        )
    } else {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE id = ?"
        )
    };
    sqlx::query_as::<_, Form>(sqlx::AssertSqlSafe(query))
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_client_and_external(
    pool: &DbPool,
    client_id: i64,
    external_id: &str,
) -> sqlx::Result<Option<Form>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE client_id = $1 AND external_id = $2"
        )
    } else {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE client_id = ? AND external_id = ?"
        )
    };
    sqlx::query_as::<_, Form>(sqlx::AssertSqlSafe(query))
        .bind(client_id)
        .bind(external_id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_client_name_and_external(
    pool: &DbPool,
    client_name: &str,
    external_id: &str,
) -> sqlx::Result<Option<Form>> {
    let ts = select_timestamp_columns("f.created_at", "f.updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT f.id, f.client_id, f.external_id, f.data, f.admin_token, f.submit_token,
                f.webhook_url, f.is_active, {ts}
         FROM forms f
         JOIN clients c ON f.client_id = c.id
         WHERE c.name = $1 AND f.external_id = $2 AND f.is_active = TRUE AND c.is_active = TRUE"
        )
    } else {
        format!(
            "SELECT f.id, f.client_id, f.external_id, f.data, f.admin_token, f.submit_token,
                f.webhook_url, f.is_active, {ts}
         FROM forms f
         JOIN clients c ON f.client_id = c.id
         WHERE c.name = ? AND f.external_id = ? AND f.is_active = TRUE AND c.is_active = TRUE"
        )
    };
    sqlx::query_as::<_, Form>(sqlx::AssertSqlSafe(query))
        .bind(client_name)
        .bind(external_id)
        .fetch_optional(pool)
        .await
}

pub async fn list(
    pool: &DbPool,
    client_name: Option<&str>,
    client_id: Option<i64>,
    name: Option<&str>,
) -> sqlx::Result<Vec<FormSummary>> {
    let mut qb = sqlx::QueryBuilder::<Db>::new({
        let ts = select_timestamp_columns("f.created_at", "f.updated_at");
        format!(
            "SELECT f.id, f.client_id, c.name AS client_name, f.external_id, f.admin_token, f.submit_token, f.is_active, {ts} \
             FROM forms f JOIN clients c ON f.client_id = c.id WHERE 1=1"
        )
    });

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

pub async fn find_by_admin_token(pool: &DbPool, token: &str) -> sqlx::Result<Option<Form>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE admin_token = $1"
        )
    } else {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE admin_token = ?"
        )
    };
    sqlx::query_as::<_, Form>(sqlx::AssertSqlSafe(query))
        .bind(token)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_submit_token(pool: &DbPool, token: &str) -> sqlx::Result<Option<Form>> {
    let ts = select_timestamp_columns("created_at", "updated_at");
    let query = if cfg!(feature = "postgres") {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE submit_token = $1 AND is_active = TRUE"
        )
    } else {
        format!(
            "SELECT id, client_id, external_id, data, admin_token, submit_token,
                webhook_url, is_active, {ts}
         FROM forms WHERE submit_token = ? AND is_active = TRUE"
        )
    };
    sqlx::query_as::<_, Form>(sqlx::AssertSqlSafe(query))
        .bind(token)
        .fetch_optional(pool)
        .await
}

pub async fn upsert(
    pool: &DbPool,
    client_id: i64,
    external_id: &str,
    data: &str,
    admin_token: &str,
    submit_token: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<Form> {
    let insert = if cfg!(feature = "postgres") {
        "INSERT INTO forms (client_id, external_id, data, admin_token, submit_token, webhook_url)
     VALUES ($1, $2, $3, $4, $5, $6)
     ON CONFLICT (client_id, external_id) DO NOTHING"
    } else {
        "INSERT OR IGNORE INTO forms
     (client_id, external_id, data, admin_token, submit_token, webhook_url)
     VALUES (?, ?, ?, ?, ?, ?)"
    };
    sqlx::query(insert)
        .bind(client_id)
        .bind(external_id)
        .bind(data)
        .bind(admin_token)
        .bind(submit_token)
        .bind(webhook_url)
        .execute(pool)
        .await?;

    let update = if cfg!(feature = "postgres") {
        "UPDATE forms SET data = $1, webhook_url = $2, updated_at = CURRENT_TIMESTAMP
     WHERE client_id = $3 AND external_id = $4"
    } else {
        "UPDATE forms SET data = ?, webhook_url = ?, updated_at = CURRENT_TIMESTAMP
     WHERE client_id = ? AND external_id = ?"
    };
    sqlx::query(update)
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

pub async fn patch(pool: &DbPool, id: i64, p: FormPatch) -> sqlx::Result<Form> {
    let current = find_by_id(pool, id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let data = p.data.as_deref().unwrap_or(&current.data);
    let webhook_url: Option<&str> = match &p.webhook_url {
        None => current.webhook_url.as_deref(),
        Some(v) => v.as_deref(),
    };
    let is_active = p.is_active.unwrap_or(current.is_active);

    let update = if cfg!(feature = "postgres") {
        "UPDATE forms SET data = $1, webhook_url = $2, is_active = $3, updated_at = CURRENT_TIMESTAMP
     WHERE id = $4"
    } else {
        "UPDATE forms SET data = ?, webhook_url = ?, is_active = ?, updated_at = CURRENT_TIMESTAMP
     WHERE id = ?"
    };
    sqlx::query(update)
        .bind(data)
        .bind(webhook_url)
        .bind(is_active)
        .bind(id)
        .execute(pool)
        .await?;

    find_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)
}

pub async fn delete(pool: &DbPool, id: i64, client_id: i64) -> sqlx::Result<bool> {
    let query = if cfg!(feature = "postgres") {
        "DELETE FROM forms WHERE id = $1 AND client_id = $2"
    } else {
        "DELETE FROM forms WHERE id = ? AND client_id = ?"
    };
    let result = sqlx::query(query)
        .bind(id)
        .bind(client_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
