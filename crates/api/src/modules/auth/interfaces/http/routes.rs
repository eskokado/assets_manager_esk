use axum::{
    routing::{get, post},
    Router,
};

use super::handlers;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::me))
}
