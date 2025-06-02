use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pms")]
#[command(about = "Prescription Management System CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    RebuildDb,
    ListDoctors(DoctorArgs),
    AddDoctor(AddDoctorArgs),
    ListPatients(PatientArgs),
    AddPatient(AddPatientArgs),
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub license_number: Option<String>,
}

#[derive(Debug, Args)]
pub struct AddDoctorArgs {
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub license_number: String,
}

#[derive(Debug, Args)]
pub struct PatientArgs {
    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub patient_number: Option<String>,
}

#[derive(Debug, Args)]
pub struct AddPatientArgs {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub dob: String,

    #[arg(long)]
    pub patient_number: Option<String>,
}
