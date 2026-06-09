use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSession {
    access_token: String,
    expires_in: i64,
    user_id: String,
    role: String,
}

impl AuthSession {
    pub fn try_new(
        access_token: String,
        expires_in: i64,
        user_id: String,
        role: String,
    ) -> Result<Self> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        if user_id.trim().is_empty() {
            return Result::err("User id is required");
        }
        if role.trim().is_empty() {
            return Result::err("Role is required");
        }
        Result::ok(Self {
            access_token,
            expires_in,
            user_id,
            role,
        })
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_session() {
        let session =
            AuthSession::try_new("token".into(), 3600, "user-1".into(), "investor".into()).unwrap();
        assert_eq!(session.access_token(), "token");
    }
}
