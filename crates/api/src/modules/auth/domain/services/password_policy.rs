use crate::modules::auth::domain::Password;
use shared_kernel::Result;

pub struct PasswordPolicy;

impl PasswordPolicy {
    pub fn validate(password: &Password) -> Result<()> {
        if password.as_str().len() < 8 {
            return Result::err("Password must be at least 8 characters");
        }
        Result::ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::auth::domain::Password;

    #[test]
    fn accepts_strong_password() {
        let password = Password::try_new("strongpass").unwrap();
        assert!(PasswordPolicy::validate(&password).is_ok());
    }
}
