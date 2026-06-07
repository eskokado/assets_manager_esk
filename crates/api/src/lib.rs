pub mod config;
pub mod db;
pub mod modules;

use axum::http::{header, Method};
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: i64,
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:3000".parse().expect("valid origin"),
            "http://127.0.0.1:3001".parse().expect("valid origin"),
        ]))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ]))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(modules::health::routes())
        .merge(modules::auth::interfaces::routes())
        .merge(modules::assets::interfaces::routes())
        .merge(modules::trading::interfaces::routes())
        .layer(cors_layer())
        .with_state(state)
}
