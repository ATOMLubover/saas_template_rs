use redis::{AsyncTypedCommands, Client, RedisResult};

#[derive(Debug)]
pub struct Cache {
    remote: Client,
}

impl Cache {
    pub fn new(remote: Client) -> Self {
        Self { remote }
    }

    pub async fn ping_remote(&self) -> RedisResult<()> {
        let mut conn = self.remote.get_multiplexed_async_connection().await?;

        conn.ping().await?;

        Ok(())
    }
}
