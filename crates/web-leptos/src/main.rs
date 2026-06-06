use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use web_leptos::app::{shell, App};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let options_for_routes = leptos_options.clone();

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            || {},
            move || shell(options_for_routes.clone()),
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Leptos SSR listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}
