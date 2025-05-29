use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Refill {
    pub id: i32,
    pub prescription_id: i32,
    pub filled_at: NaiveDateTime,
    pub quantity_dispensed: i32,
    pub consult_accepted: bool,
}
