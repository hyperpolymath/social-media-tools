use anyhow::Result;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct CacheClient {
    conn: ConnectionManager,
}

impl CacheClient {
    pub async fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        Ok(self.conn.get(key).await?)
    }

    pub async fn set(&mut self, key: &str, value: &str, ttl: usize) -> Result<()> {
        self.conn.set_ex(key, value, ttl).await?;
        Ok(())
    }
}
