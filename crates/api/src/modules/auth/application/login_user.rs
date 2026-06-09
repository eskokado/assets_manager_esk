use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use shared_kernel::{try_domain, Email, Entity, Result, UseCase};

use crate::modules::auth::application::dto::{AuthTokenOut, LoginIn};
use crate::modules::auth::domain::ports::UserRepository;
use crate::modules::auth::domain::{Password, Role};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Role,
    exp: i64,
    iat: i64,
}

pub struct LoginUser {
    repository: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_expires_in_secs: i64,
}

impl LoginUser {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        jwt_secret: String,
        jwt_expires_in_secs: i64,
    ) -> Self {
        Self {
            repository,
            jwt_secret,
            jwt_expires_in_secs,
        }
    }

    fn issue_token(&self, user_id: &str, role: Role) -> Result<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id.to_string(),
            role,
            iat: now,
            exp: now + self.jwt_expires_in_secs,
        };
        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        ) {
            Ok(token) => Result::ok(token),
            Err(e) => Result::err(format!("Failed to issue token: {e}")),
        }
    }
}

#[async_trait]
impl UseCase<LoginIn, AuthTokenOut> for LoginUser {
    async fn execute(&self, input: LoginIn) -> Result<AuthTokenOut> {
        let email = try_domain!(Email::try_new(&input.email));
        let password = try_domain!(Password::try_new(&input.password));

        let user = match try_domain!(self.repository.find_by_email(email.as_str()).await) {
            Some(user) if user.active() => user,
            _ => return Result::err("Invalid credentials"),
        };

        if !try_domain!(user.verify_password(&password)) {
            return Result::err("Invalid credentials");
        }

        let access_token = try_domain!(self.issue_token(&user.id().to_string(), user.role()));

        Result::ok(AuthTokenOut {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expires_in_secs,
            user_id: user.id().to_string(),
            role: user.role(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::{Email, EntityId, Result};
    use uuid::Uuid;

    use super::*;
    use crate::modules::auth::application::dto::LoginIn;
    use crate::modules::auth::domain::ports::UserRepository;
    use crate::modules::auth::domain::{User, UserName};

    struct MockRepo {
        user: User,
    }

    #[async_trait]
    impl UserRepository for MockRepo {
        async fn save(&self, _entity: &User) -> Result<()> {
            Result::ok(())
        }

        async fn find_by_id(&self, id: EntityId) -> Result<Option<User>> {
            if *self.user.id() == id {
                Result::ok(Some(self.user.clone()))
            } else {
                Result::ok(None)
            }
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
            if self.user.email().as_str() == email {
                Result::ok(Some(self.user.clone()))
            } else {
                Result::ok(None)
            }
        }

        async fn exists_by_email(&self, _email: &str) -> Result<bool> {
            Result::ok(false)
        }
    }

    fn sample_user() -> User {
        User::register(
            Uuid::new_v4(),
            UserName::try_new("Alice").unwrap(),
            Email::try_new("alice@example.com").unwrap(),
            Password::try_new("password123").unwrap(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn login_returns_token() {
        let user = sample_user();
        let use_case = LoginUser::new(
            Arc::new(MockRepo { user }),
            "test-secret-key-at-least-32-chars!!".into(),
            3600,
        );

        let out = use_case
            .execute(LoginIn {
                email: "alice@example.com".into(),
                password: "password123".into(),
            })
            .await
            .unwrap();

        assert!(!out.access_token.is_empty());
        assert_eq!(out.token_type, "Bearer");
    }
}
