use async_trait::async_trait;
use shared_kernel::{EntityId, Result};

use crate::modules::assets::domain::ports::PositionChecker;

/// Stub until EP-003 introduces positions table.
pub struct PositionCheckerStub;

#[async_trait]
impl PositionChecker for PositionCheckerStub {
    async fn has_open_position(&self, _asset_id: EntityId) -> Result<bool> {
        Result::ok(false)
    }
}
