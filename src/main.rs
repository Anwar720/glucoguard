mod db;
use crate::db::db_utils;
use crate::db::initialize;
use std::io::{self, Write};
mod menus;

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
    let conn = initialize::establish_connection().expect("Failed to initialize DB");
    //db_utils::print_table_info(&db_connection.unwrap()).unwrap();


    // Main loop for menu
    loop {
        print!("\nEnter 1 to Login ");
        print!("\nEnter 2 to Create an account ");
        print!("\nEnter 3 to Patient Menu ");
        print!("\nEnter 4 to Caretaker Menu ");
        print!("\nEnter 5 to Clinician Menu ");
        print!("\nEnter your choice:  ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            "1" => {
                println!("\n ---------------Login---------------");
                print!("Enter username: ");
                io::stdout().flush().unwrap();      
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();  
                print!("Enter password: ");
                io::stdout().flush().unwrap();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
            }
            "2"=> {
                println!("---------------Create Account---------------");
                print!("\nEnter 1 for Clinicians/Doctors");
                print!("\nEnter 2 for Patients");
                print!("\nEnter 3 for Caretakers");
                print!("\nEnter your role: ");
                io::stdout().flush().unwrap();
                let mut role_input = String::new();
                io::stdin().read_line(&mut role_input).unwrap();
                let role = role_input.trim();   
                // creating account based on role 
                match role {
                    "1" => println!("\nCreating account for Clinician/Doctor"),
                    // "2" => println!("\nPlease contact your Clinician to create a Patient account"),
                    "2" => {menus::patient_menu::run_patient_menu(&conn);}
                    "3" => println!("\nCreating account for Caretaker"),
                    _ => println!("\nInvalid role choice!")
                }
            }
            "3" => {
                println!("\n---------------Patient Menu---------------");
                // TODO: add view/edit/delete patient actions here
                menus::patient_menu::run_patient_menu(&conn);
            }
            "4" => {
                println!("\n---------------Caretaker Menu---------------");
                // TODO: add view/edit/delete caretaker actions here
                menus::caretaker_menu::run_caretaker_menu(&conn);
            }
            "5" => {
                println!("\n---------------Clinician Menu---------------");
                // TODO: add view/edit/delete clinician actions here
                menus::clinician_menu::run_clinician_menu(&conn);
            }
            _ => println!("Invalid choice!")
        }
    }
}