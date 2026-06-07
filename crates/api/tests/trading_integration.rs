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
async fn trading_buy_sell_flow() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/appdb".into());

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping trading E2E: database not available");
            return;
        }
    };

    api::db::run_migrations(&pool).await.expect("migrations");

    let app = router(test_state(pool));

    let investor_email = format!("trader-{}@example.com", uuid::Uuid::new_v4());
    let register_body = json!({
        "name": "Trader",
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

    let token = login_token(&app, &investor_email, "password123").await;
    let asset_id = "00000000-0000-4000-8000-000000000101";

    let buy_body = json!({
        "asset_id": asset_id,
        "quantity": "10",
        "unit_price": "25.50"
    });

    let buy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/trades/buy")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(buy_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(buy_response.status(), StatusCode::CREATED);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/trades")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(list_response.status(), StatusCode::OK);

    let sell_body = json!({
        "asset_id": asset_id,
        "quantity": "3",
        "unit_price": "26.00"
    });

    let sell_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/trades/sell")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(sell_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(sell_response.status(), StatusCode::CREATED);

    let excess_sell = json!({
        "asset_id": asset_id,
        "quantity": "100",
        "unit_price": "26.00"
    });

    let excess_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/trades/sell")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(excess_sell.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(excess_response.status(), StatusCode::CONFLICT);

    let admin_token = login_token(&app, "admin@assets.local", "Senha@12345678").await;
    let inactive_body = json!({
        "name": "Petrobras PN",
        "asset_type": "STOCK",
        "currency": "BRL",
        "active": false
    });

    let deactivate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/assets/{asset_id}"))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::from(inactive_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(deactivate_response.status(), StatusCode::CONFLICT);

    let buy_inactive = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/trades/buy")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(buy_body.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(buy_inactive.status(), StatusCode::CREATED);
}
