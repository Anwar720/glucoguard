use crate::utils;
use crate::access_control::Role;
use crate::db::queries;
use crate::menus::menu_utils::get_new_account_credentials;
use crate::session::SessionManager;
use rusqlite::Connection;

pub fn show_admin_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();

    loop {
        //check access and session validity
        if !role.has_permission(&crate::access_control::Permission::CreateClinicianAccount) {
            println!("Access denied: You do not have permission to access the admin menu.");
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

        println!("\n=== Admin Menu ===");
        println!("1. Create Clinician Account");
        println!("2. View Clinician Accounts");
        println!("3. Remove Clinician Account");
        println!("4. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {// Create Clinician Account
                println!("--- Create Clinician Account ---");
                let (username, password) = get_new_account_credentials();
                match queries::create_clinician_account(conn, &username, &password) {
                    Ok(_) => println!("Clinician account '{}' created successfully.", username),
                    Err(e) => println!("Failed to create clinician account: {}", e),
                }
            },
            2 => {// View Clinician Accounts
                println!("--- View Clinician Accounts ---");
                match queries::get_all_clinician_accounts(conn) {
                    Ok(clinicians) => {
                        for clinician in clinicians {
                            println!("ID: {}, Username: {}", clinician.id, clinician.username);
                        }
                    }
                    Err(e) => println!("Failed to retrieve clinician accounts: {}", e),
                }
            },
            3 => {// Remove Clinician Account
                println!("--- Remove Clinician Account ---");
                print!("Enter Clinician ID to remove: ");
                let clinician_id = utils::get_user_choice();
                match queries::remove_clinician_account(conn, clinician_id) {
                    Ok(_) => println!("Clinician account with ID {} removed successfully.", clinician_id),
                    Err(e) => println!("Failed to remove clinician account: {}", e),
                }
            },

            4 => {
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