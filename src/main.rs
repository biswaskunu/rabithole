use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;

struct DbPool(PgPool);

impl DbPool {
    async fn init_pool(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        Ok(Self(pool))
    }

    async fn health_check(&self) -> bool {
        let rows = sqlx::query("SELECT 1")
            .fetch_all(&self.0)
            .await;
        rows.is_ok()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://biswash@localhost/rabithole";
    let pool = DbPool::init_pool(database_url).await?;

    if !pool.health_check().await {
        eprintln!("❌ Database health check failed");
        std::process::exit(1);
    }
    println!("✅ Database connection established");

    let app = Router::new().route("/health", axum::routing::get(|| async { "RabitHole API OK" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
