use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::{cli::ListPrescriptionArgs, domain::prescription::PrescriptionDisplay};

pub async fn list_prescriptions(
    args: &ListPrescriptionArgs,
    pool: &MySqlPool,
) -> Result<Vec<PrescriptionDisplay>, sqlx::Error> {
    let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
        r#"
        SELECT p.id AS 'prescription_id', p.medication_name AS 'medicine',
        pa.name as 'patient_name', pa.date_of_birth AS 'patient_dob',
        d.name as 'doctor_name', d.license_number AS 'doctor_license_no',
        p.dosage, p.refills_remaining, p.issued_at, p.valid_until
        FROM prescriptions p
        INNER JOIN patients pa ON p.patient_id = pa.id 
        INNER JOIN doctors d ON p.doctor_id = d.id"#,
    );

    let mut conditions = vec![];

    if let Some(name) = &args.patient_name {
        if !name.is_empty() {
            conditions.push(("pa.name LIKE ", format!("%{}%", name)));
        }
    }
    if let Some(number) = &args.patient_number {
        if !number.is_empty() {
            conditions.push(("pa.patient_number = ", number.to_string()));
        }
    }
    if let Some(doc_name) = &args.doctor_name {
        if !doc_name.is_empty() {
            conditions.push(("d.name LIKE ", format!("%{}%", doc_name)));
        }
    }

    if !args.include_expired.unwrap_or(true) {
        conditions.push((
            "(p.valid_until IS NULL OR p.valid_until >= CURDATE())",
            String::new(),
        ))
    }
    if let Some(doctor_id) = args.doctor_id {
        conditions.push(("d.id = ", doctor_id.to_string()));
    }
    if !conditions.is_empty() {
        qb.push(" WHERE ");
        let mut first = true;
        for (cond, val) in conditions {
            if !first {
                qb.push(" AND ");
            }
            qb.push(cond);
            if !val.is_empty() {
                qb.push_bind(val);
            }
            first = false;
        }
    }

    qb.push(" ORDER BY d.name, pa.name, p.valid_until");

    let query = qb.build_query_as::<PrescriptionDisplay>();
    let prescriptions = query.fetch_all(pool).await?;
    Ok(prescriptions)
}
