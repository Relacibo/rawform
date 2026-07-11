pub mod admin;
pub mod auth;
pub mod definitions;
pub mod forms;
pub mod serde_util;
pub mod submit;

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use sqlx::SqlitePool;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        // Form instances (client auth)
        .route("/forms/{client_name}/{external_id}", put(forms::put_form))
        .route("/forms/{client_name}/{external_id}", get(forms::get_form))
        .route(
            "/forms/{client_name}/{external_id}",
            patch(forms::patch_form),
        )
        .route(
            "/forms/{client_name}/{external_id}",
            delete(forms::delete_form),
        )
        // Form definitions (client auth)
        .route(
            "/definitions/{client_name}/{definition_id}",
            delete(definitions::delete_definition),
        )
        // Admin token access
        .route("/admin/forms/{admin_token}", get(admin::get_form))
        .route("/admin/forms/{admin_token}", put(admin::put_form))
        .route("/admin/forms/{admin_token}", patch(admin::patch_form))
        // Public submit
        .route(
            "/submit/{client_name}/{external_id}/token",
            get(submit::get_token),
        )
        .route("/submit/{submit_token}", get(submit::get_form))
        .route("/submit/{submit_token}", post(submit::post_submit))
        .with_state(pool)
}
