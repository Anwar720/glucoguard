mod db;
mod menus;
mod auth;
mod utils;
use crate::db::db_utils;
use crate::db::initialize;
use crate::menus::login_menu;




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

    
    login_menu::show_login_menu(&db_connection);
    





}