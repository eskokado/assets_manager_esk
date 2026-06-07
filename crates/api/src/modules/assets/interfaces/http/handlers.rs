use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared_kernel::UseCase;
use uuid::Uuid;

use crate::modules::assets::application::{
    CreateAsset, CreateAssetIn, FindAssetById, ListAssets, ListAssetsIn, UpdateAsset,
    UpdateAssetIn, UpdateAssetInput,
};
use crate::modules::assets::domain::ports::AssetRepository;
use crate::modules::assets::domain::ports::PositionChecker;
use crate::modules::assets::infrastructure::{AssetRepositorySqlx, PositionCheckerStub};
use crate::modules::assets::interfaces::http::middleware::RequireAdmin;
use crate::modules::auth::interfaces::http::middleware::AuthUser;
use crate::AppState;

fn asset_repository(state: &AppState) -> Arc<dyn AssetRepository> {
    Arc::new(AssetRepositorySqlx::new(state.db.clone()))
}

fn position_checker() -> Arc<dyn PositionChecker> {
    Arc::new(PositionCheckerStub)
}

fn map_errors(errors: &[shared_kernel::DomainError], default_status: StatusCode) -> Response {
    let message = errors
        .iter()
        .map(|e| e.0.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let status = if message.contains("not found") {
        StatusCode::NOT_FOUND
    } else if message.contains("already exists") || message.contains("open positions") {
        StatusCode::CONFLICT
    } else {
        default_status
    };
    (status, message).into_response()
}

pub async fn list_assets(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListAssetsIn>,
) -> Response {
    let use_case = ListAssets::new(asset_repository(&state));
    match use_case.execute(query).await {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn get_asset(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Response {
    let use_case = FindAssetById::new(asset_repository(&state));
    match use_case.execute(id).await {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn create_asset(
    State(state): State<AppState>,
    _admin: RequireAdmin,
    Json(input): Json<CreateAssetIn>,
) -> Response {
    let use_case = CreateAsset::new(asset_repository(&state));
    match use_case.execute(input).await {
        shared_kernel::Result::Ok(out) => (StatusCode::CREATED, Json(out)).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

pub async fn update_asset(
    State(state): State<AppState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateAssetIn>,
) -> Response {
    let use_case = UpdateAsset::new(asset_repository(&state), position_checker());
    match use_case.execute(UpdateAssetInput { id, data: input }).await {
        shared_kernel::Result::Ok(out) => Json(out).into_response(),
        shared_kernel::Result::Err(errors) => map_errors(&errors, StatusCode::BAD_REQUEST),
    }
}

#[cfg(test)]
mod tests {}
