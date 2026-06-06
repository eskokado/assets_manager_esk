use crate::{Result, ValueObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn try_from_uuid(value: uuid::Uuid) -> Result<Self> {
        if value.is_nil() {
            return Result::err("Id cannot be nil");
        }
        Result::ok(Self(value))
    }

    pub fn try_from_str(value: &str) -> Result<Self> {
        match uuid::Uuid::parse_str(value.trim()) {
            Ok(parsed) => Self::try_from_uuid(parsed),
            Err(_) => Result::err("Invalid Id format"),
        }
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl ValueObject for Id {}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nil_uuid() {
        assert!(Id::try_from_uuid(uuid::Uuid::nil()).is_err());
    }

    #[test]
    fn parses_valid_uuid_string() {
        let id = uuid::Uuid::new_v4();
        let parsed = Id::try_from_str(&id.to_string()).unwrap();
        assert_eq!(parsed.as_uuid(), id);
    }

    #[test]
    fn rejects_invalid_string() {
        assert!(Id::try_from_str("not-a-uuid").is_err());
    }

    #[test]
    fn new_and_default_generate_valid_ids() {
        assert!(!Id::new().as_uuid().is_nil());
        assert!(!Id::default().as_uuid().is_nil());
    }
}
