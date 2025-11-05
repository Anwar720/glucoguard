use crate::utils;
use crate::menus::menu_utils;
use crate::access_control::Role;
use crate::auth::{generate_one_time_code};
use crate::db::queries::{insert_activation_code,
                        insert_patient_account_details_in_db,
                        get_patients_by_clinician_id};
use rusqlite::{Connection};
use crate::session::SessionManager;

//Takes in db connection and role struct:
pub fn show_clinician_menu(conn: &rusqlite::Connection,role: &Role,session_id: &str) {
    let session_manager = SessionManager::new();

    loop {
        // check access and session validity
        if !role.has_permission(&crate::access_control::Permission::ViewPatient) {
            println!("Access denied: You do not have permission to view this menu.");
            return;
        }

        if let Some(session) = queries::get_session(conn, session_id).unwrap_or(None) {
            if session.is_expired() {
                println!("Session expired. Please log in again.");
                // Remove expired session
                if let Err(e) = session_manager.remove_session(conn, session_id) {
                    println!("Failed to remove expired session: {}", e);
                }
                return;
            }
        } else {
            println!("No active session found. Please log in.");
            return;
        }

        println!("=== Clinician Menu ===");
        println!("1. Create Patient Account");
        println!("2. Edit patient data");
        println!("3. View Patients under your care");
        println!("4. Set Dosage Limits and Safety Thresholds");
        println!("5. Adjust insulin parameters");
        println!("6. Logout");
        
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {
                // Create Patient Account
                println!("--- Create Patient Account ---");
                handle_patient_account_creation(conn, role, session_id);
            },
            2 => {
                // Edit patient data
                println!("--- Edit Patient Data ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            3 => {
                // View Patients under your care
                println!("--- View Patients ---");
                show_patients_menu(conn, &role.id, session_id);
            },
            4 => {
                // Set Dosage Limits and Safety Thresholds
                println!("--- Set Dosage Limits and Safety Thresholds ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            5 => {
                // Adjust insulin parameters
                println!("--- Adjust Insulin Parameters ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            6 => {
                // Logout
                println!("Logging out...");
                // Remove session
                if let Err(e) = session_manager.remove_session(conn, session_id) {
                    println!("Failed to remove session: {}", e);
                }
                break;
            },
        }
    }
}

fn handle_patient_account_creation(conn:&rusqlite::Connection, role:&Role, session_id: &str){
    let patient = menu_utils::get_new_patient_input(role.id.clone());

    //insert patient data in db and check if successfully inserted
    match insert_patient_account_details_in_db(&conn, &patient, &session_id){
        Ok(())=>{
            let patient_activation_code = generate_one_time_code(15);
            let new_account_type = "patient";
            // insert patient activation code in db with patient data
            match insert_activation_code(conn,&patient_activation_code,&new_account_type,&patient.patient_id,&role.id){
                Ok(())=>{
                    println!(
                        "\n Patient activation code generated successfully!\n\
                        Please share this code with the patient so they can create their account.\n\
                        Activation Code: {}\n",
                        patient_activation_code
                    );
                },
                Err(e)=>{
                    println!("Error saving patient activation link");
                }
            }
        },
        Err(e)=>{
            println!("Error creating patient activation link");
        },
    }
}

fn show_patients_menu(conn: &Connection, clinician_id: &String, session_id: &str) {
    match get_patients_by_clinician_id(conn, clinician_id, &session_id) {
        Ok(patients) => {
            if patients.is_empty() {
                println!("No patients found.");
            } else {
                println!("\n--- Patients under your care ---");
                for (index, patient) in patients.iter().enumerate() {
                    println!(
                        "\t{}. {} {}",
                        index+1,patient.first_name, patient.last_name
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error retrieving patients: {}", e);
        }
    }
}
