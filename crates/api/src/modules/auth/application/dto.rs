use serde::{Deserialize, Serialize};

use crate::modules::auth::domain::Role;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterIn {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginIn {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthTokenOut {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: String,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserOut {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
}
