use chrono::Duration;
use redis::AsyncTypedCommands;
use thiserror::Error;

use crate::{config::Config, memory_db::types::MemoryDBType};

#[derive(Debug, Clone)]
pub struct MemoryDB {
    pool: deadpool_redis::Pool,
}

#[derive(Error, Debug)]
pub enum MemoryDBError {
    #[error("Serialize: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Redis: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Redis pool: {0}")]
    Pool(#[from] deadpool_redis::PoolError),
    #[error("Key not found in DB")]
    KeyMissing,
    #[error("{0}")]
    Other(String),
}

impl MemoryDB {
    pub async fn new(config: &Config) -> Self {
        let redis_config = deadpool_redis::Config::from_url(config.redis_url.clone());
        let pool = redis_config
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("Redis pool");
        MemoryDB { pool }
    }

    fn create_key<M: MemoryDBType>(id: &M::Key) -> String {
        format!("{}_{}", M::key_prefix(), id.to_string())
    }

    pub async fn write<M: MemoryDBType>(
        &self,
        id: &M::Key,
        data: M::Value,
        expiration: Duration,
    ) -> Result<(), MemoryDBError> {
        let data = serde_json::to_string(&data)?;
        let mut con = self.pool.get().await?;
        con.set_ex(
            Self::create_key::<M>(id),
            data,
            expiration.num_seconds().unsigned_abs(),
        )
        .await?;
        Ok(())
    }

    pub async fn increment<M: MemoryDBType>(
        &self,
        id: &M::Key,
        expiration: Duration,
    ) -> Result<usize, MemoryDBError> {
        let mut con = self.pool.get().await?;
        let key = Self::create_key::<M>(id);
        let new_count = con.incr(&key, 1).await?;
        if new_count == 1 {
            // The key was just created
            con.expire(&key, expiration.num_seconds()).await?;
        }
        let new_count =
            usize::try_from(new_count).map_err(|e| MemoryDBError::Other(e.to_string()))?;
        Ok(new_count)
    }

    pub async fn read<M: MemoryDBType>(&self, id: &M::Key) -> Result<M::Value, MemoryDBError> {
        let mut con = self.pool.get().await?;
        let val = con
            .get(Self::create_key::<M>(id))
            .await?
            .ok_or(MemoryDBError::KeyMissing)?;
        let data: M::Value = serde_json::from_str(&val)?;
        Ok(data)
    }

    pub async fn delete<M: MemoryDBType>(&self, id: &M::Key) -> Result<(), MemoryDBError> {
        let mut con = self.pool.get().await?;
        con.del(Self::create_key::<M>(id)).await?;
        Ok(())
    }
}
