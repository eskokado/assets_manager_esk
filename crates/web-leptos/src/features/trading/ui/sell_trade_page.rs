use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};

use crate::features::auth::AuthContext;
use crate::features::portfolio::application::LoadPortfolioUseCase;
use crate::features::portfolio::infrastructure::PortfolioHttpRepository;
use crate::features::trading::application::SellAssetUseCase;
use crate::features::trading::domain::SellTradeForm;
use crate::features::trading::infrastructure::TradeHttpRepository;

#[component]
pub fn SellTradePage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = use_navigate();
    let query = use_query_map();
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(true);
    let positions = RwSignal::new(Vec::<(String, String, String)>::new());
    let asset_id = RwSignal::new(String::new());
    let max_quantity = RwSignal::new(String::new());
    let quantity = RwSignal::new(String::new());
    let unit_price = RwSignal::new(String::new());

    Effect::new(move |_| {
        let token = auth.access_token();
        if token.is_none() {
            return;
        }
        let token = token.unwrap();
        let preselected = query
            .get()
            .get("assetId")
            .map(|value| value.to_string())
            .unwrap_or_default();
        loading.set(true);

        leptos::task::spawn_local(async move {
            let portfolio_use_case =
                LoadPortfolioUseCase::new(Arc::new(PortfolioHttpRepository::new()));

            let summary = match portfolio_use_case.execute(&token).await {
                shared_kernel::Result::Ok(value) => value,
                shared_kernel::Result::Err(errors) => {
                    error.set(Some(
                        errors
                            .iter()
                            .map(|e| e.0.as_str())
                            .collect::<Vec<_>>()
                            .join("; "),
                    ));
                    loading.set(false);
                    return;
                }
            };

            let mut rows = Vec::new();
            for position in summary.positions() {
                rows.push((
                    position.asset_id().to_string(),
                    format!(
                        "{} — disponível: {}",
                        position.ticker(),
                        position.quantity()
                    ),
                    position.quantity().to_string(),
                ));
            }
            positions.set(rows);

            if !preselected.is_empty() {
                if let Some((_, _, max)) =
                    positions.get().iter().find(|(id, _, _)| id == &preselected)
                {
                    asset_id.set(preselected);
                    max_quantity.set(max.clone());
                } else {
                    error.set(Some(
                        "Ativo não encontrado na carteira ou sem posição aberta.".into(),
                    ));
                }
            }

            loading.set(false);
        });
    });

    let on_asset_change = move |ev: leptos::ev::Event| {
        let selected = event_target_value(&ev);
        asset_id.set(selected.clone());
        if let Some((_, _, max)) = positions.get().iter().find(|(id, _, _)| id == &selected) {
            max_quantity.set(max.clone());
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let token = auth.access_token().unwrap_or_default();
        if token.is_empty() {
            return;
        }
        error.set(None);

        let form = match SellTradeForm::try_new(
            asset_id.get(),
            quantity.get(),
            unit_price.get(),
            max_quantity.get(),
        ) {
            shared_kernel::Result::Ok(value) => value,
            shared_kernel::Result::Err(errors) => {
                error.set(Some(
                    errors
                        .iter()
                        .map(|e| e.0.as_str())
                        .collect::<Vec<_>>()
                        .join("; "),
                ));
                return;
            }
        };

        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            let use_case = SellAssetUseCase::new(Arc::new(TradeHttpRepository::new()));
            match use_case.execute(&token, &form).await {
                shared_kernel::Result::Ok(_) => {
                    navigate("/trades", Default::default());
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
        });
    };

    view! {
        <div class="mx-auto max-w-lg space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Vender ativo"</h1>
                <A href="/portfolio">
                    <span class="text-sm text-primary underline">"Ver carteira"</span>
                </A>
            </div>
            <Show when=move || loading.get()>
                <p class="text-sm text-muted-foreground">"Carregando posições..."</p>
            </Show>
            <form class="space-y-4" on:submit=on_submit>
                <div>
                    <label class="mb-1 block text-sm">"Ativo"</label>
                    <select
                        class="w-full rounded-md border border-border bg-background px-3 py-2"
                        on:change=on_asset_change
                        prop:value=move || asset_id.get()
                        required
                    >
                        <option value="">"Selecione..."</option>
                        <For
                            each=move || positions.get()
                            key=|(id, _, _)| id.clone()
                            children=move |(id, label, _)| view! {
                                <option value=id.clone()>{label}</option>
                            }
                        />
                    </select>
                </div>
                <Show when=move || !max_quantity.get().is_empty()>
                    <p class="text-sm text-muted-foreground">
                        {move || format!("Saldo disponível: {}", max_quantity.get())}
                    </p>
                </Show>
                <div>
                    <label class="mb-1 block text-sm">"Quantidade"</label>
                    <input
                        class="w-full rounded-md border border-border bg-background px-3 py-2"
                        type="number"
                        step="0.00000001"
                        min="0"
                        prop:value=move || quantity.get()
                        on:input=move |ev| quantity.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div>
                    <label class="mb-1 block text-sm">"Preço unitário"</label>
                    <input
                        class="w-full rounded-md border border-border bg-background px-3 py-2"
                        type="number"
                        step="0.01"
                        min="0"
                        prop:value=move || unit_price.get()
                        on:input=move |ev| unit_price.set(event_target_value(&ev))
                        required
                    />
                </div>
                <Show when=move || error.get().is_some()>
                    <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
                </Show>
                <button type="submit" class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground">
                    "Confirmar venda"
                </button>
            </form>
        </div>
    }
}
