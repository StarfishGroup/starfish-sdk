use crate::config::RedisConfig;
use anyhow::Result;
use bb8_redis::{bb8::Pool, RedisConnectionManager};

pub async fn init(config: &RedisConfig) -> Result<Pool<RedisConnectionManager>> {
	let manager = RedisConnectionManager::new(config.url.to_owned())?;
	let pool = Pool::builder().max_size(num_cpus::get() as u32).build(manager).await?;
	Ok(pool)
}
