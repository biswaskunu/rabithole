use axum::{routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to Postgres");

    // Simple health check route
    let app = Router::new()
        .route("/health", get(|| async { "RabitHole API OK" }))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 RabitHole API listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.make_service_fn(|&_| async { Ok::<_, axum::Error>(())) })
        .await?;
    Ok(())
}
