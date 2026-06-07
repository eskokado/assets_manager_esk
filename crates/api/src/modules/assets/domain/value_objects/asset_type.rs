use serde::{Deserialize, Serialize};

use shared_kernel::{Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    Stock,
    Fii,
    Etf,
    Bond,
}

impl AssetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stock => "STOCK",
            Self::Fii => "FII",
            Self::Etf => "ETF",
            Self::Bond => "BOND",
        }
    }

    pub fn try_from_str(raw: &str) -> Result<Self> {
        match raw.trim().to_uppercase().as_str() {
            "STOCK" => Result::ok(Self::Stock),
            "FII" => Result::ok(Self::Fii),
            "ETF" => Result::ok(Self::Etf),
            "BOND" => Result::ok(Self::Bond),
            _ => Result::err(format!("Invalid asset type: {raw}")),
        }
    }
}

impl ValueObject for AssetType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_asset_types() {
        assert_eq!(AssetType::try_from_str("stock").unwrap(), AssetType::Stock);
        assert_eq!(AssetType::try_from_str("FII").unwrap(), AssetType::Fii);
        assert!(AssetType::try_from_str("crypto").is_err());
    }
}
