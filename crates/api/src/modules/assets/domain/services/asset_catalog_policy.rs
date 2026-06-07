use shared_kernel::{try_domain, EntityId, Result};

use crate::modules::assets::domain::ports::PositionChecker;

pub struct AssetCatalogPolicy;

impl AssetCatalogPolicy {
    pub async fn ensure_can_deactivate(
        checker: &dyn PositionChecker,
        asset_id: EntityId,
    ) -> Result<()> {
        if try_domain!(checker.has_open_position(asset_id).await) {
            return Result::err("Cannot deactivate asset with open positions");
        }
        Result::ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use uuid::Uuid;

    use super::*;
    use crate::modules::assets::domain::ports::PositionChecker;

    struct MockChecker {
        has_position: bool,
    }

    #[async_trait]
    impl PositionChecker for MockChecker {
        async fn has_open_position(&self, _asset_id: EntityId) -> Result<bool> {
            Result::ok(self.has_position)
        }
    }

    #[tokio::test]
    async fn blocks_deactivation_with_open_position() {
        let checker = MockChecker { has_position: true };
        assert!(
            AssetCatalogPolicy::ensure_can_deactivate(&checker, Uuid::new_v4())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn allows_deactivation_without_position() {
        let checker = MockChecker {
            has_position: false,
        };
        assert!(
            AssetCatalogPolicy::ensure_can_deactivate(&checker, Uuid::new_v4())
                .await
                .is_ok()
        );
    }
}
