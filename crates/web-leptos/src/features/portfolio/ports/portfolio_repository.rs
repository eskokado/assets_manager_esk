use async_trait::async_trait;
use shared_kernel::Result;

use crate::features::portfolio::domain::{PortfolioSummary, PositionDetail};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PortfolioRepository: Send + Sync {
    async fn load_portfolio(&self, access_token: &str) -> Result<PortfolioSummary>;

    async fn load_position(&self, access_token: &str, asset_id: &str) -> Result<PositionDetail>;
}
