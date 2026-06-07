use rust_decimal::Decimal;
use shared_kernel::{Money, Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitPrice {
    money: Money,
}

impl UnitPrice {
    pub fn try_new(amount: Decimal, currency: impl AsRef<str>) -> Result<Self> {
        if amount <= Decimal::ZERO {
            return Result::err("Unit price must be greater than zero");
        }
        let money = shared_kernel::try_domain!(Money::try_new(amount, currency));
        Result::ok(Self { money })
    }

    pub fn from_money(money: Money) -> Result<Self> {
        if money.amount() <= Decimal::ZERO {
            return Result::err("Unit price must be greater than zero");
        }
        Result::ok(Self { money })
    }

    pub fn amount(&self) -> Decimal {
        self.money.amount()
    }

    pub fn currency(&self) -> &str {
        self.money.currency()
    }

    pub fn money(&self) -> Money {
        self.money
    }
}

impl ValueObject for UnitPrice {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn accepts_positive_price() {
        let price = UnitPrice::try_new(dec!(25.50), "BRL").unwrap();
        assert_eq!(price.amount(), dec!(25.50));
    }

    #[test]
    fn rejects_non_positive() {
        assert!(UnitPrice::try_new(dec!(0), "BRL").is_err());
    }
}
