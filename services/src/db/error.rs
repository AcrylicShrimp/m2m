use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbServiceError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
