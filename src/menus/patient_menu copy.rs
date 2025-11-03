use std::io::{self, Write};
use crate::db::initialize;

pub fn run_patient_menu(conn: &rusqlite::Connection) {
    loop {
        println!("\n--- Patient Management ---");
        println!("1) View Patient");
        println!("2) Edit Patient");
        println!("3) Delete Patient");
        println!("4) Create Patient");
        println!("0) Back");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                // TODO: implement view patient by id or name
                println!("View Patient: not implemented yet.");
            }
            "2" => {
                // TODO: implement edit patient
                println!("Edit Patient: not implemented yet.");
            }
            "3" => {
                // TODO: implement delete patient
                println!("Delete Patient: not implemented yet.");
            }
            "4" => create_patient_flow(conn),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}

pub fn create_patient_flow(conn: &rusqlite::Connection) {
    println!("\n---------------Create Patient---------------");

    print!("Enter full name: ");
    io::stdout().flush().unwrap();
    let mut full_name = String::new();
    io::stdin().read_line(&mut full_name).unwrap();
    let full_name = full_name.trim();

    print!("Enter date of birth (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    let mut dob = String::new();
    io::stdin().read_line(&mut dob).unwrap();
    let dob = dob.trim();

    print!("Enter basal insulin rate (e.g., 0.8): ");
    io::stdout().flush().unwrap();
    let mut basal_s = String::new();
    io::stdin().read_line(&mut basal_s).unwrap();
    let basal_rate: f32 = match basal_s.trim().parse() {
        Ok(v) => v,
        Err(_) => { eprintln!("Invalid basal rate."); return; }
    };

    print!("Enter bolus insulin rate (e.g., 1.2): ");
    io::stdout().flush().unwrap();
    let mut bolus_s = String::new();
    io::stdin().read_line(&mut bolus_s).unwrap();
    let bolus_rate: f32 = match bolus_s.trim().parse() {
        Ok(v) => v,
        Err(_) => { eprintln!("Invalid bolus rate."); return; }
    };

    match initialize::create_patient(conn, full_name, dob, basal_rate, bolus_rate) {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            println!("Patient created successfully with id {}.", id);
        }
        Err(e) => eprintln!("Failed to create patient: {}", e),
    }
}