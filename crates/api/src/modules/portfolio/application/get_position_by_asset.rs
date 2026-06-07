use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};

use crate::modules::portfolio::application::dto::{GetPositionInput, PositionOut};
use crate::modules::portfolio::domain::PortfolioQuery;

pub struct GetPositionByAsset {
    query: Arc<dyn PortfolioQuery>,
}

impl GetPositionByAsset {
    pub fn new(query: Arc<dyn PortfolioQuery>) -> Self {
        Self { query }
    }
}

#[async_trait]
impl UseCase<GetPositionInput, PositionOut> for GetPositionByAsset {
    async fn execute(&self, input: GetPositionInput) -> Result<PositionOut> {
        let view = try_domain!(
            self.query
                .find_position(input.user_id, input.asset_id)
                .await
        );
        match view {
            Some(value) => Result::ok(PositionOut::from_view(&value)),
            None => Result::err("Position not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use rust_decimal_macros::dec;
    use shared_kernel::{EntityId, Result, UseCase};
    use uuid::Uuid;

    use super::*;
    use crate::modules::portfolio::domain::{PortfolioQuery, PositionView};
    use crate::modules::trading::domain::value_objects::{Quantity, UnitPrice};
    use crate::modules::trading::domain::Position;

    struct MockQuery {
        view: Option<PositionView>,
    }

    #[async_trait]
    impl PortfolioQuery for MockQuery {
        async fn find_by_user_id(&self, _user_id: EntityId) -> Result<Vec<PositionView>> {
            Result::ok(vec![])
        }

        async fn find_position(
            &self,
            _user_id: EntityId,
            _asset_id: EntityId,
        ) -> Result<Option<PositionView>> {
            Result::ok(self.view.clone())
        }
    }

    fn sample_view() -> PositionView {
        let user_id = Uuid::new_v4();
        let asset_id = Uuid::new_v4();
        PositionView {
            position: Position::new(
                user_id,
                asset_id,
                Quantity::try_new(dec!(4)).unwrap(),
                UnitPrice::try_new(dec!(25), "BRL").unwrap(),
            ),
            asset_ticker: "VALE3".into(),
            asset_name: "Vale".into(),
        }
    }

    #[tokio::test]
    async fn returns_position_detail() {
        let view = sample_view();
        let asset_id = view.position.asset_id();
        let use_case = GetPositionByAsset::new(Arc::new(MockQuery { view: Some(view) }));
        let out = use_case
            .execute(GetPositionInput {
                user_id: Uuid::new_v4(),
                asset_id,
            })
            .await
            .unwrap();
        assert_eq!(out.asset_ticker, "VALE3");
        assert_eq!(out.total_invested, "100");
    }

    #[tokio::test]
    async fn returns_not_found() {
        let use_case = GetPositionByAsset::new(Arc::new(MockQuery { view: None }));
        assert!(use_case
            .execute(GetPositionInput {
                user_id: Uuid::new_v4(),
                asset_id: Uuid::new_v4(),
            })
            .await
            .is_err());
    }
}
