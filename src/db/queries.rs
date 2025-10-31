//For DB quaries like inserting data, fetching data etc.
use crate::db::models::{User};
use uuid::Uuid;
use crate::auth;
use chrono::Utc;
use rusqlite::{Result,params};

// check if username exists and return boolean
fn check_user_name_exists(conn: &rusqlite::Connection, username: &str) -> Result<bool> {
    // Prepare returns a Result<Statement, Error>, so unwrap or use `?`
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE user_name = ?1")?;
    // Now stmt is a Statement, so you can call query_row on it
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;
    
    Ok(count > 0)
}


// create user using username, password, and role and insert into database
pub fn create_user(conn: &rusqlite::Connection, username: &str, password: &str, role: &str) -> Result<()> {
   // Check if username already exists
    if check_user_name_exists(conn, username)? {
        return Err(rusqlite::Error::ExecuteReturnedResults); 
    }
    // Hash the password
    let password_hash = auth::hash_password(password)
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    // create new user instance
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        user_name: username.to_string(),
        password_hash: password_hash,
        role: role.to_string(),
        created_at: Utc::now().to_rfc3339(),
        last_login: None,
    };
    let sql = "INSERT INTO users (id, user_name, password_hash, role, created_at, last_login) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
    conn.execute(sql, params![new_user.id,new_user.user_name,new_user.password_hash,new_user.role,new_user.created_at,new_user.last_login])?;
    Ok(())
}

// fetch user by username and return User struct
pub fn get_user_by_username(conn: &rusqlite::Connection, username: &str) -> Result<Option<User>> {
    // prepare SQL statement to fetch user by username 
    let mut sql_statement = conn.prepare("SELECT id, user_name, password_hash, role, created_at, last_login FROM users WHERE user_name = ?1")?;
    // execute query and map result to User struct
    let user_iter = sql_statement.query_map([username], |row| {
        Ok(User {
            id: row.get(0)?,
            user_name: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            created_at: row.get(4)?,
            last_login: row.get(5)?,
        })
    })?;
    
    // return the first user found or None
    for user in user_iter {
        return Ok(Some(user?));
    }
    
    Ok(None)
}
