use crate::utils;
use crate::access_control::Role; 
use crate::session::SessionManager;
use rusqlite::Connection;

pub fn show_caretaker_menu(conn: &rusqlite::Connection,role:&Role,session_id: &str) {
    let session_manager = SessionManager::new();
    
    loop {

         // Fetch session from the database
        let session = match session_manager.get_session_by_id(conn, &session_id) {
            Some(s) => s,
            None => {
                println!("Invalid or expired session. Please log in again.");
                return;
            }
        };

        // Check expiration
        if session.is_expired() {
            println!("Session has expired. Logging you out...");
            if let Err(e) = session_manager.remove_session(conn, &session_id) {
                println!("Failed to remove session: {}", e);
            }
            return;
        }
        

        println!("=== CareTaker Menu ===");
        println!("1) View Patient Summary");
        println!("2) Request Insulin for Patient");
        println!("3. Logout");
        print!("Enter your choice: ");
        let choice = utils::get_user_choice();

        match choice {
            1 => {
                //view_patient_summary_flow(conn)
            },
            2 => {
                //request_insulin_flow(conn)
            }, 
            3 => {
                println!("Logging out...");
                if let Err(e) = session_manager.remove_session(conn, &session_id) {
                    println!("Failed to remove session: {}", e);
                } else {
                    println!("Session removed. Goodbye!");
                }
                return;
            },
            _ => println!("Invalid choice"),
        }
    }
}





// /* ---------- Insulin Request with Validation ---------- */

// fn request_insulin_flow(conn: &rusqlite::Connection) {
//     println!("\nRequest Insulin");

//     let Some(patient_id) = prompt_parse_i32("Enter patient ID: ") else { return; };

//     // Fetch patient with safety limits
//     struct PatientSafety {
//         patient_id: i32,
//         first_name: String,
//         last_name: String,
//         max_dosage: f32,
//         low_glucose_threshold: f32,
//         high_glucose_threshold: f32,
//     }

//     let patient = match conn.query_row(
//         "SELECT patient_id, first_name, last_name, max_dosage, low_glucose_threshold, high_glucose_threshold
//          FROM patients
//          WHERE patient_id = ?1",
//         rusqlite::params![patient_id],
//         |row| {
//             Ok(PatientSafety {
//                 patient_id: row.get(0)?,
//                 first_name: row.get(1)?,
//                 last_name: row.get(2)?,
//                 max_dosage: row.get(3)?,
//                 low_glucose_threshold: row.get(4)?,
//                 high_glucose_threshold: row.get(5)?,
//             })
//         },
//     ).optional() {
//         Ok(Some(p)) => p,
//         Ok(None) => {
//             println!("No patient found with ID {}.", patient_id);
//             return;
//         }
//         Err(e) => {
//             eprintln!("Lookup failed: {}", e);
//             return;
//         }
//     };

//     println!("Patient: {} {} (ID {})", patient.first_name, patient.last_name, patient.patient_id);
//     println!("Max dosage: {} units", patient.max_dosage);

//     // Check for recent insulin request (4-hour cooldown)
//     match conn.query_row(
//         "SELECT dosage_time 
//          FROM insulin_logs
//          WHERE patient_id = ?1
//          ORDER BY datetime(dosage_time) DESC
//          LIMIT 1",
//         rusqlite::params![patient_id],
//         |row| row.get::<_, String>(0),
//     ).optional() {
//         Ok(Some(last_dosage_time)) => {
//             // Check if last request was within 4 hours
//             let time_check: bool = conn.query_row(
//                 "SELECT (julianday('now') - julianday(?1)) * 24.0 < 4.0",
//                 rusqlite::params![last_dosage_time],
//                 |row| row.get(0),
//             ).unwrap_or(false);

