use dialoguer::Select;
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::{cli::AddDoctorArgs, domain::doctor::Doctor, errors::AppError};

pub async fn list_doctors(
    pool: &MySqlPool,
    name: &Option<String>,
    license_number: &Option<String>,
) -> Result<Vec<Doctor>, sqlx::Error> {
    let mut qb: QueryBuilder<MySql> =
        QueryBuilder::new("SELECT id, name, license_number FROM doctors");
    let mut has_where = false;

    if name.is_some() || license_number.is_some() {
        qb.push(" WHERE ");
    }

    if let Some(name_val) = name {
        qb.push("name LIKE ");
        qb.push_bind(format!("%{}%", name_val));
        has_where = true;
    }
    if let Some(number_val) = license_number {
        if has_where {
            qb.push(" AND ");
        }
        qb.push("license_number = ");
        qb.push_bind(number_val);
    }
    qb.push(" ORDER BY name");

    let query = qb.build_query_as::<Doctor>();
    let doctors = query.fetch_all(pool).await?;
    Ok(doctors)
}

pub async fn add_doctor(args: &AddDoctorArgs, pool: &MySqlPool) -> Result<(), AppError> {
    let name = args.name.trim();
    let license_number = args.license_number.trim();
    // Validation of name and license number passed in. These cannot be empty
    if name.is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".into()));
    }
    if license_number.is_empty() {
        return Err(AppError::ValidationError(
            "License number cannot be empty".into(),
        ));
    }
    // Search for existing doctors
    let existing = find_by_name_or_license_number(name, license_number, pool).await?;

    if existing.is_empty() {
        let doctor = insert_new_doctor(name, license_number, pool).await?;
        println!("✅ Doctor added: {}", doctor);
        return Ok(());
    }

    // Show matches
    println!("⚠ Found existing doctors:");
    for (i, doctor) in existing.iter().enumerate() {
        println!("  [{}] {}", i + 1, doctor);
    }

    // Prompt for action
    let options = vec![
        "Update doctor's license number of existing doctor",
        "Update doctor's name given the License Number",
        "Insert new doctor",
        "Cancel",
    ];
    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .interact()?;

    match selection {
        0 => {
            let existing_strings: Vec<String> =
                existing.iter().map(|doctor| doctor.to_string()).collect();
            let doctor_idx = Select::new()
                .with_prompt("Select doctor to update")
                .items(&existing_strings)
                .interact()?;

            let doctor = &existing[doctor_idx];
            update_license_number_from_name(doctor.id, license_number, pool).await?;
            println!("✅ Updated doctor license number for ID {}", doctor.id);
            Ok(())
        }
        1 => {
            let existing_strings: Vec<String> =
                existing.iter().map(|doctor| doctor.to_string()).collect();
            let doctor_idx = Select::new()
                .with_prompt("Select doctor to update")
                .items(&existing_strings)
                .interact()?;

            let doctor = &existing[doctor_idx];
            update_name_from_license_number(doctor.id, name, pool).await?;
            println!("✅ Update doctor name for ID {}", doctor.id);
            Ok(())
        }
        2 => {
            let doctor = insert_new_doctor(name, license_number, pool).await?;
            println!("✅ Duplicate doctor added: {}", doctor);
            Ok(())
        }
        _ => Err(AppError::Cancelled),
    }
}

async fn find_by_name_or_license_number(
    name: &str,
    license_number: &str,
    pool: &MySqlPool,
) -> Result<Vec<Doctor>, sqlx::Error> {
    sqlx::query_as!(
        Doctor,
        r#"
    SELECT id, name, license_number
    FROM doctors
    WHERE name = ?
    OR license_number = ?
    "#,
        name,
        license_number
    )
    .fetch_all(pool)
    .await
}

async fn insert_new_doctor(
    name: &str,
    licence_number: &str,
    pool: &MySqlPool,
) -> Result<Doctor, sqlx::Error> {
    let result = sqlx::query!(
        r#"
    INSERT INTO doctors(name, license_number)
    VALUE(?, ?)
    "#,
        name,
        licence_number
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_id();

    sqlx::query_as!(
        Doctor,
        r#"
    SELECT id, name, license_number
    FROM doctors
    WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await
}

async fn update_license_number_from_name(
    id: i32,
    new_licence_number: &str,
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE doctors
    SET license_number = ?
    WHERE id = ?"#,
        new_licence_number,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn update_name_from_license_number(
    id: i32,
    new_name: &str,
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE doctors
    SET name = ?
    WHERE id = ?"#,
        new_name,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}
