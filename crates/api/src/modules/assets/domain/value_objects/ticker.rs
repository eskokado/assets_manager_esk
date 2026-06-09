use shared_kernel::{Result, ValueObject};

const MAX_LENGTH: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticker(String);

impl Ticker {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let value = raw.as_ref().trim().to_uppercase();
        if value.is_empty() {
            return Result::err("Ticker is required");
        }
        if value.len() > MAX_LENGTH {
            return Result::err(format!("Ticker must be at most {MAX_LENGTH} characters"));
        }
        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Result::err("Ticker must be alphanumeric");
        }
        Result::ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Ticker {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_to_uppercase() {
        assert_eq!(Ticker::try_new("petr4").unwrap().as_str(), "PETR4");
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(Ticker::try_new("PET-4").is_err());
    }
}
