use std::sync::Arc;

use leptos::prelude::*;

use crate::features::auth::application::LoadProfileUseCase;
use crate::features::auth::context::AuthContext;
use crate::features::auth::domain::UserProfile;
use crate::features::auth::AuthHttpRepository;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let profile = RwSignal::new(None::<UserProfile>);
    let error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        let Some(token) = auth.access_token() else {
            return;
        };
        leptos::task::spawn_local(async move {
            let use_case = LoadProfileUseCase::new(Arc::new(AuthHttpRepository::new()));
            match use_case.execute(&token).await {
                shared_kernel::Result::Ok(user) => profile.set(Some(user)),
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
        });
    });

    view! {
        <div class="p-8">
            <h1 class="text-2xl font-bold">"Perfil"</h1>
            <Show when=move || error.get().is_some()>
                <p class="mt-2 text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <Show when=move || profile.get().is_some()>
                {move || {
                    profile.get().map(|user| view! {
                        <dl class="mt-4 space-y-2">
                            <div><dt class="font-medium">"Nome"</dt><dd>{user.name().to_string()}</dd></div>
                            <div><dt class="font-medium">"E-mail"</dt><dd>{user.email().to_string()}</dd></div>
                            <div><dt class="font-medium">"Perfil"</dt><dd>{user.role().to_string()}</dd></div>
                        </dl>
                    })
                }}
            </Show>
        </div>
    }
}
