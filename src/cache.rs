use redis::{AsyncTypedCommands, Client, RedisResult};

#[derive(Debug)]
pub struct Cache {
    remote: Client,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let redis_url = std::env::var("REDIS_URL")
            .map_err(|err| anyhow::anyhow!("Error when acquiring REDIS_URL: {}", err))?;

        let client = Client::open(redis_url.as_str())
            .map_err(|err| anyhow::anyhow!("Error when connecting to Redis: {}", err))?;

        Ok(Self { remote: client })
    }

    pub fn get_remote(&self) -> &Client {
        &self.remote
    }

    pub async fn ping_remote(&self) -> RedisResult<()> {
        let mut conn = self.remote.get_multiplexed_async_connection().await?;

        conn.ping().await?;

        Ok(())
    }
}
