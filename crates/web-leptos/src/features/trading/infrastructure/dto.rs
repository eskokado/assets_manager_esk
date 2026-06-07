use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TradeDto {
    pub id: String,
    pub asset_id: String,
    pub ticker: Option<String>,
    pub side: String,
    pub quantity: String,
    pub unit_price: String,
    pub total: String,
    pub traded_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTradesResponseDto {
    pub items: Vec<TradeDto>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct BuyTradeRequestDto {
    pub asset_id: String,
    pub quantity: String,
    pub unit_price: String,
}

#[derive(Debug, Serialize)]
pub struct SellTradeRequestDto {
    pub asset_id: String,
    pub quantity: String,
    pub unit_price: String,
}
