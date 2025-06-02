use std::fmt;

use chrono::NaiveDate;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub patient_number: Option<String>,
}

impl fmt::Display for Patient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from(&self.name);
        result.push_str(" (");
        result.push_str(self.date_of_birth.format("%d/%m/%Y").to_string().as_str());
        result.push(')');
        if let Some(pn) = &self.patient_number {
            result.push_str(": ");
            result.push_str(pn);
        }

        write!(f, "{}", result)
    }
}
