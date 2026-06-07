use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::assets::domain::ports::AssetRepository;
use crate::modules::trading::application::dto::{SellAssetInput, TradeOut};
use crate::modules::trading::domain::ports::{PortfolioRepository, TradeRepository};
use crate::modules::trading::domain::services::{
    PortfolioCalculator, SellValidator, TradeExecutor,
};
use crate::modules::trading::domain::Trade;

pub struct SellAsset {
    pool: PgPool,
    asset_repository: Arc<dyn AssetRepository>,
    trade_repository: Arc<dyn TradeRepository>,
    portfolio_repository: Arc<dyn PortfolioRepository>,
}

impl SellAsset {
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
impl UseCase<SellAssetInput, TradeOut> for SellAsset {
    async fn execute(&self, input: SellAssetInput) -> Result<TradeOut> {
        let asset_id = try_domain!(parse_uuid(&input.data.asset_id, "Invalid asset id"));
        let asset = match try_domain!(self.asset_repository.find_by_id(asset_id).await) {
            Some(value) => value,
            None => return Result::err("Asset not found"),
        };

        let (quantity, unit_price) = try_domain!(Trade::validate_inputs(
            &input.data.quantity,
            &input.data.unit_price,
            asset.currency().as_str(),
        ));

        let mut tx = match self.pool.begin().await {
            Ok(value) => value,
            Err(e) => return Result::err(format!("Failed to start transaction: {e}")),
        };

        let current = try_domain!(
            self.portfolio_repository
                .find_by_user_and_asset_tx(&mut tx, input.user_id, asset_id)
                .await
        );
        let available = try_domain!(SellValidator::ensure_has_position(
            current.as_ref().map(|p| p.quantity().value())
        ));
        try_domain!(SellValidator::validate_available(
            available,
            quantity.value()
        ));

        let traded_at = try_domain!(parse_traded_at(input.data.traded_at.as_deref()));
        let trade = Trade::create(
            Uuid::new_v4(),
            input.user_id,
            asset_id,
            TradeExecutor::side_for_sell(),
            quantity,
            unit_price,
            traded_at,
        );

        let updated = try_domain!(PortfolioCalculator::apply_sell(
            current.as_ref().expect("position checked"),
            quantity.value(),
        ));

        try_domain!(self.trade_repository.save_tx(&mut tx, &trade).await);
        match updated {
            Some(position) => {
                try_domain!(
                    self.portfolio_repository
                        .upsert_tx(&mut tx, &position)
                        .await
                );
            }
            None => {
                try_domain!(
                    self.portfolio_repository
                        .delete_tx(&mut tx, input.user_id, asset_id)
                        .await
                );
            }
        }

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

fn parse_traded_at(value: Option<&str>) -> Result<chrono::DateTime<chrono::Utc>> {
    match value {
        Some(raw) if !raw.trim().is_empty() => {
            match chrono::DateTime::parse_from_rfc3339(raw.trim()) {
                Ok(dt) => Result::ok(dt.with_timezone(&chrono::Utc)),
                Err(_) => Result::err("Invalid traded_at; use RFC3339"),
            }
        }
        _ => Result::ok(chrono::Utc::now()),
    }
}
