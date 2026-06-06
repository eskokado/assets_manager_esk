use leptos::prelude::*;

use crate::features::auth::ui::RegisterForm;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="flex min-h-screen items-center justify-center bg-background p-4">
            <RegisterForm/>
        </div>
    }
}
