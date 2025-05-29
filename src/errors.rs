use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Other error: {0}")]
    Other(String),
}
