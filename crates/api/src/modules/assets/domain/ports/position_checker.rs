use async_trait::async_trait;
use shared_kernel::{EntityId, Result};

#[async_trait]
pub trait PositionChecker: Send + Sync {
    async fn has_open_position(&self, asset_id: EntityId) -> Result<bool>;
}
