//Helper and Common Utilities
use std::io::{self, Write};


// reads user choice from menu table and returns as integer
pub fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(0)
}
