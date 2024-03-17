use crate::config::MongoConfig;
use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

pub async fn init(config: &MongoConfig) -> Result<Client> {
	let mut options = ClientOptions::parse(&config.url).await?;
	options.app_name = Some("StarFish".into());
	let client = Client::with_options(options)?;
	Ok(client)
}
