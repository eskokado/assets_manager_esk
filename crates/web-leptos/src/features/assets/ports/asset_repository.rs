use async_trait::async_trait;
use shared_kernel::Result;

use crate::features::assets::domain::{AssetForm, AssetListItem};

#[derive(Debug, Clone)]
pub struct ListAssetsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub asset_type: Option<String>,
    pub active: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PaginatedAssetList {
    pub items: Vec<AssetListItem>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AssetRepository: Send + Sync {
    async fn list(&self, access_token: &str, query: ListAssetsQuery) -> Result<PaginatedAssetList>;
    async fn load(&self, access_token: &str, id: &str) -> Result<AssetForm>;
    async fn save(&self, access_token: &str, form: &AssetForm) -> Result<AssetListItem>;
}
