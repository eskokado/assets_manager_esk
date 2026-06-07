use leptos::prelude::*;
use leptos_router::components::A;

use crate::features::auth::AuthContext;
use crate::shared::shell_navigation::main_nav_items;

#[component]
pub fn SidebarMenu(open: RwSignal<bool>) -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let items = Memo::new(move |_| {
        let is_admin = auth
            .session()
            .with(|s| s.as_ref().is_some_and(|v| v.role() == "admin"));
        main_nav_items(auth.is_authenticated(), is_admin)
    });

    view! {
        <aside
            class=move || format!(
                "fixed inset-y-0 left-0 z-40 w-64 border-r border-border bg-muted/30 transition-transform md:static md:translate-x-0 {}",
                if open.get() { "translate-x-0" } else { "-translate-x-full" }
            )
        >
            <nav class="flex flex-col gap-1 p-4">
                <For
                    each=move || items.get()
                    key=|item| item.href
                    children=move |item| view! {
                        <A href=item.href>
                            <span class="block rounded-md px-3 py-2 text-sm hover:bg-muted">
                                {item.label}
                            </span>
                        </A>
                    }
                />
            </nav>
        </aside>
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-30 bg-black/50 md:hidden"
                on:click=move |_| open.set(false)
            />
        </Show>
    }
}
