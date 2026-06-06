use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthTokenDto {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequestDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserProfileDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}
