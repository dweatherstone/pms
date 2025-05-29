use sqlx::MySqlPool;

use crate::domain::doctor::Doctor;

pub async fn get_all_doctors(pool: &MySqlPool) -> Result<Vec<Doctor>, sqlx::Error> {
    sqlx::query_as::<_, Doctor>("SELECT * FROM doctors")
        .fetch_all(pool)
        .await
}
