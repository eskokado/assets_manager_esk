use axum::{
    extract::FromRef,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use shared_kernel::EntityId;
use uuid::Uuid;

use crate::modules::auth::domain::Role;
use crate::AppState;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: EntityId,
    pub role: Role,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Claims {
    sub: String,
    role: Role,
    exp: i64,
    iat: i64,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::extract::FromRef;

        let app_state = AppState::from_ref(state);
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::Unauthorized)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(app_state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::Unauthorized)?;

        let user_id =
            Uuid::parse_str(&token_data.claims.sub).map_err(|_| AuthError::Unauthorized)?;

        Ok(AuthUser {
            user_id,
            role: token_data.claims.role,
        })
    }
}

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
