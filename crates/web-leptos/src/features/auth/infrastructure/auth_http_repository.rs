use async_trait::async_trait;
use reqwest::Client;
use shared_kernel::{DomainError, Result};

use crate::features::auth::domain::{AuthSession, UserProfile};
use crate::features::auth::infrastructure::dto::{
    AuthTokenDto, LoginRequestDto, RegisterRequestDto, UserProfileDto,
};
use crate::features::auth::ports::AuthRepository;
use crate::shared::config::api_base_url;

pub struct AuthHttpRepository {
    client: Client,
    base_url: String,
}

impl AuthHttpRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: api_base_url(),
        }
    }

    async fn parse_error(res: reqwest::Response) -> Vec<DomainError> {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        vec![DomainError::new(format!("HTTP {status}: {body}"))]
    }
}

impl Default for AuthHttpRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AuthRepository for AuthHttpRepository {
    async fn login(&self, email: &str, password: &str) -> Result<AuthSession> {
        let url = format!("{}/api/auth/login", self.base_url);
        let res = self
            .client
            .post(url)
            .json(&LoginRequestDto {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: AuthTokenDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        AuthSession::try_new(dto.access_token, dto.expires_in, dto.user_id, dto.role)
    }

    async fn register(&self, name: &str, email: &str, password: &str) -> Result<()> {
        let url = format!("{}/api/auth/register", self.base_url);
        let res = self
            .client
            .post(url)
            .json(&RegisterRequestDto {
                name: name.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        Result::ok(())
    }

    async fn load_profile(&self, access_token: &str) -> Result<UserProfile> {
        let url = format!("{}/api/auth/me", self.base_url);
        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: UserProfileDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        UserProfile::try_new(dto.id, dto.name, dto.email, dto.role)
    }
}
