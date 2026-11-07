use sqlx::PgPool;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    // Use database URL with search_path=public to ensure tables are found
    let pool = sqlx::PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn health_check(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1 AS one")
        .fetch_one(pool)
        .await
        .is_ok()
}