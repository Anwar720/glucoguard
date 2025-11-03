// SQLite Database initializaiton and connection management


//-----------------------Database table creation functions-----------------------//
fn create_users_table(conn:&rusqlite::Connection)->rusqlite::Result<()> { 
    // SQL to create users table
    let sql = "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT NOT NULL PRIMARY KEY ,
            user_name TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_login TEXT
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_patients_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    // SQL to create patients table
    let sql = "
        CREATE TABLE IF NOT EXISTS patients (
            patient_id INTEGER PRIMARY KEY UNIQUE,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            date_of_birth TEXT NOT NULL,
            basal_rate REAL NOT NULL,
            bolus_rate REAL NOT NULL,
            max_dosage REAL NOT NULL,
            low_glucose_threshold REAL NOT NULL,
            high_glucose_threshold REAL NOT NULL,
            clinician_id INTEGER NOT NULL,
            caretaker_id INTEGER NOT NULL
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_patient_care_team_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    // SQL to create patient_care_team table
    let sql = "
        CREATE TABLE IF NOT EXISTS patient_care_team (
            care_taker_id INTEGER NOT NULL,
            patient_id_list TEXT NOT NULL
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_glucose_readings_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS glucose_readings (
            reading_id INTEGER PRIMARY KEY UNIQUE,
            patient_id INTEGER NOT NULL,
            glucose_level REAL NOT NULL,
            reading_time TEXT NOT NULL,
            status TEXT NOT NULL
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_insulin_logs_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS insulin_logs (
            dosage_id INTEGER PRIMARY KEY UNIQUE,
            patient_id INTEGER NOT NULL,
            action_type TEXT NOT NULL,
            dosage_units REAL NOT NULL,
            requested_by TEXT NOT NULL,
            dosage_time TEXT NOT NULL
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_alerts_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS alerts (
            alert_id INTEGER PRIMARY KEY UNIQUE,
            patient_id INTEGER NOT NULL,
            alert_type TEXT NOT NULL,
            alert_message TEXT NOT NULL,
            alert_time TEXT NOT NULL,
            is_resolved BOOLEAN NOT NULL,
            resolved_by TEXT
        )";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_meal_logs_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS meal_logs (
            meal_id INTEGER PRIMARY KEY UNIQUE,
            patient_id INTEGER NOT NULL,
            carbohydrate_amount REAL NOT NULL,
            meal_time TEXT NOT NULL
        )";
    conn.execute(sql, [])?;
    Ok(())
}
pub fn create_session_table(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS sessions (
            session_id INTEGER PRIMARY KEY UNIQUE,
            user_id INTEGER NOT NULL,
            creation_time TEXT NOT NULL,
            expiration_time TEXT
        )";
    conn.execute(sql, [])?;
    Ok(())
}

// generating all tables for the database
pub fn initialize_database(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    create_users_table(conn)?;
    create_patients_table(conn)?;
    create_patient_care_team_table(conn)?;
    create_glucose_readings_table(conn)?;
    create_insulin_logs_table(conn)?;
    create_alerts_table(conn)?;
    create_meal_logs_table(conn)?;
    create_session_table(conn)?;
    println!("Successfully connected to database...");
    Ok(())
}


//-----------------------Establishing database connection -----------------------//

pub fn establish_connection() -> rusqlite::Result<rusqlite::Connection>{
     // Open the database connection
    let connection = rusqlite::Connection::open("./data/database.db")?;
    
  // Initialize database tables if they don't exist
    initialize_database(&connection)?;
    
    Ok(connection)
}



