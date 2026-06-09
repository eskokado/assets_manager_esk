use chrono::{DateTime, Utc};
use shared_kernel::{try_domain, Email, EntityId, Result};
use sqlx::FromRow;

use crate::modules::auth::domain::{HashPassword, Role, User, UserName, UserRecordData};

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: EntityId,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

impl UserRecord {
    pub fn to_domain(&self) -> Result<User> {
        shared_kernel::Result::ok(User::reconstitute(
            self.id,
            try_domain!(UserName::try_new(&self.name)),
            try_domain!(Email::try_new(&self.email)),
            try_domain!(HashPassword::try_new(&self.password_hash)),
            try_domain!(Role::try_from_str(&self.role)),
            self.active,
            self.created_at,
        ))
    }
}

impl From<UserRecordData> for UserRecord {
    fn from(data: UserRecordData) -> Self {
        Self {
            id: data.id,
            name: data.name,
            email: data.email,
            password_hash: data.password_hash,
            role: data.role,
            active: data.active,
            created_at: data.created_at,
        }
    }
}
