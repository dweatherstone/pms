use std::fmt;

use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub license_number: String,
}

impl fmt::Display for Doctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.license_number)
    }
}
