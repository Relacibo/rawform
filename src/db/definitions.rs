use sqlx::SqlitePool;

use crate::models::Definition;

pub async fn insert(pool: &SqlitePool, client_id: i64, data: &str) -> sqlx::Result<Definition> {
    sqlx::query("INSERT INTO form_definitions (client_id, data) VALUES (?, ?)")
        .bind(client_id)
        .bind(data)
        .execute(pool)
        .await?;
    sqlx::query_as::<_, Definition>(
        "SELECT id, client_id, data, created_at FROM form_definitions
         WHERE client_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(client_id)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Definition>> {
    sqlx::query_as::<_, Definition>(
        "SELECT id, client_id, data, created_at FROM form_definitions WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_by_client(pool: &SqlitePool, client_id: i64) -> sqlx::Result<Vec<Definition>> {
    sqlx::query_as::<_, Definition>(
        "SELECT id, client_id, data, created_at FROM form_definitions
         WHERE client_id = ? ORDER BY id DESC",
    )
    .bind(client_id)
    .fetch_all(pool)
    .await
}

/// Returns true if a row was deleted, false if not found.
/// Fails with FK error if the definition is still referenced by a form instance.
pub async fn delete(pool: &SqlitePool, id: i64, client_id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM form_definitions WHERE id = ? AND client_id = ?")
        .bind(id)
        .bind(client_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
