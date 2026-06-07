use axum::{routing::get, Router};

use super::handlers;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/trades/buy", axum::routing::post(handlers::buy_trade))
        .route(
            "/api/trades/sell",
            axum::routing::post(handlers::sell_trade),
        )
        .route("/api/trades", get(handlers::list_trades))
        .route("/api/trades/{id}", get(handlers::get_trade))
}
