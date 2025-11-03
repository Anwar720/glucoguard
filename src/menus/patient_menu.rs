use crate::utils;
use crate::access_control::Role;
use crate::db::queries::{insert_activation_code,add_caretaker_team_member};
use crate::auth::{generate_one_time_code};
use uuid::Uuid;
use crate::session::SessionManager;
use rusqlite::Connection;

pub fn show_patient_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
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

        // Check role is patient
        if session.role != "patient"{
            println!("Invalid access rights to view page");
            return;
        }

        println!("=== Patient Menu ===");
        println!("1. Create Caretaker activation code.");
        println!("2. example Action");
        println!("3. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {
                create_and_display_caretaker_activation_code(conn,role, &session_id);
            },
            2 => println!("example functionality..."), // Placeholder for actual functionality
            3 => {
                println!("Logging out...");
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
            if let Err(e) = add_caretaker_team_member(conn, user_id.as_str(), role.id.as_str(), &session_id) {
                eprintln!(" Failed to add caretaker team member: {}", e);
            }

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