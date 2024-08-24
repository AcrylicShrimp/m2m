mod connection;
mod error;

pub use connection::*;
pub use error::*;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DbService {
    pool: PgPool,
}

impl DbService {
    pub async fn new(
        connection_info: DbConnectionInfo,
        pool_config: DbConnectionPoolConfig,
    ) -> Result<Self, DbServiceError> {
        let url = connection_info.into_url();
        let pool = PgPoolOptions::new()
            .max_connections(pool_config.max_connections)
            .connect(&url)
            .await?;

        Ok(Self { pool })
    }
}
