use serde::{Deserialize, Serialize};
use shared_kernel::Entity;

use crate::modules::trading::domain::value_objects::TradeTotal;
use crate::modules::trading::domain::Trade;

#[derive(Debug, Clone, Deserialize)]
pub struct BuyIn {
    pub asset_id: String,
    pub quantity: String,
    pub unit_price: String,
    pub traded_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SellIn {
    pub asset_id: String,
    pub quantity: String,
    pub unit_price: String,
    pub traded_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeOut {
    pub id: String,
    pub asset_id: String,
    pub ticker: Option<String>,
    pub side: String,
    pub quantity: String,
    pub unit_price: String,
    pub total: String,
    pub currency: String,
    pub traded_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTradesIn {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub side: Option<String>,
    pub asset_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListTradesOut {
    pub items: Vec<TradeOut>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub struct BuyAssetInput {
    pub user_id: shared_kernel::EntityId,
    pub data: BuyIn,
}

#[derive(Debug, Clone)]
pub struct SellAssetInput {
    pub user_id: shared_kernel::EntityId,
    pub data: SellIn,
}

#[derive(Debug, Clone)]
pub struct FindTradeInput {
    pub user_id: shared_kernel::EntityId,
    pub trade_id: shared_kernel::EntityId,
}

#[derive(Debug, Clone)]
pub struct ListTradesQuery {
    pub user_id: shared_kernel::EntityId,
    pub filters: ListTradesIn,
}

impl TradeOut {
    pub fn from_trade(trade: &Trade, ticker: Option<String>) -> Self {
        let total = match TradeTotal::compute(trade.quantity(), trade.unit_price()) {
            shared_kernel::Result::Ok(value) => value.amount().to_string(),
            shared_kernel::Result::Err(_) => String::new(),
        };
        Self {
            id: trade.id().to_string(),
            asset_id: trade.asset_id().to_string(),
            ticker,
            side: trade.side().as_str().to_string(),
            quantity: trade.quantity().value().to_string(),
            unit_price: trade.unit_price().amount().to_string(),
            total,
            currency: trade.unit_price().currency().to_string(),
            traded_at: trade.traded_at().to_rfc3339(),
            created_at: trade.created_at().to_rfc3339(),
        }
    }
}
