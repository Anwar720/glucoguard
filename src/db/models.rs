// core data models for database interaction

#[derive(Debug)]
pub struct User{
    pub id: String,
    pub user_name: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
    pub last_login: Option<String>
}
pub struct Patient{
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
pub struct PatientCareTeam{
    care_taker_id: i32,
    patient_id_list: Vec<i32>
}
pub struct GlucoseReading{
    reading_id: i32,
    patient_id: i32,
    glucose_level: f32,
    reading_time: String,
    status: String
}
pub struct InsulinLog{
    dosage_id: i32,
    patient_id: i32,
    action_type: String,
    dosage_units: f32,
    requested_by: String,
    dosage_time: String
}
pub struct Alerts{
    alert_id: i32,
    patient_id: i32,
    alert_type: String,
    alert_message: String,
    alert_time: String,
    is_resolved: bool,
    resolved_by: Option<String>,
}
pub struct MealLog{
    meal_id: i32,
    patient_id: i32,
    carbohydrate_amount: f32,
    meal_time: String
}
pub struct Session{
    session_id: i32,
    user_id: i32,
    creation_time: String,
    expiration_time: Option<String>
}