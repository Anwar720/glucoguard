use crate::utils;

pub fn show_admin_menu(conn: &rusqlite::Connection) {
    loop {
        println!("=== Admin Menu ===");
        println!("1. Create Clinician Account");
        println!("2. ");
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