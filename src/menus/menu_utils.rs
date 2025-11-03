// helper functions for menu
use std::io::{self, Write};
use crate::db::models::{Patient};

/// Prompts the user to create a new account (username + password)
pub fn get_new_account_credentials() -> io::Result<(String, String)> {
    // Prompt for username
    print!("Enter a new username: ");
    io::stdout().flush()?; // flush to show prompt
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    // Loop until passwords match
    loop {
        // Prompt for password 
        let mut password1 = String::new();
        println!("Enter a new password: ");
        io::stdin().read_line(&mut password1)?;
        let password1 = password1.trim().to_string(); 

        let mut password2 = String::new();
        println!("Confirm your password: ");
        io::stdin().read_line(&mut password2)?;
        let password2 = password2.trim().to_string(); 

        if password1 != password2 {
            println!("Passwords do not match. Please try again.\n");
            continue; // retry
        }

        if password1.is_empty() {
            println!("Password cannot be empty. Please try again.\n");
            continue; // retry
        }

        return Ok((username, password1));
    }
}

// validate patient struct fields
pub fn validate_patient(patient: &Patient) -> Result<(), String> {
    if patient.first_name.trim().is_empty() {
        return Err("First name cannot be empty".to_string());
    }
    if patient.last_name.trim().is_empty() {
        return Err("Last name cannot be empty".to_string());
    }
    if patient.basal_rate < 0.0 || patient.bolus_rate < 0.0 || patient.max_dosage < 0.0 {
        return Err("Dosage rates cannot be negative".to_string());
    }
    if patient.low_glucose_threshold >= patient.high_glucose_threshold {
        return Err("Low glucose threshold must be less than high glucose threshold".to_string());
    }
    Ok(())
}

// Helper to read a line from stdin
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// pub fn get_new_patient_input() -> Patient {
    // loop {

    //     // Get inputs
    //     let first_name = read_input("Enter patient first name: ");
    //     let last_name = read_input("Enter patient last name: ");
    //     let date_of_birth = read_input("Enter date of birth (YYYY-MM-DD): ");
    //     let basal_rate: f32 = read_input("Enter basal rate: ").parse().unwrap_or(0);
    //     let bolus_rate: f32 = read_input("Enter bolus rate: ").parse().unwrap_or(0);
    //     let max_dosage: f32 = read_input("Enter max dosage: ").parse().unwrap_or(0);
    //     let low_glucose_threshold: f32 = read_input("Enter low glucose threshold: ").parse().unwrap_or(0);
    //     let high_glucose_threshold: f32 = read_input("Enter high glucose threshold: ").parse().unwrap_or(0);

    //     // Create patient struct
    //     let patient = Patient {
    //         patient_id: String::new,
    //         first_name: first_name.clone(),
    //         last_name: last_name.clone(),
    //         date_of_birth: date_of_birth.clone(),
    //         basal_rate,
    //         bolus_rate,
    //         max_dosage,
    //         low_glucose_threshold,
    //         high_glucose_threshold,
    //         clinician_id: String::new(),
    //         caretaker_id: String::new(),
    //     };

    //     // Validate
    //     if let Err(err) = validate_patient(&patient) {
    //         eprintln!("Invalid input: {}. Please try again.\n", err);
    //         continue; // retry loop
    //     }

    //     return patient;
    // }
// }