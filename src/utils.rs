//Helper and Common Utilities
use std::io::{self, Write};
use chrono::Utc;

// reads user choice from menu table and returns as integer
pub fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(0)
}

pub fn get_current_time_string()->String{
    Utc::now().to_rfc3339()
}

//catch error in input strin
pub fn input_text()