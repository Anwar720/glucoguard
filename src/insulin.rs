//Insulin Delivery Logic
//dependencies
use std::time::{SystemTime};
use std::thread;

const SAFE_LOWER: f32 = 70;
const SAFE_UPPER: f32 = 180;

//--------------------Ensure Sensor is Operational-----------------//
//-----------------------------------------------------------------//

//---------------Callibrate sensor readings to patient-------------//
//-----------------------------------------------------------------//

//-----------------Sensor gets and analyzes readings---------------//
//-----------------------------------------------------------------//
//store insulin reading and time reading was taken
struct Insulin_Level{
	//insulin level measured in mg/dL
	let level : f32,
	//timestamp
	let now = System::time();
}

//function to calculate rate of change of
//insulin levels in the body and returns
//rate in mg/dL per seconds
fn rate_of_change_insullin(rdg_one: &Insulin_Level, rdg_two: &Insulin_Level) -> f32{
	//subtract reading 1 and reading 2
	let r_change = rdg_two.level - rdg_one.level;
	//get elapsed time in seconds
	let elapsed = rdg_two.now - rdg_one.now;
	let elapsed = elapsed.as_secs(f32);

	(r_change/elapsed)
}

//function to get glucose readings
//run as a background process
fn start_readings(){
	//vector to store insulin readings
	let mut insulin_vector:Vec<Insulin_Level> = Vec::new();
	//continually get glucose readings
	//in a background process
	thread::spawn(||{
		'glucose_readings: loop{
			insulin_vector.push(val);
		}
	})
}

//-----------Generate alerts based on hardware or software---------//
//-----------------------------------------------------------------//
//function to generate alerts when
//1. glucose is high
//2. glucose is low
fn generate_alerts(glucose: &f32){
	if glucose>SAFE_UPPER{
		println!("Hyperglyceme Alert!: Glucose is at {} mg/dL", glucose);
	}
	if glucose<SAFE_Lower{
		println!("Hypoglycemea Alert!: Glucose is at {} mg/dL", glucose);
	}
}

//dependecies

//pump object to simulate pump features
struct Pump{
	insulin_cart_units; u16;
	insulin_cart_inserted: bool;
	battery_level: u8;
	pump_inserted: bool;
	needle_inserted: bool;

	//sensor connection
		//bool
		//sensor name
		//sensor address
}
//---------Check Pump is Operational---------//
//-------------------------------------------//
//methods to check pump components are 
//connected and operating correctly
impl Pump{
	fn check_insulin_cart(){

	}
	fn check_pump_inserted(){

	}
	fn check_pump_battery(){

	}
	fn check_sensor_connection(){

	}
}

//-------------Pump Insulin-------------//
//--------------------------------------//
//pump - command from cgm
//basal
fn pump_basal(units: &u8){
	//
}
//bolus
fn pump_bolus(units: &u8){
	//
}

//stop pump - command from cgm
fn stop_pump(){
	//
}
//transmit state

//-------------Generate Alerts-------------//
//-----------------------------------------//
//alert
//1.low insulin in pump
//2.low batter in pump
fn low_insulin_alert(pump_units: &u16, battery_level: &u8){
	if pump_units < 50{
		println!("Low pump units: {} units remaining", pump_units);
	}

	if battery_level < 25{
		println!("Low battery: {}%", battery_level);
	}
}

//dependencies

//user profiles
struct Clinician{};
struct Patient{};
struct Caretaker{};

//authenticating credentials

//-----------------Login------------------//

//--------------UI Features---------------//
//Store CGM readings
//View//
	//glucose readings
	//change in glucose levels
//Communicate with CGM//
//Logs//
	//user
		//requests
			//basal rate
			//bolus injections
		//updates
			//caretaker information
			//user information
			//clinician information
			//basal rate
			//bolus injections
	//cgm
		//functionality
		//glucose readings
		//glucose alerts
	//pump
		//functionality
		//alerts
//Alerts//
	//receive//
	//generate//