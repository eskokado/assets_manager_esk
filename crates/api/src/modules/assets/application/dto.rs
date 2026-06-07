use serde::{Deserialize, Serialize};
use shared_kernel::Entity;

use crate::modules::assets::domain::{Asset, AssetType};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAssetIn {
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAssetIn {
    pub name: String,
    pub asset_type: String,
    pub currency: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetOut {
    pub id: String,
    pub ticker: String,
    pub name: String,
    pub asset_type: AssetType,
    pub currency: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAssetsIn {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub asset_type: Option<String>,
    pub active: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListAssetsOut {
    pub items: Vec<AssetOut>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

impl AssetOut {
    pub fn from_asset(asset: &Asset) -> Self {
        Self {
            id: asset.id().to_string(),
            ticker: asset.ticker().as_str().to_string(),
            name: asset.name().as_str().to_string(),
            asset_type: asset.asset_type(),
            currency: asset.currency().as_str().to_string(),
            active: asset.active(),
            created_at: asset.created_at().to_rfc3339(),
            updated_at: asset.updated_at().to_rfc3339(),
        }
    }
}
