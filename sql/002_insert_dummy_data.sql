INSERT INTO doctors (name, license_number)
VALUES ('Dr. Alice Smith', 'A12345');

INSERT INTO patients (name, date_of_birth, patient_number)
VALUES ('John Doe', '1980-01-01', 'B98765');

INSERT INTO prescriptions (
    patient_id, doctor_id, medication_name, dosage,
    total_refills, refills_remaining, valid_until)
VALUES
(1, 1, 'Amoxicillin', '500mg 3x/day', 2, 2, '2025-12-31');
