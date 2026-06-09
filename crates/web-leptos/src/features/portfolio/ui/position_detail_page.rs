use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::features::auth::AuthContext;
use crate::features::portfolio::application::LoadPositionUseCase;
use crate::features::portfolio::domain::PositionDetail;
use crate::features::portfolio::infrastructure::PortfolioHttpRepository;

#[component]
pub fn PositionDetailPage() -> impl IntoView {
    let auth = AuthContext::use_ctx();
    let params = use_params_map();
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(true);
    let detail = RwSignal::new(None::<PositionDetail>);

    Effect::new(move |_| {
        let token = auth.access_token();
        let asset_id = params
            .get()
            .get("id")
            .map(|value| value.to_string())
            .unwrap_or_default();
        if token.is_none() || asset_id.is_empty() {
            return;
        }
        let token = token.unwrap();
        loading.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            let use_case = LoadPositionUseCase::new(Arc::new(PortfolioHttpRepository::new()));
            match use_case.execute(&token, &asset_id).await {
                shared_kernel::Result::Ok(value) => detail.set(Some(value)),
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
        <div class="mx-auto max-w-lg space-y-4">
            <A href="/portfolio">
                <span class="text-sm text-primary underline">"← Voltar para carteira"</span>
            </A>
            <Show when=move || loading.get()>
                <p class="text-sm text-muted-foreground">"Carregando posição..."</p>
            </Show>
            <Show when=move || error.get().is_some()>
                <div class="space-y-2">
                    <p class="text-sm text-red-600">{move || error.get().unwrap_or_default()}</p>
                    <A href="/portfolio">
                        <span class="text-sm text-primary underline">"Voltar para carteira"</span>
                    </A>
                </div>
            </Show>
            <Show when=move || detail.get().is_some()>
                {move || {
                    let position = detail.get().unwrap();
                    let sell_href = format!("/trades/sell?assetId={}", position.asset_id());
                    view! {
                        <h1 class="text-2xl font-semibold">{position.ticker().to_string()}</h1>
                        <p class="text-muted-foreground">{position.name().to_string()}</p>
                        <dl class="space-y-2 text-sm">
                            <div class="flex justify-between">
                                <dt>"Quantidade"</dt>
                                <dd>{position.quantity().to_string()}</dd>
                            </div>
                            <div class="flex justify-between">
                                <dt>"Preço médio"</dt>
                                <dd>{position.average_price().to_string()}</dd>
                            </div>
                            <div class="flex justify-between font-medium">
                                <dt>"Total investido"</dt>
                                <dd>{position.total_invested().to_string()}</dd>
                            </div>
                        </dl>
                        <A href=sell_href>
                            <span class="inline-block rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground">
                                "Vender"
                            </span>
                        </A>
                    }
                }}
            </Show>
        </div>
    }
}
