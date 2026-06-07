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
async fn portfolio_after_trades_flow() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/appdb".into());

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping portfolio E2E: database not available");
            return;
        }
    };

    api::db::run_migrations(&pool).await.expect("migrations");

    let app = router(test_state(pool));

    let investor_email = format!("holder-{}@example.com", uuid::Uuid::new_v4());
    let register_body = json!({
        "name": "Holder",
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

    let empty_portfolio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/portfolio")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(empty_portfolio.status(), StatusCode::OK);
    let empty_bytes = empty_portfolio
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let empty_json: serde_json::Value = serde_json::from_slice(&empty_bytes).expect("json");
    assert_eq!(empty_json["positions"].as_array().unwrap().len(), 0);

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

    let portfolio_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/portfolio")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(portfolio_response.status(), StatusCode::OK);

    let portfolio_bytes = portfolio_response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let portfolio_json: serde_json::Value = serde_json::from_slice(&portfolio_bytes).expect("json");
    let positions = portfolio_json["positions"].as_array().unwrap();
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0]["quantity"].as_str().unwrap(), "10");
    assert_eq!(positions[0]["average_price"].as_str().unwrap(), "25.5");
    assert_eq!(positions[0]["asset_ticker"].as_str().unwrap(), "PETR4");

    let detail_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/portfolio/assets/{asset_id}"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(detail_response.status(), StatusCode::OK);

    let sell_all = json!({
        "asset_id": asset_id,
        "quantity": "10",
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
                .body(Body::from(sell_all.to_string()))
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(sell_response.status(), StatusCode::CREATED);

    let detail_after_sell = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/portfolio/assets/{asset_id}"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(detail_after_sell.status(), StatusCode::NOT_FOUND);

    let portfolio_empty = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/portfolio")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");
    assert_eq!(portfolio_empty.status(), StatusCode::OK);
    let empty_after_bytes = portfolio_empty
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let empty_after_json: serde_json::Value =
        serde_json::from_slice(&empty_after_bytes).expect("json");
    assert_eq!(empty_after_json["positions"].as_array().unwrap().len(), 0);
}
