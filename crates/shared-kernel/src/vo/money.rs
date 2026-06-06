use rust_decimal::Decimal;

use crate::{Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    amount: Decimal,
    currency: [u8; 3],
}

impl Money {
    pub fn try_new(amount: Decimal, currency: impl AsRef<str>) -> Result<Self> {
        if amount.is_sign_negative() {
            return Result::err("Money amount must be >= 0");
        }
        let code = currency.as_ref().trim().to_uppercase();
        if code.len() != 3 || !code.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Result::err("Currency must be a 3-letter ISO code");
        }
        let mut currency_bytes = [0u8; 3];
        currency_bytes.copy_from_slice(code.as_bytes());
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

impl ValueObject for Money {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn accepts_non_negative_amount() {
        let money = Money::try_new(dec!(10.50), "BRL").unwrap();
        assert_eq!(money.amount(), dec!(10.50));
        assert_eq!(money.currency(), "BRL");
    }

    #[test]
    fn rejects_negative_amount() {
        assert!(Money::try_new(dec!(-1), "BRL").is_err());
    }

    #[test]
    fn rejects_invalid_currency() {
        assert!(Money::try_new(dec!(1), "BR").is_err());
    }
}
