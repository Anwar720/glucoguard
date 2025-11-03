//For DB quaries like inserting data, fetching data etc.
use crate::db::models::{User,Patient};
use uuid::Uuid;
use crate::auth;
use chrono::Utc;
use rusqlite::{params, Connection, Result,OptionalExtension};
use crate::utils::{get_current_time_string};
use std::error::Error;


// check if username exists and return boolean
pub fn check_user_name_exists(conn: &rusqlite::Connection, username: &str) -> Result<bool> {
    // Prepare returns a Result<Statement, Error>, so unwrap or use `?`
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE user_name = ?1")?;
    // Now stmt is a Statement, so you can call query_row on it
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;
    
    Ok(count > 0)
}


// create user using username, password, and role and insert into database
// pass user_id as None  , to create a new user_id
pub fn create_user(
    conn: &Connection,
    username: &str,
    password: &str,
    role: &str,
    user_id: Option<String>, // optional user_id for creating accounts with user_id that exists in code_activation table.
) -> Result<()> {
    // Check if username already exists
    if check_user_name_exists(conn, username)? {
        eprintln!(" Username '{}' already exists.", username);
        return Err(rusqlite::Error::ExecuteReturnedResults);
    }

    // Hash password
    let password_hash = match auth::hash_password(password) {
        Ok(hash) => hash,
        Err(_) => {
            eprintln!(" Failed to hash password.");
            return Err(rusqlite::Error::InvalidQuery);
        }
    };

    // Use provided user_id or generate new one
    let user_id = user_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    // Create new user
    let new_user = User {
        id: user_id,
        user_name: username.to_string(),
        password_hash,
        role: role.to_string(),
        created_at: Utc::now().to_rfc3339(),
        last_login: None,
    };

    // Insert user
    let sql = "
        INSERT INTO users (id, user_name, password_hash, role, created_at, last_login)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    ";

    conn.execute(
        sql,
        params![
            new_user.id,
            new_user.user_name,
            new_user.password_hash,
            new_user.role,
            new_user.created_at,
            new_user.last_login
        ],
    )?;

    println!("User account successfull created.");

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

// create patient account from patient object
pub fn insert_patient_account_details_in_db(conn: &rusqlite::Connection, patient: &Patient) -> Result<()> {
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

// insert patient activation code for patient to create account
pub fn insert_activation_code(conn: &rusqlite::Connection,code: &str,user_type: &str,user_id: &str,issuer_id: &str) -> Result<()> {
    let sql = "
        INSERT INTO activation_codes(
            code,
            user_type,
            user_id,
            issuer_id,
            created_at
        ) VALUES (?1, ?2, ?3, ?4,?5)
    ";

    conn.execute(
        sql,
        params![code, user_type, user_id, issuer_id, get_current_time_string()],
    )?;

    Ok(())
}

#[derive(Debug)]
pub struct PatientSummary {
    pub patient_id: String,
    pub first_name: String,
    pub last_name: String,
}

pub fn get_patients_by_clinician_id(conn: &Connection, clinician_id: &String) -> Result<Vec<PatientSummary>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT patient_id, first_name, last_name 
        FROM patients 
        WHERE clinician_id = ?1"
    )?;

    // get all patients with given clinician_id
    let patient_iter = stmt.query_map([clinician_id], |row| {
        Ok(PatientSummary {
            patient_id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
        })
    })?;

    // iterate through patient_iter and push patient structs into vector
    let mut patients = Vec::new();
    for patient in patient_iter {
        patients.push(patient?);
    }

    Ok(patients)
}

    
pub struct ActivationCodeInfo {
    pub user_type: String,
    pub user_id: String,
}


pub fn validate_activation_code(
    conn: &Connection,
    code: &str
) -> Result<Option<ActivationCodeInfo>> {
    let sql = "
        SELECT user_type, user_id
        FROM activation_codes
        WHERE code = ?1
    ";

    let mut stmt = conn.prepare(sql)?;

    // .optional() requires OptionalExtension trait
    let info = stmt.query_row(params![code], |row| {
        Ok(ActivationCodeInfo {
            user_type: row.get(0)?,
            user_id: row.get(1)?,
        })
    }).optional()?; // <-- now works

    Ok(info)
}

// Removes an activation code from the database after it has been used
pub fn remove_activation_code(conn: &Connection, code: &str) -> Result<()> {
    let sql = "DELETE FROM activation_codes WHERE code = ?1";
    
    conn.execute(sql, params![code])?;
    
    Ok(())
}


/// Adds a caretaker team member to the database
pub fn add_caretaker_team_member(
    conn: &Connection,
    caretaker_id: &str,
    patient_id: &str, // comma-separated patient IDs
) -> Result<()> {
    let sql = "
        INSERT INTO caretaker_team (care_taker_id, patient_id_list)
        VALUES (?1, ?2)
    ";

    conn.execute(sql, params![caretaker_id, patient_id])?;

    Ok(())
}
