use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_kernel::{EntityId, Result};
use sqlx::{Postgres, Transaction};

use crate::modules::trading::domain::{Position, Trade};

#[derive(Debug, Clone)]
pub struct TradeFilters {
    pub side: Option<String>,
    pub asset_id: Option<EntityId>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub struct PaginatedTrades {
    pub items: Vec<TradeWithTicker>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct TradeWithTicker {
    pub trade: Trade,
    pub ticker: String,
}

#[async_trait]
pub trait TradeRepository: Send + Sync {
    async fn save(&self, trade: &Trade) -> Result<()>;
    async fn save_tx(&self, tx: &mut Transaction<'_, Postgres>, trade: &Trade) -> Result<()>;
    async fn find_by_id(&self, id: EntityId) -> Result<Option<Trade>>;
    async fn find_by_user_id(
        &self,
        user_id: EntityId,
        filters: TradeFilters,
    ) -> Result<PaginatedTrades>;
    async fn find_by_id_for_user(
        &self,
        id: EntityId,
        user_id: EntityId,
    ) -> Result<Option<TradeWithTicker>>;
}

#[async_trait]
pub trait PortfolioRepository: Send + Sync {
    async fn find_by_user_and_asset(
        &self,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<Position>>;
    async fn find_by_user_and_asset_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<Position>>;
    async fn upsert(&self, position: &Position) -> Result<()>;
    async fn upsert_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        position: &Position,
    ) -> Result<()>;
    async fn delete_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<()>;
    async fn has_open_position_for_asset(&self, asset_id: EntityId) -> Result<bool>;
}
