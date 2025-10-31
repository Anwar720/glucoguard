// login menu
use std::io::{self, Write};
use crate::db::queries;
use crate::auth;
use rpassword::read_password;

pub struct LoginResult {
    pub success: bool,
    pub user_id: Option<String>,
    pub role: Option<String>,
}

fn user_login(conn:&rusqlite::Connection ,username:&str, password:&str)-> LoginResult{
    //return template for failed login 
    let failed_login = LoginResult {
        success: false,
        user_id: None,
        role: None,
    };

    // fetch user by username 
    let user = match queries::get_user_by_username(conn, username) {
        Ok(u) => u,
        Err(_) => {
            return failed_login;
        }
    };
    
    // check if user exists
    if let Some(user) = user {
        let password_is_valid = match auth::verify_password(password, &user.password_hash) {
            Ok(valid) => valid,
            Err(_) => {
                println!("Login failed due to internal error.");
                return failed_login;
            }
        };
    
        // if username and password match return successful login
        if password_is_valid {
            return LoginResult {
                success: true,
                user_id: Some(user.id),
                role: Some(user.role),
            };
        }
    }
    
        // return failed login
        LoginResult {
            success: false,
            user_id: None,
            role: None,
        }
    
}


pub fn show_login_menu(conn: &rusqlite::Connection) {
    println!("\n --------------- Login ---------------");
    loop{
        print!("Enter username: ");
        io::stdout().flush().unwrap();      
        let mut username = String::new();
        io::stdin().read_line(&mut username);
        username = username.trim().to_string();
        print!("Enter password: ");
        io::stdout().flush().unwrap();
        let password = read_password().expect("Failed to read password");
        let password = password.trim().to_string();

        // call login function to validate username and password
        let login_result = user_login(&conn,&username,&password);
        if login_result.success {
            println!("Login successful! User ID: {}, Role: {}", 
            login_result.user_id.unwrap(), login_result.role.unwrap());
            break;
        }
        // generic error message for failed login 
        println!("Username or Password is incorrect.");
    }
    println!("Login successful! Welcome!");
}