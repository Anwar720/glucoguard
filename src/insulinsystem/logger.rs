//read from cgm dataset
//get latest reading from insulin pump
//combine data to write next reading
//write next enty to patient cgm table every 20 seconds

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use chrono::prelude::*;
use serde::Deserialize;
use std::error::Error;
use crate::db::queries;
use crate::insulinsystem::{insulin, cgm};
use rusqlite::{params, Connection, Result};
use csv;

//---------------------CGM Dataset Reading--------------------//
// Define the structure of a CGM record
#[derive(Debug, Deserialize)]
struct CgmRecord {
    RecID: u32,
    ParentHDeviceUploadsID: u32,
    PtID: u32,
    SiteID: u32,
    DeviceDtTmDaysFromEnroll: i32,
    DeviceTm: String,
    DexInternalDtTmDaysFromEnroll: i32,
    DexInternalTm: String,
    RecordType: String,
    GlucoseValue: f32,
}
// function to read and parse the CGM dataset
pub fn read_a_line_from_cgm_dataset(
    file_path: &str,
    line_number: usize,
) -> Result<CgmRecord, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut record_iter = rdr.deserialize();
    for (i, result) in record_iter.enumerate() {
        let record: CgmRecord = result?;
        if i == line_number {
            return Ok(record);
        }
    }
    Err("Line number out of range".into())
}

//write first cgm reading to patient table
pub fn first_write_cgm_reading_to_patient_table(
    conn: &Connection,
    patient_id: &str,
) -> Result<(), Box<dyn Error>> {
    let table_name = format!("cgm_data_user_{}", patient_id);
    let timestamp = Utc::now().naive_utc().to_string();
    let cgm_record = read_a_line_from_cgm_dataset("./data/cgm_dataset.csv", 0)?;
    let glucose_level = cgm_record.GlucoseValue;
    let trend = 0;
    let alert_generated = 0;
    let reading_date = Utc::now().naive_utc().to_string();

    conn.execute(
        &format!(
            "INSERT INTO {} (
                id,
                timestamp,
                glucose_level,
                trend,
                alert_generated,
                reading_date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            table_name
        ),
        params![
            patient_id,
            timestamp,
            glucose_level,
            trend,
            alert_generated,
            reading_date
        ],
    )?;
    Ok(())
}

//---------------------Write to table--------------------//
pub fn write_cgm_reading_to_patient_table(
    conn: &Connection,
    patient_id: &str,
    glucose_level: f32,
    last_line_read: usize,
) -> Result<usize, Box<dyn Error>> {
    // get table name
    let table_name = format!("cgm_data_user_{}", patient_id);

    // Prepare other fields
    let timestamp = Utc::now().naive_utc().to_string();
    let trend = 0; // Placeholder for trend data
    let alert_generated = 0; // Initially no alert generated
    let reading_date = Utc::now().naive_utc().to_string();

    // read next record from CGM dataset
    let cgm_record = read_a_line_from_cgm_dataset(
        "./data/cgm_dataset.csv",
        last_line_read + 1,
    )?;

    // get glucose level from record and insulin pump data
    let glucose_level = (cgm_record.GlucoseValue + glucose_level) / 2.0;

    // Insert the CGM reading into the patient's CGM table
    conn.execute(
        &format!(
            "INSERT INTO {} (
                id,
                timestamp,
                glucose_level,
                trend,
                alert_generated,
                reading_date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            table_name
        ),
        params![
            patient_id,
            timestamp,
            glucose_level,
            trend,
            alert_generated,
            reading_date
        ],
    )?;

    Ok(last_line_read + 1)
}

        
// Insert the CGM reading into the patient's CGM table