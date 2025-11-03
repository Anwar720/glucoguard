use crate::utils;
use crate::access_control::Role; 
use crate::session::SessionManager;
use rusqlite::Connection;

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
        
        println!("=== CareTaker Menu ===");
        println!("1. View Patients");
        println!("2. Example Action");
        println!("3. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {

            1 => println!("example functionality.."), // Placeholder for actual functionality
            2 => println!("example functionality.."), // Placeholder for actual functionality
            3 => {
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