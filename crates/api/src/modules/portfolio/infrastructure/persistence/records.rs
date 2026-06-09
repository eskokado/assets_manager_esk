use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use shared_kernel::Result;
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::portfolio::domain::PositionView;
use crate::modules::trading::domain::value_objects::{Quantity, UnitPrice};
use crate::modules::trading::domain::Position;

#[derive(Debug, Clone, FromRow)]
pub struct PositionWithAssetRecord {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub currency: String,
    pub updated_at: DateTime<Utc>,
    pub ticker: String,
    pub name: String,
}

impl PositionWithAssetRecord {
    pub fn to_view(&self) -> Result<PositionView> {
        let quantity = shared_kernel::try_domain!(Quantity::try_new(self.quantity));
        let average_price =
            shared_kernel::try_domain!(UnitPrice::try_new(self.average_price, &self.currency));
        let position = Position::reconstitute(
            self.user_id,
            self.asset_id,
            quantity,
            average_price,
            self.updated_at,
        );
        shared_kernel::Result::ok(PositionView {
            position,
            asset_ticker: self.ticker.clone(),
            asset_name: self.name.clone(),
        })
    }
}
