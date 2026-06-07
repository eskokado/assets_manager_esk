use crate::components::{app_footer::AppFooter, sidebar_menu::SidebarMenu};
use crate::features::auth::application::LoadProfileUseCase;
use crate::features::auth::context::AuthContext;
use crate::features::auth::AuthHttpRepository;
use leptos::prelude::*;

#[component]
pub fn AdminShell(children: Children) -> impl IntoView {
    let sidebar_open = RwSignal::new(true);
    provide_context(sidebar_open);

    let auth = AuthContext::use_ctx();

    Effect::new(move |_| {
        if let (Some(token), None) = (auth.access_token(), auth.profile().get()) {
            leptos::task::spawn_local(async move {
                let use_case =
                    LoadProfileUseCase::new(std::sync::Arc::new(AuthHttpRepository::new()));
                if let shared_kernel::Result::Ok(user) = use_case.execute(&token).await {
                    auth.profile().set(Some(user));
                }
            });
        }
    });

    view! {
        <div class="flex min-h-screen bg-background text-foreground">
            <SidebarMenu open=sidebar_open/>
            <div class="flex flex-1 flex-col">
                <header class="flex h-16 items-center justify-between border-b border-border bg-card/50 backdrop-blur-sm px-6 sticky top-0 z-30">
                    <div class="flex items-center gap-4">
                        <button
                            type="button"
                            class="rounded-lg p-2 hover:bg-accent hover:text-accent-foreground transition-all duration-200"
                            on:click=move |_| sidebar_open.update(|v| *v = !*v)
                            aria-label="Toggle Menu"
                        >
                            <span class="text-xl">"☰"</span>
                        </button>
                        <span class="font-bold text-xl bg-gradient-to-r from-primary to-primary/60 bg-clip-text text-transparent">
                            "Assets Manage"
                        </span>
                    </div>

                    <div class="flex items-center gap-4">
                        <div class="hidden sm:flex flex-col items-end text-right">
                            <Show when=move || auth.profile().get().is_some()>
                                {move || {
                                    auth.profile().get().map(|user| view! {
                                        <span class="text-sm font-semibold leading-none mb-1">{user.name().to_string()}</span>
                                        <span class="text-xs text-muted-foreground leading-none">{user.email().to_string()}</span>
                                    })
                                }}
                            </Show>
                        </div>
                        <a
                            href="/profile"
                            class="flex h-10 w-10 items-center justify-center rounded-full bg-secondary hover:bg-secondary/80 text-secondary-foreground transition-all duration-200 shadow-sm border border-border"
                            title="Ver Perfil"
                        >
                            <span class="text-lg">"👤"</span>
                        </a>
                    </div>
                </header>
                <main class="flex-1 overflow-auto p-4 md:p-6">
                    {children()}
                </main>
                <AppFooter/>
            </div>
        </div>
    }
}
