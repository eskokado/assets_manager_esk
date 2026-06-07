use async_trait::async_trait;
use shared_kernel::{EntityId, Result};

use crate::modules::trading::domain::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionView {
    pub position: Position,
    pub asset_ticker: String,
    pub asset_name: String,
}

#[async_trait]
pub trait PortfolioQuery: Send + Sync {
    async fn find_by_user_id(&self, user_id: EntityId) -> Result<Vec<PositionView>>;

    async fn find_position(
        &self,
        user_id: EntityId,
        asset_id: EntityId,
    ) -> Result<Option<PositionView>>;
}
