use crate::config::DatabaseConfig;
use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init(config: &DatabaseConfig) -> Result<DatabaseConnection> {
	let mut opt = ConnectOptions::new(&config.url);
	opt.max_connections(config.max_conn);
	opt.sqlx_logging(config.show_sql);
	let db = Database::connect(opt).await?;
	Ok(db)
}
