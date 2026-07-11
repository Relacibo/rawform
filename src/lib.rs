pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod models;

use axum::Router;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .nest("/api/v1", api::router(pool))
        .nest_service("/", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
