use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::features::auth::AuthContext;
use crate::shared::shell_navigation::main_nav_items;

#[component]
pub fn SidebarMenu(open: RwSignal<bool>) -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let location = use_location();
    let items = Memo::new(move |_| {
        let is_admin = auth
            .session()
            .with(|s| s.as_ref().is_some_and(|v| v.role() == "admin"));
        main_nav_items(
            auth.is_authenticated(),
            is_admin,
            auth.is_authenticated(),
            auth.is_authenticated(),
        )
    });

    view! {
        <aside
            class=move || format!(
                "fixed inset-y-0 left-0 z-40 {} border-r border-border bg-card/80 backdrop-blur-md transition-all duration-300 ease-in-out md:sticky md:top-0 md:h-screen md:translate-x-0 shadow-lg {}",
                if open.get() { "w-64 translate-x-0" } else { "w-20 -translate-x-full md:translate-x-0" },
                if open.get() { "" } else { "overflow-hidden" }
            )
        >
            <div class="flex flex-col h-full py-6">
                <nav class="flex-1 px-3 space-y-2">
                    <For
                        each=move || items.get()
                        key=|item| item.href
                        children=move |item| {
                            let item_label = item.label;
                            let item_icon = item.icon;
                            let item_href = item.href;
                            let is_active = {
                                let href = item_href;
                                move || location.pathname.get() == href
                            };
                            view! {
                                <A
                                    href=item_href
                                    attr:class=move || {
                                        let base = "flex items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-muted-foreground hover:bg-secondary hover:text-secondary-foreground transition-all duration-200 group relative";
                                        if is_active() {
                                            format!("{} bg-primary/10 text-primary shadow-sm", base)
                                        } else {
                                            base.to_string()
                                        }
                                    }
                                >
                                    <span class="text-xl shrink-0 transition-transform duration-200 group-hover:scale-110">
                                        {item_icon}
                                    </span>
                                    <span
                                        class=move || format!(
                                            "whitespace-nowrap transition-all duration-300 {}",
                                            if open.get() { "opacity-100 translate-x-0" } else { "opacity-0 -translate-x-4 pointer-events-none" }
                                        )
                                    >
                                        {item_label}
                                    </span>

                                    <Show when=move || !open.get()>
                                        <div class="absolute left-full ml-2 px-2 py-1 bg-popover text-popover-foreground text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-50 shadow-md border border-border pointer-events-none">
                                            {item_label}
                                        </div>
                                    </Show>
                                </A>
                            }
                        }
                    />
                </nav>

                <div class="px-3 mt-auto border-t border-border pt-6">
                    <button
                        on:click=move |_| auth.clear()
                        class="flex w-full items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-red-500 hover:bg-red-50 dark:hover:bg-red-950/20 transition-all duration-200 group relative"
                    >
                        <span class="text-xl shrink-0 transition-transform duration-200 group-hover:scale-110">
                            "🚪"
                        </span>
                        <span
                            class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if open.get() { "opacity-100 translate-x-0" } else { "opacity-0 -translate-x-4 pointer-events-none" }
                            )
                        >
                            "Sair"
                        </span>

                        <Show when=move || !open.get()>
                            <div class="absolute left-full ml-2 px-2 py-1 bg-red-600 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-50 shadow-md pointer-events-none">
                                "Sair"
                            </div>
                        </Show>
                    </button>
                </div>
            </div>
        </aside>
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-30 bg-black/50 md:hidden"
                on:click=move |_| open.set(false)
            />
        </Show>
    }
}
