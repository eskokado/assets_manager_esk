use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Entity, EntityId, Result, UseCase};

use crate::modules::auth::application::dto::UserOut;
use crate::modules::auth::domain::ports::UserRepository;

pub struct GetCurrentUser {
    repository: Arc<dyn UserRepository>,
}

impl GetCurrentUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<EntityId, UserOut> for GetCurrentUser {
    async fn execute(&self, user_id: EntityId) -> Result<UserOut> {
        let user = match try_domain!(self.repository.find_by_id(user_id).await) {
            Some(user) if user.active() => user,
            _ => return Result::err("User not found"),
        };

        Result::ok(UserOut {
            id: user.id().to_string(),
            name: user.name().as_str().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role(),
        })
    }
}
