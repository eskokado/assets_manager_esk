use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::features::assets::application::{LoadAssetUseCase, SaveAssetUseCase};
use crate::features::assets::domain::AssetForm;
use crate::features::assets::infrastructure::AssetHttpRepository;
use crate::features::auth::AuthContext;

#[component]
pub fn AssetFormPage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let navigate = leptos_router::hooks::use_navigate();
    let params = use_params_map();
    let asset_id = Memo::new(move |_| params.get().get("id").map(|s| s.to_string()));

    let ticker = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let asset_type = RwSignal::new("STOCK".to_string());
    let currency = RwSignal::new("BRL".to_string());
    let active = RwSignal::new(true);
    let ticker_error = RwSignal::new(None::<String>);
    let form_error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);
    let is_edit = Memo::new(move |_| asset_id.get().is_some());

    Effect::new(move |_| {
        if let Some(id) = asset_id.get() {
            let token = auth.access_token();
            if token.is_none() {
                return;
            }
            let token = token.unwrap();
            loading.set(true);

            leptos::task::spawn_local(async move {
                let use_case = LoadAssetUseCase::new(Arc::new(AssetHttpRepository::new()));
                match use_case.execute(&token, &id).await {
                    shared_kernel::Result::Ok(form) => {
                        ticker.set(form.ticker().to_string());
                        name.set(form.name().to_string());
                        asset_type.set(form.asset_type().to_string());
                        currency.set(form.currency().to_string());
                        active.set(form.active());
                    }
                    shared_kernel::Result::Err(errors) => {
                        form_error.set(Some(
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
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        ticker_error.set(None);
        form_error.set(None);

        let token = auth.access_token().unwrap_or_default();
        let form = AssetForm::try_new(
            asset_id.get(),
            ticker.get(),
            name.get(),
            asset_type.get(),
            currency.get(),
            active.get(),
        );

        let form = match form {
            shared_kernel::Result::Ok(value) => value,
            shared_kernel::Result::Err(errors) => {
                form_error.set(Some(
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

        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            let use_case = SaveAssetUseCase::new(Arc::new(AssetHttpRepository::new()));
            match use_case.execute(&token, &form).await {
                shared_kernel::Result::Ok(_) => navigate("/admin/assets", Default::default()),
                shared_kernel::Result::Err(errors) => {
                    let message = errors
                        .iter()
                        .map(|e| e.0.as_str())
                        .collect::<Vec<_>>()
                        .join("; ");
                    if message.to_lowercase().contains("ticker") {
                        ticker_error.set(Some(message.clone()));
                    } else {
                        form_error.set(Some(message));
                    }
                }
            }
            loading.set(false);
        });
    };

    view! {
        <div class="mx-auto max-w-xl space-y-4">
            <h1 class="text-2xl font-semibold">
                {move || if is_edit.get() { "Editar ativo" } else { "Novo ativo" }}
            </h1>
            <form class="space-y-4 rounded-lg border border-border p-6" on:submit=on_submit>
                <div>
                    <label class="mb-1 block text-sm" for="asset-ticker">"Ticker"</label>
                    <input
                        id="asset-ticker"
                        class="w-full rounded-md border border-border px-3 py-2 disabled:opacity-60"
                        prop:value=move || ticker.get()
                        on:input=move |ev| ticker.set(event_target_value(&ev))
                        disabled=move || is_edit.get()
                        required
                    />
                    <Show when=move || ticker_error.get().is_some()>
                        <p class="mt-1 text-sm text-red-600">{move || ticker_error.get().unwrap_or_default()}</p>
                    </Show>
                </div>
                <div>
                    <label class="mb-1 block text-sm" for="asset-name">"Nome"</label>
                    <input
                        id="asset-name"
                        class="w-full rounded-md border border-border px-3 py-2"
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div>
                    <label class="mb-1 block text-sm" for="asset-type">"Tipo"</label>
                    <select
                        id="asset-type"
                        class="w-full rounded-md border border-border bg-background px-3 py-2 text-foreground"
                        prop:value=move || asset_type.get()
                        on:change=move |ev| asset_type.set(event_target_value(&ev))
                    >
                        <option value="STOCK">"STOCK"</option>
                        <option value="FII">"FII"</option>
                        <option value="ETF">"ETF"</option>
                        <option value="BOND">"BOND"</option>
                    </select>
                </div>
                <div>
                    <label class="mb-1 block text-sm" for="asset-currency">"Moeda"</label>
                    <input
                        id="asset-currency"
                        class="w-full rounded-md border border-border px-3 py-2"
                        prop:value=move || currency.get()
                        on:input=move |ev| currency.set(event_target_value(&ev))
                        required
                    />
                </div>
                <Show when=move || is_edit.get()>
                    <label class="flex items-center gap-2 text-sm">
                        <input
                            type="checkbox"
                            prop:checked=move || active.get()
                            on:change=move |ev| active.set(event_target_checked(&ev))
                        />
                        "Ativo"
                    </label>
                </Show>
                <Show when=move || form_error.get().is_some()>
                    <p class="text-sm text-red-600">{move || form_error.get().unwrap_or_default()}</p>
                </Show>
                <button
                    class="rounded-md bg-primary px-4 py-2 text-primary-foreground disabled:opacity-50"
                    type="submit"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "Salvando..." } else { "Salvar" }}
                </button>
            </form>
        </div>
    }
}
