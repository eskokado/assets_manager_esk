use leptos::prelude::*;

use crate::features::auth::domain::{AuthSession, UserProfile};
use crate::features::auth::session_storage;

#[derive(Clone, Copy)]
pub struct AuthContext {
    session: RwSignal<Option<AuthSession>>,
    profile: RwSignal<Option<UserProfile>>,
}

impl AuthContext {
    pub fn provide() -> Self {
        let session = RwSignal::new(None::<AuthSession>);
        let profile = RwSignal::new(None::<UserProfile>);
        let ctx = Self { session, profile };
        if let Some(stored) = session_storage::load_session() {
            session.set(Some(stored));
        }
        provide_context(ctx);
        ctx
    }

    pub fn use_ctx() -> Self {
        expect_context::<Self>()
    }

    pub fn session(&self) -> RwSignal<Option<AuthSession>> {
        self.session
    }

    pub fn profile(&self) -> RwSignal<Option<UserProfile>> {
        self.profile
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.with(|s| s.is_some())
    }

    pub fn access_token(&self) -> Option<String> {
        self.session
            .with(|s| s.as_ref().map(|v| v.access_token().to_string()))
    }

    pub fn set_session(&self, session: AuthSession) {
        session_storage::save_session(&session);
        self.session.set(Some(session));
    }

    pub fn clear(&self) {
        session_storage::clear_session();
        self.session.set(None);
        self.profile.set(None);
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
