//For DB quaries like inserting data, fetching data etc.
use crate::db::models::{User,Patient};
use uuid::Uuid;
use crate::auth;
use chrono::Utc;
use rusqlite::{Result,params,Connection};
use crate::session::Session;
use std::time::UNIX_EPOCH;
use tokio::time::Duration;

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

/// Fetches all usernames with role clinician
pub fn get_all_clinicians(conn: &rusqlite::Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT user_name FROM users WHERE role = ?1")?;
    
    let clinician_iter = stmt.query_map(["clinician"], |row| {
        row.get(0) // get the first column: user_name
    })?;

    // Collect into a vector
    let mut usernames = Vec::new();
    for username_result in clinician_iter {
        usernames.push(username_result?);
    }

    Ok(usernames)
}

// create patient account from patient struct
pub fn create_patient_account(conn: &rusqlite::Connection, patient: &Patient) -> Result<()> {
    let sql = "
        INSERT INTO patients (
            patient_id,
            first_name,
            last_name,
            date_of_birth,
            basal_rate,
            bolus_rate,
            max_dosage,
            low_glucose_threshold,
            high_glucose_threshold,
            clinician_id,
            caretaker_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
    ";

    conn.execute(
        sql,
        params![
            patient.patient_id,
            patient.first_name,
            patient.last_name,
            patient.date_of_birth,
            patient.basal_rate,
            patient.bolus_rate,
            patient.max_dosage,
            patient.low_glucose_threshold,
            patient.high_glucose_threshold,
            patient.clinician_id,
            patient.caretaker_id
        ]
    )?;

    Ok(())
}

//add a session entry
pub fn add_session_to_db(conn: &rusqlite::Connection, session: &Session) -> rusqlite::Result<()> {
    // Convert create_time to UNIX timestamp
    let creation_time = session.create_time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Convert expiration_time to seconds
    let expiration_time = session.exp_time.as_secs();

    let sql = "
        INSERT INTO session (
            session_id,
            username,
            creation_time,
            expiration_time
        ) VALUES (?1, ?2, ?3, ?4)
    ";

    conn.execute(
        sql,
        params![
            session.session_id,
            session.username,
            creation_time,
            expiration_time
        ]
    )?;

    Ok(())
}

//remove a session entry
pub fn remove_session(conn: &rusqlite::Connection, session_id: &str) -> rusqlite::Result<()> {
    let sql = "DELETE FROM session WHERE session_id = ?1";
    conn.execute(sql, [session_id])?;
    Ok(())
}

//get a session
pub fn get_session(conn: &Connection, username: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT session_id, username, creation_time, expiration_time FROM session WHERE username = ?1"
    )?;
    
    let mut rows = stmt.query([username])?;
    
    if let Some(row) = rows.next()? {
        let session_id: String = row.get(0)?;
        let username: String = row.get(1)?;
        let create_time_secs: u64 = row.get(2)?;
        let exp_time_secs: u64 = row.get(3)?;
        let session = Session {
            session_id,
            username,
            create_time: UNIX_EPOCH + Duration::from_secs(create_time_secs),
            exp_time: Duration::from_secs(exp_time_secs),
        };
        Ok(Some(session))
    } else {
        Ok(None) //session not found
    }
}

// fetch by session_id
pub fn get_session_by_id(conn: &Connection, session_id: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT session_id, username, creation_time, expiration_time FROM session WHERE session_id = ?1"
    )?;

    let mut rows = stmt.query([session_id])?;

    if let Some(row) = rows.next()? {
        let session_id: String = row.get(0)?;
        let username: String = row.get(1)?;
        let create_time_secs: u64 = row.get(2)?;
        let exp_time_secs: u64 = row.get(3)?;

        Ok(Some(Session {
            session_id,
            username,
            create_time: UNIX_EPOCH + Duration::from_secs(create_time_secs),
            exp_time: Duration::from_secs(exp_time_secs),
        }))
    } else {
        Ok(None)
    }
}

// remove expired sessions
pub fn remove_expired_sessions(conn: &Connection) -> Result<()> {
    let now_secs = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    conn.execute(
        "DELETE FROM session WHERE (?1 - creation_time) > expiration_time",
        params![now_secs],
    )?;
    Ok(())
}