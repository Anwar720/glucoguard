// SQLite Database set up and connection handling

#[derive(Debug)]
struct User{
    id: i32,
    user_name: String,
    password_hash: String,
    role: String,
    created_at: String,
    last_login: Option<String>
}
struct Patient{
    patient_id: i32,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    basal_rate: f32,
    bolus_rate: f32,
    max_dosage: f32,
    low_glucose_threshold: f32,
    high_glucose_threshold: f32,
    clinician_id: i32,
    caretaker_id: i32
}
struct PatientCareTeam{
    care_taker_id: i32,
    patient_id_list: Vec<i32>
}
struct GlucoseReading{
    reading_id: i32,
    patient_id: i32,
    glucose_level: f32,
    reading_time: String,
    status: String
}
struct InsulinLog{
    dosage_id: i32,
    patient_id: i32,
    action_type: String,
    dosage_units: f32,
    requested_by: String,
    dosage_time: String
}
struct Alerts{
    alert_id: i32,
    patient_id: i32,
    alert_type: String,
    alert_message: String,
    alert_time: String,
    is_resolved: bool,
    resolved_by: Option<String>,
}
struct MealLog{
    meal_id: i32,
    patient_id: i32,
    carbohydrate_amount: f32,
    meal_time: String
}


fn create_users_table(conn:&rusqlite::Connection)->rusqlite::Result<()> { 
    // SQL to create users table
    let sql = "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
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

// generating all tables using create functions
pub fn initialize_database(conn:&rusqlite::Connection)->rusqlite::Result<()> {
    create_users_table(conn)?;
    create_patients_table(conn)?;
    create_patient_care_team_table(conn)?;
    create_glucose_readings_table(conn)?;
    create_insulin_logs_table(conn)?;
    create_alerts_table(conn)?;
    create_meal_logs_table(conn)?;
    println!("Database initialized successfully.");
    Ok(())
}
pub fn establish_connection() -> rusqlite::Result<rusqlite::Connection>{
     // Open the database connection
    let connection = rusqlite::Connection::open("./data/database.db")?;
    
    // Initialize database tables if they don't exist
    initialize_database(&connection)?;
    
    Ok(connection)
}


//used to print table info for debugging
fn print_table_info(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table in tables {
        let table_name = table?;
        println!("Table: {}", table_name);

        let mut col_stmt = conn.prepare(&format!("PRAGMA table_info('{}')", table_name))?;
        let columns = col_stmt.query_map([], |row| {
            Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?)) // (name, type)
        })?;

        for col in columns {
            let (name, col_type) = col?;
            println!("  {}: {}", name, col_type);
        }
    }

    Ok(())
}