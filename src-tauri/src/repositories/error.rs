use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database Error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity Not Found")]
    NotFound
}