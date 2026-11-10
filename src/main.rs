use axum::Router;

mod auth;
mod db;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "RabitHole API OK" }))
        .nest("/api", auth::setup_routes());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    println!("RabitHole API on port {port}");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}