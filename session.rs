/*
Securely track logged-in users.
Associate each session with a unique token.
Support session expiration (time-based).
Store active sessions in memory (or optionally persist to disk)
*/

use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use rand::{distributions::Alphanumeric, Rng};
use std::sync{Arc, Mutex};

use crate::user::User;

#[derive(clone, Debug)]
pub struct Session {
	pub sessionID: String,
	pub userID: User,
	pub create_time: SystemTime,
	pub exp_time: Duration,
}

impl Session {
	pub fn is_expired(&self) -> bool{
		self.create_time.elapsed().unwrap_or_default() > self.exp_time
	}
}

#[derive(Clone)]
pub struct SessionManager {
	session: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionManager {
	pub fn new() -> Self {
		Self {
			session: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	///create a new session for a user
	pub fn create_session(&self, user: User) -> String {
		let sessionID: String = rand::thread_rng()
			.sample_tier(&Alphanumeric)
			.take(32)
			.map(char::from)
			.collect();
	}

	let session = Session {
		sessionID: sessionID.clone(),
		user,
		create_time: SystemTime::now(),
		//1 hour from creation time
		exp_time: Duratione::from_sec(60*60),
	};

	self.sessions.lock().unwrap.insert(sessionID.clone(), session);
	
	sessionID

	//validate a session

	//destroy a session
}