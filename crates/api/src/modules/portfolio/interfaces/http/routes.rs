use axum::{routing::get, Router};

use super::handlers;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/portfolio", get(handlers::get_portfolio))
        .route(
            "/api/portfolio/assets/{asset_id}",
            get(handlers::get_position),
        )
}
