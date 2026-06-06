use leptos::prelude::*;

#[component]
pub fn PasswordInput(
    value: RwSignal<String>,
    #[prop(default = "Senha".to_string())] label: String,
    #[prop(default = "password".to_string())] id: String,
) -> impl IntoView {
    let show_password = RwSignal::new(false);

    view! {
        <div>
            <label class="mb-1 block text-sm" for=id.clone()>{label.clone()}</label>
            <div class="relative">
                <input
                    id=id
                    class="w-full rounded-md border border-border py-2 pl-3 pr-10"
                    type=move || if show_password.get() { "text" } else { "password" }
                    prop:value=move || value.get()
                    on:input=move |ev| value.set(event_target_value(&ev))
                    required
                />
                <button
                    class="absolute inset-y-0 right-0 flex items-center px-3 text-muted-foreground hover:text-foreground"
                    type="button"
                    aria-label=move || {
                        if show_password.get() {
                            "Ocultar senha".to_string()
                        } else {
                            "Mostrar senha".to_string()
                        }
                    }
                    on:click=move |_| show_password.update(|visible| *visible = !*visible)
                >
                    <Show
                        when=move || show_password.get()
                        fallback=|| view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="1.5"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.964-7.178Z"
                                />
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                                />
                            </svg>
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="1.5"
                            aria-hidden="true"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88"
                            />
                        </svg>
                    </Show>
                </button>
            </div>
        </div>
    }
}
