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
    ListPrescriptions(ListPrescriptionArgs),
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

#[derive(Debug, Args)]
pub struct ListPrescriptionArgs {
    #[arg(long)]
    pub patient_name: Option<String>,

    #[arg(long)]
    pub patient_number: Option<String>,

    #[arg(long)]
    pub doctor_name: Option<String>,

    #[arg(long)]
    pub doctor_id: Option<i32>,

    #[arg(long)]
    pub include_expired: Option<bool>,
}

impl ListPrescriptionArgs {
    pub fn sanitised(&self) -> Self {
        Self {
            patient_name: self.patient_name.as_ref().map(|s| s.trim().to_owned()),
            patient_number: self.patient_number.as_ref().map(|s| s.trim().to_owned()),
            doctor_name: self.doctor_name.as_ref().map(|s| s.trim().to_owned()),
            doctor_id: self.doctor_id,
            include_expired: self.include_expired,
        }
    }
}
