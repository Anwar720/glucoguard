use rusqlite::{Connection, Result, OptionalExtension,params};
use chrono::{NaiveDateTime, Local,TimeZone};

// Fetch patient with safety limits
    pub struct PatientSafety {
        pub patient_id: String,
        pub first_name: String,
        pub last_name: String,
        pub max_dosage: f32,
        pub low_glucose_threshold: f32,
        pub high_glucose_threshold: f32,
        pub basal_rate:f32,
        pub bolus_rate:f32,
    }
    // hold glucose reading data
    #[derive(Debug)]
    pub struct GlucoseReading {
        pub glucose_level: f32,
        pub reading_time: String,
        pub status: String,
    }

/// # Return Type
/// - **Result<Option<String>>**
///   - `Ok(Some(patient_id))` → A patient was found for the clinician.
///   - `Ok(None)` → No patient found for that clinician_id.
///   - `Err(e)` → A database error occurred during the query.
///
pub fn get_one_patient_by_clinician_id(conn: &Connection, clinician_id: &str) -> rusqlite::Result<String> {
    let result: Option<String> = conn
        .query_row(
            "SELECT patient_id FROM patients WHERE clinician_id = ?1 LIMIT 1",
            rusqlite::params![clinician_id],
            |row| row.get(0),
        )
        .optional()?; // converts "no row" into Ok(None)

    // Return empty string if no patient found
    Ok(result.unwrap_or_else(|| "".to_string()))
}
pub fn get_one_patient_by_caretaker_id(conn: &Connection, caretaker_id: &str) -> rusqlite::Result<String> {
    let result: Option<String> = conn
        .query_row(
            "SELECT patient_id FROM patients WHERE caretaker_id = ?1 LIMIT 1",
            rusqlite::params![caretaker_id],
            |row| row.get(0),
        )
        .optional()?; // converts "no row" into Ok(None)

    // Return empty string if no patient found
    Ok(result.unwrap_or_else(|| "".to_string()))
}


//# Return Type
/// - **Result<Option<PatientSafety>>**
///   - `Ok(Some(PatientSafety))` → A patient record was found and successfully parsed.
///   - `Ok(None)` → No matching patient was found in the database.
///   - `Err(e)` → A database or query error occurred during lookup.
///
pub fn get_patient_data_from_patient_table(conn: &Connection, patient_id: &str) -> Result<Option<PatientSafety>> {
    let patient = conn
        .query_row(
            "SELECT patient_id, first_name, last_name, max_dosage,basal_rate,bolus_rate, low_glucose_threshold, high_glucose_threshold
            FROM patients
            WHERE patient_id = ?1",
            rusqlite::params![patient_id],
            |row| {
                Ok(PatientSafety {
                    patient_id: row.get(0)?,
                    first_name: row.get(1)?,
                    last_name: row.get(2)?,
                    max_dosage: row.get(3)?,
                    basal_rate:row.get(4)?,
                    bolus_rate:row.get(5)?,
                    low_glucose_threshold: row.get(6)?,
                    high_glucose_threshold: row.get(7)?,
                    
                })
            },
        )
        .optional()?; // This converts “no rows” into Ok(None)


    Ok(patient)
}


