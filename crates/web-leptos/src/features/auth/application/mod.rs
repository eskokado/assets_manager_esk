use std::sync::Arc;

use shared_kernel::Result;

use crate::features::auth::domain::AuthSession;
use crate::features::auth::ports::AuthRepository;

pub struct LoginUseCase {
    repository: Arc<dyn AuthRepository>,
}

impl LoginUseCase {
    pub fn new(repository: Arc<dyn AuthRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<AuthSession> {
        if email.trim().is_empty() || password.is_empty() {
            return Result::err("Email and password are required");
        }
        self.repository.login(email, password).await
    }
}

pub struct RegisterUseCase {
    repository: Arc<dyn AuthRepository>,
}

impl RegisterUseCase {
    pub fn new(repository: Arc<dyn AuthRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, name: &str, email: &str, password: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Result::err("Name is required");
        }
        if email.trim().is_empty() {
            return Result::err("Email is required");
        }
        if password.len() < 8 {
            return Result::err("Password must be at least 8 characters");
        }
        self.repository.register(name, email, password).await
    }
}

pub struct LoadProfileUseCase {
    repository: Arc<dyn AuthRepository>,
}

impl LoadProfileUseCase {
    pub fn new(repository: Arc<dyn AuthRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        access_token: &str,
    ) -> Result<crate::features::auth::domain::UserProfile> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        self.repository.load_profile(access_token).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::Result;

    use super::*;
    use crate::features::auth::domain::{AuthSession, UserProfile};

    struct MockRepo;

    #[async_trait]
    impl AuthRepository for MockRepo {
        async fn login(&self, email: &str, password: &str) -> Result<AuthSession> {
            if email == "alice@example.com" && password == "password123" {
                AuthSession::try_new("token".into(), 3600, "1".into(), "investor".into())
            } else {
                Result::err("Invalid credentials")
            }
        }

        async fn register(&self, _name: &str, _email: &str, _password: &str) -> Result<()> {
            Result::ok(())
        }

        async fn load_profile(&self, token: &str) -> Result<UserProfile> {
            if token == "token" {
                UserProfile::try_new(
                    "1".into(),
                    "Alice".into(),
                    "alice@example.com".into(),
                    "investor".into(),
                )
            } else {
                Result::err("Unauthorized")
            }
        }
    }

    #[tokio::test]
    async fn login_use_case_returns_session() {
        let use_case = LoginUseCase::new(Arc::new(MockRepo));
        let session = use_case
            .execute("alice@example.com", "password123")
            .await
            .unwrap();
        assert_eq!(session.access_token(), "token");
    }

    #[tokio::test]
    async fn register_use_case_validates_password() {
        let use_case = RegisterUseCase::new(Arc::new(MockRepo));
        assert!(use_case
            .execute("Alice", "alice@example.com", "short")
            .await
            .is_err());
    }

    #[tokio::test]
    async fn load_profile_use_case_returns_profile() {
        let use_case = LoadProfileUseCase::new(Arc::new(MockRepo));
        let profile = use_case.execute("token").await.unwrap();
        assert_eq!(profile.name(), "Alice");
    }
}
