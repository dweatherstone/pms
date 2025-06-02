use chrono::NaiveDate;
use dialoguer::Select;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::{cli::AddPatientArgs, domain::patient::Patient, errors::AppError};

pub async fn list_patients(
    pool: &MySqlPool,
    name: &Option<String>,
    patient_number: &Option<String>,
) -> Result<Vec<Patient>, sqlx::Error> {
    let mut query_builder: QueryBuilder<MySql> =
        QueryBuilder::new("SELECT id, name, patient_number, date_of_birth FROM patients");

    let mut has_where = false;

    if name.is_some() || patient_number.is_some() {
        query_builder.push(" WHERE ");
    }

    if let Some(name_val) = name {
        query_builder.push("name LIKE ");
        query_builder.push_bind(format!("%{}%", name_val));
        has_where = true;
    }
    if let Some(number_val) = patient_number {
        if has_where {
            query_builder.push(" AND ");
        }
        query_builder.push("patient_number = ");
        query_builder.push_bind(number_val);
    }
    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<Patient>();
    let patients = query.fetch_all(pool).await?;
    Ok(patients)
}

pub async fn add_patient(args: &AddPatientArgs, pool: &MySqlPool) -> Result<(), AppError> {
    let name = args.name.trim();
    // Validate input
    let dob = NaiveDate::parse_from_str(&args.dob, "%Y-%m-%d").map_err(|_| {
        AppError::ValidationError("Invalid date format (expected YYYY-MM-DD".into())
    })?;

    if name.is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".into()));
    }
    // Search for existing patients
    let existing = find_by_name_and_dob(name, &dob, pool).await?;

    if existing.is_empty() {
        // Insert new
        let patient = insert_new_patient(name, &dob, args.patient_number.as_deref(), pool).await?;
        println!("✅ Patient added: {}", patient);
        return Ok(());
    }

    // Show matches
    println!("⚠ Found existing patients:");
    for (i, patient) in existing.iter().enumerate() {
        println!("  [{}] {}", i + 1, patient);
    }

    // Prompt for action
    let options = vec![
        "Update patient number of existing patient",
        "Insert new patient anyway",
        "Cancel",
    ];
    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .interact()?;

    match selection {
        0 => {
            let existing_strings: Vec<String> =
                existing.iter().map(|patient| patient.to_string()).collect();
            let patient_idx = Select::new()
                .with_prompt("Select patient to update")
                .items(&existing_strings)
                .interact()?;

            let patient = &existing[patient_idx];

            if let Some(pn) = &args.patient_number {
                update_patient_number(patient.id, pn.trim(), pool).await?;
                println!("✅ Updated patient number for ID {}", patient.id);
                Ok(())
            } else {
                Err(AppError::ValidationError(
                    "No patient number provided to update".into(),
                ))
            }
        }
        1 => {
            let patient =
                insert_new_patient(name, &dob, args.patient_number.as_deref(), pool).await?;
            println!("✅ Duplicate patient added: {}", patient);
            Ok(())
        }
        _ => Err(AppError::Cancelled),
    }
}

async fn find_by_name_and_dob(
    name: &str,
    dob: &NaiveDate,
    pool: &MySqlPool,
) -> Result<Vec<Patient>, sqlx::Error> {
    sqlx::query_as!(
        Patient,
        r#"
        SELECT id, name, date_of_birth, patient_number 
        FROM patients 
        WHERE name = ? 
        AND date_of_birth = ?
        "#,
        name,
        dob
    )
    .fetch_all(pool)
    .await
}

async fn insert_new_patient(
    name: &str,
    dob: &NaiveDate,
    patient_number: Option<&str>,
    pool: &MySqlPool,
) -> Result<Patient, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO patients(name, date_of_birth, patient_number)
        VALUES (?, ?, ?)
        "#,
        name,
        dob,
        patient_number
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_id();

    sqlx::query_as!(
        Patient,
        r#"
        SELECT id, name, patient_number, date_of_birth
        FROM patients
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

async fn update_patient_number(
    id: i32,
    new_number: &str,
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE patients
    SET patient_number = ?
    WHERE id = ?"#,
        new_number,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}
