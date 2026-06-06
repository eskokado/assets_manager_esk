use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared_kernel::UseCase;

use crate::modules::auth::application::{
    GetCurrentUser, LoginIn, LoginUser, RegisterIn, RegisterUser,
};
use crate::modules::auth::domain::ports::UserRepository;
use crate::modules::auth::infrastructure::UserRepositorySqlx;
use crate::modules::auth::interfaces::http::middleware::AuthUser;
use crate::AppState;

fn user_repository(state: &AppState) -> Arc<dyn UserRepository> {
    Arc::new(UserRepositorySqlx::new(state.db.clone()))
}

pub async fn register(State(state): State<AppState>, Json(input): Json<RegisterIn>) -> Response {
    let use_case = RegisterUser::new(user_repository(&state));
    match use_case.execute(input).await {
        shared_kernel::Result::Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        shared_kernel::Result::Err(errors) => {
            let message = errors
                .iter()
                .map(|e| e.0.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            let status = if message.contains("already registered") {
                StatusCode::CONFLICT
            } else {
                StatusCode::BAD_REQUEST
            };
            (status, message).into_response()
        }
    }
}

pub async fn login(State(state): State<AppState>, Json(input): Json<LoginIn>) -> Response {
    let use_case = LoginUser::new(
        user_repository(&state),
        state.jwt_secret.clone(),
        state.jwt_expires_in_secs,
    );
    match use_case.execute(input).await {
        shared_kernel::Result::Ok(token) => Json(token).into_response(),
        shared_kernel::Result::Err(_) => {
            (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
        }
    }
}

pub async fn me(State(state): State<AppState>, auth: AuthUser) -> Response {
    let use_case = GetCurrentUser::new(user_repository(&state));
    match use_case.execute(auth.user_id).await {
        shared_kernel::Result::Ok(user) => Json(user).into_response(),
        shared_kernel::Result::Err(errors) => {
            let message = errors
                .iter()
                .map(|e| e.0.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            (StatusCode::NOT_FOUND, message).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::auth::application::{AuthTokenOut, UserOut};

    #[test]
    fn login_in_and_user_out_types_exist() {
        let _ = std::any::type_name::<LoginIn>();
        let _ = std::any::type_name::<UserOut>();
        let _ = std::any::type_name::<AuthTokenOut>();
    }
}
