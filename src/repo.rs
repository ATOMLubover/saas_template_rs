use sqlx::{PgPool, postgres::PgPoolOptions};

pub mod user;

#[derive(Debug)]
pub struct Repo {
    pool: PgPool,
}

impl Repo {
    const MAX_CONNECTIONS: u32 = 5;

    pub async fn new() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| anyhow::anyhow!("Error when acquiring DATABASE_URL: {}", e))?;

        let pool = PgPoolOptions::new()
            .max_connections(Self::MAX_CONNECTIONS)
            .connect(&database_url)
            .await
            .map_err(|e| anyhow::anyhow!("Error when connecting to database: {}", e))?;

        Ok(Self { pool })
    }

    pub async fn ping(&self) -> anyhow::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;

        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
