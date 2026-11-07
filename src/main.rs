use axum::Router;
use std::net::SocketAddr;

mod auth;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "RabitHole API OK" }))
        .route("/topics/:slug", axum::routing::get(routes::topics::get_topic))
        .route("/search", axum::routing::get(routes::search::search_topics))
        .nest("/auth", auth::setup_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 RabitHole API on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
