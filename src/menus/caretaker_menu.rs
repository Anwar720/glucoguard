use crate::utils;
use crate::access_control::Role; 
use crate::session::SessionManager;
use rusqlite::Connection;
use crate::insulin::{display_patient_glucose_readings,
        get_patient_data_from_patient_table,
        get_patient_insulin_data,
        get_one_patient_by_caretaker_id,
        display_patient_complete_glucose_insulin_history,
        show_patient_current_basal_bolus_limits
};


pub fn show_caretaker_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();
    
    loop {

         // Fetch session from the database
        let session = match session_manager.get_session_by_id(conn, &session_id) {
            Some(s) => s,
            None => {
                println!("Invalid or expired session. Please log in again.");
                return;
            }
        };

        // Check expiration
        if session.is_expired() {
            println!("Session has expired. Logging you out...");
            if let Err(e) = session_manager.remove_session(conn, &session_id) {
                println!("Failed to remove session: {}", e);
            }
            return;
        }
        
        // Check role is Admin
        if session.role != "caretaker"{
            println!("Invalid access rights to view page");
            return;
        }

        println!("=== CareTaker Menu ===");

        println!("1) View most recent glucose readings.");
        println!("2) View current basal and bolus options.");
        println!("3) Request bolus insulin dose.");
        println!("4) Configure basal insulin dose time.");
        println!("5) View patient insulin history.");
        println!("6. Logout");
        println!("Enter your choice: ");
        let choice = utils::get_user_choice();


        // get patient being treated by caretaker
        let current_patient_id:String =  get_one_patient_by_caretaker_id(&conn,&session.user_id ).expect("REASON");

        match choice {

            1 => {
                //View the patient’s most recent glucose readings.
                display_patient_glucose_readings(&conn, &current_patient_id, true);
            },
            2 => {
                // View the patient’s current basal rate and bolus insulin options.
                show_patient_current_basal_bolus_limits(conn,&current_patient_id);
            }, 
            3 => {
                //Request a bolus insulin dose.
                // – Caretakers cannot request more than the prescribed maximum dose or violate safety limits.
                // – Caretakers cannot request more than one dose per every four hours (corresponding to
                // three meals a day).

            }, 
            4 => {
                //Configure basal insulin dose time.
                // – Caretakers can adjust the basal insulin dose, which will be effective within 24 hours, so as
                // not to overlap a previous dose.
                // – Caretakers cannot request more than the prescribed maximum dose or violate safety limits.

            }, 
            5 => {
                //Review historical insulin delivery and glucose data.
                display_patient_complete_glucose_insulin_history(conn,&current_patient_id);
            }, 
            6 => {
                println!("Logging out...");
                if let Err(e) = session_manager.remove_session(conn, &session_id) {
                    println!("Failed to remove session: {}", e);
                } else {
                    println!("Session removed. Goodbye!");
                }
                return;
            },
            _ => println!("Invalid choice"),
        }
    }
}

