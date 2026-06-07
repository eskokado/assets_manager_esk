use std::sync::Arc;

use shared_kernel::Result;

use crate::features::portfolio::domain::{PortfolioSummary, PositionDetail};
use crate::features::portfolio::ports::PortfolioRepository;

pub struct LoadPortfolioUseCase {
    repository: Arc<dyn PortfolioRepository>,
}

impl LoadPortfolioUseCase {
    pub fn new(repository: Arc<dyn PortfolioRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str) -> Result<PortfolioSummary> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        self.repository.load_portfolio(access_token).await
    }
}

pub struct LoadPositionUseCase {
    repository: Arc<dyn PortfolioRepository>,
}

impl LoadPositionUseCase {
    pub fn new(repository: Arc<dyn PortfolioRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str, asset_id: &str) -> Result<PositionDetail> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        if asset_id.trim().is_empty() {
            return Result::err("Asset id is required");
        }
        self.repository.load_position(access_token, asset_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::Result;

    use super::*;
    use crate::features::portfolio::domain::{PortfolioSummary, PositionDetail};

    struct MockPortfolioRepo {
        portfolio: PortfolioSummary,
        detail: Option<PositionDetail>,
    }

    #[async_trait]
    impl PortfolioRepository for MockPortfolioRepo {
        async fn load_portfolio(&self, _access_token: &str) -> Result<PortfolioSummary> {
            Result::ok(self.portfolio.clone())
        }

        async fn load_position(
            &self,
            _access_token: &str,
            _asset_id: &str,
        ) -> Result<PositionDetail> {
            match &self.detail {
                Some(value) => Result::ok(value.clone()),
                None => Result::err("Position not found"),
            }
        }
    }

    #[tokio::test]
    async fn load_portfolio_returns_empty_summary() {
        let use_case = LoadPortfolioUseCase::new(Arc::new(MockPortfolioRepo {
            portfolio: PortfolioSummary::try_new("0".into(), vec![]).unwrap(),
            detail: None,
        }));
        let result = use_case.execute("token").await.unwrap();
        assert!(result.positions().is_empty());
    }

    #[tokio::test]
    async fn load_position_returns_not_found() {
        let use_case = LoadPositionUseCase::new(Arc::new(MockPortfolioRepo {
            portfolio: PortfolioSummary::try_new("0".into(), vec![]).unwrap(),
            detail: None,
        }));
        assert!(use_case.execute("token", "asset-id").await.is_err());
    }

    #[tokio::test]
    async fn load_position_returns_detail() {
        let detail = PositionDetail::try_new(
            "asset-1".into(),
            "PETR4".into(),
            "Petrobras".into(),
            "10".into(),
            "25.5".into(),
            "255".into(),
        )
        .unwrap();
        let use_case = LoadPositionUseCase::new(Arc::new(MockPortfolioRepo {
            portfolio: PortfolioSummary::try_new("255".into(), vec![]).unwrap(),
            detail: Some(detail.clone()),
        }));
        let result = use_case.execute("token", "asset-1").await.unwrap();
        assert_eq!(result.ticker(), "PETR4");
    }
}
