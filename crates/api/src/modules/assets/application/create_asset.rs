use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};
use uuid::Uuid;

use crate::modules::assets::application::dto::{AssetOut, CreateAssetIn};
use crate::modules::assets::domain::ports::AssetRepository;
use crate::modules::assets::domain::Asset;

pub struct CreateAsset {
    repository: Arc<dyn AssetRepository>,
}

impl CreateAsset {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<CreateAssetIn, AssetOut> for CreateAsset {
    async fn execute(&self, input: CreateAssetIn) -> Result<AssetOut> {
        let (ticker, name, asset_type, currency) = try_domain!(Asset::validate_create(
            &input.ticker,
            &input.name,
            &input.asset_type,
            input.currency.as_deref(),
        ));

        if try_domain!(self.repository.find_by_ticker(ticker.as_str()).await).is_some() {
            return Result::err("Ticker already exists");
        }

        let asset = Asset::create(Uuid::new_v4(), ticker, name, asset_type, currency);
        try_domain!(self.repository.save(&asset).await);

        Result::ok(AssetOut::from_asset(&asset))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::{Entity, EntityId, Result};

    use super::*;
    use crate::modules::assets::application::dto::CreateAssetIn;
    use crate::modules::assets::domain::ports::{AssetFilters, AssetRepository, PaginatedAssets};

    struct MockRepo {
        assets: tokio::sync::Mutex<Vec<Asset>>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self {
                assets: tokio::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl AssetRepository for MockRepo {
        async fn save(&self, entity: &Asset) -> Result<()> {
            self.assets.lock().await.push(entity.clone());
            Result::ok(())
        }

        async fn update(&self, entity: &Asset) -> Result<()> {
            let mut assets = self.assets.lock().await;
            if let Some(item) = assets.iter_mut().find(|a| *a.id() == *entity.id()) {
                *item = entity.clone();
            }
            Result::ok(())
        }

        async fn find_by_id(&self, id: EntityId) -> Result<Option<Asset>> {
            Result::ok(
                self.assets
                    .lock()
                    .await
                    .iter()
                    .find(|a| *a.id() == id)
                    .cloned(),
            )
        }

        async fn find_by_ticker(&self, ticker: &str) -> Result<Option<Asset>> {
            Result::ok(
                self.assets
                    .lock()
                    .await
                    .iter()
                    .find(|a| a.ticker().as_str() == ticker)
                    .cloned(),
            )
        }

        async fn find_all(&self, _filters: AssetFilters) -> Result<PaginatedAssets> {
            let items = self.assets.lock().await.clone();
            let total = items.len() as i64;
            Result::ok(PaginatedAssets { items, total })
        }
    }

    #[tokio::test]
    async fn creates_asset() {
        let use_case = CreateAsset::new(Arc::new(MockRepo::new()));
        let out = use_case
            .execute(CreateAssetIn {
                ticker: "petr4".into(),
                name: "Petrobras".into(),
                asset_type: "STOCK".into(),
                currency: Some("BRL".into()),
            })
            .await
            .unwrap();

        assert_eq!(out.ticker, "PETR4");
        assert!(out.active);
    }

    #[tokio::test]
    async fn rejects_duplicate_ticker() {
        let repo = Arc::new(MockRepo::new());
        let use_case = CreateAsset::new(repo.clone());
        let input = CreateAssetIn {
            ticker: "PETR4".into(),
            name: "Petrobras".into(),
            asset_type: "STOCK".into(),
            currency: None,
        };
        use_case.execute(input.clone()).await.unwrap();
        assert!(use_case.execute(input).await.is_err());
    }
}
