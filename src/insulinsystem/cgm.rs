use rusqlite::{params, Connection, Result};
use std::time::Duration;
use crate::insulinsystem::{insulin, logger};
use crate::alerts;
//use crate::db::queries;
use std::thread;
//use std::io;
use rusqlite::Error as SqlError;
use std::error::Error;
use chrono::Duration as ChronoDuration;

//-----------------------------CGM Reading Struct-----------------------------//
struct CgmReading {
	pub entry: i32,
	pub id: String,
	pub timestamp: String,
	pub glucose_level: f32,
	pub trend: Option<i32>,
	pub alert_generated: i32,
	pub reading_date: String,
}
//-----------------------------CGM Table-----------------------------//
//create a new table for a new user
pub fn create_user_cgm_table(conn: &Connection, patient_id: &str) -> Result<(), Box<dyn Error>> {
	// create table name dynamically
	let table_name = get_user_cgm_table_name(patient_id);
	conn.execute(
		&format!(
			"CREATE TABLE IF NOT EXISTS {} (
				entry INTEGER PRIMARY KEY AUTOINCREMENT,
				id TEXT NOT NULL,
				timestamp TEXT NOT NULL,
				glucose_level FLOAT NOT NULL,
				trend INTEGER,
				alert_generated INTEGER DEFAULT 0,
				reading_date TEXT NOT NULL
			)",
			table_name
		),
		[],
	)?;
	match logger::first_write_cgm_reading_to_patient_table( &conn, &patient_id) {
		Ok(it) => it,
		Err(err) => return Err(err),
	};
	Ok(())
}
//get current line
pub fn query_entry(conn: &Connection, patient_id: &str) -> Result<usize> {
	let table_name = get_user_cgm_table_name(patient_id);
	let mut stmt = conn.prepare(&format!(
		"SELECT entry FROM {} ORDER BY entry DESC LIMIT 1",
		table_name
	))?;
	let entry: usize = stmt.query_row([], |row| row.get(0))?;
	Ok(entry)
}

//get user cgm table name
pub fn get_user_cgm_table_name(patient_id: &str) -> String {
	format!("cgm_data_user_{}", patient_id)
}
//remove user cgm table
pub fn remove_user_cgm_table(conn: &Connection, patient_id: &str) -> rusqlite::Result<()> {
	//remove table
	let table_name = get_user_cgm_table_name(patient_id);
	conn.execute(
		&format!("DROP TABLE IF EXISTS {}", table_name),
		[],
	)?;
	Ok(())
}
//---------------------CGM Reading and Processing--------------------//
//read form glucose table every 20 seconds
pub fn read_cgm_periodically(
	conn: &Connection, 
	patient_id: &str, 
	table_name: &str) -> Result<(), Box<dyn Error>> {
    loop {
        println!("Reading CGM data for user {}...", patient_id);
        read_cgm_and_process(conn, patient_id, table_name)?;
        thread::sleep(Duration::from_secs(20));
    }
}
//read form glucose table
pub fn read_cgm_and_process(
	conn: &Connection, 
	patient_id: &str, 
	table_name: &str) -> Result<(), Box<dyn Error>> {
	// Get the latest CGM reading that has not been processed
	let mut stmt = conn.prepare(&format!(
		"SELECT 
		entry,
		id, 
		timestamp, 
		glucose_level, 
		trend, 
		alert_generated, 
		reading_date 
		FROM {} 
		WHERE alert_generated = 0 
		ORDER BY entry ASC 
		LIMIT 1",
		table_name
	))?;
	let cgm_iter = stmt.query_map([], |row| {
    Ok(CgmReading {
        entry: row.get(0)?,         
        id: row.get(1)?,         
        timestamp: row.get(2)?,
        glucose_level: row.get(3)?,
        trend: row.get(4)?,
        alert_generated: row.get(5)?,
        reading_date: row.get(6)?,
    })
})?;
	// Process the CGM reading
	for cgm_reading in cgm_iter {
		let reading = cgm_reading?;
		let timestamp = &reading.timestamp;
		let glucose_level = reading.glucose_level;
		// Check for hypoglycemia
		if glucose_level < 70.0 {
			alerts::generate_hypoglycemia_alert(
				patient_id, 
				glucose_level, 
				timestamp)?;
		}
		// Check for hyperglycemia
		else if glucose_level > 180.0 {
			alerts::generate_hyperglycemia_alert(
				patient_id, 
				glucose_level, 
				timestamp)?;
			// Process insulin delivery
			insulin::process_insulin_delivery(
				conn, 
				patient_id, 
				glucose_level, 
				timestamp)?;
		}
		// Mark the reading as processed
		conn.execute(
			&format!(
				"UPDATE {} SET alert_generated = 1 WHERE entry = ?1",
				table_name
			),
			params![reading.entry],
		)?;
	}
	Ok(())
}