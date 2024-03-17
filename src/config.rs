use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogConfig {
	#[serde(default = "default_log_level")]
	pub level: String,
	#[serde(default = "default_log_to_console")]
	pub console: bool,
	#[serde(default)]
	pub file: bool,
}

fn default_log_level() -> String {
	"info".into()
}

fn default_log_to_console() -> bool {
	true
}

#[cfg(feature = "web")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebConfig {
	#[serde(default = "default_web_bind")]
	pub bind: String,
}
#[cfg(feature = "web")]
fn default_web_bind() -> String {
	"0.0.0.0:10000".into()
}

#[cfg(feature = "database")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseConfig {
	#[serde(default)]
	pub url: String,
	#[serde(default = "default_database_max_conn")]
	pub max_conn: u32,
	#[serde(default)]
	pub show_sql: bool,
}

#[cfg(feature = "database")]
fn default_database_max_conn() -> u32 {
	num_cpus::get() as u32
}

#[cfg(feature = "redis")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RedisConfig {
	/// redis://[<username>][:<password>@]<hostname>[:port][/<db>]
	#[serde(default)]
	pub url: String,
}

#[cfg(feature = "mongo")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MongoConfig {
	/// mongodb://user:pass@localhost:27017
	#[serde(default)]
	pub url: String,
}

#[cfg(feature = "influx")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfluxConfig {
	#[serde(default)]
	pub url: String,
	#[serde(default)]
	pub org: String,
	#[serde(default)]
	pub token: String,
}

#[cfg(feature = "config")]
pub fn init<'de, T: Deserialize<'de>>(config_name: &str) -> anyhow::Result<T> {
	let settings = config::Config::builder()
		.add_source(config::File::with_name(config_name).required(false))
		.add_source(config::Environment::with_prefix("app").try_parsing(true).separator("__"))
		.build()?;
	let config: T = settings.try_deserialize()?;
	Ok(config)
}
