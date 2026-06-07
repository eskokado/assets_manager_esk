use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, EntityId, Result, UseCase};

use crate::modules::assets::application::dto::AssetOut;
use crate::modules::assets::domain::ports::AssetRepository;

pub struct FindAssetById {
    repository: Arc<dyn AssetRepository>,
}

impl FindAssetById {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<EntityId, AssetOut> for FindAssetById {
    async fn execute(&self, id: EntityId) -> Result<AssetOut> {
        let asset = match try_domain!(self.repository.find_by_id(id).await) {
            Some(asset) => asset,
            None => return Result::err("Asset not found"),
        };
        Result::ok(AssetOut::from_asset(&asset))
    }
}
