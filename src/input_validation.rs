//input validation helper functions
use chrono::NaiveDate;
use std::io::{self, Write};

// Secure input reader (loops until valid input)
pub fn read_non_empty_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        //if input is not empty return data
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }else{
            println!("\nInput can't be empty.")
        }
    }
}

// validate data to format dd-MM-YYYY
pub fn read_valid_date_dd_mm_yyyy(prompt: &str) -> String {
    loop {
        let input = read_non_empty_input(prompt);
        if NaiveDate::parse_from_str(&input, "%m-%d-%Y").is_ok() {
            return input;
        }else {
            println!("Invalid date format. Please use MM-DD-YYYY.");
        }
    }
}
// Read and validate a floating number
pub fn read_valid_float(prompt: &str, min: f32, max: f32) -> f32 {
    loop {
        let input = read_non_empty_input(prompt);
        match input.parse::<f32>() {
            Ok(value) if value >= min && value <= max => return value,
            _ => println!(" Invalid number. Please enter a value between {} and {}.", min, max),
        }
    }
}