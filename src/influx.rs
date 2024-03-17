use crate::config::InfluxConfig;
use anyhow::Result;
use influxdb2::Client;

pub fn init(config: &InfluxConfig) -> Result<Client> {
	let client = Client::new(&config.url, &config.org, &config.token);
	Ok(client)
}
