use rocket::figment::Figment;
use services::db::{DbService, DbServiceError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceCreationError {
    #[error("db service error: {0}")]
    DbServiceError(#[from] DbServiceError),
    #[error("figment error: {0}")]
    FigmentError(#[from] figment::Error),
}

pub async fn create_db_service(config: &Figment) -> Result<DbService, ServiceCreationError> {
    let connection_info = config.extract()?;
    let pool_config = config.extract()?;
    Ok(DbService::new(connection_info, pool_config).await?)
}
