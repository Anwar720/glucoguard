use crate::utils;
use crate::access_control::Role; 

pub fn show_caretaker_menu(conn: &rusqlite::Connection,role:&Role) {
    loop {
        println!("=== CareTaker Menu ===");
        println!("1. View Patients");
        println!("2. example Action");
        println!("3. Logout");

        let choice = utils::get_user_choice();

        match choice {
            1 => println!("example functionality.."), // Placeholder for actual functionality
            2 => println!("example functionality.."), // Placeholder for actual functionality
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}
