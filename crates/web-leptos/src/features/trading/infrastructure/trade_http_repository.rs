use async_trait::async_trait;
use reqwest::Client;
use shared_kernel::{DomainError, Result};

use crate::features::trading::domain::{BuyTradeForm, SellTradeForm, TradeListItem};
use crate::features::trading::infrastructure::dto::{
    BuyTradeRequestDto, ListTradesResponseDto, SellTradeRequestDto, TradeDto,
};
use crate::features::trading::ports::{ListTradesQuery, PaginatedTradeList, TradeRepository};
use crate::shared::config::api_base_url;

pub struct TradeHttpRepository {
    client: Client,
    base_url: String,
}

impl TradeHttpRepository {
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

    fn map_trade(dto: TradeDto) -> Result<TradeListItem> {
        TradeListItem::try_new(
            dto.id,
            dto.side,
            dto.ticker.unwrap_or_else(|| "-".into()),
            dto.quantity,
            dto.unit_price,
            dto.total,
            dto.traded_at,
        )
    }
}

impl Default for TradeHttpRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TradeRepository for TradeHttpRepository {
    async fn list(&self, access_token: &str, query: ListTradesQuery) -> Result<PaginatedTradeList> {
        let mut url = match reqwest::Url::parse(&format!("{}/api/trades", self.base_url)) {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(page) = query.page {
                pairs.append_pair("page", &page.to_string());
            }
            if let Some(limit) = query.limit {
                pairs.append_pair("limit", &limit.to_string());
            }
        }

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

        let dto: ListTradesResponseDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        let mut items = Vec::with_capacity(dto.items.len());
        for item in dto.items {
            match Self::map_trade(item) {
                shared_kernel::Result::Ok(mapped) => items.push(mapped),
                shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
            }
        }

        Result::ok(PaginatedTradeList {
            items,
            total: dto.total,
        })
    }

    async fn buy(&self, access_token: &str, form: &BuyTradeForm) -> Result<TradeListItem> {
        let url = format!("{}/api/trades/buy", self.base_url);
        let res = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .json(&BuyTradeRequestDto {
                asset_id: form.asset_id().to_string(),
                quantity: form.quantity().to_string(),
                unit_price: form.unit_price().to_string(),
            })
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: TradeDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };
        Self::map_trade(dto)
    }

    async fn sell(&self, access_token: &str, form: &SellTradeForm) -> Result<TradeListItem> {
        let url = format!("{}/api/trades/sell", self.base_url);
        let res = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .json(&SellTradeRequestDto {
                asset_id: form.asset_id().to_string(),
                quantity: form.quantity().to_string(),
                unit_price: form.unit_price().to_string(),
            })
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: TradeDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };
        Self::map_trade(dto)
    }
}
