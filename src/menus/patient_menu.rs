
use crate::utils;
use crate::access_control::Role;
use crate::db::queries::{insert_activation_code,add_caretaker_team_member};
use crate::auth::{generate_one_time_code};
use uuid::Uuid;

pub fn show_patient_menu(conn: &rusqlite::Connection,role:&Role) {
    loop {
        println!("=== Patient Menu ===");
        println!("1. Create Caretaker activation code.");
        println!("2. example Action");
        println!("3. Logout");

        let choice = utils::get_user_choice();

        match choice {
            1 => {
                create_and_display_caretaker_activation_code(conn,role);
            },
            2 => println!("example functionality..."), // Placeholder for actual functionality
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}
pub fn create_and_display_caretaker_activation_code(
    conn: &rusqlite::Connection,
    role: &Role 
) {
    // Generate a one-time activation code
    let activation_code = generate_one_time_code(15);

    let new_account_type = "caretaker";
    let user_id = Uuid::new_v4().to_string();

    // Insert activation code into DB
    match insert_activation_code(conn, &activation_code, new_account_type, user_id.as_str(), role.id.as_str()) {
        Ok(()) => {
            // Add caretaker to team
            if let Err(e) = add_caretaker_team_member(conn, user_id.as_str(), role.id.as_str()) {
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