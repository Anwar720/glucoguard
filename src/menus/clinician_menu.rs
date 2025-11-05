use crate::utils;
use crate::menus::menu_utils;
use crate::access_control::Role;
use crate::auth::{generate_one_time_code};
use crate::db::queries::{insert_activation_code,
                        insert_patient_account_details_in_db,
                        get_patients_by_clinician_id};
use rusqlite::{Connection,Result};
use crate::session::SessionManager;
use crate::insulin::{get_one_patient_by_clinician_id,display_patient_complete_glucose_insulin_history,
                        get_patient_data_from_patient_table};
use std::io::{self, Write};
use crate::input_validation::{read_non_empty_input,read_valid_date_dd_mm_yyyy,read_valid_float};

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

        // Check role is Admin
        if session.role != "clinician"{
            println!("Invalid access rights to view page");
            return;
        }

        println!("=== Clinician Menu ===");
        println!("1. View patient insulin history.");
        println!("2. Edit patient insulin limits.");// 
        println!("3. Edit patient glucose limits.");
        println!("4. View patient info");
        println!("5. Create Patient Account");
        println!("6. Logout");
        println!("Enter your choice: ");

        let choice = utils::get_user_choice();

        // get patient being treated by clinician 
        let current_patient_id:String =  get_one_patient_by_clinician_id(&conn,&session.user_id ).expect("REASON");;

        match choice {
        
                1 => {
                    // requres that we have a valid patient_id for clinician 
                    if current_patient_id.is_empty(){
                        println!("Cannot perform this action because no patient is assigned.");
                        continue;
                    }

                    //View logs of all insulin deliveries and glucose readings.
                    // request_insulin_flow(conn,&session.user_id);
                    display_patient_complete_glucose_insulin_history(conn,&current_patient_id);
                }, 
                2 =>{
                    // requres that we have a valid patient_id for clinician 
                    if current_patient_id.is_empty(){
                        println!("Cannot perform this action because no patient is assigned.");
                        continue;
                    }

                    //Adjust insulin delivery parameters based on patient needs.
                    // basal and bolus modifications
                        if let Some((bolus, basal)) = prompt_new_bolus_basal_limits() {
                            println!("New limits set - Bolus: {:.2}, Basal: {:.2}", bolus, basal);
                            match update_patient_bolus_basal(&conn, &current_patient_id, bolus, basal) {
                                Ok(rows_updated) if rows_updated > 0 => println!("Patient limits updated successfully."),
                                Ok(_) => println!("No patient found with that ID."),
                                Err(e) => eprintln!("Error updating patient limits"),
                            }
                        }
                },
                3=>{
                    // requres that we have a valid patient_id for clinician 
                    if current_patient_id.is_empty(){
                        println!("Cannot perform this action because no patient is assigned.");
                        continue;
                    }

                    //Set dosage limits, safety thresholds, and alert conditions.
                    // prompts user for max,min glucose and update into patients table
                    match update_patient_max_min_glucose(&conn, &current_patient_id) {
                        Ok(rows_updated) if rows_updated > 0 => println!("Patient glucose limits updated successfully."),
                        Ok(_) => println!("No patient found with that ID."),
                        Err(e) => eprintln!("Error updating patient limits"),
                    }
                },
                4=>{
                    // view patient info 
                    show_patient_data(conn, &current_patient_id)
                },
                5=>{
                    // get patient data and create patient account 
                    handle_patient_account_creation(&conn,role, &session_id);
                },
                6 => {
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

fn handle_patient_account_creation(conn:&rusqlite::Connection, role:&Role, session_id: &str){
    let patient = menu_utils::get_new_patient_input(role.id.clone());

    //insert patient data in db and check if successfully inserted
    match insert_patient_account_details_in_db(&conn, &patient, &session_id){
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

fn show_patients_menu(conn: &Connection, clinician_id: &String, session_id: &str) {
    match get_patients_by_clinician_id(conn, clinician_id, &session_id) {
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


/// Prompts the user to enter new bolus and basal insulin limits.
/// Returns a tuple: (bolus_limit, basal_limit)
pub fn prompt_new_bolus_basal_limits() -> Option<(f32, f32)> {

    let basal_rate = read_valid_float("Basal Rate (0–100): ", 0.0, 100.0);
    let bolus_rate = read_valid_float("Bolus Rate (0–100): ", 0.0, 100.0);

    Some((bolus_rate, basal_rate))
}


///// Updates the bolus and basal insulin limits for a given patient.
pub fn update_patient_bolus_basal(conn: &Connection,patient_id: &str,bolus: f32,basal: f32,
) -> Result<usize> {
    conn.execute(
        "UPDATE patients
         SET bolus_rate = ?1,
             basal_rate = ?2
         WHERE patient_id = ?3",
        rusqlite::params![bolus, basal, patient_id],
    )
}

pub fn update_patient_max_min_glucose(conn: &Connection,patient_id: &str) -> Result<usize> {

    let low_glucose_threshold = read_valid_float("Low Glucose Threshold (0–100): ", 0.0, 100.0);
    let high_glucose_threshold = read_valid_float("High Glucose Threshold (100–1000): ", 100.0, 1000.0);

    conn.execute(
        "UPDATE patients
         SET low_glucose_threshold = ?1,
            high_glucose_threshold = ?2
         WHERE patient_id = ?3",
        rusqlite::params![low_glucose_threshold, high_glucose_threshold, patient_id],
    )
}


fn show_patient_data(conn: &rusqlite::Connection, patient_id: &str) {
    match get_patient_data_from_patient_table(conn, patient_id) {
        Ok(Some(patient)) => {
            println!("\n--------Patient Info--------");
            println!("Name: {} {}", patient.first_name, patient.last_name);
            println!("Max Dosage: {:.2} units", patient.max_dosage);
            println!("Glucose Thresholds: low {:.1}, high {:.1} \n",
                     patient.low_glucose_threshold, patient.high_glucose_threshold);
        }
        Ok(None) => {
            println!("No patient data found ");
        }
        Err(e) => {
            eprintln!("Error fetching patient data: {}", e);
        }
    }
}
