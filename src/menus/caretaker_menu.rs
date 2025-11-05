use crate::utils;
use crate::access_control::Role; 
use crate::session::SessionManager;
use rusqlite::Connection;
use crate::alerts;
use::crate::insulinsystem::{cgm, insulin};

pub fn show_caretaker_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();
    
    loop {
        //check access and session validity
        if !role.has_permission(&crate::access_control::Permission::ViewGlucose) {
            println!("Access denied: You do not have permission to access the caretaker menu.");
            return;
        }
        if let Some(session) = crate::db::queries::get_session(conn, session_id).unwrap_or(None) {
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

        println!("=== CareTaker Menu ===");
        println!("1. View Patient's Recent Glucose Readings");
        println!("2. View Patient's Current Basal Rate and Bolus Insulin Options");
        println!("3. Request Bolus Insulin Dose");
        println!("4. Configure Basal Insulin Dose Time");
        println!("5. Review Historical Insulin Delivery and Glucose Data");
        println!("6. View Alerts");
        println!("7. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {
                // View Patient's Recent Glucose Readings
                println!("--- View Patient's Recent Glucose Readings ---");
                cgm::view_patient_glucose_readings(conn);
            },
            2 => {
                // View Patient's Current Basal Rate and Bolus Insulin Options
                println!("--- View Patient's Current Basal Rate and Bolus Insulin Options ---");
                queries::view_patient_insulin_options(conn);
            },
            3 => {
                // Request Bolus Insulin Dose
                println!("--- Request Bolus Insulin Dose ---");
                insulin::request_bolus_dose(conn);
            },
            4 => {
                // Configure Basal Insulin Dose Time
                println!("--- Configure Basal Insulin Dose Time ---");
                queries::adjust_basalL_rate(conn);
            },
            5 => {
                // Review Historical Insulin Delivery and Glucose Data
                println!("--- Review Historical Insulin Delivery and Glucose Data ---");
                cgm::review_historical_data(conn);
            },
            6 => {
                // View Alerts
                println!("--- View Alerts ---");
                alerts::view_alerts_hypo_and_hyper(conn);
            },
            7 => {
                println!("Logging out...");
                // Synchronous session removal
                if let Err(e) = session_manager.remove_session(conn, session_id) {
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