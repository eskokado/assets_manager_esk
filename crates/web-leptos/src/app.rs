use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::features::auth::{AuthProvider, RequireAuth};
use crate::layouts::AdminShell;
use crate::pages::{
    dashboard::DashboardPage, examples::ExamplesPage, login::LoginPage, profile::ProfilePage,
    register::RegisterPage,
};

#[component]
fn PrivateShell(children: Children) -> impl IntoView {
    view! {
        <RequireAuth>
            <AdminShell>{children()}</AdminShell>
        </RequireAuth>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="pt-BR" attr:class="dark" />
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
                </Routes>
            </Router>
        </AuthProvider>
    }
}
