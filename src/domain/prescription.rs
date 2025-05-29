use chrono::{NaiveDate, NaiveDateTime};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Prescription {
    pub id: i32,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub total_refills: i32,
    pub refills_remaining: i32,
    pub max_tablets_per_30_days: i32,
    pub issued_at: NaiveDateTime,
    pub valid_until: NaiveDate,
}
