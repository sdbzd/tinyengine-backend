use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use crate::config::AppConfig;
use crate::error::Result;

pub async fn create_pool(config: &AppConfig) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database_url())
        .await?;

    Ok(pool)
}
