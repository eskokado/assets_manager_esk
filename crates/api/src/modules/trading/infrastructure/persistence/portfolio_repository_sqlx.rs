use async_trait::async_trait;
use shared_kernel::{EntityId, Result};
use sqlx::{PgPool, Postgres, Transaction};

use crate::modules::assets::domain::ports::PositionChecker;
use crate::modules::trading::domain::ports::PortfolioRepository;
use crate::modules::trading::domain::Position;
use crate::modules::trading::infrastructure::persistence::records::PositionRecord;

pub struct PortfolioRepositorySqlx {
    pool: PgPool,
}

impl PortfolioRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn upsert_position(
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        position: &Position,
    ) -> Result<()> {
        let record = PositionRecord::from_position(position);
        match sqlx::query(
            r#"
            INSERT INTO positions (user_id, asset_id, quantity, average_price, currency, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, asset_id)
            DO UPDATE SET quantity = EXCLUDED.quantity, average_price = EXCLUDED.average_price, currency = EXCLUDED.currency, updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(record.user_id)
        .bind(record.asset_id)
        .bind(record.quantity)
        .bind(record.average_price)
        .bind(record.currency)
        .bind(record.updated_at)
        .execute(executor)
        .await
        {
            Ok(_) => Result::ok(()),
            Err(e) => Result::err(format!("Failed to upsert position: {e}")),
        }
    }

    async fn fetch_position(
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<Position>> {
        match sqlx::query_as::<_, PositionRecord>(
            "SELECT user_id, asset_id, quantity, average_price, currency, updated_at FROM positions WHERE user_id = $1 AND asset_id = $2",
        )
        .bind(user_id)
        .bind(asset_id)
        .fetch_optional(executor)
        .await
        {
            Ok(Some(record)) => record.to_domain().map(Some),
            Ok(None) => Result::ok(None),
            Err(e) => Result::err(format!("Failed to find position: {e}")),
        }
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepositorySqlx {
    async fn find_by_user_and_asset(
        &self,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<Position>> {
        Self::fetch_position(&self.pool, user_id, asset_id).await
    }

    async fn find_by_user_and_asset_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<Position>> {
        Self::fetch_position(&mut **tx, user_id, asset_id).await
    }

    async fn upsert(&self, position: &Position) -> Result<()> {
        Self::upsert_position(&self.pool, position).await
    }

    async fn upsert_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        position: &Position,
    ) -> Result<()> {
        Self::upsert_position(&mut **tx, position).await
    }

    async fn delete_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<()> {
        match sqlx::query("DELETE FROM positions WHERE user_id = $1 AND asset_id = $2")
            .bind(user_id)
            .bind(asset_id)
            .execute(&mut **tx)
            .await
        {
            Ok(_) => Result::ok(()),
            Err(e) => Result::err(format!("Failed to delete position: {e}")),
        }
    }

    async fn has_open_position_for_asset(&self, asset_id: EntityId) -> Result<bool> {
        match sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM positions WHERE asset_id = $1 AND quantity > 0",
        )
        .bind(asset_id)
        .fetch_one(&self.pool)
        .await
        {
            Ok(count) => Result::ok(count > 0),
            Err(e) => Result::err(format!("Failed to check positions: {e}")),
        }
    }
}

pub struct PositionCheckerSqlx {
    pool: PgPool,
}

impl PositionCheckerSqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PositionChecker for PositionCheckerSqlx {
    async fn has_open_position(&self, asset_id: EntityId) -> Result<bool> {
        PortfolioRepositorySqlx::new(self.pool.clone())
            .has_open_position_for_asset(asset_id)
            .await
    }
}
