use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="text-2xl font-bold">"Dashboard"</h1>
            <p class="mt-2 text-muted-foreground">"Carteira de investimentos — bootstrap EP-000 concluído."</p>
        </div>
    }
}
