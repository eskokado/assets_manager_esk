use leptos::prelude::*;

use crate::features::auth::domain::AuthSession;

const STORAGE_KEY: &str = "assets_manage_auth_session";

#[derive(Clone, Copy)]
pub struct AuthContext {
    session: RwSignal<Option<AuthSession>>,
}

impl AuthContext {
    pub fn provide() -> Self {
        let session = RwSignal::new(None::<AuthSession>);
        let ctx = Self { session };
        ctx.hydrate_from_storage();
        provide_context(ctx);
        ctx
    }

    pub fn use_ctx() -> Self {
        expect_context::<Self>()
    }

    pub fn session(&self) -> RwSignal<Option<AuthSession>> {
        self.session
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.with(|s| s.is_some())
    }

    pub fn access_token(&self) -> Option<String> {
        self.session
            .with(|s| s.as_ref().map(|v| v.access_token().to_string()))
    }

    pub fn set_session(&self, session: AuthSession) {
        self.session.set(Some(session.clone()));
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

    pub fn clear(&self) {
        self.session.set(None);
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item(STORAGE_KEY);
            }
        }
    }

    fn hydrate_from_storage(&self) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(Some(storage)) = window.local_storage() else {
            return;
        };
        let Ok(Some(raw)) = storage.get_item(STORAGE_KEY) else {
            return;
        };
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let (Some(token), Some(expires), Some(user_id), Some(role)) = (
                value.get("access_token").and_then(|v| v.as_str()),
                value.get("expires_in").and_then(|v| v.as_i64()),
                value.get("user_id").and_then(|v| v.as_str()),
                value.get("role").and_then(|v| v.as_str()),
            ) {
                if let shared_kernel::Result::Ok(session) = AuthSession::try_new(
                    token.to_string(),
                    expires,
                    user_id.to_string(),
                    role.to_string(),
                ) {
                    self.session.set(Some(session));
                }
            }
        }
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    AuthContext::provide();
    children()
}

#[component]
pub fn RequireAuth(children: Children) -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if !auth.is_authenticated() {
            navigate("/login", Default::default());
        }
    });

    children()
}
