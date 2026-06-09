use chrono::{DateTime, Utc};
use shared_kernel::{Entity, EntityId, Result};

use super::value_objects::{Quantity, TradeSide, UnitPrice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    id: EntityId,
    user_id: EntityId,
    asset_id: EntityId,
    side: TradeSide,
    quantity: Quantity,
    unit_price: UnitPrice,
    traded_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Entity for Trade {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl Trade {
    pub fn create(
        id: EntityId,
        user_id: EntityId,
        asset_id: EntityId,
        side: TradeSide,
        quantity: Quantity,
        unit_price: UnitPrice,
        traded_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            asset_id,
            side,
            quantity,
            unit_price,
            traded_at,
            created_at: Utc::now(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: EntityId,
        user_id: EntityId,
        asset_id: EntityId,
        side: TradeSide,
        quantity: Quantity,
        unit_price: UnitPrice,
        traded_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            asset_id,
            side,
            quantity,
            unit_price,
            traded_at,
            created_at,
        }
    }

    pub fn validate_inputs(
        quantity: &str,
        unit_price: &str,
        currency: &str,
    ) -> Result<(Quantity, UnitPrice)> {
        let qty_decimal = shared_kernel::try_domain!(parse_decimal(quantity));
        let price_decimal = shared_kernel::try_domain!(parse_decimal(unit_price));
        let quantity = shared_kernel::try_domain!(Quantity::try_new(qty_decimal));
        let unit_price = shared_kernel::try_domain!(UnitPrice::try_new(price_decimal, currency));
        shared_kernel::Result::ok((quantity, unit_price))
    }

    pub fn user_id(&self) -> EntityId {
        self.user_id
    }

    pub fn asset_id(&self) -> EntityId {
        self.asset_id
    }

    pub fn side(&self) -> TradeSide {
        self.side
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn unit_price(&self) -> UnitPrice {
        self.unit_price
    }

    pub fn traded_at(&self) -> DateTime<Utc> {
        self.traded_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    user_id: EntityId,
    asset_id: EntityId,
    quantity: Quantity,
    average_price: UnitPrice,
    updated_at: DateTime<Utc>,
}

impl Position {
    pub fn new(
        user_id: EntityId,
        asset_id: EntityId,
        quantity: Quantity,
        average_price: UnitPrice,
    ) -> Self {
        Self {
            user_id,
            asset_id,
            quantity,
            average_price,
            updated_at: Utc::now(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        user_id: EntityId,
        asset_id: EntityId,
        quantity: Quantity,
        average_price: UnitPrice,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            asset_id,
            quantity,
            average_price,
            updated_at,
        }
    }

    pub fn user_id(&self) -> EntityId {
        self.user_id
    }

    pub fn asset_id(&self) -> EntityId {
        self.asset_id
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn average_price(&self) -> UnitPrice {
        self.average_price
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn with_quantity_and_average(
        mut self,
        quantity: Quantity,
        average_price: UnitPrice,
    ) -> Self {
        self.quantity = quantity;
        self.average_price = average_price;
        self.updated_at = Utc::now();
        self
    }
}

fn parse_decimal(value: &str) -> Result<rust_decimal::Decimal> {
    match value.trim().parse::<rust_decimal::Decimal>() {
        Ok(decimal) => shared_kernel::Result::ok(decimal),
        Err(_) => shared_kernel::Result::err("Invalid decimal value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    #[test]
    fn trade_create_is_immutable_snapshot() {
        let trade = Trade::create(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            TradeSide::Buy,
            Quantity::try_new(dec!(10)).unwrap(),
            UnitPrice::try_new(dec!(5), "BRL").unwrap(),
            Utc::now(),
        );
        assert_eq!(trade.side(), TradeSide::Buy);
    }

    #[test]
    fn validates_inputs() {
        let (qty, price) = Trade::validate_inputs("10", "5.5", "BRL").unwrap();
        assert_eq!(qty.value(), dec!(10));
        assert_eq!(price.amount(), dec!(5.5));
    }
}
