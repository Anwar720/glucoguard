
use crate::utils;

pub fn show_clinician_menu(conn: &rusqlite::Connection) {
    loop {
        println!("=== Clinician Menu ===");
        println!("1. View Patients");
        println!("2. Create Patient Account");
        println!("3. Logout");

        let choice = utils::get_user_choice();

        match choice {
            1 => println!("Viewing patients..."), // Placeholder for actual functionality
            2 => println!("Creating patient account..."), // Placeholder for actual functionality
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}