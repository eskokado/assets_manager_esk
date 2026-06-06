use bcrypt::{hash, verify, DEFAULT_COST};

use super::Password;
use shared_kernel::{Result, ValueObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashPassword(String);

impl HashPassword {
    pub fn try_new(hash: impl Into<String>) -> Result<Self> {
        let value = hash.into();
        if value.is_empty() {
            return Result::err("Password hash is required");
        }
        Result::ok(Self(value))
    }

    pub fn from_password(password: &Password) -> Result<Self> {
        let hashed = match hash(password.as_str(), DEFAULT_COST) {
            Ok(value) => value,
            Err(e) => return Result::err(format!("Failed to hash password: {e}")),
        };
        Self::try_new(hashed)
    }

    pub fn verify(&self, password: &Password) -> Result<bool> {
        match verify(password.as_str(), self.as_str()) {
            Ok(ok) => Result::ok(ok),
            Err(e) => Result::err(format!("Failed to verify password: {e}")),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for HashPassword {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies() {
        let password = Password::try_new("secret123").unwrap();
        let hash = HashPassword::from_password(&password).unwrap();
        assert!(hash.verify(&password).unwrap());
        assert!(!hash
            .verify(&Password::try_new("wrongpass1").unwrap())
            .unwrap());
    }
}
