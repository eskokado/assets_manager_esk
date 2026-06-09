use async_trait::async_trait;
use shared_kernel::{EntityId, Result};
use sqlx::PgPool;

use crate::modules::assets::domain::ports::{AssetFilters, AssetRepository, PaginatedAssets};
use crate::modules::assets::domain::Asset;
use crate::modules::assets::infrastructure::persistence::asset_record::AssetRecord;

pub struct AssetRepositorySqlx {
    pool: PgPool,
}

impl AssetRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AssetRepository for AssetRepositorySqlx {
    async fn save(&self, entity: &Asset) -> Result<()> {
        let record = AssetRecord::from(entity.to_record());
        match sqlx::query(
            r#"
            INSERT INTO assets (id, ticker, name, asset_type, currency, active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(record.id)
        .bind(record.ticker)
        .bind(record.name)
        .bind(record.asset_type)
        .bind(record.currency)
        .bind(record.active)
        .bind(record.created_at)
        .bind(record.updated_at)
        .execute(&self.pool)
        .await
        {
            Ok(_) => Result::ok(()),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.constraint().is_some() {
                        return Result::err("Ticker already exists");
                    }
                }
                Result::err(format!("Failed to save asset: {e}"))
            }
        }
    }

    async fn update(&self, entity: &Asset) -> Result<()> {
        let record = AssetRecord::from(entity.to_record());
        match sqlx::query(
            r#"
            UPDATE assets
            SET name = $2, asset_type = $3, currency = $4, active = $5, updated_at = $6
            WHERE id = $1
            "#,
        )
        .bind(record.id)
        .bind(record.name)
        .bind(record.asset_type)
        .bind(record.currency)
        .bind(record.active)
        .bind(record.updated_at)
        .execute(&self.pool)
        .await
        {
            Ok(result) if result.rows_affected() > 0 => Result::ok(()),
            Ok(_) => Result::err("Asset not found"),
            Err(e) => Result::err(format!("Failed to update asset: {e}")),
        }
    }

    async fn find_by_id(&self, id: EntityId) -> Result<Option<Asset>> {
        match sqlx::query_as::<_, AssetRecord>(
            "SELECT id, ticker, name, asset_type, currency, active, created_at, updated_at FROM assets WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(record) => match record {
                Some(record) => record.to_domain().map(Some),
                None => Result::ok(None),
            },
            Err(e) => Result::err(format!("Failed to find asset: {e}")),
        }
    }

    async fn find_by_ticker(&self, ticker: &str) -> Result<Option<Asset>> {
        match sqlx::query_as::<_, AssetRecord>(
            "SELECT id, ticker, name, asset_type, currency, active, created_at, updated_at FROM assets WHERE ticker = $1",
        )
        .bind(ticker.to_uppercase())
        .fetch_optional(&self.pool)
        .await
        {
            Ok(record) => match record {
                Some(record) => record.to_domain().map(Some),
                None => Result::ok(None),
            },
            Err(e) => Result::err(format!("Failed to find asset by ticker: {e}")),
        }
    }

    async fn find_all(&self, filters: AssetFilters) -> Result<PaginatedAssets> {
        let offset = ((filters.page.saturating_sub(1)) * filters.limit) as i64;
        let limit = filters.limit as i64;
        let search = filters
            .search
            .as_ref()
            .map(|value| format!("%{}%", value.trim()));

        let total: i64 = match sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM assets
            WHERE ($1::text IS NULL OR asset_type = $1)
              AND ($2::bool IS NULL OR active = $2)
              AND ($3::text IS NULL OR ticker ILIKE $3 OR name ILIKE $3)
            "#,
        )
        .bind(filters.asset_type.as_deref())
        .bind(filters.active)
        .bind(search.as_deref())
        .fetch_one(&self.pool)
        .await
        {
            Ok(count) => count,
            Err(e) => return Result::err(format!("Failed to count assets: {e}")),
        };

        let records = match sqlx::query_as::<_, AssetRecord>(
            r#"
            SELECT id, ticker, name, asset_type, currency, active, created_at, updated_at
            FROM assets
            WHERE ($1::text IS NULL OR asset_type = $1)
              AND ($2::bool IS NULL OR active = $2)
              AND ($3::text IS NULL OR ticker ILIKE $3 OR name ILIKE $3)
            ORDER BY ticker ASC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(filters.asset_type.as_deref())
        .bind(filters.active)
        .bind(search.as_deref())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        {
            Ok(records) => records,
            Err(e) => return Result::err(format!("Failed to list assets: {e}")),
        };

        let mut items = Vec::with_capacity(records.len());
        for record in records {
            items.push(shared_kernel::try_domain!(record.to_domain()));
        }

        Result::ok(PaginatedAssets { items, total })
    }
}
