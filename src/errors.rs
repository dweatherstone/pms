use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("User cancelled operation")]
    Cancelled,

    #[error("Other error: {0}")]
    Other(String),
}
