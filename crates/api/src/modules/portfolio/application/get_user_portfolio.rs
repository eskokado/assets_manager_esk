use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};

use crate::modules::portfolio::application::dto::{GetUserPortfolioInput, PortfolioOut};
use crate::modules::portfolio::domain::PortfolioQuery;

pub struct GetUserPortfolio {
    query: Arc<dyn PortfolioQuery>,
}

impl GetUserPortfolio {
    pub fn new(query: Arc<dyn PortfolioQuery>) -> Self {
        Self { query }
    }
}

#[async_trait]
impl UseCase<GetUserPortfolioInput, PortfolioOut> for GetUserPortfolio {
    async fn execute(&self, input: GetUserPortfolioInput) -> Result<PortfolioOut> {
        let views = try_domain!(self.query.find_by_user_id(input.user_id).await);
        Result::ok(PortfolioOut::from_views(&views))
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
        views: Vec<PositionView>,
    }

    #[async_trait]
    impl PortfolioQuery for MockQuery {
        async fn find_by_user_id(&self, _user_id: EntityId) -> Result<Vec<PositionView>> {
            Result::ok(self.views.clone())
        }

        async fn find_position(
            &self,
            _user_id: EntityId,
            _asset_id: EntityId,
        ) -> Result<Option<PositionView>> {
            Result::ok(None)
        }
    }

    fn sample_view() -> PositionView {
        let user_id = Uuid::new_v4();
        let asset_id = Uuid::new_v4();
        PositionView {
            position: Position::new(
                user_id,
                asset_id,
                Quantity::try_new(dec!(10)).unwrap(),
                UnitPrice::try_new(dec!(5), "BRL").unwrap(),
            ),
            asset_ticker: "PETR4".into(),
            asset_name: "Petrobras".into(),
        }
    }

    #[tokio::test]
    async fn returns_empty_portfolio() {
        let use_case = GetUserPortfolio::new(Arc::new(MockQuery { views: vec![] }));
        let out = use_case
            .execute(GetUserPortfolioInput {
                user_id: Uuid::new_v4(),
            })
            .await
            .unwrap();
        assert!(out.positions.is_empty());
        assert_eq!(out.total_invested, "0");
    }

    #[tokio::test]
    async fn returns_positions_with_total() {
        let use_case = GetUserPortfolio::new(Arc::new(MockQuery {
            views: vec![sample_view()],
        }));
        let out = use_case
            .execute(GetUserPortfolioInput {
                user_id: Uuid::new_v4(),
            })
            .await
            .unwrap();
        assert_eq!(out.positions.len(), 1);
        assert_eq!(out.total_invested, "50");
        assert_eq!(out.positions[0].asset_ticker, "PETR4");
    }
}
