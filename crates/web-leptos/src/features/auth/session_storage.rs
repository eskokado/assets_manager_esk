use crate::features::auth::domain::AuthSession;

#[cfg(target_arch = "wasm32")]
const STORAGE_KEY: &str = "assets_manage_auth_session";

#[cfg(target_arch = "wasm32")]
pub fn load_session() -> Option<AuthSession> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let raw = storage.get_item(STORAGE_KEY).ok()??;
    let value = serde_json::from_str::<serde_json::Value>(&raw).ok()?;
    let token = value.get("access_token")?.as_str()?;
    let expires = value.get("expires_in")?.as_i64()?;
    let user_id = value.get("user_id")?.as_str()?;
    let role = value.get("role")?.as_str()?;
    match AuthSession::try_new(
        token.to_string(),
        expires,
        user_id.to_string(),
        role.to_string(),
    ) {
        shared_kernel::Result::Ok(session) => Some(session),
        shared_kernel::Result::Err(_) => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_session() -> Option<AuthSession> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn save_session(session: &AuthSession) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(
                STORAGE_KEY,
                &serde_json::json!({
                    "access_token": session.access_token(),
                    "expires_in": session.expires_in(),
                    "user_id": session.user_id(),
                    "role": session.role(),
                })
                .to_string(),
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_session(_session: &AuthSession) {}

#[cfg(target_arch = "wasm32")]
pub fn clear_session() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(STORAGE_KEY);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_session() {}
