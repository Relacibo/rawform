pub mod admin;
pub mod forms;
pub mod submit;

use axum::{Router, routing::{get, patch, post, put}};
use sqlx::SqlitePool;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        // Client-authenticated form management
        .route("/forms/:client_name/:external_id", put(forms::put_form))
        .route("/forms/:client_name/:external_id", get(forms::get_form))
        .route("/forms/:client_name/:external_id", patch(forms::patch_form))
        // Admin token access
        .route("/admin/forms/:admin_token", get(admin::get_form))
        .route("/admin/forms/:admin_token", put(admin::put_form))
        .route("/admin/forms/:admin_token", patch(admin::patch_form))
        // Form submission
        .route("/submit/:submit_token", get(submit::get_form))
        .route("/submit/:submit_token", post(submit::post_submit))
        .with_state(pool)
}
