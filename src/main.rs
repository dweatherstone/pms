use dotenvy::dotenv;
use pms::{
    db::{doctor::get_all_doctors, get_db_pool},
    errors::AppError,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let pool = get_db_pool().await?;

    let doctors = get_all_doctors(&pool).await?;
    println!("Doctors: {:?}", doctors);
    Ok(())
}

#[allow(dead_code)]
async fn test_connection() {
    match get_db_pool().await {
        Ok(pool) => {
            println!("✅ Successfully connected to the database!");
            // Do something simple like a ping
            if let Err(e) = sqlx::query("SELECT 1").execute(&pool).await {
                eprintln!("❌ Ping failed: {}", e);
            } else {
                println!("✅ Ping query succeeded");
            }
        }
        Err(e) => {
            eprintln!("❌ Error connecting to database: {}", e);
        }
    }
}
