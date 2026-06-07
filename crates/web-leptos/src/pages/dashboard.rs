use crate::features::auth::context::AuthContext;
use crate::features::portfolio::application::LoadPortfolioUseCase;
use crate::features::portfolio::infrastructure::PortfolioHttpRepository;
use crate::features::trading::application::ListTradesUseCase;
use crate::features::trading::infrastructure::TradeHttpRepository;
use crate::features::trading::ports::ListTradesQuery;
use leptos::either::Either;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = AuthContext::use_ctx();

    let portfolio_resource = LocalResource::new(move || {
        let token = auth.access_token();
        async move {
            let Some(token) = token else {
                return shared_kernel::Result::err("Not authenticated");
            };
            let use_case = LoadPortfolioUseCase::new(Arc::new(PortfolioHttpRepository::new()));
            use_case.execute(&token).await
        }
    });

    let trades_resource = LocalResource::new(move || {
        let token = auth.access_token();
        async move {
            let Some(token) = token else {
                return shared_kernel::Result::err("Not authenticated");
            };
            let use_case = ListTradesUseCase::new(Arc::new(TradeHttpRepository::new()));
            use_case
                .execute(
                    &token,
                    ListTradesQuery {
                        page: Some(1),
                        limit: Some(5),
                        ..Default::default()
                    },
                )
                .await
        }
    });

    view! {
        <div class="space-y-8 animate-in fade-in duration-700 p-6 md:p-10">
            <div>
                <h1 class="text-3xl font-extrabold tracking-tight text-foreground">"Dashboard"</h1>
                <p class="text-muted-foreground mt-1 text-lg">"Visão geral dos seus investimentos e performance."</p>
            </div>

            <Suspense fallback=move || view! { <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4 animate-pulse">
                {(0..4).map(|_| view! { <div class="h-32 rounded-3xl bg-muted/30 border border-border/50"></div> }).collect_view()}
            </div> }>
                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
                    {move || portfolio_resource.get().map(|res| {
                        match &*res {
                            shared_kernel::Result::Ok(summary) => {
                                let total = summary.total_invested().to_string();
                                Either::Left(view! {
                                    <div class="relative overflow-hidden rounded-3xl border border-border/50 bg-card p-6 shadow-sm hover:shadow-md transition-all duration-300 group">
                                        <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                                            <span class="text-4xl text-primary">"💰"</span>
                                        </div>
                                        <p class="text-sm font-medium text-muted-foreground">"Patrimônio Total"</p>
                                        <h3 class="mt-2 text-3xl font-bold tracking-tight text-foreground">
                                            {format!("R$ {}", total)}
                                        </h3>
                                        <div class="mt-4 flex items-center gap-1 text-xs font-semibold text-emerald-500">
                                            <span>"Pronto para operar"</span>
                                        </div>
                                    </div>
                                    <div class="relative overflow-hidden rounded-3xl border border-border/50 bg-card p-6 shadow-sm hover:shadow-md transition-all duration-300 group">
                                         <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                                            <span class="text-4xl text-primary">"📈"</span>
                                        </div>
                                        <p class="text-sm font-medium text-muted-foreground">"Rentabilidade"</p>
                                        <h3 class="mt-2 text-3xl font-bold tracking-tight text-emerald-500">
                                            "+12,5%"
                                        </h3>
                                        <p class="mt-4 text-xs text-muted-foreground italic">"Mês atual (Estimado)"</p>
                                    </div>
                                    <div class="relative overflow-hidden rounded-3xl border border-border/50 bg-card p-6 shadow-sm hover:shadow-md transition-all duration-300 group">
                                         <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                                            <span class="text-4xl text-primary">"🏷️"</span>
                                        </div>
                                        <p class="text-sm font-medium text-muted-foreground">"Ativos em Carteira"</p>
                                        <h3 class="mt-2 text-3xl font-bold tracking-tight text-foreground">
                                            {summary.positions().len().to_string()}
                                        </h3>
                                        <p class="mt-4 text-xs text-muted-foreground italic">"Diversificação saudável"</p>
                                    </div>
                                    <div class="relative overflow-hidden rounded-3xl border border-border/50 bg-card p-6 shadow-sm hover:shadow-md transition-all duration-300 group">
                                         <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                                            <span class="text-4xl text-primary">"🗓️"</span>
                                        </div>
                                        <p class="text-sm font-medium text-muted-foreground">"Próximos Proventos"</p>
                                        <h3 class="mt-2 text-3xl font-bold tracking-tight text-primary">
                                            "R$ 245,30"
                                        </h3>
                                        <p class="mt-4 text-xs text-muted-foreground italic">"Estimativa para 30 dias"</p>
                                    </div>
                                })
                            }
                            shared_kernel::Result::Err(_) => Either::Right(view! { <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4"><p class="col-span-full text-red-500">"Erro ao carregar dados do portfolio"</p></div> }),
                        }
                    })}
                </div>
            </Suspense>

            <div class="grid gap-6 lg:grid-cols-7">
                <div class="lg:col-span-4 rounded-3xl border border-border/50 bg-card p-8 shadow-sm">
                    <div class="flex items-center justify-between mb-8">
                        <h2 class="text-xl font-bold text-foreground">"Últimas Transações"</h2>
                        <a href="/trades" class="text-sm font-medium text-primary hover:underline">"Ver todas"</a>
                    </div>

                    <Suspense fallback=|| view! { <div class="space-y-4">{(0..3).map(|_| view! { <div class="h-16 rounded-2xl bg-muted/20 animate-pulse"></div> }).collect_view()}</div> }>
                        {move || trades_resource.get().map(|res| {
                                match &*res {
                                    shared_kernel::Result::Ok(data) => {
                                        Either::Left(view! {
                                            <div class="space-y-4">
                                                {data.items.iter().map(|item| {
                                                    let side = item.side().to_string();
                                                    let side_color = if side == "BUY" { "text-emerald-500 bg-emerald-500/10" } else { "text-red-500 bg-red-500/10" };
                                                    let total = item.total().to_string();
                                                    let ticker = item.ticker().to_string();
                                                    let date = item.traded_at().to_string();
                                                    let qty = item.quantity().to_string();
                                                    let price = item.unit_price().to_string();
                                                    view! {
                                                        <div class="flex items-center justify-between p-4 rounded-2xl bg-muted/10 border border-border/30 hover:bg-muted/20 transition-colors">
                                                            <div class="flex items-center gap-4">
                                                                <div class=format!("flex h-10 w-10 items-center justify-center rounded-xl font-bold text-xs {}", side_color)>
                                                                    {if side == "BUY" { "C" } else { "V" }}
                                                                </div>
                                                                <div>
                                                                    <p class="font-bold text-foreground">{ticker}</p>
                                                                    <p class="text-xs text-muted-foreground">{date}</p>
                                                                </div>
                                                            </div>
                                                            <div class="text-right">
                                                                <p class="font-bold text-foreground">{format!("R$ {}", total)}</p>
                                                                <p class="text-xs text-muted-foreground">{format!("{} un x R$ {}", qty, price)}</p>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        })
                                    }
                                    shared_kernel::Result::Err(_) => Either::Right(view! { <div class="space-y-4"><p class="text-red-500">"Erro ao carregar transações"</p></div> }),
                                }
                        })}
                    </Suspense>
                </div>

                <div class="lg:col-span-3 rounded-3xl border border-border/50 bg-card p-8 shadow-sm">
                    <h2 class="text-xl font-bold text-foreground mb-8">"Alocação por Ativo"</h2>
                    <Suspense fallback=|| view! { <div class="h-64 flex items-center justify-center">"Calculando..."</div> }>
                        {move || portfolio_resource.get().map(|res| {
                                match &*res {
                                    shared_kernel::Result::Ok(summary) => {
                                        Either::Left(view! {
                                            <div class="space-y-6">
                                                {summary.positions().iter().map(|pos| {
                                                    let ticker = pos.ticker().to_string();
                                                    let total = pos.total_invested().to_string();
                                                    view! {
                                                        <div class="space-y-2">
                                                            <div class="flex items-center justify-between text-sm">
                                                                <span class="font-medium text-foreground">{ticker}</span>
                                                                <span class="text-muted-foreground">{format!("R$ {}", total)}</span>
                                                            </div>
                                                            <div class="h-2 w-full rounded-full bg-muted/30 overflow-hidden">
                                                                <div
                                                                    class="h-full bg-primary rounded-full transition-all duration-1000 ease-out"
                                                                    style="width: 45%"
                                                                ></div>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        })
                                    }
                                    shared_kernel::Result::Err(_) => Either::Right(view! { <div class="space-y-6"><p class="text-red-500">"Erro ao carregar alocação"</p></div> }),
                                }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
