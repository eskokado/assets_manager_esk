use chrono::{DateTime, Utc};
use shared_kernel::{combine2, Email, Entity, EntityId, Result};

use super::value_objects::{HashPassword, Password, Role, UserName};

#[derive(Debug, Clone)]
pub struct UserRecordData {
    pub id: EntityId,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: EntityId,
    name: UserName,
    email: Email,
    password_hash: HashPassword,
    role: Role,
    active: bool,
    created_at: DateTime<Utc>,
}

impl Entity for User {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl User {
    pub fn register(
        id: EntityId,
        name: UserName,
        email: Email,
        password: Password,
    ) -> Result<Self> {
        let password_hash = shared_kernel::try_domain!(HashPassword::from_password(&password));
        shared_kernel::Result::ok(Self {
            id,
            name,
            email,
            password_hash,
            role: Role::default_investor(),
            active: true,
            created_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: EntityId,
        name: UserName,
        email: Email,
        password_hash: HashPassword,
        role: Role,
        active: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
            role,
            active,
            created_at,
        }
    }

    pub fn verify_password(&self, password: &Password) -> Result<bool> {
        self.password_hash.verify(password)
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &HashPassword {
        &self.password_hash
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn to_record(&self) -> UserRecordData {
        UserRecordData {
            id: *self.id(),
            name: self.name().as_str().to_string(),
            email: self.email().as_str().to_string(),
            password_hash: self.password_hash().as_str().to_string(),
            role: self.role().as_str().to_string(),
            active: self.active(),
            created_at: self.created_at(),
        }
    }

    pub fn validate_register(
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(UserName, Email, Password)> {
        let (name, email) =
            shared_kernel::try_domain!(combine2(UserName::try_new(name), Email::try_new(email)));
        let password = shared_kernel::try_domain!(Password::try_new(password));
        shared_kernel::Result::ok((name, email, password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn register_creates_investor_with_hash() {
        let user = User::register(
            Uuid::new_v4(),
            UserName::try_new("Alice").unwrap(),
            Email::try_new("alice@example.com").unwrap(),
            Password::try_new("password123").unwrap(),
        )
        .unwrap();

        assert_eq!(user.role(), Role::Investor);
        assert!(user.active());
        assert_ne!(user.password_hash().as_str(), "password123");
    }

    #[test]
    fn verify_password_works() {
        let password = Password::try_new("password123").unwrap();
        let user = User::register(
            Uuid::new_v4(),
            UserName::try_new("Alice").unwrap(),
            Email::try_new("alice@example.com").unwrap(),
            password.clone(),
        )
        .unwrap();

        assert!(user.verify_password(&password).unwrap());
        assert!(!user
            .verify_password(&Password::try_new("wrongpass1").unwrap())
            .unwrap());
    }
}
