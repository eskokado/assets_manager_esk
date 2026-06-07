use shared_kernel::{Result, ValueObject};

const MAX_LENGTH: usize = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetName(String);

impl AssetName {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let value = raw.as_ref().trim();
        if value.is_empty() {
            return Result::err("Asset name is required");
        }
        if value.len() > MAX_LENGTH {
            return Result::err(format!("Asset name must be at most {MAX_LENGTH} characters"));
        }
        Result::ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for AssetName {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_name() {
        assert_eq!(
            AssetName::try_new("Petrobras").unwrap().as_str(),
            "Petrobras"
        );
    }

    #[test]
    fn rejects_empty_name() {
        assert!(AssetName::try_new("  ").is_err());
    }
}
