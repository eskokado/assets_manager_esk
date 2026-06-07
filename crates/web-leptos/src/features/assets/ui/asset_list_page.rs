use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::features::assets::application::ListAssetsUseCase;
use crate::features::assets::infrastructure::AssetHttpRepository;
use crate::features::assets::ports::ListAssetsQuery;
use crate::features::auth::AuthContext;

#[component]
pub fn AssetListPage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(true);
    let items = RwSignal::new(Vec::new());
    let total = RwSignal::new(0i64);

    Effect::new(move |_| {
        let token = auth.access_token();
        if token.is_none() {
            return;
        }
        let token = token.unwrap();
        loading.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            let use_case = ListAssetsUseCase::new(Arc::new(AssetHttpRepository::new()));
            match use_case
                .execute(
                    &token,
                    ListAssetsQuery {
                        page: Some(1),
                        limit: Some(50),
                        asset_type: None,
                        active: None,
                        search: None,
                    },
                )
                .await
            {
                shared_kernel::Result::Ok(page) => {
                    items.set(page.items);
                    total.set(page.total);
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
    });

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Ativos"</h1>
                <A href="/admin/assets/new">
                    <span class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground">"Novo ativo"</span>
                </A>
            </div>
            <Show when=move || loading.get()>
                <p class="text-sm text-muted-foreground">"Carregando..."</p>
            </Show>
            <Show when=move || error.get().is_some()>
                <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <Show when=move || !loading.get() && error.get().is_none()>
                <p class="text-sm text-muted-foreground">{move || format!("Total: {}", total.get())}</p>
                <div class="overflow-x-auto rounded-lg border border-border">
                    <table class="min-w-full text-sm">
                        <thead class="bg-muted/40">
                            <tr>
                                <th class="px-4 py-2 text-left">"Ticker"</th>
                                <th class="px-4 py-2 text-left">"Nome"</th>
                                <th class="px-4 py-2 text-left">"Tipo"</th>
                                <th class="px-4 py-2 text-left">"Status"</th>
                                <th class="px-4 py-2 text-left">"Ações"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || items.get()
                                key=|item| item.id().to_string()
                                children=move |item| {
                                    let id = item.id().to_string();
                                    let ticker = item.ticker().to_string();
                                    let name = item.name().to_string();
                                    let asset_type = item.asset_type().to_string();
                                    let active = item.active();
                                    view! {
                                        <tr class="border-t border-border">
                                            <td class="px-4 py-2">{ticker}</td>
                                            <td class="px-4 py-2">{name}</td>
                                            <td class="px-4 py-2">{asset_type}</td>
                                            <td class="px-4 py-2">{if active { "Ativo" } else { "Inativo" }}</td>
                                            <td class="px-4 py-2">
                                                <A href=format!("/admin/assets/{id}/edit")>
                                                    <span class="underline">"Editar"</span>
                                                </A>
                                            </td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>
                    </table>
                </div>
            </Show>
        </div>
    }
}
