use std::io::{self, Write};
use crate::db::initialize;
use rusqlite::OptionalExtension;

pub fn run_patient_menu(conn: &rusqlite::Connection) {
    loop {
        println!("\n--- Patient Management ---");
        println!("1) Create Patient");
        println!("2) View Patient");
        println!("3) Edit Patient");
        println!("4) Delete Patient");
        println!("5) GCM Readings");
        println!("6) Request Insulin"); 
        println!("0) Back");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => create_patient_flow(conn),
            "2" => view_patient_flow(conn),
            "3" => edit_patient_flow(conn),
            "4" => delete_patient_flow(conn),
            "5" => view_glucose_readings_flow(conn),
            "6" => request_insulin_flow(conn),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}

/* ---------- Insulin Request ---------- */

fn request_insulin_flow(conn: &rusqlite::Connection) {
    println!("\nRequest Insulin");

    let Some(patient_id) = prompt_parse::<i32>("Enter patient ID: ") else { return; };

    // optional: verify patient exists
    match find_patient_by_id(conn, patient_id) {
        Ok(Some(p)) => println!("Patient: {} {} (ID {})", p.first_name, p.last_name, p.patient_id),
        Ok(None) => {
            println!("No patient found with ID {}.", patient_id);
            return;
        }
        Err(e) => {
            eprintln!("Lookup failed: {}", e);
            return;
        }
    }

    // action type: Basal/Bolus/Correction/etc.
    let action_type = prompt_string("Action type (e.g., Bolus, Basal): ");
    if action_type.trim().is_empty() {
        eprintln!("Action type is required.");
        return;
    }

    // dosage in units
    let Some(dosage_units) = prompt_parse::<f32>("Dosage units (e.g., 1.5): ") else { return; };

    // who requested (e.g., patient username or clinician)
    let requested_by = prompt_string("Requested by (name or id): ");
    if requested_by.trim().is_empty() {
        eprintln!("Requested by is required.");
        return;
    }

    match insert_insulin_log(conn, patient_id, &action_type, dosage_units, &requested_by) {
        Ok(_) => println!("Insulin request logged."),
        Err(e) => eprintln!("Failed to log insulin request: {}", e),
    }
}

fn insert_insulin_log(
    conn: &rusqlite::Connection,
    patient_id: i32,
    action_type: &str,
    dosage_units: f32,
    requested_by: &str,
) -> rusqlite::Result<usize> {
    // Table (from schema):
    // insulin_logs(dosage_id INTEGER PK, patient_id INTEGER NOT NULL,
    //              action_type TEXT NOT NULL, dosage_units REAL NOT NULL,
    //              requested_by TEXT NOT NULL, dosage_time TEXT NOT NULL)
    conn.execute(
        "INSERT INTO insulin_logs (patient_id, action_type, dosage_units, requested_by, dosage_time)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        rusqlite::params![patient_id, action_type, dosage_units, requested_by],
    )
}

/* ---------- Create ---------- */

pub fn create_patient_flow(conn: &rusqlite::Connection) {
    println!("\nCreate Patient");

    let full_name = prompt_string("Full name: ");
    let dob = prompt_string("Date of birth (YYYY-MM-DD): ");

    let basal_rate: f32 = match prompt_parse::<f32>("Basal insulin rate (e.g., 0.8): ") {
        Some(v) => v,
        None => return,
    };
    let bolus_rate: f32 = match prompt_parse::<f32>("Bolus insulin rate (e.g., 1.2): ") {
        Some(v) => v,
        None => return,
    };

    match initialize::create_patient(conn, &full_name, &dob, basal_rate, bolus_rate) {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            println!("Patient created successfully with id {}.", id);
        }
        Err(e) => eprintln!("Failed to create patient: {}", e),
    }
}

/* ---------- View ---------- */

fn view_patient_flow(conn: &rusqlite::Connection) {
    println!("\nView Patient:");
    println!(" a) Search by ID");
    println!(" b) Search by Name");
    print!("Choose a or b: ");
    io::stdout().flush().unwrap();

    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m = m.trim();

    if m.eq_ignore_ascii_case("a") {
        let Some(id) = prompt_parse::<i32>("Enter patient ID: ") else { return; };
        match find_patient_by_id(conn, id) {
            Ok(Some(p)) => print_patient(&p),
            Ok(None) => println!("No patient found with ID {}.", id),
            Err(e) => eprintln!("Search failed: {}", e),
        }
    } else if m.eq_ignore_ascii_case("b") {
        let name = prompt_string("Enter name (first, last, or full): ");
        match find_patients_by_name(conn, &name) {
            Ok(list) if list.is_empty() => println!("No patients found matching '{}'.", name),
            Ok(list) => for p in list { print_patient(&p); },
            Err(e) => eprintln!("Search failed: {}", e),
        }
    } else {
        println!("Invalid choice.");
    }
}