//             if time_check {
//                 println!("❌ ERROR: Cannot request insulin.");
//                 println!("Last insulin request was at {}.", last_dosage_time);
//                 println!("Caretakers can only request one dose per every 4 hours.");
//                 println!("Please wait at least 4 hours between requests.");
//                 return;
//             } else {
//                 println!("✓ Last request was more than 4 hours ago (at {}).", last_dosage_time);
//             }
//         }
//         Ok(None) => {
//             println!("✓ No previous insulin requests found for this patient.");
//         }
//         Err(e) => {
//             eprintln!("Warning: Could not check previous requests: {}", e);
//             // Continue anyway, but log the warning
//         }
//     }

//     // Check latest glucose reading
//     let latest_glucose = match conn.query_row(
//         "SELECT glucose_level FROM glucose_readings
//          WHERE patient_id = ?1
//          ORDER BY datetime(reading_time) DESC
//          LIMIT 1",
//         rusqlite::params![patient_id],
//         |row| row.get::<_, f32>(0),
//     ).optional() {
//         Ok(Some(level)) => {
//             println!("Latest glucose: {:.1} mg/dL (thresholds: low {} | high {})", 
//                      level, patient.low_glucose_threshold, patient.high_glucose_threshold);

//             // Safety check: BLOCK if glucose is already low
//             if level < patient.low_glucose_threshold {
//                 println!("❌ ERROR: Cannot request insulin.");
//                 println!("Glucose is below safe threshold ({:.1} < {:.1})", 
//                          level, patient.low_glucose_threshold);
//                 println!("Request rejected for safety reasons.");
//                 return; // Block instead of just warning
//             }
//             // Optional: Also block if glucose is too high (dangerous to add more insulin)
//             if level > patient.high_glucose_threshold + 50.0 { // e.g., way above threshold
//                 println!("❌ ERROR: Glucose is critically high ({:.1} > {:.1})", 
//                          level, patient.high_glucose_threshold);
//                 println!("Consult clinician before requesting additional insulin.");
//                 return;
//             }
//             Some(level)
//         }
//         Ok(None) => {
//             println!("⚠️  WARNING: No glucose readings found for this patient.");
//             Some(-1.0) // sentinel value
//         }
//         Err(_) => None,
//     };

//     // action type: Basal/Bolus/Correction/etc.
//     let action_type = prompt_string("Action type (e.g., Bolus, Basal): ");
//     if action_type.trim().is_empty() {
//         eprintln!("Action type is required.");
//         return;
//     }

//         // action type: Basal/Bolus/Correction/etc.
//         let action_type = prompt_string("Action type (e.g., Bolus, Basal): ");
//         if action_type.trim().is_empty() {
//             eprintln!("Action type is required.");
//             return;
//         }

//         // For Basal insulin: enforce 24-hour effective window (no overlap)
//         if action_type.trim().eq_ignore_ascii_case("Basal") {
//             match conn.query_row(
//                 "SELECT dosage_time, dosage_units
//                  FROM insulin_logs
//                  WHERE patient_id = ?1
//                    AND LOWER(action_type) = 'basal'
//                  ORDER BY datetime(dosage_time) DESC
//                  LIMIT 1",
//                 rusqlite::params![patient_id],
//                 |row| {
//                     Ok((
//                         row.get::<_, String>(0)?,
//                         row.get::<_, f32>(1)?,
//                     ))
//                 },
//             ).optional() {
//                 Ok(Some((last_basal_time, last_basal_dose))) => {
//                     // Check if last basal adjustment was within 24 hours
//                     let within_24h: bool = conn.query_row(
//                         "SELECT (julianday('now') - julianday(?1)) * 24.0 < 24.0",
//                         rusqlite::params![last_basal_time],
//                         |row| row.get(0),
//                     ).unwrap_or(false);

//                     if within_24h {
//                         let hours_remaining = conn.query_row::<f64, _, _>(
//                             "SELECT 24.0 - ((julianday('now') - julianday(?1)) * 24.0)",
//                             rusqlite::params![last_basal_time],
//                             |row| row.get(0),
//                         ).unwrap_or(0.0);

