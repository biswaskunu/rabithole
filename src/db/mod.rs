use sqlx::PgPool;

pub struct DbPool(pub PgPool);

impl DbPool {
    pub async fn init_pool(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        Ok(Self(pool))
    }

    pub async fn health_check(&self) -> bool {
        sqlx::query!("SELECT 1")
            .fetch_one(&self.0)
            .await
            .is_ok()
    }
}
