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

async fn login_token(app: &axum::Router, email: &str, password: &str) -> String {
    let login_body = json!({ "email": email, "password": password });
    let response = app
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

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    json["access_token"]
        .as_str()
        .expect("access_token")
        .to_string()
}

#[tokio::test]
async fn assets_admin_crud_and_investor_forbidden() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/appdb".into());

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping assets E2E: database not available");
            return;
        }
    };

    api::db::run_migrations(&pool).await.expect("migrations");

    let app = router(test_state(pool));

    let admin_token = login_token(&app, "admin@assets.local", "Senha@12345678").await;

    let ticker = format!("TST{}", &uuid::Uuid::new_v4().to_string()[..4].to_uppercase());
    let create_body = json!({
        "ticker": ticker,
        "name": "Test Asset",
        "asset_type": "STOCK",
        "currency": "BRL"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assets")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::from(create_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/assets")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(list_response.status(), StatusCode::OK);

    let investor_email = format!("investor-{}@example.com", uuid::Uuid::new_v4());
    let register_body = json!({
        "name": "Investor",
        "email": investor_email,
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

    let investor_token = login_token(&app, &investor_email, "password123").await;

    let investor_list = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/assets")
                .header("authorization", format!("Bearer {investor_token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(investor_list.status(), StatusCode::OK);

    let investor_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assets")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {investor_token}"))
                .body(Body::from(create_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(investor_create.status(), StatusCode::FORBIDDEN);

    let duplicate_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assets")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::from(create_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(duplicate_create.status(), StatusCode::CONFLICT);
}
