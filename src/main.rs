use clap::Parser;
use dotenvy::dotenv;
use pms::{
    cli::{Cli, Commands},
    db::{self, get_db_pool},
    errors::AppError,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let pool = get_db_pool().await?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::RebuildDb => {
            db::run_sql_scripts(&pool).await?;
        }
        Commands::ListDoctors(args) => {
            let doctors = db::list_doctors(&pool, &args.name, &args.license_number).await?;
            for doctor in doctors {
                println!("{}", doctor);
            }
        }
        Commands::AddDoctor(args) => {
            db::add_doctor(args, &pool).await?;
        }
        Commands::ListPatients(args) => {
            let patients = db::list_patients(&pool, &args.name, &args.patient_number).await?;
            for patient in patients {
                println!("{}", patient);
            }
        }
        Commands::AddPatient(args) => {
            db::add_patient(args, &pool).await?;
        }
    }

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
