use crate::utils;

pub fn show_care_taker_menu(conn: &rusqlite::Connection) {
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
