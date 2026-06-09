use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_kernel::{try_domain, Result, UseCase};
use uuid::Uuid;

use crate::modules::trading::application::dto::{ListTradesOut, ListTradesQuery, TradeOut};
use crate::modules::trading::domain::ports::{TradeFilters, TradeRepository};

pub struct ListTrades {
    repository: Arc<dyn TradeRepository>,
}

impl ListTrades {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<ListTradesQuery, ListTradesOut> for ListTrades {
    async fn execute(&self, input: ListTradesQuery) -> Result<ListTradesOut> {
        let page = input.filters.page.unwrap_or(1).max(1);
        let limit = input.filters.limit.unwrap_or(20).clamp(1, 100);

        let asset_id = match input.filters.asset_id.as_deref() {
            Some(value) if !value.trim().is_empty() => {
                Some(try_domain!(parse_uuid(value, "Invalid asset id filter")))
            }
            _ => None,
        };

        let filters = TradeFilters {
            side: input.filters.side.clone(),
            asset_id,
            from: try_domain!(parse_datetime(input.filters.from.as_deref())),
            to: try_domain!(parse_datetime(input.filters.to.as_deref())),
            page,
            limit,
        };

        let page_result = try_domain!(
            self.repository
                .find_by_user_id(input.user_id, filters)
                .await
        );

        let items = page_result
            .items
            .iter()
            .map(|item| TradeOut::from_trade(&item.trade, Some(item.ticker.clone())))
            .collect();

        Result::ok(ListTradesOut {
            items,
            total: page_result.total,
            page,
            limit,
        })
    }
}

fn parse_uuid(value: &str, message: &str) -> Result<Uuid> {
    match Uuid::parse_str(value.trim()) {
        Ok(id) => Result::ok(id),
        Err(_) => Result::err(message),
    }
}

fn parse_datetime(value: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    match value {
        Some(raw) if !raw.trim().is_empty() => match DateTime::parse_from_rfc3339(raw.trim()) {
            Ok(dt) => Result::ok(Some(dt.with_timezone(&Utc))),
            Err(_) => Result::err("Invalid date filter; use RFC3339"),
        },
        _ => Result::ok(None),
    }
}
