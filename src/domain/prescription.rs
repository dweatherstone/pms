use std::fmt;

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
    pub valid_until: Option<NaiveDate>,
}

impl fmt::Display for Prescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dosage = self
            .dosage
            .clone()
            .unwrap_or(String::from("Not specified "));
        write!(
            f,
            "{} (Doctor #{}, Patient #{}, {}x {}, Refills left: {})",
            self.id,
            self.doctor_id,
            self.patient_id,
            dosage,
            self.medication_name,
            self.refills_remaining
        )
    }
}

#[derive(Debug, FromRow)]
pub struct PrescriptionDisplay {
    pub prescription_id: i32,
    pub medicine: String,
    pub patient_name: String,
    pub patient_dob: NaiveDate,
    pub doctor_name: String,
    pub doctor_license_no: String,
    pub dosage: Option<String>,
    pub refills_remaining: i32,
    pub issued_at: NaiveDateTime,
    pub valid_until: Option<NaiveDate>,
}

impl fmt::Display for PrescriptionDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dosage_str = self.dosage.as_deref().map_or("", |d| d);
        let valid_until_str = self
            .valid_until
            .map(|v| v.format("%d/%m/%Y").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        write!(
            f,
            "[{}] {} {} for {} ({}) by {} ({}) on {}. {} refills remaining until {}",
            self.prescription_id,
            self.medicine,
            dosage_str,
            self.patient_name,
            self.patient_dob.format("%d/%m/%Y"),
            self.doctor_name,
            self.doctor_license_no,
            self.issued_at.format("%d/%m/%Y"),
            self.refills_remaining,
            valid_until_str,
        )
    }
}
