/*
Securely track logged-in users.
Associate each session with a unique token.
Support session expiration (time-based).
Store active sessions in memory (or optionally persist to disk)
*/

use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

use crate::user::User;

#[derive(Clone, Debug)]
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

    /// Create a new session for a user
    pub fn create_session(&self, user: User) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Use the user info and timestamp as input to the hash
        let base = format!("{}_{}_{}", user.name, now, user.password_hash);

        // Hash to get a unique, deterministic ID
        let mut hasher = Sha256::new();
        hasher.update(base);
        let hash_bytes = hasher.finalize();
        let session_id = format!("{:x}", hash_bytes);

        // Create the Session
        let session = Session {
            session_id: session_id.clone(),
            user,
            create_time: SystemTime::now(),
            exp_time: Duration::from_secs(60 * 60), // 1 hour
        };

        // Store session in map
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);

        session_id
    }


}