// returns patient glucose history data as a vector
pub fn get_patient_glucose_history(conn: &Connection,patient_id: &str, display_just_latest_one: bool,) -> Result<Vec<(String, f32, String)>> {
    // Build query based on flag
    let query = if display_just_latest_one {
        "SELECT reading_time, glucose_level, status
         FROM glucose_readings
         WHERE patient_id = ?1
         ORDER BY datetime(reading_time) DESC
         LIMIT 1"
    } else {
        "SELECT reading_time, glucose_level, status
         FROM glucose_readings
         WHERE patient_id = ?1
         ORDER BY datetime(reading_time) DESC"
    };

    let mut stmt = conn.prepare(query)?;
    let readings = stmt
        .query_map(params![patient_id], |row| {
            Ok((
                row.get::<_, String>(0)?, // reading_time
                row.get::<_, f32>(1)?,    // glucose_level
                row.get::<_, String>(2)?, // status
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(readings)
}


pub fn get_patient_insulin_data(conn: &Connection, patient_id: &str, display_just_latest_one: bool) -> Result<()> {
    // Build query depending on whether only the latest record should be shown
    let query = if display_just_latest_one {
        "SELECT dosage_id, action_type, dosage_units, requested_by, dosage_time
         FROM insulin_logs
         WHERE patient_id = ?1
         ORDER BY datetime(dosage_time) DESC
         LIMIT 1"
    } else {
        "SELECT dosage_id, action_type, dosage_units, requested_by, dosage_time
         FROM insulin_logs
         WHERE patient_id = ?1
         ORDER BY datetime(dosage_time) DESC"
    };

    let mut stmt = conn.prepare(query)?;
    let insulin_iter = stmt.query_map([patient_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,    // dosage_id
            row.get::<_, String>(1)?, // action_type
            row.get::<_, f32>(2)?,    // dosage_units
            row.get::<_, String>(3)?, // requested_by
            row.get::<_, String>(4)?, // dosage_time
        ))
    })?;

    let mut found = false;

    if display_just_latest_one {
        println!("\n--- Latest Insulin Log ---");
    } else {
        println!("\n--- Insulin Delivery History ---");
    }

    for entry in insulin_iter {
        found = true;
        let (dosage_id, action_type, dosage_units, requested_by, time_str) = entry?;

        // Convert timestamp to readable format
        let formatted_time = if let Ok(parsed_time) =
            NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S")
        {
            let local_time = Local.from_utc_datetime(&parsed_time);
            local_time.format("%b %d, %Y %I:%M %p").to_string()
        } else {
            format!("Unparsed: {}", time_str)
        };

        println!(
            "* {}| {} | {:.1} units |",
            formatted_time, action_type, dosage_units);
    }

    if !found {
        println!("No insulin logs found for this patient.");
    }

    Ok(())
}

// displays patient glucose readings and if display_just_latest_one is true then just display just one latest reading
pub fn display_patient_glucose_readings(conn: &Connection, patient_id: &str, display_just_latest_glucose: bool) {
    match get_patient_glucose_history(conn, patient_id,display_just_latest_glucose) {
        Ok(readings) => {
            if readings.is_empty() {
                println!("No glucose readings found for this patient.");
            } else {
                println!("\n--- Glucose Reading History ---");


                for (time_str, glucose, status) in readings {
                    if let Ok(parsed_time) = NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S") {
                        let local_time = Local.from_utc_datetime(&parsed_time);
                        let formatted_time = local_time.format("%b %d, %Y %I:%M %p");
                        println!(
                            "* {} | Glucose: {:.1} mg/dL | Status: {}",
                            formatted_time, glucose, status
                        );
                    } else {
                        println!(
                            "* {} | Glucose: {:.1} mg/dL | Status: {} (unparsed time)",
                            time_str, glucose, status
                        );
                    }
                }
            }
        }
        Err(e) => eprintln!("Error retrieving glucose readings: {}", e),
    }
}

pub fn display_patient_complete_glucose_insulin_history(conn: &Connection, patient_id: &str){
    println!("running display patient data:--------");
    // retrieve patient info from patient table in database
    match get_patient_data_from_patient_table(&conn, patient_id) {
        Ok(Some(patient)) => {
            // display all glucose data for patient
            display_patient_glucose_readings(&conn, patient_id, false);
            get_patient_insulin_data(&conn, patient_id, false);
        },
        Ok(None) => println!("No patient found."),
        Err(e) => eprintln!("Error: {}", e),
    }


}

pub fn show_patient_current_basal_bolus_limits(conn:&rusqlite::Connection, patient_id:&String){
    match get_patient_data_from_patient_table(conn, patient_id) {
        Ok(Some(patient)) => {
            println!("\n--------Patient dosage info --------");
            println!("Name: {} {}", patient.first_name, patient.last_name);
            println!("Max Dosage: {:.2} units", patient.max_dosage);
            println!("Basal rate: {:.1}, Bolus rate: {:.1} \n",
                    patient.basal_rate, patient.bolus_rate);
        }
        Ok(None) => {
            println!("No patient found with ID: {}", patient_id);
        }
        Err(e) => {
            eprintln!("Error fetching patient data: {}", e);
        }
    }
}