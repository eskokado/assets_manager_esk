use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AssetDto {
    pub id: String,
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub currency: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListAssetsResponseDto {
    pub items: Vec<AssetDto>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize)]
pub struct CreateAssetRequestDto {
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateAssetRequestDto {
    pub name: String,
    pub asset_type: String,
    pub currency: String,
    pub active: bool,
}
