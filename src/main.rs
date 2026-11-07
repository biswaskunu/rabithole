use axum::Router;
use std::net::SocketAddr;

mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "RabitHole API OK" }))
        .route("/topics/:slug", axum::routing::get(routes::topics::get_topic));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 RabitHole API on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
