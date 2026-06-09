use rust_decimal::Decimal;
use shared_kernel::{Result, ValueObject};

use super::{Quantity, UnitPrice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradeTotal {
    amount: Decimal,
    currency: [u8; 3],
}

impl TradeTotal {
    pub fn compute(quantity: Quantity, unit_price: UnitPrice) -> Result<Self> {
        let amount = quantity.value() * unit_price.amount();
        let currency = unit_price.currency();
        let mut currency_bytes = [0u8; 3];
        currency_bytes.copy_from_slice(currency.as_bytes());
        Result::ok(Self {
            amount,
            currency: currency_bytes,
        })
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> &str {
        std::str::from_utf8(&self.currency).expect("valid currency code")
    }
}

impl ValueObject for TradeTotal {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn computes_total() {
        let qty = Quantity::try_new(dec!(10)).unwrap();
        let price = UnitPrice::try_new(dec!(5), "BRL").unwrap();
        let total = TradeTotal::compute(qty, price).unwrap();
        assert_eq!(total.amount(), dec!(50));
    }
}
