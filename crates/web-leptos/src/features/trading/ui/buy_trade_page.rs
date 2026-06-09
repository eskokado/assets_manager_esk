use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::features::assets::infrastructure::AssetHttpRepository;
use crate::features::auth::AuthContext;
use crate::features::trading::application::{BuyAssetUseCase, LoadActiveAssetsUseCase};
use crate::features::trading::domain::BuyTradeForm;
use crate::features::trading::infrastructure::TradeHttpRepository;

#[component]
pub fn BuyTradePage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = use_navigate();
    let error = RwSignal::new(None::<String>);
    let loading_assets = RwSignal::new(true);
    let assets = RwSignal::new(Vec::<(String, String)>::new());
    let asset_id = RwSignal::new(String::new());
    let quantity = RwSignal::new(String::new());
    let unit_price = RwSignal::new(String::new());

    Effect::new(move |_| {
        let token = auth.access_token();
        if token.is_none() {
            return;
        }
        let token = token.unwrap();
        loading_assets.set(true);

        leptos::task::spawn_local(async move {
            let use_case = LoadActiveAssetsUseCase::new(Arc::new(AssetHttpRepository::new()));
            match use_case.execute(&token).await {
                shared_kernel::Result::Ok(items) => {
                    assets.set(
                        items
                            .into_iter()
                            .map(|a| (a.id().to_string(), format!("{} — {}", a.ticker(), a.name())))
                            .collect(),
                    );
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
            loading_assets.set(false);
        });
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(None);

        let token = auth.access_token().unwrap_or_default();
        if token.is_empty() {
            return;
        }

        let form = BuyTradeForm::try_new(asset_id.get(), quantity.get(), unit_price.get());

        let form = match form {
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
            let use_case = BuyAssetUseCase::new(Arc::new(TradeHttpRepository::new()));
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
            <h1 class="text-2xl font-semibold">"Comprar ativo"</h1>
            <Show when=move || loading_assets.get()>
                <p class="text-sm text-muted-foreground">"Carregando ativos..."</p>
            </Show>
            <form class="space-y-4" on:submit=on_submit>
                <div>
                    <label class="mb-1 block text-sm">"Ativo"</label>
                    <select
                        class="w-full rounded-md border border-border bg-background px-3 py-2"
                        on:change=move |ev| asset_id.set(event_target_value(&ev))
                        required
                    >
                        <option value="">"Selecione..."</option>
                        <For
                            each=move || assets.get()
                            key=|(id, _)| id.clone()
                            children=move |(id, label)| view! {
                                <option value=id.clone()>{label}</option>
                            }
                        />
                    </select>
                </div>
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
                    "Confirmar compra"
                </button>
            </form>
        </div>
    }
}
