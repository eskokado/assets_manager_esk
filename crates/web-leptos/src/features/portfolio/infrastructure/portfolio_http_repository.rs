use async_trait::async_trait;
use reqwest::Client;
use shared_kernel::{DomainError, Result};

use crate::features::portfolio::domain::{PortfolioSummary, PositionDetail, PositionSummary};
use crate::features::portfolio::infrastructure::dto::{PortfolioResponseDto, PositionDto};
use crate::features::portfolio::ports::PortfolioRepository;
use crate::shared::config::api_base_url;

pub struct PortfolioHttpRepository {
    client: Client,
    base_url: String,
}

impl PortfolioHttpRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: api_base_url(),
        }
    }

    async fn parse_error(res: reqwest::Response) -> Vec<DomainError> {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        vec![DomainError::new(format!("HTTP {status}: {body}"))]
    }

    fn map_position(dto: PositionDto) -> Result<PositionSummary> {
        PositionSummary::try_new(
            dto.asset_id,
            dto.asset_ticker,
            dto.asset_name,
            dto.quantity,
            dto.average_price,
            dto.total_invested,
        )
    }

    fn map_detail(dto: PositionDto) -> Result<PositionDetail> {
        PositionDetail::try_new(
            dto.asset_id,
            dto.asset_ticker,
            dto.asset_name,
            dto.quantity,
            dto.average_price,
            dto.total_invested,
        )
    }
}

impl Default for PortfolioHttpRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PortfolioRepository for PortfolioHttpRepository {
    async fn load_portfolio(&self, access_token: &str) -> Result<PortfolioSummary> {
        let url = format!("{}/api/portfolio", self.base_url);
        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: PortfolioResponseDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        let mut positions = Vec::with_capacity(dto.positions.len());
        for item in dto.positions {
            match Self::map_position(item) {
                shared_kernel::Result::Ok(mapped) => positions.push(mapped),
                shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
            }
        }

        PortfolioSummary::try_new(dto.total_invested, positions)
    }

    async fn load_position(&self, access_token: &str, asset_id: &str) -> Result<PositionDetail> {
        let url = format!("{}/api/portfolio/assets/{asset_id}", self.base_url);
        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: PositionDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        Self::map_detail(dto)
    }
}
