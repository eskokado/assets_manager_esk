use async_trait::async_trait;
use shared_kernel::{EntityId, Result};
use sqlx::PgPool;

use crate::modules::auth::domain::ports::UserRepository;
use crate::modules::auth::domain::User;
use crate::modules::auth::infrastructure::persistence::user_record::UserRecord;

pub struct UserRepositorySqlx {
    pool: PgPool,
}

impl UserRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySqlx {
    async fn save(&self, entity: &User) -> Result<()> {
        let record = UserRecord::from(entity.to_record());
        match sqlx::query(
            r#"
            INSERT INTO users (id, name, email, password_hash, role, active, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                role = EXCLUDED.role,
                active = EXCLUDED.active
            "#,
        )
        .bind(record.id)
        .bind(record.name)
        .bind(record.email)
        .bind(record.password_hash)
        .bind(record.role)
        .bind(record.active)
        .bind(record.created_at)
        .execute(&self.pool)
        .await
        {
            Ok(_) => Result::ok(()),
            Err(e) => Result::err(format!("Failed to save user: {e}")),
        }
    }

    async fn find_by_id(&self, id: EntityId) -> Result<Option<User>> {
        match sqlx::query_as::<_, UserRecord>(
            "SELECT id, name, email, password_hash, role, active, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(record) => match record {
                Some(record) => record.to_domain().map(Some),
                None => Result::ok(None),
            },
            Err(e) => Result::err(format!("Failed to find user: {e}")),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        match sqlx::query_as::<_, UserRecord>(
            "SELECT id, name, email, password_hash, role, active, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(record) => match record {
                Some(record) => record.to_domain().map(Some),
                None => Result::ok(None),
            },
            Err(e) => Result::err(format!("Failed to find user: {e}")),
        }
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool> {
        match sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(&self.pool)
            .await
        {
            Ok(exists) => Result::ok(exists),
            Err(e) => Result::err(format!("Failed to check email: {e}")),
        }
    }
}
