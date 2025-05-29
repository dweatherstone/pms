pub mod doctor;

use std::env;

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::errors::AppError;

pub async fn get_db_pool() -> Result<MySqlPool, AppError> {
    let database_url =
        env::var("DATABASE_URL").map_err(|_| AppError::MissingEnvVar("DATABASE_URL"))?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
