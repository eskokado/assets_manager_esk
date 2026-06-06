use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    id: String,
    name: String,
    email: String,
    role: String,
}

impl UserProfile {
    pub fn try_new(id: String, name: String, email: String, role: String) -> Result<Self> {
        if id.trim().is_empty() {
            return Result::err("Profile id is required");
        }
        if name.trim().is_empty() {
            return Result::err("Profile name is required");
        }
        if email.trim().is_empty() {
            return Result::err("Profile email is required");
        }
        Result::ok(Self {
            id,
            name,
            email,
            role,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_profile() {
        let profile = UserProfile::try_new(
            "1".into(),
            "Alice".into(),
            "alice@example.com".into(),
            "investor".into(),
        )
        .unwrap();
        assert_eq!(profile.email(), "alice@example.com");
    }
}
