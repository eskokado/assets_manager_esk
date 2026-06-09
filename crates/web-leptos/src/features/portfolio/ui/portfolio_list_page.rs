use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::features::auth::AuthContext;
use crate::features::portfolio::application::LoadPortfolioUseCase;
use crate::features::portfolio::domain::PortfolioSummary;
use crate::features::portfolio::infrastructure::PortfolioHttpRepository;

#[component]
pub fn PortfolioListPage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(true);
    let portfolio = RwSignal::new(None::<PortfolioSummary>);

    Effect::new(move |_| {
        let token = auth.access_token();
        if token.is_none() {
            return;
        }
        let token = token.unwrap();
        loading.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            let use_case = LoadPortfolioUseCase::new(Arc::new(PortfolioHttpRepository::new()));
            match use_case.execute(&token).await {
                shared_kernel::Result::Ok(summary) => portfolio.set(Some(summary)),
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
                <h1 class="text-2xl font-semibold">"Carteira"</h1>
                <A href="/trades/buy">
                    <span class="rounded-md bg-primary px-3 py-2 text-sm text-primary-foreground">
                        "Comprar ativo"
                    </span>
                </A>
            </div>
            <Show when=move || loading.get()>
                <p class="text-sm text-muted-foreground">"Carregando carteira..."</p>
            </Show>
            <Show when=move || error.get().is_some()>
                <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
            </Show>
            <Show when=move || {
                !loading.get() && portfolio.get().is_some_and(|p| p.positions().is_empty())
            }>
                <div class="rounded-md border border-border p-6 text-center">
                    <p class="text-muted-foreground">"Você ainda não possui posições abertas."</p>
                    <A href="/trades/buy">
                        <span class="mt-3 inline-block text-sm text-primary underline">
                            "Registrar primeira compra"
                        </span>
                    </A>
                </div>
            </Show>
            <Show when=move || portfolio.get().is_some_and(|p| !p.positions().is_empty())>
                {move || {
                    let summary = portfolio.get().unwrap();
                    view! {
                        <p class="text-sm text-muted-foreground">
                            {format!("Total investido: {}", summary.total_invested())}
                        </p>
                        <div class="overflow-x-auto rounded-md border border-border">
                            <table class="min-w-full text-sm">
                                <thead class="bg-muted/40">
                                    <tr>
                                        <th class="px-4 py-2 text-left">"Ticker"</th>
                                        <th class="px-4 py-2 text-left">"Nome"</th>
                                        <th class="px-4 py-2 text-right">"Qtd"</th>
                                        <th class="px-4 py-2 text-right">"Preço médio"</th>
                                        <th class="px-4 py-2 text-right">"Total"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || summary.positions().to_vec()
                                        key=|position| position.asset_id().to_string()
                                        children=move |position| {
                                            let asset_id = position.asset_id().to_string();
                                            let ticker = position.ticker().to_string();
                                            let name = position.name().to_string();
                                            let quantity = position.quantity().to_string();
                                            let average_price = position.average_price().to_string();
                                            let total_invested = position.total_invested().to_string();
                                            view! {
                                                <tr class="border-t border-border">
                                                    <td class="px-4 py-2">
                                                        <A href=format!("/portfolio/assets/{asset_id}")>
                                                            {ticker}
                                                        </A>
                                                    </td>
                                                    <td class="px-4 py-2">{name}</td>
                                                    <td class="px-4 py-2 text-right">{quantity}</td>
                                                    <td class="px-4 py-2 text-right">{average_price}</td>
                                                    <td class="px-4 py-2 text-right">{total_invested}</td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
