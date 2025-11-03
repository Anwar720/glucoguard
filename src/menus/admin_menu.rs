use crate::utils;
use crate::access_control::Role;
use crate::db::queries;
use crate::menus::menu_utils::get_new_account_credentials;
use crate::session::SessionManager;
use rusqlite::Connection;

pub fn show_admin_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();

    loop {

        // Fetch session from the database
        let session = match session_manager.get_session_by_id(conn, session_id) {
            Some(s) => s,
            None => {
                println!("Invalid or expired session. Please log in again.");
                return;
            }
        };

        // Check expiration
        if session.is_expired() {
            println!("Session has expired. Please log in again.");
            return;
        }

        // Check user role is Admin
        if session.role != "admin"{
            println!("Invalid access rights to view page");
            return;
        }

        println!("\n=== Admin Menu ===");
        println!("1. Create Clinician Account");
        println!("2. View Clinician Account List");
        println!("3. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {

                // Get username and password input from user
                match get_new_account_credentials() {
                    Ok((username, password)) => {
                        // Create the user in the database
                        match queries::create_user(&conn, &username, &password, "clinician",None) {
                            Ok(_) => println!("\nClinician account successfully created."),
                            Err(e) => println!("\nError creating account: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to read input: {}", e),
                }
            }

            2 => {
                // Display list of clinicians
                match queries::get_all_clinicians(conn) {
                    Ok(clinicians) => {
                        println!("\nClinician accounts:");
                        for name in clinicians {
                            println!("- {}", name);
                        }
                    }
                    Err(e) => println!("Failed to fetch clinicians: {}", e),
                }

            }, 

            3 => {
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