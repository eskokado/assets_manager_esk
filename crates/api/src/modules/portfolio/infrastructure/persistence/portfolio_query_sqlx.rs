use async_trait::async_trait;
use shared_kernel::{EntityId, Result};
use sqlx::PgPool;

use crate::modules::portfolio::domain::{PortfolioQuery, PositionView};
use crate::modules::portfolio::infrastructure::persistence::records::PositionWithAssetRecord;

pub struct PortfolioQuerySqlx {
    pool: PgPool,
}

impl PortfolioQuerySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn base_select() -> &'static str {
        r#"
        SELECT p.user_id, p.asset_id, p.quantity, p.average_price, p.currency, p.updated_at,
               a.ticker, a.name
        FROM positions p
        INNER JOIN assets a ON a.id = p.asset_id
        "#
    }
}

#[async_trait]
impl PortfolioQuery for PortfolioQuerySqlx {
    async fn find_by_user_id(&self, user_id: EntityId) -> Result<Vec<PositionView>> {
        let query = format!(
            "{} WHERE p.user_id = $1 AND p.quantity > 0 ORDER BY a.ticker ASC",
            Self::base_select()
        );
        match sqlx::query_as::<_, PositionWithAssetRecord>(&query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
        {
            Ok(records) => {
                let mut views = Vec::with_capacity(records.len());
                for record in records {
                    views.push(shared_kernel::try_domain!(record.to_view()));
                }
                Result::ok(views)
            }
            Err(e) => Result::err(format!("Failed to load portfolio: {e}")),
        }
    }

    async fn find_position(
        &self,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<PositionView>> {
        let query = format!(
            "{} WHERE p.user_id = $1 AND p.asset_id = $2 AND p.quantity > 0",
            Self::base_select()
        );
        match sqlx::query_as::<_, PositionWithAssetRecord>(&query)
            .bind(user_id)
            .bind(asset_id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(Some(record)) => record.to_view().map(Some),
            Ok(None) => Result::ok(None),
            Err(e) => Result::err(format!("Failed to load position: {e}")),
        }
    }
}
