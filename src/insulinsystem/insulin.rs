///contains pump logic for insulin delivery

//when called by cgm module pump insulin based on glucose readings
//writes to insulin_alerts.log
//writes to patients cgm table

use rusqlite::{params, Connection, Result};
use crate::alerts;
use crate::db::queries;
use crate::insulinsystem::{logger,cgm};
use std::error::Error;

// insulin delivery based on clinical guidelines
pub fn process_insulin_delivery(
	conn: &Connection, 
	patient_id: &str, 
	glucose_level: f32, 
	timestamp: &str) -> Result<(), Box<dyn Error>> {
	// Log insulin delivery action
	if glucose_level > 180.0 {
		// get basal rate for patient
		let basal_rate = queries::get_basal_rate_for_patient(conn, patient_id)?; // fetch basal rate from DB
		// pump insulin based on basal rate
		let new_glucose_reading = pump_basal_insulin(glucose_level, basal_rate);
		// logger module to write insulin delivery to patient cgm table
		let last_line_read  = cgm::query_entry(conn, patient_id)?;
		logger::write_cgm_reading_to_patient_table(
			conn, 
			patient_id,
			new_glucose_reading,
			last_line_read)?;
		// generate alert for hyperglycemia
		alerts::generate_alert_hyper_pump (
			patient_id, 
			glucose_level, 
			basal_rate,
			timestamp)?;
	} else if glucose_level < 70.0 {
		// generate alert for hypoglycemia
		alerts::generate_alert_hypo_pump (
			patient_id, 
			glucose_level, 
			timestamp)?;
	} else {
		// nothing
	}
	// Update patient's CGM data to reflect insulin delivery
	update_alert_generated_field(conn, patient_id, timestamp)?;
	Ok(())
}

pub fn update_alert_generated_field(
	conn: &Connection,
	patient_id: &str,
	timestamp: &str) -> Result<()> {
	// Update the CGM reading to indicate insulin was delivered
	let table_name = format!("cgm_data_user_{}", patient_id);
	conn.execute(
		&format!(
			"UPDATE {} 
			SET alert_generated = 1 
			WHERE timestamp = ?1",
			table_name
		),
		params![timestamp],
	)?;
	// call moodule to log insulin delivery in insulin delivery table
	Ok(())
}

// pump basal insulin based on basal rate
pub fn pump_basal_insulin(
	current_insulin_level: f32,
	basal_rate: f32,
)-> f32 {
	(current_insulin_level - basal_rate)
}
// pump bolus insulin when requested
pub fn pump_bolus_insulin(
	current_insulin_level: f32,
	bolus_amount: f32,
)-> f32 {
	(current_insulin_level - bolus_amount)
}
