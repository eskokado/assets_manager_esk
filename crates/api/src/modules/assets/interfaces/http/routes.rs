use axum::{routing::get, Router};

use super::handlers;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/assets",
            get(handlers::list_assets).post(handlers::create_asset),
        )
        .route(
            "/api/assets/{id}",
            get(handlers::get_asset).put(handlers::update_asset),
        )
}
