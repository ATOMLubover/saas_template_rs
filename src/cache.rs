use redis::{AsyncTypedCommands, Client, RedisResult};

#[derive(Debug)]
pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let redis_url = std::env::var("REDIS_URL")
            .map_err(|e| anyhow::anyhow!("Error when acquiring REDIS_URL: {}", e))?;

        let client = Client::open(redis_url.as_str())
            .map_err(|e| anyhow::anyhow!("Error when connecting to Redis: {}", e))?;

        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn ping(&self) -> RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        conn.ping().await?;

        Ok(())
    }
}
