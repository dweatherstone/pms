pub mod doctor;
pub mod patient;

use std::{env, fs};

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::cli::{AddDoctorArgs, AddPatientArgs};
use crate::domain::doctor::Doctor;
use crate::domain::patient::Patient;
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

pub async fn run_sql_scripts(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let schema =
        fs::read_to_string("sql/001_create_tables.sql").expect("Failed to read schema SQL");
    let seed =
        fs::read_to_string("sql/002_insert_dummy_data.sql").expect("Failed to read seed SQL");

    // Split and run each statement individually
    split_and_run_script(&schema, pool).await?;
    split_and_run_script(&seed, pool).await?;

    println!("Database schema and dummy data successfully loaded.");
    Ok(())
}

pub async fn list_doctors(
    pool: &MySqlPool,
    name: &Option<String>,
    license_number: &Option<String>,
) -> Result<Vec<Doctor>, sqlx::Error> {
    doctor::list_doctors(pool, name, license_number).await
}

pub async fn add_doctor(args: &AddDoctorArgs, pool: &MySqlPool) -> Result<(), AppError> {
    doctor::add_doctor(args, pool).await
}

pub async fn list_patients(
    pool: &MySqlPool,
    name: &Option<String>,
    patient_number: &Option<String>,
) -> Result<Vec<Patient>, sqlx::Error> {
    patient::list_patients(pool, name, patient_number).await
}

pub async fn add_patient(args: &AddPatientArgs, pool: &MySqlPool) -> Result<(), AppError> {
    patient::add_patient(args, pool).await
}

async fn split_and_run_script(script: &str, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    for statement in script.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sqlx::query(trimmed).execute(pool).await?;
        }
    }
    Ok(())
}
