use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use shared_kernel::{try_domain, Result, UseCase};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::assets::domain::ports::AssetRepository;
use crate::modules::trading::application::dto::{BuyAssetInput, TradeOut};
use crate::modules::trading::domain::ports::{PortfolioRepository, TradeRepository};
use crate::modules::trading::domain::services::{PortfolioCalculator, TradeExecutor};
use crate::modules::trading::domain::value_objects::TradeSide;
use crate::modules::trading::domain::Trade;

pub struct BuyAsset {
    pool: PgPool,
    asset_repository: Arc<dyn AssetRepository>,
    trade_repository: Arc<dyn TradeRepository>,
    portfolio_repository: Arc<dyn PortfolioRepository>,
}

impl BuyAsset {
    pub fn new(
        pool: PgPool,
        asset_repository: Arc<dyn AssetRepository>,
        trade_repository: Arc<dyn TradeRepository>,
        portfolio_repository: Arc<dyn PortfolioRepository>,
    ) -> Self {
        Self {
            pool,
            asset_repository,
            trade_repository,
            portfolio_repository,
        }
    }
}

#[async_trait]
impl UseCase<BuyAssetInput, TradeOut> for BuyAsset {
    async fn execute(&self, input: BuyAssetInput) -> Result<TradeOut> {
        let asset_id = try_domain!(parse_uuid(&input.data.asset_id, "Invalid asset id"));
        let asset = match try_domain!(self.asset_repository.find_by_id(asset_id).await) {
            Some(value) => value,
            None => return Result::err("Asset not found"),
        };
        try_domain!(TradeExecutor::ensure_buy_allowed(&asset));

        let (quantity, unit_price) = try_domain!(Trade::validate_inputs(
            &input.data.quantity,
            &input.data.unit_price,
            asset.currency().as_str(),
        ));

        let traded_at = try_domain!(parse_traded_at(input.data.traded_at.as_deref()));
        let trade = Trade::create(
            Uuid::new_v4(),
            input.user_id,
            asset_id,
            TradeSide::Buy,
            quantity,
            unit_price,
            traded_at,
        );

        let mut tx = match self.pool.begin().await {
            Ok(value) => value,
            Err(e) => return Result::err(format!("Failed to start transaction: {e}")),
        };

        let current = try_domain!(
            self.portfolio_repository
                .find_by_user_and_asset_tx(&mut tx, input.user_id, asset_id)
                .await
        );

        let position = try_domain!(PortfolioCalculator::apply_buy(
            input.user_id,
            asset_id,
            current.as_ref(),
            quantity.value(),
            unit_price.amount(),
            unit_price.currency(),
        ));

        try_domain!(self.trade_repository.save_tx(&mut tx, &trade).await);
        try_domain!(
            self.portfolio_repository
                .upsert_tx(&mut tx, &position)
                .await
        );

        if let Err(e) = tx.commit().await {
            return Result::err(format!("Failed to commit transaction: {e}"));
        }

        Result::ok(TradeOut::from_trade(
            &trade,
            Some(asset.ticker().as_str().to_string()),
        ))
    }
}

fn parse_uuid(value: &str, message: &str) -> Result<Uuid> {
    match Uuid::parse_str(value.trim()) {
        Ok(id) => Result::ok(id),
        Err(_) => Result::err(message),
    }
}

