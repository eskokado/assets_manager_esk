pub mod config;
pub mod db;
pub mod modules;

use axum::Router;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: i64,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(modules::health::routes())
        .merge(modules::auth::interfaces::routes())
        .with_state(state)
}
