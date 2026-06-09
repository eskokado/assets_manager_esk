use leptos::prelude::*;

use crate::features::auth::ui::LoginForm;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="flex min-h-screen items-center justify-center bg-background p-4">
            <LoginForm/>
        </div>
    }
}
