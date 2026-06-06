use crate::{Result, ValueObject};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let normalized = raw.as_ref().trim().to_lowercase();
        if normalized.is_empty() {
            return Result::err("Email is required");
        }
        if normalized.len() > 254 {
            return Result::err("Email is too long");
        }
        if !normalized.contains('@') || normalized.starts_with('@') || normalized.ends_with('@') {
            return Result::err("Invalid email format");
        }
        let (local, domain) = normalized.split_once('@').expect("email contains @");
        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Result::err("Invalid email format");
        }
        Result::ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Email {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_to_lowercase() {
        let email = Email::try_new("User@Example.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn rejects_invalid_email() {
        assert!(Email::try_new("not-an-email").is_err());
        assert!(Email::try_new("").is_err());
    }
}
