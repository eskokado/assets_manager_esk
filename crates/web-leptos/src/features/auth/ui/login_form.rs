use std::sync::Arc;

use leptos::prelude::*;

use crate::features::auth::application::LoginUseCase;
use crate::features::auth::context::AuthContext;
use crate::features::auth::ui::PasswordInput;
use crate::features::auth::AuthHttpRepository;

#[component]
pub fn LoginForm() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = leptos_router::hooks::use_navigate();
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let auth = auth;
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let use_case = LoginUseCase::new(Arc::new(AuthHttpRepository::new()));
            match use_case.execute(&email_val, &password_val).await {
                shared_kernel::Result::Ok(session) => {
                    auth.set_session(session);
                    navigate("/", Default::default());
                }
                shared_kernel::Result::Err(errors) => {
                    error.set(Some(
                        errors
                            .iter()
                            .map(|e| e.0.as_str())
                            .collect::<Vec<_>>()
                            .join("; "),
                    ));
                }
            }
            loading.set(false);
        });
    };

    view! {
        <form class="mx-auto max-w-md space-y-4 rounded-lg border border-border p-6" on:submit=on_submit>
            <h1 class="text-xl font-semibold">"Login"</h1>
            <div>
                <label class="mb-1 block text-sm" for="login-email">"E-mail"</label>
                <input
                    id="login-email"
                    class="w-full rounded-md border border-border px-3 py-2"
                    type="email"
                    prop:value=move || email.get()
                    on:input=move |ev| email.set(event_target_value(&ev))
                    required
                />
            </div>
            <PasswordInput value=password id="login-password".to_string() />
            <Show when=move || error.get().is_some()>
                <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <button
                class="w-full rounded-md bg-primary px-4 py-2 text-primary-foreground disabled:opacity-50"
                type="submit"
                disabled=move || loading.get()
            >
                {move || if loading.get() { "Entrando..." } else { "Entrar" }}
            </button>
            <p class="text-sm">
                "Não tem conta? "
                <a href="/register" class="underline">"Registrar"</a>
            </p>
        </form>
    }
}
