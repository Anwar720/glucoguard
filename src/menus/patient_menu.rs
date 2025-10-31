
use crate::utils;

pub fn show_patient_menu(conn: &rusqlite::Connection) {
    loop {
        println!("=== Patient Menu ===");
        println!("1. example Action");
        println!("2. example Action");
        println!("3. Logout");

        let choice = utils::get_user_choice();

        match choice {
            1 => println!("example functionality.."), // Placeholder for actual functionality
            2 => println!("example functionality..."), // Placeholder for actual functionality
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}