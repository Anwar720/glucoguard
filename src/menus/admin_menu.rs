use crate::utils;
use crate::access_control::Role;
use crate::db::queries;
use crate::menus::menu_utils::get_new_account_credentials;

pub fn show_admin_menu(conn: &rusqlite::Connection,role:&Role) {
    loop {
        println!("\n=== Admin Menu ===");
        println!("1. Create Clinician Account");
        println!("2. View Clinician Account List");
        println!("3. Logout");

        let choice = utils::get_user_choice();

        match choice {
            1 => {
                // Get username and password from user
                match get_new_account_credentials() {
                    Ok((username, password)) => {
                        // Create the user in the database
                        match queries::create_user(&conn, &username, &password, "clinician") {
                            Ok(_) => println!("\nClinician account successfully created."),
                            Err(e) => println!("\nError creating account."),
                        }
                    }
                    Err(e) => eprintln!("Failed to read input: {}", e),
                }
            },
            2 =>{
                // display list of clinician usernames
                let clinicians = queries::get_all_clinicians(&conn);
                println!("Clinician accounts:");
                for username in clinicians {
                    println!("- {:?}", username);
                }
            } , 
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}