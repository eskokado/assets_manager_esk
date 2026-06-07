use shared_kernel::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl TradeSide {
    pub fn try_from_str(value: &str) -> Result<Self> {
        match value.trim().to_uppercase().as_str() {
            "BUY" => Result::ok(Self::Buy),
            "SELL" => Result::ok(Self::Sell),
            _ => Result::err("Trade side must be BUY or SELL"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_buy_and_sell() {
        assert_eq!(TradeSide::try_from_str("buy").unwrap(), TradeSide::Buy);
        assert_eq!(TradeSide::try_from_str("SELL").unwrap(), TradeSide::Sell);
        assert!(TradeSide::try_from_str("HOLD").is_err());
    }
}
