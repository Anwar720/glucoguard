mod db;
mod session;
mod menus;
mod auth;
mod utils;
mod access_control;
use crate::session::SessionManager;
use crate::db::db_utils;
use crate::db::initialize;
// use crate::access_control;
use crate::menus::{login_menu,admin_menu,patient_menu,caretaker_menu,clinician_menu};



fn main() {

let logo = r#"
    _____ _                  _____                    _ 
  / ____| |                / ____|                   | |
 | |  __| |_   _  ___ ___ | |  __ _   _  __ _ _ __ __| |
 | | |_ | | | | |/ __/ _ \| | |_ | | | |/ _` | '__/ _` |
 | |__| | | |_| | (_| (_) | |__| | |_| | (_| | | | (_| |
  \_____|_|\__,_|\___\___/ \_____|\__,_|\__,_|_|  \__,_|"#;
                                                        
                                                        
println!("{}", logo);

    // Initialize the database connection
    let db_connection = initialize::establish_connection().unwrap();


   // db_utils::print_table_info(&db_connection.unwrap()).unwrap();

    //validate login and get user id and role
    let user_option = login_menu::show_login_menu(&db_connection);
    // create a user permission instance
    let role = access_control::Role::new(&user_option.role);

    let session_manager = SessionManager::new();

    match role.name.as_str() {
    "admin" => admin_menu::show_admin_menu(&db_connection, &role, &user_option.session_id),
    "clinician" => clinician_menu::show_clinician_menu(&db_connection, &user_option.session_id),
    "patient" => patient_menu::show_patient_menu(&db_connection, &user_option.session_id),
    "caretaker" => caretaker_menu::show_caretaker_menu(&db_connection, &user_option.session_id),
    _ => {
      // log error
      }
    }
}