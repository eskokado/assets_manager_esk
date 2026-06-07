use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::features::auth::AuthContext;
use crate::features::trading::application::ListTradesUseCase;
use crate::features::trading::infrastructure::TradeHttpRepository;
use crate::features::trading::ports::ListTradesQuery;

#[component]
pub fn TradeListPage() -> impl IntoView {
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
            let use_case = ListTradesUseCase::new(Arc::new(TradeHttpRepository::new()));
            match use_case
                .execute(
                    &token,
                    ListTradesQuery {
                        page: Some(1),
                        limit: Some(50),
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
                <h1 class="text-2xl font-semibold">"Operações"</h1>
                <div class="flex gap-2">
                    <A href="/trades/buy">
                        <span class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground">"Comprar"</span>
                    </A>
                    <A href="/trades/sell">
                        <span class="rounded-md border border-border px-4 py-2 text-sm">"Vender"</span>
                    </A>
                </div>
            </div>
            <Show when=move || loading.get()>
                <p class="text-sm text-muted-foreground">"Carregando..."</p>
            </Show>
            <Show when=move || error.get().is_some()>
                <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <Show when=move || !loading.get() && error.get().is_none() && items.get().is_empty()>
                <p class="text-sm text-muted-foreground">"Nenhuma operação registrada."</p>
            </Show>
            <Show when=move || !loading.get() && error.get().is_none() && !items.get().is_empty()>
                <p class="text-sm text-muted-foreground">{move || format!("Total: {}", total.get())}</p>
                <div class="overflow-x-auto rounded-lg border border-border">
                    <table class="min-w-full text-sm">
                        <thead class="bg-muted/40">
                            <tr>
                                <th class="px-4 py-2 text-left">"Data"</th>
                                <th class="px-4 py-2 text-left">"Lado"</th>
                                <th class="px-4 py-2 text-left">"Ticker"</th>
                                <th class="px-4 py-2 text-left">"Qtd"</th>
                                <th class="px-4 py-2 text-left">"Preço"</th>
                                <th class="px-4 py-2 text-left">"Total"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || items.get()
                                key=|item| item.id().to_string()
                                children=move |item| {
                                    view! {
                                        <tr class="border-t border-border">
                                            <td class="px-4 py-2">{item.traded_at().to_string()}</td>
                                            <td class="px-4 py-2">{item.side().to_string()}</td>
                                            <td class="px-4 py-2">{item.ticker().to_string()}</td>
                                            <td class="px-4 py-2">{item.quantity().to_string()}</td>
                                            <td class="px-4 py-2">{item.unit_price().to_string()}</td>
                                            <td class="px-4 py-2">{item.total().to_string()}</td>
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
