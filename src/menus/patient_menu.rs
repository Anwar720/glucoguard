use crate::utils;
use crate::access_control::Role;
use crate::db::queries::{insert_activation_code,
                        add_caretaker_team_member,
                        add_caretaker_to_patient_account};
use crate::auth::{generate_one_time_code};
use uuid::Uuid;
use crate::session::SessionManager;
use crate::db::queries;
use crate::alerts;

pub fn show_patient_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();
    
    loop {
        //check access and session validity
        if !role.has_permission(&crate::access_control::Permission::ViewGlucoseReadings) {
            println!("Access denied: You do not have permission to access the patient menu.");
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

        println!("=== Patient Menu ===");
        println!("1. Create Caretaker Activation Code");
        println!("2. View Glucose Readings");
        println!("3. View Insulin Rates");
        println!("4. Request Bolus Dose");
        println!("5. Edit Basal Dose");
        println!("6. Review Historical Data");
        println!("7. View Alerts");
        println!("8. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();
        match choice {
            1 => {
                // Create Caretaker Activation Code
                println!("--- Create Caretaker Activation Code ---");
                create_and_display_caretaker_activation_code(conn, role, session_id);
            },
            2 => {
                // View Glucose Readings
                println!("--- View Glucose Readings ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            3 => {
                // View Insulin Rates
                println!("--- View Insulin Rates ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            4 => {
                // Request Bolus Dose
                println!("--- Request Bolus Dose ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            5 => {
                // Edit Basal Dose
                println!("--- Edit Basal Dose ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            6 => {
                // Review Historical Data
                println!("--- Review Historical Data ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            7 => {
                // View Alerts
                println!("--- View Alerts ---");
                // Functionality to be implemented
                println!("Feature under development.");
            },
            8 => {
                /println!("Logging out...");
                // Synchronous session removal
                if let Err(e) = session_manager.remove_session(conn, session_id) {
                    println!("Failed to remove session: {}", e);
                } else {
                    println!("Session removed. Goodbye!");
                }
                return;
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
pub fn create_and_display_caretaker_activation_code(
    conn: &rusqlite::Connection,
    role: &Role,
    session_id: &str
) {
    // Generate a one-time activation code
    let activation_code = generate_one_time_code(15);

    let new_account_type = "caretaker";
    let user_id = Uuid::new_v4().to_string();

    // Insert activation code into DB
    match insert_activation_code(conn, &activation_code, new_account_type, user_id.as_str(), role.id.as_str()) {
        Ok(()) => {
            // Add caretaker to team
            if let Err(e) = add_caretaker_team_member(conn, user_id.as_str(), role.id.as_str(), session_id) {
                eprintln!(" Failed to add caretaker team member: {}", e);
}

            // add caretaker user_id to patient table
            add_caretaker_to_patient_account(conn,role.id.as_str(),user_id.as_str());
            // Display activation code for clinician to share
            println!(
                "\n Caretaker activation code generated successfully!\n\
                Please share this code with the caretaker so they can create their account.\n\
                Activation Code: {}\n",
                activation_code
            );
        }
        Err(e) => {
            eprintln!(" Error saving caretaker activation code: {}", e);
        }
    }
}

