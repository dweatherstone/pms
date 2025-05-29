use chrono::NaiveDate;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub date_of_birth: NaiveDate,
}
