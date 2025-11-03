
use crate::utils;
use crate::menus::menu_utils;
use crate::access_control::Role;
use crate::auth::{generate_one_time_code};
use crate::db::queries::{insert_activation_code,
                        insert_patient_account_details_in_db,
                        get_patients_by_clinician_id};
use rusqlite::{Connection};
use crate::session::SessionManager;

//Takes in db connection and role struct:
    // Role{
    //      name: String,
    //      id: String, // user id 
    //      permissions: HashSet<Permission>,
    // }
pub fn show_clinician_menu(conn: &rusqlite::Connection,role: &Role,session_id: &str) {
    
    let session_manager = SessionManager::new();

    loop {
        // Fetch session from the database
        let session = match session_manager.get_session_by_id(conn, session_id) {
            Some(s) => s,
            None => {
                println!("Invalid or expired session. Please log in again.");
                return;
            }
        };

        // Check if session is expired
        if session.is_expired() {
            println!("Session has expired. Logging you out...");
            if let Err(e) = session_manager.remove_session(conn, session_id) {
                println!("Failed to remove session: {}", e);
            }
            return;
        }

         // Fetch session from the database
        let session = match session_manager.get_session_by_id(conn, session_id) {
            Some(s) => s,
            None => {
                println!("Invalid or expired session. Please log in again.");
                return;
            }
        };

        // Check if session is expired
        if session.is_expired() {
            println!("Session has expired. Logging you out...");
            if let Err(e) = session_manager.remove_session(conn, session_id) {
                println!("Failed to remove session: {}", e);
            }
            return;
        }


        println!("=== Clinician Menu ===");
        println!("1. View Patients");
        println!("2. Create Patient Account");
        println!("3. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {
                show_patients_menu(conn,&role.id);
            }, // Placeholder for actual functionality
            2 =>{ // get patient data and create patient account 
                handle_patient_account_creation(&conn,role);
            },
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



fn handle_patient_account_creation(conn:&rusqlite::Connection,role:&Role){
    let patient = menu_utils::get_new_patient_input(role.id.clone());


    //insert patient data in db and check if successfully inserted
    match insert_patient_account_details_in_db(&conn,&patient){
        Ok(())=>{
            let patient_activation_code = generate_one_time_code(15);
            let new_account_type = "patient";
            // insert patient activation code in db with patient data
            match insert_activation_code(conn,&patient_activation_code,&new_account_type,&patient.patient_id,&role.id){
                Ok(())=>{
                    println!(
                        "\n Patient activation code generated successfully!\n\
                        Please share this code with the patient so they can create their account.\n\
                        Activation Code: {}\n",
                        patient_activation_code
                    );
                },
                Err(e)=>{
                    println!("Error saving patient activation link");
                }
            }
        },
        Err(e)=>{
            println!("Error creating patient activation link");
        },
    }

}

fn show_patients_menu(conn: &Connection, clinician_id: &String) {
    match get_patients_by_clinician_id(conn, clinician_id) {
        Ok(patients) => {
            if patients.is_empty() {
                println!("No patients found.");
            } else {
                println!("\n--- Patients under your care ---");
                for (index, patient) in patients.iter().enumerate() {
                    println!(
                        "\t{}. {} {}",
                        index+1,patient.first_name, patient.last_name
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error retrieving patients: {}", e);
        }
    }
}