/* ---------- Edit ---------- */

fn edit_patient_flow(conn: &rusqlite::Connection) {
    println!("\nEdit Patient");
    let Some(id) = prompt_parse::<i32>("Enter patient ID to edit: ") else { return; };

    let Ok(opt) = find_patient_by_id(conn, id) else {
        eprintln!("Lookup failed.");
        return;
    };
    let Some(current) = opt else {
        println!("No patient found with ID {}.", id);
        return;
    };

    println!("Press Enter to keep current value shown in [brackets].");

    println!("First Name [{}]", current.first_name);
    let first_name = prompt_optional("New First Name: ");

    println!("Last Name [{}]", current.last_name);
    let last_name = prompt_optional("New Last Name: ");

    println!("Date of Birth [{}]", current.date_of_birth);
    let date_of_birth = prompt_optional("New DOB (YYYY-MM-DD): ");

    println!("Basal Rate [{}]", current.basal_rate);
    let basal_rate = prompt_optional("New Basal (e.g., 0.8): ");

    println!("Bolus Rate [{}]", current.bolus_rate);
    let bolus_rate = prompt_optional("New Bolus (e.g., 1.2): ");

    match update_patient(
        conn,
        id,
        first_name.as_deref(),
        last_name.as_deref(),
        date_of_birth.as_deref(),
        basal_rate.as_deref(),
        bolus_rate.as_deref(),
    ) {
        Ok(n) if n > 0 => println!("Patient {} updated.", id),
        Ok(_) => println!("No changes applied."),
        Err(e) => eprintln!("Update failed: {}", e),
    }
}

/* ---------- Delete ---------- */

fn delete_patient_flow(conn: &rusqlite::Connection) {
    println!("\nDelete Patient");
    let Some(id) = prompt_parse::<i32>("Enter patient ID to delete: ") else { return; };

    match find_patient_by_id(conn, id) {
        Ok(Some(p)) => {
            print!("Confirm delete patient {} {} (ID {})? [y/N]: ", p.first_name, p.last_name, p.patient_id);
            io::stdout().flush().unwrap();
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).unwrap();
            if ans.trim().eq_ignore_ascii_case("y") {
                match delete_patient_by_id(conn, id) {
                    Ok(0) => println!("No patient found with ID {}.", id),
                    Ok(_) => println!("Patient {} deleted.", id),
                    Err(e) => eprintln!("Delete failed: {}", e),
                }
            } else {
                println!("Cancelled.");
            }
        }
        Ok(None) => println!("No patient found with ID {}.", id),
        Err(e) => eprintln!("Lookup failed: {}", e),
    }
}

/* ---------- Data + Queries ---------- */

struct Patient {
    patient_id: i32,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    basal_rate: f32,
    bolus_rate: f32,
}

fn print_patient(p: &Patient) {
    println!(
        "ID: {} | Name: {} {} | DOB: {} | Basal: {} | Bolus: {}",
        p.patient_id, p.first_name, p.last_name, p.date_of_birth, p.basal_rate, p.bolus_rate
    );
}

fn find_patient_by_id(conn: &rusqlite::Connection, id: i32) -> rusqlite::Result<Option<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT patient_id, first_name, last_name, date_of_birth, basal_rate, bolus_rate
         FROM patients
         WHERE patient_id = ?1",
    )?;

    stmt.query_row(rusqlite::params![id], |row| {
        Ok(Patient {
            patient_id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            date_of_birth: row.get(3)?,
            basal_rate: row.get(4)?,
            bolus_rate: row.get(5)?,
        })
    })
    .optional()
}

