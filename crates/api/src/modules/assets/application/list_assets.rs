use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};

use crate::modules::assets::application::dto::{AssetOut, ListAssetsIn, ListAssetsOut};
use crate::modules::assets::domain::ports::{AssetFilters, AssetRepository};

pub struct ListAssets {
    repository: Arc<dyn AssetRepository>,
}

impl ListAssets {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<ListAssetsIn, ListAssetsOut> for ListAssets {
    async fn execute(&self, input: ListAssetsIn) -> Result<ListAssetsOut> {
        let page = input.page.unwrap_or(1).max(1);
        let limit = input.limit.unwrap_or(20).clamp(1, 100);

        let filters = AssetFilters {
            asset_type: input.asset_type,
            active: input.active,
            search: input.search,
            page,
            limit,
        };

        let page_result = try_domain!(self.repository.find_all(filters).await);
        let items = page_result.items.iter().map(AssetOut::from_asset).collect();

        Result::ok(ListAssetsOut {
            items,
            total: page_result.total,
            page,
            limit,
        })
    }
}
