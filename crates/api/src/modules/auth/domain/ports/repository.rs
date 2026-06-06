use async_trait::async_trait;
use shared_kernel::{EntityId, Result};

use crate::modules::auth::domain::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, entity: &User) -> Result<()>;
    async fn find_by_id(&self, id: EntityId) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn exists_by_email(&self, email: &str) -> Result<bool>;
}