fn find_patients_by_name(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<Vec<Patient>> {
    let like = format!("%{}%", name);
    let mut stmt = conn.prepare(
        "SELECT patient_id, first_name, last_name, date_of_birth, basal_rate, bolus_rate
         FROM patients
         WHERE first_name LIKE ?1
            OR last_name LIKE ?1
            OR (first_name || ' ' || last_name) LIKE ?1
         ORDER BY last_name, first_name, patient_id",
    )?;

    let rows = stmt.query_map(rusqlite::params![like], |row| {
        Ok(Patient {
            patient_id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            date_of_birth: row.get(3)?,
            basal_rate: row.get(4)?,
            bolus_rate: row.get(5)?,
        })
    })?;

    let mut results = Vec::new();
    for r in rows {
        results.push(r?);
    }
    Ok(results)
}

fn update_patient(
    conn: &rusqlite::Connection,
    id: i32,
    first_name: Option<&str>,
    last_name: Option<&str>,
    dob: Option<&str>,
    basal_rate: Option<&str>,
    bolus_rate: Option<&str>,
) -> rusqlite::Result<usize> {
    let mut sets: Vec<&str> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(v) = first_name.filter(|s| !s.is_empty()) {
        sets.push("first_name = ?");
        params.push(Box::new(v.to_string()));
    }
    if let Some(v) = last_name.filter(|s| !s.is_empty()) {
        sets.push("last_name = ?");
        params.push(Box::new(v.to_string()));
    }
    if let Some(v) = dob.filter(|s| !s.is_empty()) {
        sets.push("date_of_birth = ?");
        params.push(Box::new(v.to_string()));
    }
    if let Some(v) = basal_rate.filter(|s| !s.is_empty()) {
        let val: f32 = v.parse().map_err(|_| rusqlite::Error::FromSqlConversionFailure(
            0, rusqlite::types::Type::Real, Box::new(std::fmt::Error),
        ))?;
        sets.push("basal_rate = ?");
        params.push(Box::new(val));
    }
    if let Some(v) = bolus_rate.filter(|s| !s.is_empty()) {
        let val: f32 = v.parse().map_err(|_| rusqlite::Error::FromSqlConversionFailure(
            0, rusqlite::types::Type::Real, Box::new(std::fmt::Error),
        ))?;
        sets.push("bolus_rate = ?");
        params.push(Box::new(val));
    }

    if sets.is_empty() {
        return Ok(0);
    }

    let sql = format!("UPDATE patients SET {} WHERE patient_id = ?", sets.join(", "));
    params.push(Box::new(id));

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| &**b as &dyn rusqlite::ToSql).collect();
    conn.execute(&sql, param_refs.as_slice())
}

fn delete_patient_by_id(conn: &rusqlite::Connection, id: i32) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM patients WHERE patient_id = ?1", rusqlite::params![id])
}

/* ---------- Glucose Readings ---------- */

struct GlucoseReading {
    reading_id: i32,
    patient_id: i32,
    glucose_level: f32,
    reading_time: String,
    status: String,
}

fn view_glucose_readings_flow(conn: &rusqlite::Connection) {
    println!("\nView Glucose Readings");

    let Some(patient_id) = prompt_parse::<i32>("Enter patient ID: ") else { return; };

    // Optional: ask for a limit; empty = no limit
    let limit_input = prompt_optional("How many latest readings? (press Enter for all): ");
    let limit = match limit_input.as_deref() {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n > 0 => Some(n),
            _ => {
                eprintln!("Invalid number; showing all.");
                None
            }
        },
        _ => None,
    };

    match get_glucose_readings(conn, patient_id, limit) {
        Ok(list) if list.is_empty() => println!("No glucose readings for patient {}.", patient_id),
        Ok(list) => {
            println!("reading_id | time                | level | status");
            for r in list {
                println!("{:<10} | {:<19} | {:>5.1} | {}", r.reading_id, r.reading_time, r.glucose_level, r.status);
            }
        }
        Err(e) => eprintln!("Failed to fetch readings: {}", e),
    }
}

fn get_glucose_readings(
    conn: &rusqlite::Connection,
    patient_id: i32,
    limit: Option<usize>,
) -> rusqlite::Result<Vec<GlucoseReading>> {
    let mut sql = String::from(
        "SELECT reading_id, patient_id, glucose_level, reading_time, status
         FROM glucose_readings
         WHERE patient_id = ?1
         ORDER BY reading_time DESC",
    );
    if let Some(n) = limit {
        sql.push_str(&format!(" LIMIT {}", n));
    }

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(rusqlite::params![patient_id], |row| {
        Ok(GlucoseReading {
            reading_id: row.get(0)?,
            patient_id: row.get(1)?,
            glucose_level: row.get(2)?,
            reading_time: row.get(3)?,
            status: row.get(4)?,
        })
    })?;

    let mut results = Vec::new();
    for r in rows {
        results.push(r?);
    }
    Ok(results)
}

/* ---------- Prompt helpers ---------- */

fn prompt_string(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn prompt_optional(label: &str) -> Option<String> {
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn prompt_parse<T: std::str::FromStr>(label: &str) -> Option<T> {
    let s = prompt_string(label);
    match s.parse::<T>() {
        Ok(v) => Some(v),
        Err(_) => { eprintln!("Invalid value."); None }
    }
}