use serde::{Deserialize, Serialize};

use shared_kernel::{Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Investor,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Investor => "investor",
        }
    }

    pub fn try_from_str(raw: &str) -> Result<Self> {
        match raw {
            "admin" => Result::ok(Self::Admin),
            "investor" => Result::ok(Self::Investor),
            _ => Result::err(format!("Invalid role: {raw}")),
        }
    }

    pub fn default_investor() -> Self {
        Self::Investor
    }
}

impl ValueObject for Role {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_roles() {
        assert_eq!(Role::try_from_str("admin").unwrap(), Role::Admin);
        assert_eq!(Role::try_from_str("investor").unwrap(), Role::Investor);
        assert!(Role::try_from_str("guest").is_err());
    }
}
