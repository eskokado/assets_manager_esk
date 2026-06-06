use async_trait::async_trait;
use shared_kernel::Result;

use crate::features::auth::domain::{AuthSession, UserProfile};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AuthRepository: Send + Sync {
    async fn login(&self, email: &str, password: &str) -> Result<AuthSession>;
    async fn register(&self, name: &str, email: &str, password: &str) -> Result<()>;
    async fn load_profile(&self, access_token: &str) -> Result<UserProfile>;
}
