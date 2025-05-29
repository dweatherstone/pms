use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub license_number: String,
}
