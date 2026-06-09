use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared_kernel::UseCase;
use uuid::Uuid;

use crate::modules::auth::interfaces::http::middleware::AuthUser;
use crate::modules::portfolio::application::{
    GetPositionByAsset, GetPositionInput, GetUserPortfolio, GetUserPortfolioInput,
};
use crate::modules::portfolio::domain::PortfolioQuery;
use crate::modules::portfolio::infrastructure::PortfolioQuerySqlx;
use crate::AppState;

fn portfolio_query(state: &AppState) -> Arc<dyn PortfolioQuery> {
    Arc::new(PortfolioQuerySqlx::new(state.db.clone()))
}

fn map_errors(errors: &[shared_kernel::DomainError], default_status: StatusCode) -> Response {
    let message = errors
        .iter()
        .map(|e| e.0.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let status = if message.contains("not found") {
        StatusCode::NOT_FOUND
    } else {
        default_status
    };
    (status, message).into_response()
}

pub async fn get_portfolio(State(state): State<AppState>, auth: AuthUser) -> Response {
    let use_case = GetUserPortfolio::new(portfolio_query(&state));
    match use_case
        .execute(GetUserPortfolioInput {
            user_id: auth.user_id,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn get_position(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(asset_id): Path<Uuid>,
) -> Response {
    let use_case = GetPositionByAsset::new(portfolio_query(&state));
    match use_case
        .execute(GetPositionInput {
            user_id: auth.user_id,
            asset_id,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}
