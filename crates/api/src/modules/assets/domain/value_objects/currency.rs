use shared_kernel::{Result, ValueObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Currency(String);

impl Currency {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let code = raw.as_ref().trim().to_uppercase();
        if code.len() != 3 || !code.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Result::err("Currency must be a 3-letter ISO code");
        }
        Result::ok(Self(code))
    }

    pub fn brl() -> Self {
        Self("BRL".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Currency {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_iso_code() {
        assert_eq!(Currency::try_new("brl").unwrap().as_str(), "BRL");
    }

    #[test]
    fn rejects_invalid_code() {
        assert!(Currency::try_new("BR").is_err());
    }
}
