use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Entity, Result, UseCase};
use uuid::Uuid;

use crate::modules::auth::application::dto::{RegisterIn, UserOut};
use crate::modules::auth::domain::ports::UserRepository;
use crate::modules::auth::domain::services::PasswordPolicy;
use crate::modules::auth::domain::User;

pub struct RegisterUser {
    repository: Arc<dyn UserRepository>,
}

impl RegisterUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<RegisterIn, UserOut> for RegisterUser {
    async fn execute(&self, input: RegisterIn) -> Result<UserOut> {
        let (name, email, password) = try_domain!(User::validate_register(
            &input.name,
            &input.email,
            &input.password
        ));
        try_domain!(PasswordPolicy::validate(&password));

        if try_domain!(self.repository.exists_by_email(email.as_str()).await) {
            return Result::err("Email already registered");
        }

        let user = try_domain!(User::register(Uuid::new_v4(), name, email, password));
        try_domain!(self.repository.save(&user).await);

        Result::ok(UserOut {
            id: user.id().to_string(),
            name: user.name().as_str().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::{EntityId, Result};

    use super::*;
    use crate::modules::auth::application::dto::RegisterIn;
    use crate::modules::auth::domain::ports::UserRepository;
    use crate::modules::auth::domain::User;
    use shared_kernel::Entity;

    struct MockRepo {
        users: tokio::sync::Mutex<Vec<User>>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self {
                users: tokio::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockRepo {
        async fn save(&self, entity: &User) -> Result<()> {
            self.users.lock().await.push(entity.clone());
            Result::ok(())
        }

        async fn find_by_id(&self, id: EntityId) -> Result<Option<User>> {
            Result::ok(
                self.users
                    .lock()
                    .await
                    .iter()
                    .find(|u| *u.id() == id)
                    .cloned(),
            )
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
            Result::ok(
                self.users
                    .lock()
                    .await
                    .iter()
                    .find(|u| u.email().as_str() == email)
                    .cloned(),
            )
        }

        async fn exists_by_email(&self, email: &str) -> Result<bool> {
            Result::ok(
                self.users
                    .lock()
                    .await
                    .iter()
                    .any(|u| u.email().as_str() == email),
            )
        }
    }

    #[tokio::test]
    async fn registers_new_user() {
        let use_case = RegisterUser::new(Arc::new(MockRepo::new()));
        let out = use_case
            .execute(RegisterIn {
                name: "Alice".into(),
                email: "alice@example.com".into(),
                password: "password123".into(),
            })
            .await
            .unwrap();

        assert_eq!(out.email, "alice@example.com");
        assert_eq!(out.role, crate::modules::auth::domain::Role::Investor);
    }

    #[tokio::test]
    async fn rejects_duplicate_email() {
        let repo = Arc::new(MockRepo::new());
        let use_case = RegisterUser::new(repo.clone());
        let input = RegisterIn {
            name: "Alice".into(),
            email: "alice@example.com".into(),
            password: "password123".into(),
        };
        use_case.execute(input.clone()).await.unwrap();
        assert!(use_case.execute(input).await.is_err());
    }
}
