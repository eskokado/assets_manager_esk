use std::sync::Arc;

use leptos::prelude::*;

use crate::features::auth::application::RegisterUseCase;
use crate::features::auth::AuthHttpRepository;

#[component]
pub fn RegisterForm() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        error.set(None);

        let name_val = name.get();
        let email_val = email.get();
        let password_val = password.get();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let use_case = RegisterUseCase::new(Arc::new(AuthHttpRepository::new()));
            match use_case.execute(&name_val, &email_val, &password_val).await {
                shared_kernel::Result::Ok(()) => navigate("/login", Default::default()),
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
            <h1 class="text-xl font-semibold">"Registrar"</h1>
            <div>
                <label class="mb-1 block text-sm">"Nome"</label>
                <input
                    class="w-full rounded-md border border-border px-3 py-2"
                    type="text"
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                    required
                />
            </div>
            <div>
                <label class="mb-1 block text-sm">"E-mail"</label>
                <input
                    class="w-full rounded-md border border-border px-3 py-2"
                    type="email"
                    prop:value=move || email.get()
                    on:input=move |ev| email.set(event_target_value(&ev))
                    required
                />
            </div>
            <div>
                <label class="mb-1 block text-sm">"Senha"</label>
                <input
                    class="w-full rounded-md border border-border px-3 py-2"
                    type="password"
                    prop:value=move || password.get()
                    on:input=move |ev| password.set(event_target_value(&ev))
                    required
                />
            </div>
            <Show when=move || error.get().is_some()>
                <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <button
                class="w-full rounded-md bg-primary px-4 py-2 text-primary-foreground disabled:opacity-50"
                type="submit"
                disabled=move || loading.get()
            >
                {move || if loading.get() { "Registrando..." } else { "Criar conta" }}
            </button>
            <p class="text-sm">
                "Já tem conta? "
                <a href="/login" class="underline">"Login"</a>
            </p>
        </form>
    }
}
