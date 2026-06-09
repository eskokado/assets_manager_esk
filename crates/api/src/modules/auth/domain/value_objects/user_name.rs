use shared_kernel::{Result, ValueObject};

const MAX_LENGTH: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let value = raw.as_ref().trim();
        if value.is_empty() {
            return Result::err("Name is required");
        }
        if value.len() > MAX_LENGTH {
            return Result::err(format!("Name must be at most {MAX_LENGTH} characters"));
        }
        Result::ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for UserName {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_name() {
        assert_eq!(UserName::try_new("Alice").unwrap().as_str(), "Alice");
    }

    #[test]
    fn rejects_empty_name() {
        assert!(UserName::try_new("  ").is_err());
    }
}
