use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

use api::{router, AppState};

fn test_state(pool: sqlx::PgPool) -> AppState {
    AppState {
        db: pool,
        jwt_secret: "test-secret-key-at-least-32-chars!!".into(),
        jwt_expires_in_secs: 3600,
    }
}

#[tokio::test]
async fn auth_register_login_me_flow() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/appdb".into());

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping auth E2E: database not available");
            return;
        }
    };

    api::db::run_migrations(&pool).await.expect("migrations");

    let app = router(test_state(pool));

    let email = format!("user-{}@example.com", uuid::Uuid::new_v4());
    let register_body = json!({
        "name": "Test User",
        "email": email,
        "password": "password123"
    });

    let register_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(register_response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": email,
        "password": "password123"
    });

    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(login_response.status(), StatusCode::OK);

    let login_bytes = login_response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let login_json: serde_json::Value = serde_json::from_slice(&login_bytes).expect("json");
    let token = login_json["access_token"]
        .as_str()
        .expect("access_token")
        .to_string();

    let me_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/auth/me")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(me_response.status(), StatusCode::OK);

    let me_bytes = me_response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let me_json: serde_json::Value = serde_json::from_slice(&me_bytes).expect("json");
    assert_eq!(me_json["email"], email);
}