//                         println!("❌ ERROR: Cannot adjust basal insulin dose.");
//                         println!("Last basal adjustment was at {} (dose: {:.2} units).", 
//                                  last_basal_time, last_basal_dose);
//                         println!("Basal adjustments are effective for 24 hours to prevent overlap.");
//                         println!("Please wait {:.1} more hours before the next adjustment.", hours_remaining);
//                         return;
//                     } else {
//                         println!("✓ Last basal adjustment was more than 24 hours ago (at {}).", last_basal_time);
//                     }
//                 }
//                 Ok(None) => {
//                     println!("✓ No previous basal insulin adjustments found.");
//                 }
//                 Err(e) => {
//                     eprintln!("Warning: Could not check previous basal adjustments: {}", e);
//                     // Continue with caution
//                 }
//             }
//         }

//     // dosage in units - with max validation
//     let Some(dosage_units) = prompt_parse_f32("Dosage units (e.g., 1.5): ") else { return; };

//     // Validate dosage doesn't exceed max
//     if dosage_units > patient.max_dosage {
//         eprintln!("❌ ERROR: Dosage {:.2} exceeds maximum allowed dose of {:.2} units.", 
//                   dosage_units, patient.max_dosage);
//         eprintln!("Request rejected.");
//         return;
//     }

//     if dosage_units <= 0.0 {
//         eprintln!("❌ ERROR: Dosage must be positive.");
//         return;
//     }

//     // who requested (caretaker)
//     let requested_by = prompt_string("Requested by (your name/id): ");
//     if requested_by.trim().is_empty() {
//         eprintln!("Requested by is required.");
//         return;
//     }

//     // Final confirmation
//     println!("\n--- Request Summary ---");
//     println!("Patient: {} {} (ID {})", patient.first_name, patient.last_name, patient.patient_id);
//     println!("Type: {}", action_type);
//     println!("Dosage: {:.2} units (max: {:.2})", dosage_units, patient.max_dosage);
//     if let Some(glucose) = latest_glucose {
//         if glucose > 0.0 {
//             println!("Latest glucose: {:.1} mg/dL", glucose);
//         }
//     }
//     print!("Confirm request? [y/N]: ");
//     io::stdout().flush().unwrap();
//     let mut confirm = String::new();
//     io::stdin().read_line(&mut confirm).unwrap();
//     if !confirm.trim().eq_ignore_ascii_case("y") {
//         println!("Request cancelled.");
//         return;
//     }

//     match insert_insulin_log(conn, patient_id, &action_type, dosage_units, &requested_by) {
//         Ok(_) => println!("✓ Insulin request logged successfully."),
//         Err(e) => eprintln!("Failed to log insulin request: {}", e),
//     }
// }

// /* ---------- Helper Functions ---------- */

// fn insert_insulin_log(
//     conn: &rusqlite::Connection,
//     patient_id: i32,
//     action_type: &str,
//     dosage_units: f32,
//     requested_by: &str,
// ) -> rusqlite::Result<usize> {
//     conn.execute(
//         "INSERT INTO insulin_logs (patient_id, action_type, dosage_units, requested_by, dosage_time)
//          VALUES (?1, ?2, ?3, ?4, datetime('now'))",
//         rusqlite::params![patient_id, action_type, dosage_units, requested_by],
//     )
// }

// // You'll need to add view_patient_summary_flow if it's not already there
// // For now, adding a stub:
// fn view_patient_summary_flow(_conn: &rusqlite::Connection) {
//     println!("View Patient Summary");
// }

// /* ---------- Prompt helpers ---------- */

// fn prompt_string(label: &str) -> String {
//     print!("{}", label);
//     io::stdout().flush().unwrap();
//     let mut s = String::new();
//     io::stdin().read_line(&mut s).unwrap();
//     s.trim().to_string()
// }

// fn prompt_parse_i32(label: &str) -> Option<i32> {
//     let s = prompt_string(label);
//     match s.parse::<i32>() {
//         Ok(v) => Some(v),
//         Err(_) => { eprintln!("Invalid integer."); None }
//     }
// }

// fn prompt_parse_f32(label: &str) -> Option<f32> {
//     let s = prompt_string(label);
//     match s.parse::<f32>() {
//         Ok(v) => Some(v),
//         Err(_) => { eprintln!("Invalid number."); None }
//     }
// }