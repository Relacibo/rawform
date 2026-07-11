use sqlx::SqlitePool;

use crate::models::Submission;

pub async fn insert(pool: &SqlitePool, form_id: i64, values: &str) -> sqlx::Result<Submission> {
    sqlx::query("INSERT INTO submissions (form_id, values) VALUES (?, ?)")
        .bind(form_id)
        .bind(values)
        .execute(pool)
        .await?;

    sqlx::query_as::<_, Submission>(
        "SELECT id, form_id, values, submitted_at
         FROM submissions WHERE form_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(form_id)
    .fetch_one(pool)
    .await
}

pub async fn list_by_form(pool: &SqlitePool, form_id: i64) -> sqlx::Result<Vec<Submission>> {
    sqlx::query_as::<_, Submission>(
        "SELECT id, form_id, values, submitted_at
         FROM submissions WHERE form_id = ? ORDER BY id DESC",
    )
    .bind(form_id)
    .fetch_all(pool)
    .await
}
