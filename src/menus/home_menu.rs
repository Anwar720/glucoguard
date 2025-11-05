use std::io::{self, Write};

/// Displays the home menu and returns the user's choice.
/// Returns:
/// - `0` → Exit
/// - `1` → Sign In
/// - `2` → Sign Up
pub fn show_home_menu() -> u8 {
    loop {
        println!("\n========== Welcome to GlucoGuard ==========");
        println!("0. Exit");
        println!("1. Sign In");
        println!("2. Sign Up with Activation code.");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        // Read user input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(choice) = input.trim().parse::<u8>() {
                // Validate choice
                if choice == 0 || choice == 1 || choice == 2 {
                    return choice;
                }
            }
        }
        // Invalid input handling
        println!("Invalid choice. Please enter 0 or 1 or  2");
    }
}