fn parse_traded_at(value: Option<&str>) -> Result<chrono::DateTime<Utc>> {
    match value {
        Some(raw) if !raw.trim().is_empty() => {
            match chrono::DateTime::parse_from_rfc3339(raw.trim()) {
                Ok(dt) => Result::ok(dt.with_timezone(&Utc)),
                Err(_) => Result::err("Invalid traded_at; use RFC3339"),
            }
        }
        _ => Result::ok(Utc::now()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::{Entity, EntityId, Result};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Postgres, Transaction};
    use uuid::Uuid;

    use super::*;
    use crate::modules::assets::domain::ports::{AssetFilters, AssetRepository, PaginatedAssets};
    use crate::modules::assets::domain::value_objects::{AssetName, AssetType, Currency, Ticker};
    use crate::modules::assets::domain::Asset;
    use crate::modules::trading::application::dto::BuyIn;
    use crate::modules::trading::domain::ports::{
        PaginatedTrades, PortfolioRepository, TradeFilters, TradeRepository, TradeWithTicker,
    };
    use crate::modules::trading::domain::Position;

    struct MockAssetRepo {
        asset: Asset,
    }

    #[async_trait]
    impl AssetRepository for MockAssetRepo {
        async fn save(&self, _entity: &Asset) -> Result<()> {
            Result::ok(())
        }

        async fn update(&self, _entity: &Asset) -> Result<()> {
            Result::ok(())
        }

        async fn find_by_id(&self, id: EntityId) -> Result<Option<Asset>> {
            if id == *self.asset.id() {
                Result::ok(Some(self.asset.clone()))
            } else {
                Result::ok(None)
            }
        }

        async fn find_by_ticker(&self, _ticker: &str) -> Result<Option<Asset>> {
            Result::ok(None)
        }

        async fn find_all(&self, _filters: AssetFilters) -> Result<PaginatedAssets> {
            Result::ok(PaginatedAssets {
                items: vec![],
                total: 0,
            })
        }
    }

    struct MockTradeRepo;

    #[async_trait]
    impl TradeRepository for MockTradeRepo {
        async fn save(&self, _trade: &Trade) -> Result<()> {
            Result::ok(())
        }

        async fn save_tx(&self, _tx: &mut Transaction<'_, Postgres>, _trade: &Trade) -> Result<()> {
            Result::ok(())
        }

        async fn find_by_id(&self, _id: EntityId) -> Result<Option<Trade>> {
            Result::ok(None)
        }

        async fn find_by_user_id(
            &self,
            _user_id: EntityId,
            _filters: TradeFilters,
        ) -> Result<PaginatedTrades> {
            Result::ok(PaginatedTrades {
                items: vec![],
                total: 0,
            })
        }

        async fn find_by_id_for_user(
            &self,
            _id: EntityId,
            _user_id: EntityId,
        ) -> Result<Option<TradeWithTicker>> {
            Result::ok(None)
        }
    }

    struct MockPortfolioRepo {
        positions: tokio::sync::Mutex<Vec<Position>>,
    }

    impl MockPortfolioRepo {
        fn new() -> Self {
            Self {
                positions: tokio::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl PortfolioRepository for MockPortfolioRepo {
        async fn find_by_user_and_asset(
            &self,
            user_id: EntityId,
            asset_id: EntityId,
        ) -> Result<Option<Position>> {
            Result::ok(
                self.positions
                    .lock()
                    .await
                    .iter()
                    .find(|p| p.user_id() == user_id && p.asset_id() == asset_id)
                    .cloned(),
            )
        }

        async fn find_by_user_and_asset_tx(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: EntityId,
            asset_id: EntityId,
        ) -> Result<Option<Position>> {
            self.find_by_user_and_asset(user_id, asset_id).await
        }

        async fn upsert(&self, position: &Position) -> Result<()> {
            let mut positions = self.positions.lock().await;
            if let Some(item) = positions
                .iter_mut()
                .find(|p| p.user_id() == position.user_id() && p.asset_id() == position.asset_id())
            {
                *item = position.clone();
            } else {
                positions.push(position.clone());
            }
            Result::ok(())
        }

        async fn upsert_tx(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            position: &Position,
        ) -> Result<()> {
            self.upsert(position).await
        }

        async fn delete_tx(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: EntityId,
            asset_id: EntityId,
        ) -> Result<()> {
            self.positions
                .lock()
                .await
                .retain(|p| !(p.user_id() == user_id && p.asset_id() == asset_id));
            Result::ok(())
        }

        async fn has_open_position_for_asset(&self, _asset_id: EntityId) -> Result<bool> {
            Result::ok(false)
        }
    }

    async fn test_pool() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/appdb".into());
        PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .ok()
    }

    #[tokio::test]
    async fn rejects_inactive_asset() {
        let Some(pool) = test_pool().await else {
            eprintln!("Skipping BuyAsset test: database not available");
            return;
        };

        let asset_id = Uuid::new_v4();
        let mut asset = Asset::create(
            asset_id,
            Ticker::try_new("INAC").unwrap(),
            AssetName::try_new("Inactive").unwrap(),
            AssetType::Stock,
            Currency::brl(),
        );
        asset.deactivate();

        let use_case = BuyAsset::new(
            pool,
            Arc::new(MockAssetRepo { asset }),
            Arc::new(MockTradeRepo),
            Arc::new(MockPortfolioRepo::new()),
        );

        let result = use_case
            .execute(BuyAssetInput {
                user_id: Uuid::new_v4(),
                data: BuyIn {
                    asset_id: asset_id.to_string(),
                    quantity: "1".into(),
                    unit_price: "10".into(),
                    traded_at: None,
                },
            })
            .await;

        assert!(result.is_err());
    }
}
