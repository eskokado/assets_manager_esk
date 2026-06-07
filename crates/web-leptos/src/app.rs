use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[cfg(feature = "ssr")]
use leptos::hydration::{AutoReload, HydrationScripts};
#[cfg(feature = "ssr")]
use leptos_config::LeptosOptions;

use crate::features::assets::{AssetFormPage, AssetListPage, RequireAdmin as RequireAdminAssets};
use crate::features::auth::{AuthProvider, RequireAuth};
use crate::features::trading::{BuyTradePage, SellTradePage, TradeListPage};
use crate::layouts::AdminShell;
use crate::pages::{
    dashboard::DashboardPage, examples::ExamplesPage, login::LoginPage, profile::ProfilePage,
    register::RegisterPage,
};

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="pt-BR" class="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
fn PrivateShell(children: Children) -> impl IntoView {
    view! {
        <RequireAuth>
            <AdminShell>{children()}</AdminShell>
        </RequireAuth>
    }
}

#[component]
fn AdminShellGuard(children: Children) -> impl IntoView {
    view! {
        <RequireAuth>
            <RequireAdminAssets>
                <AdminShell>{children()}</AdminShell>
            </RequireAdminAssets>
        </RequireAuth>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/web-leptos.css"/>
        <Title text="Assets Manage"/>
        <AuthProvider>
            <Router>
                <Routes fallback=|| view! { <p class="p-8">"Página não encontrada"</p> }>
                    <Route path=path!("/login") view=LoginPage/>
                    <Route path=path!("/register") view=RegisterPage/>
                    <Route path=path!("/") view=move || view! {
                        <PrivateShell>
                            <DashboardPage/>
                        </PrivateShell>
                    }/>
                    <Route path=path!("/examples") view=move || view! {
                        <PrivateShell>
                            <ExamplesPage/>
                        </PrivateShell>
                    }/>
                    <Route path=path!("/profile") view=move || view! {
                        <PrivateShell>
                            <ProfilePage/>
                        </PrivateShell>
                    }/>
                    <Route path=path!("/admin/assets") view=move || view! {
                        <AdminShellGuard>
                            <AssetListPage/>
                        </AdminShellGuard>
                    }/>
                    <Route path=path!("/admin/assets/new") view=move || view! {
                        <AdminShellGuard>
                            <AssetFormPage/>
                        </AdminShellGuard>
                    }/>
                    <Route path=path!("/admin/assets/:id/edit") view=move || view! {
                        <AdminShellGuard>
                            <AssetFormPage/>
                        </AdminShellGuard>
                    }/>
                    <Route path=path!("/trades") view=move || view! {
                        <PrivateShell>
                            <TradeListPage/>
                        </PrivateShell>
                    }/>
                    <Route path=path!("/trades/buy") view=move || view! {
                        <PrivateShell>
                            <BuyTradePage/>
                        </PrivateShell>
                    }/>
                    <Route path=path!("/trades/sell") view=move || view! {
                        <PrivateShell>
                            <SellTradePage/>
                        </PrivateShell>
                    }/>
                </Routes>
            </Router>
        </AuthProvider>
    }
}
