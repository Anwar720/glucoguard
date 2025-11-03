// login menu
use std::io::{self, Write};
use crate::db::queries;
use crate::auth;
use rpassword::read_password;
use crate::session::SessionManager;

pub struct LoginResult {
    pub success: bool,
    pub user_id: String,
    pub role: String,
    pub session_id: String
}

fn user_login(conn:&rusqlite::Connection ,username:&str, password:&str) -> LoginResult{
    //return template for failed login 
    let failed_login = LoginResult {
        success: false,
        user_id:String::new(),
        role:String::new(),
        session_id: String::new()
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
                user_id: user.id,
                role: user.role.to_string(),
                session_id: String::new()
            };
        }
    }
    
        // return failed login
        LoginResult {
            success: false,
            user_id: String::new(),
            role: String::new(),
            session_id: String::new(),
        }
    
}


pub fn show_login_menu(conn: &rusqlite::Connection) -> LoginResult {
    println!("\n --------------- Login ---------------");

    let session_manager = SessionManager::new();

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
        let mut login_result = user_login(&conn,&username,&password);
        if login_result.success {
            //create a session on successful login
            // Create DB session
            match session_manager.create_session(conn, username.clone()) {
                Ok(session_id) => {
                    login_result.session_id = session_id; // set session_id
                    println!("Login successful. Session created: {}", login_result.session_id);
                    return login_result;
                }
                Err(e) => {
                    eprintln!("Failed to create session: {}", e);
                    return login_result;
                }
            }
        }
        // generic error message for failed login 
        println!("Username or Password is incorrect.");
    }
}