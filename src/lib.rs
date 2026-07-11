pub mod api;
pub mod cli;
pub mod db;
pub mod error;
pub mod models;

use axum::Router;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

pub fn app(pool: db::DbPool) -> Router {
    Router::new()
        .nest("/api/v1", api::router(pool))
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
