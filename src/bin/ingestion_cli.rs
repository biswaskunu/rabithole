
#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://biswash@localhost/rabithole")
        .await
        .expect("Failed to connect to Postgres");

    // Simple health check
    let rows = sqlx::query!("SELECT 1 as test")
        .fetch_all(&pool)
        .await
        .expect("Query failed");

    println!("DB connection OK, rows: {:?}", rows);
    println!("Phase 1 in progress - ingestion service ready");
}
