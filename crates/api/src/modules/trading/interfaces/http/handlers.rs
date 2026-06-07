use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared_kernel::UseCase;
use uuid::Uuid;

use crate::modules::assets::domain::ports::AssetRepository;
use crate::modules::assets::infrastructure::AssetRepositorySqlx;
use crate::modules::auth::interfaces::http::middleware::AuthUser;
use crate::modules::trading::application::{
    BuyAsset, BuyAssetInput, BuyIn, FindTradeById, FindTradeInput, ListTrades, ListTradesIn,
    ListTradesQuery, SellAsset, SellAssetInput, SellIn,
};
use crate::modules::trading::domain::ports::{PortfolioRepository, TradeRepository};
use crate::modules::trading::infrastructure::{PortfolioRepositorySqlx, TradeRepositorySqlx};
use crate::AppState;

fn asset_repository(state: &AppState) -> Arc<dyn AssetRepository> {
    Arc::new(AssetRepositorySqlx::new(state.db.clone()))
}

fn trade_repository(state: &AppState) -> Arc<dyn TradeRepository> {
    Arc::new(TradeRepositorySqlx::new(state.db.clone()))
}

fn portfolio_repository(state: &AppState) -> Arc<dyn PortfolioRepository> {
    Arc::new(PortfolioRepositorySqlx::new(state.db.clone()))
}

fn map_errors(errors: &[shared_kernel::DomainError], default_status: StatusCode) -> Response {
    let message = errors
        .iter()
        .map(|e| e.0.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let status = if message.contains("not found") {
        StatusCode::NOT_FOUND
    } else if message.contains("Insufficient balance")
        || message.contains("inactive")
        || message.contains("open positions")
    {
        StatusCode::CONFLICT
    } else {
        default_status
    };
    (status, message).into_response()
}

pub async fn buy_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(input): Json<BuyIn>,
) -> Response {
    let use_case = BuyAsset::new(
        state.db.clone(),
        asset_repository(&state),
        trade_repository(&state),
        portfolio_repository(&state),
    );
    match use_case
        .execute(BuyAssetInput {
            user_id: auth.user_id,
            data: input,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => (StatusCode::CREATED, Json(out)).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn sell_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(input): Json<SellIn>,
) -> Response {
    let use_case = SellAsset::new(
        state.db.clone(),
        asset_repository(&state),
        trade_repository(&state),
        portfolio_repository(&state),
    );
    match use_case
        .execute(SellAssetInput {
            user_id: auth.user_id,
            data: input,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => (StatusCode::CREATED, Json(out)).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn list_trades(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListTradesIn>,
) -> Response {
    let use_case = ListTrades::new(trade_repository(&state));
    match use_case
        .execute(ListTradesQuery {
            user_id: auth.user_id,
            filters: query,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn get_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Response {
    let use_case = FindTradeById::new(trade_repository(&state));
    match use_case
        .execute(FindTradeInput {
            user_id: auth.user_id,
            trade_id: id,
        })
        .await
    {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}
