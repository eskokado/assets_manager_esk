use shared_kernel::{Result, ValueObject};

const MIN_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn try_new(raw: impl AsRef<str>) -> Result<Self> {
        let value = raw.as_ref();
        if value.len() < MIN_LENGTH {
            return Result::err(format!("Password must be at least {MIN_LENGTH} characters"));
        }
        Result::ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Password {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_password() {
        assert!(Password::try_new("secret123").is_ok());
    }

    #[test]
    fn rejects_short_password() {
        assert!(Password::try_new("short").is_err());
    }
}
