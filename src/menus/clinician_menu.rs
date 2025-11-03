use std::io::{self, Write};
use crate::db::initialize;
use rusqlite::OptionalExtension;

pub fn run_clinician_menu(conn: &rusqlite::Connection) {
    loop {
        println!("\n--- Clinician Menu ---");
        println!("0) Back");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}