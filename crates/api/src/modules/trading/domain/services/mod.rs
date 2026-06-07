use shared_kernel::{EntityId, Result};

use crate::modules::assets::domain::Asset;
use crate::modules::trading::domain::value_objects::TradeSide;

pub struct TradeExecutor;

impl TradeExecutor {
    pub fn ensure_buy_allowed(asset: &Asset) -> Result<()> {
        if !asset.is_available_for_buy() {
            return Result::err("Asset is inactive and cannot be purchased");
        }
        Result::ok(())
    }

    pub fn side_for_buy() -> TradeSide {
        TradeSide::Buy
    }

    pub fn side_for_sell() -> TradeSide {
        TradeSide::Sell
    }
}

pub struct SellValidator;

impl SellValidator {
    pub fn validate_available(
        available: rust_decimal::Decimal,
        requested: rust_decimal::Decimal,
    ) -> Result<()> {
        if requested > available {
            return Result::err(format!("Insufficient balance: available {available}"));
        }
        Result::ok(())
    }

    pub fn ensure_has_position(
        available: Option<rust_decimal::Decimal>,
    ) -> Result<rust_decimal::Decimal> {
        match available {
            Some(qty) if qty > rust_decimal::Decimal::ZERO => Result::ok(qty),
            _ => Result::err("No position for asset"),
        }
    }
}

pub struct PositionValidator;

impl PositionValidator {
    pub fn ensure_non_negative(quantity: rust_decimal::Decimal) -> Result<()> {
        if quantity.is_sign_negative() {
            return Result::err("Position quantity cannot be negative");
        }
        Result::ok(())
    }
}

pub struct PortfolioCalculator;

impl PortfolioCalculator {
    pub fn apply_buy(
        user_id: EntityId,
        asset_id: EntityId,
        current: Option<&super::Position>,
        buy_qty: rust_decimal::Decimal,
        buy_price: rust_decimal::Decimal,
        currency: &str,
    ) -> Result<super::Position> {
        use super::value_objects::{Quantity, UnitPrice};

        let buy_quantity = shared_kernel::try_domain!(Quantity::try_new(buy_qty));
        let buy_unit_price = shared_kernel::try_domain!(UnitPrice::try_new(buy_price, currency));

        match current {
            None => Result::ok(super::Position::new(
                user_id,
                asset_id,
                buy_quantity,
                buy_unit_price,
            )),
            Some(position) => {
                let old_qty = position.quantity().value();
                let old_avg = position.average_price().amount();
                let new_qty = old_qty + buy_qty;
                let new_avg = (old_qty * old_avg + buy_qty * buy_price) / new_qty;
                let new_quantity = shared_kernel::try_domain!(Quantity::try_new(new_qty));
                let new_average = shared_kernel::try_domain!(UnitPrice::try_new(new_avg, currency));
                Result::ok(
                    position
                        .clone()
                        .with_quantity_and_average(new_quantity, new_average),
                )
            }
        }
    }

    pub fn apply_sell(
        current: &super::Position,
        sell_qty: rust_decimal::Decimal,
    ) -> Result<Option<super::Position>> {
        use super::value_objects::Quantity;

        let remaining = current.quantity().value() - sell_qty;
        shared_kernel::try_domain!(PositionValidator::ensure_non_negative(remaining));
        if remaining.is_zero() {
            return Result::ok(None);
        }
        let new_quantity = shared_kernel::try_domain!(Quantity::try_new(remaining));
        Result::ok(Some(
            current
                .clone()
                .with_quantity_and_average(new_quantity, current.average_price()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::assets::domain::value_objects::{AssetName, AssetType, Currency, Ticker};
    use crate::modules::assets::domain::Asset;
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    #[test]
    fn blocks_buy_on_inactive_asset() {
        let mut asset = Asset::create(
            Uuid::new_v4(),
            Ticker::try_new("PETR4").unwrap(),
            AssetName::try_new("Petrobras").unwrap(),
            AssetType::Stock,
            Currency::brl(),
        );
        asset.deactivate();
        assert!(TradeExecutor::ensure_buy_allowed(&asset).is_err());
    }

    #[test]
    fn sell_validator_rejects_excess() {
        assert!(SellValidator::validate_available(dec!(5), dec!(6)).is_err());
        assert!(SellValidator::validate_available(dec!(5), dec!(5)).is_ok());
    }

    #[test]
    fn portfolio_calculator_buy_and_sell() {
        let user_id = Uuid::new_v4();
        let asset_id = Uuid::new_v4();
        let first =
            PortfolioCalculator::apply_buy(user_id, asset_id, None, dec!(10), dec!(5), "BRL")
                .unwrap();
        assert_eq!(first.quantity().value(), dec!(10));

        let second = PortfolioCalculator::apply_buy(
            user_id,
            asset_id,
            Some(&first),
            dec!(10),
            dec!(7),
            "BRL",
        )
        .unwrap();
        assert_eq!(second.quantity().value(), dec!(20));
        assert_eq!(second.average_price().amount(), dec!(6));

        let after_partial = PortfolioCalculator::apply_sell(&second, dec!(5)).unwrap();
        assert_eq!(after_partial.as_ref().unwrap().quantity().value(), dec!(15));

        let closed =
            PortfolioCalculator::apply_sell(after_partial.as_ref().unwrap(), dec!(15)).unwrap();
        assert!(closed.is_none());
    }
}
