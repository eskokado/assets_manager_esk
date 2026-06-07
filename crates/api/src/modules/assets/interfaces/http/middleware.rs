use axum::extract::FromRef;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

use crate::modules::auth::domain::Role;
use crate::modules::auth::interfaces::http::middleware::AuthUser;
use crate::AppState;

#[derive(Debug, Clone)]
pub struct RequireAdmin {
    pub auth: AuthUser,
}

impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AdminError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = AuthUser::from_request_parts(parts, state)
            .await
            .map_err(|_| AdminError::Unauthorized)?;

        if auth.role != Role::Admin {
            return Err(AdminError::Forbidden);
        }

        Ok(Self { auth })
    }
}

#[derive(Debug)]
pub enum AdminError {
    Unauthorized,
    Forbidden,
}

impl IntoResponse for AdminError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
        }
    }
}
