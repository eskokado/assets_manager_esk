use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use shared_kernel::{Entity, Result};
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::trading::domain::value_objects::{Quantity, TradeSide, UnitPrice};
use crate::modules::trading::domain::Trade;

#[derive(Debug, Clone, FromRow)]
pub struct TradeRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub side: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub currency: String,
    pub traded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TradeRecord {
    pub fn from_trade(trade: &Trade) -> Self {
        Self {
            id: *trade.id(),
            user_id: trade.user_id(),
            asset_id: trade.asset_id(),
            side: trade.side().as_str().to_string(),
            quantity: trade.quantity().value(),
            unit_price: trade.unit_price().amount(),
            currency: trade.unit_price().currency().to_string(),
            traded_at: trade.traded_at(),
            created_at: trade.created_at(),
        }
    }

    pub fn to_domain(&self) -> Result<Trade> {
        let side = shared_kernel::try_domain!(TradeSide::try_from_str(&self.side));
        let quantity = shared_kernel::try_domain!(Quantity::try_new(self.quantity));
        let unit_price =
            shared_kernel::try_domain!(UnitPrice::try_new(self.unit_price, &self.currency));
        shared_kernel::Result::ok(Trade::reconstitute(
            self.id,
            self.user_id,
            self.asset_id,
            side,
            quantity,
            unit_price,
            self.traded_at,
            self.created_at,
        ))
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct TradeWithTickerRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub side: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub currency: String,
    pub traded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub ticker: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct PositionRecord {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub currency: String,
    pub updated_at: DateTime<Utc>,
}

impl PositionRecord {
    pub fn from_position(position: &crate::modules::trading::domain::Position) -> Self {
        Self {
            user_id: position.user_id(),
            asset_id: position.asset_id(),
            quantity: position.quantity().value(),
            average_price: position.average_price().amount(),
            currency: position.average_price().currency().to_string(),
            updated_at: position.updated_at(),
        }
    }

    pub fn to_domain(&self) -> Result<crate::modules::trading::domain::Position> {
        use crate::modules::trading::domain::Position;

        let quantity = shared_kernel::try_domain!(Quantity::try_new(self.quantity));
        let average_price =
            shared_kernel::try_domain!(UnitPrice::try_new(self.average_price, &self.currency));
        shared_kernel::Result::ok(Position::reconstitute(
            self.user_id,
            self.asset_id,
            quantity,
            average_price,
            self.updated_at,
        ))
    }
}
