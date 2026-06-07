use rust_decimal::Decimal;
use shared_kernel::{Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity {
    value: Decimal,
}

impl Quantity {
    pub fn try_new(value: Decimal) -> Result<Self> {
        if value <= Decimal::ZERO {
            return Result::err("Quantity must be greater than zero");
        }
        Result::ok(Self { value })
    }

    pub fn value(&self) -> Decimal {
        self.value
    }
}

impl ValueObject for Quantity {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn accepts_positive() {
        let qty = Quantity::try_new(dec!(10)).unwrap();
        assert_eq!(qty.value(), dec!(10));
    }

    #[test]
    fn rejects_zero_and_negative() {
        assert!(Quantity::try_new(dec!(0)).is_err());
        assert!(Quantity::try_new(dec!(-1)).is_err());
    }
}
