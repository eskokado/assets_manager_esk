use async_trait::async_trait;
use shared_kernel::{EntityId, Result};

use crate::modules::assets::domain::entity::Asset;

#[derive(Debug, Clone)]
pub struct AssetFilters {
    pub asset_type: Option<String>,
    pub active: Option<bool>,
    pub search: Option<String>,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub struct PaginatedAssets {
    pub items: Vec<Asset>,
    pub total: i64,
}

#[async_trait]
pub trait AssetRepository: Send + Sync {
    async fn save(&self, entity: &Asset) -> Result<()>;
    async fn update(&self, entity: &Asset) -> Result<()>;
    async fn find_by_id(&self, id: EntityId) -> Result<Option<Asset>>;
    async fn find_by_ticker(&self, ticker: &str) -> Result<Option<Asset>>;
    async fn find_all(&self, filters: AssetFilters) -> Result<PaginatedAssets>;
}
