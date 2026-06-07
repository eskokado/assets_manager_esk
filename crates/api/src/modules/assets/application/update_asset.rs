use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, EntityId, Result, UseCase};

use crate::modules::assets::application::dto::{AssetOut, UpdateAssetIn};
use crate::modules::assets::domain::ports::{AssetRepository, PositionChecker};
use crate::modules::assets::domain::services::AssetCatalogPolicy;
use crate::modules::assets::domain::Asset;

pub struct UpdateAssetInput {
    pub id: EntityId,
    pub data: UpdateAssetIn,
}

pub struct UpdateAsset {
    repository: Arc<dyn AssetRepository>,
    position_checker: Arc<dyn PositionChecker>,
}

impl UpdateAsset {
    pub fn new(
        repository: Arc<dyn AssetRepository>,
        position_checker: Arc<dyn PositionChecker>,
    ) -> Self {
        Self {
            repository,
            position_checker,
        }
    }
}

#[async_trait]
impl UseCase<UpdateAssetInput, AssetOut> for UpdateAsset {
    async fn execute(&self, input: UpdateAssetInput) -> Result<AssetOut> {
        let mut asset = match try_domain!(self.repository.find_by_id(input.id).await) {
            Some(asset) => asset,
            None => return Result::err("Asset not found"),
        };

        let (name, asset_type, currency) = try_domain!(Asset::validate_update(
            &input.data.name,
            &input.data.asset_type,
            &input.data.currency,
        ));

        if !input.data.active && asset.active() {
            try_domain!(
                AssetCatalogPolicy::ensure_can_deactivate(
                    self.position_checker.as_ref(),
                    input.id
                )
                .await
            );
        }

        asset.update_details(name, asset_type, currency);
        asset.set_active(input.data.active);
        try_domain!(self.repository.update(&asset).await);

        Result::ok(AssetOut::from_asset(&asset))
    }
}
