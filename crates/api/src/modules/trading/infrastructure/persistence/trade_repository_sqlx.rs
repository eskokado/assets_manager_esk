use async_trait::async_trait;
use shared_kernel::{EntityId, Result};
use sqlx::{PgPool, Postgres, Transaction};

use crate::modules::trading::domain::ports::{
    PaginatedTrades, TradeFilters, TradeRepository, TradeWithTicker,
};
use crate::modules::trading::domain::Trade;
use crate::modules::trading::infrastructure::persistence::records::{
    TradeRecord, TradeWithTickerRecord,
};

pub struct TradeRepositorySqlx {
    pool: PgPool,
}

impl TradeRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn insert_trade(
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        trade: &Trade,
    ) -> Result<()> {
        let record = TradeRecord::from_trade(trade);
        match sqlx::query(
            r#"
            INSERT INTO trades (id, user_id, asset_id, side, quantity, unit_price, currency, traded_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(record.id)
        .bind(record.user_id)
        .bind(record.asset_id)
        .bind(record.side)
        .bind(record.quantity)
        .bind(record.unit_price)
        .bind(record.currency)
        .bind(record.traded_at)
        .bind(record.created_at)
        .execute(executor)
        .await
        {
            Ok(_) => Result::ok(()),
            Err(e) => Result::err(format!("Failed to save trade: {e}")),
        }
    }
}

#[async_trait]
impl TradeRepository for TradeRepositorySqlx {
    async fn save(&self, trade: &Trade) -> Result<()> {
        Self::insert_trade(&self.pool, trade).await
    }

    async fn save_tx(&self, tx: &mut Transaction<'_, Postgres>, trade: &Trade) -> Result<()> {
        Self::insert_trade(&mut **tx, trade).await
    }

    async fn find_by_id(&self, id: EntityId) -> Result<Option<Trade>> {
        match sqlx::query_as::<_, TradeRecord>(
            "SELECT id, user_id, asset_id, side, quantity, unit_price, currency, traded_at, created_at FROM trades WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(Some(record)) => record.to_domain().map(Some),
            Ok(None) => Result::ok(None),
            Err(e) => Result::err(format!("Failed to find trade: {e}")),
        }
    }

    async fn find_by_user_id(
        &self,
        user_id: EntityId,
        filters: TradeFilters,
    ) -> Result<PaginatedTrades> {
        let offset = ((filters.page.saturating_sub(1)) * filters.limit) as i64;
        let limit = filters.limit as i64;

        let mut count_query = String::from("SELECT COUNT(*) FROM trades WHERE user_id = $1");
        let mut list_query = String::from(
            "SELECT t.id, t.user_id, t.asset_id, t.side, t.quantity, t.unit_price, t.currency, t.traded_at, t.created_at, a.ticker FROM trades t INNER JOIN assets a ON a.id = t.asset_id WHERE t.user_id = $1",
        );

        let mut bind_index = 2;
        if filters.side.is_some() {
            count_query.push_str(&format!(" AND side = ${bind_index}"));
            list_query.push_str(&format!(" AND t.side = ${bind_index}"));
            bind_index += 1;
        }
        if filters.asset_id.is_some() {
            count_query.push_str(&format!(" AND asset_id = ${bind_index}"));
            list_query.push_str(&format!(" AND t.asset_id = ${bind_index}"));
            bind_index += 1;
        }
        if filters.from.is_some() {
            count_query.push_str(&format!(" AND traded_at >= ${bind_index}"));
            list_query.push_str(&format!(" AND t.traded_at >= ${bind_index}"));
            bind_index += 1;
        }
        if filters.to.is_some() {
            count_query.push_str(&format!(" AND traded_at <= ${bind_index}"));
            list_query.push_str(&format!(" AND t.traded_at <= ${bind_index}"));
            let _ = bind_index;
        }

        list_query.push_str(" ORDER BY t.traded_at DESC LIMIT ");
        list_query.push_str(&limit.to_string());
        list_query.push_str(" OFFSET ");
        list_query.push_str(&offset.to_string());

        let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query).bind(user_id);
        let mut list_builder =
            sqlx::query_as::<_, TradeWithTickerRecord>(&list_query).bind(user_id);

        if let Some(side) = filters.side.as_deref() {
            count_builder = count_builder.bind(side);
            list_builder = list_builder.bind(side);
        }
        if let Some(asset_id) = filters.asset_id {
            count_builder = count_builder.bind(asset_id);
            list_builder = list_builder.bind(asset_id);
        }
        if let Some(from) = filters.from {
            count_builder = count_builder.bind(from);
            list_builder = list_builder.bind(from);
        }
        if let Some(to) = filters.to {
            count_builder = count_builder.bind(to);
            list_builder = list_builder.bind(to);
        }

        let total = match count_builder.fetch_one(&self.pool).await {
            Ok(value) => value,
            Err(e) => return Result::err(format!("Failed to count trades: {e}")),
        };

        let records = match list_builder.fetch_all(&self.pool).await {
            Ok(value) => value,
            Err(e) => return Result::err(format!("Failed to list trades: {e}")),
        };

        let mut items = Vec::with_capacity(records.len());
        for record in records {
            let trade = shared_kernel::try_domain!(TradeRecord {
                id: record.id,
                user_id: record.user_id,
                asset_id: record.asset_id,
                side: record.side,
                quantity: record.quantity,
                unit_price: record.unit_price,
                currency: record.currency,
                traded_at: record.traded_at,
                created_at: record.created_at,
            }
            .to_domain());
            items.push(TradeWithTicker {
                trade,
                ticker: record.ticker,
            });
        }

        Result::ok(PaginatedTrades { items, total })
    }

    async fn find_by_id_for_user(
        &self,
        id: EntityId,
        user_id: EntityId,
    ) -> Result<Option<TradeWithTicker>> {
        match sqlx::query_as::<_, TradeWithTickerRecord>(
            r#"
            SELECT t.id, t.user_id, t.asset_id, t.side, t.quantity, t.unit_price, t.currency, t.traded_at, t.created_at, a.ticker
            FROM trades t
            INNER JOIN assets a ON a.id = t.asset_id
            WHERE t.id = $1 AND t.user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(Some(record)) => {
                let trade = shared_kernel::try_domain!(TradeRecord {
                    id: record.id,
                    user_id: record.user_id,
                    asset_id: record.asset_id,
                    side: record.side,
                    quantity: record.quantity,
                    unit_price: record.unit_price,
                    currency: record.currency,
                    traded_at: record.traded_at,
                    created_at: record.created_at,
                }
                .to_domain());
                Result::ok(Some(TradeWithTicker {
                    trade,
                    ticker: record.ticker,
                }))
            }
            Ok(None) => Result::ok(None),
            Err(e) => Result::err(format!("Failed to find trade: {e}")),
        }
    }
}
