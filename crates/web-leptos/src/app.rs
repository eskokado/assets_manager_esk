use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::layouts::AdminShell;
use crate::pages::{dashboard::DashboardPage, examples::ExamplesPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="pt-BR" attr:class="dark" />
        <Stylesheet id="leptos" href="/pkg/web-leptos.css"/>
        <Title text="Assets Manage"/>
        <Router>
            <AdminShell>
                <Routes fallback=|| view! { <p class="p-8">"Página não encontrada"</p> }>
                    <Route path=path!("/") view=DashboardPage/>
                    <Route path=path!("/examples") view=ExamplesPage/>
                </Routes>
            </AdminShell>
        </Router>
    }
}
