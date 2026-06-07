use leptos::prelude::*;

use crate::features::auth::AuthContext;

#[component]
pub fn RequireAdmin(children: Children) -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        let session = auth.session().get();
        match session {
            None => navigate("/login", Default::default()),
            Some(value) if value.role() != "admin" => navigate("/", Default::default()),
            _ => {}
        }
    });

    children()
}
