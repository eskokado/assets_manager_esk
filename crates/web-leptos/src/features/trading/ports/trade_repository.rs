use async_trait::async_trait;
use shared_kernel::Result;

use crate::features::trading::domain::{BuyTradeForm, SellTradeForm, TradeListItem};

#[derive(Debug, Clone, Default)]
pub struct ListTradesQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PaginatedTradeList {
    pub items: Vec<TradeListItem>,
    pub total: i64,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TradeRepository: Send + Sync {
    async fn list(&self, access_token: &str, query: ListTradesQuery) -> Result<PaginatedTradeList>;
    async fn buy(&self, access_token: &str, form: &BuyTradeForm) -> Result<TradeListItem>;
    async fn sell(&self, access_token: &str, form: &SellTradeForm) -> Result<TradeListItem>;
}
