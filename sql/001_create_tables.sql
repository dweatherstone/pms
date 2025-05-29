-- Drop tables if they exist (for dev convenience)
DROP TABLE IF EXISTS refills;
DROP TABLE IF EXISTS prescriptions;
DROP TABLE IF EXISTS patients;
DROP TABLE IF EXISTS doctors;

CREATE TABLE doctors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    license_number VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE patients (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    date_of_birth DATE NOT NULL
);

CREATE TABLE prescriptions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    patient_id INT NOT NULL,
    doctor_id INT NOT NULL,
    medication_name VARCHAR(100) NOT NULL,
    dosage VARCHAR(100),
    total_refills INT NOT NULL DEFAULT 0,
    refills_remaining INT NOT NULL DEFAULT 0,
    max_tablets_per_30_days INT NOT NULL DEFAULT 30,
    issued_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    valid_until DATE NOT NULL,
    FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE,
    FOREIGN KEY (doctor_id) REFERENCES doctors(id) ON DELETE CASCADE
);

CREATE TABLE refills (
    id INT AUTO_INCREMENT PRIMARY KEY,
    prescription_id INT NOT NULL,
    filled_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    quantity_dispensed INT NOT NULL,
    consult_accepted BOOLEAN NOT NULL,
    FOREIGN KEY (prescription_id) REFERENCES prescriptions(id) ON DELETE CASCADE
);
