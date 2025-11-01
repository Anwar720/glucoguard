/*
Securely track logged-in users.
Associate each session with a unique token.
Support session expiration (time-based).
Store active sessions in memory (or optionally persist to disk)
*/

use std::collections::HashMap;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::Time;
use rand::RngCore;

use crate::user::User;

#[derive(Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub user: String,
    pub create_time: SystemTime,
    pub exp_time: Duration,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.create_time.elapsed().unwrap_or_default() > self.exp_time
    }
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new session for a user
    pub async fn create_session(&self, user: User) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Use user info and timestamp as input to the hash
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let session_id = hex::encode(bytes);

        // Create the Session
        let session = Session {
            session_id: session_id.clone(),
            user,
            create_time: SystemTime::now(),
            exp_time: Duration::from_secs(60 * 60), // 1 hour
        };

        // Store session in map
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        session_id
    }

    //get a valid session
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned().filter(|s| !s.is_expired())
    }

    // remove sessions by ID
    pub async fn clean_up_expired(&self) {
        let mut sessions = self.sessions.write().await;
        let expired_ids: Vec<String> = sessions
            .iter()
            .filter(|(_, s)| s.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        for id in expired_ids {
            sessions.remove(&id);
        }
    }
    //clean up sessions in the background
    pub fn start_cleanup_task(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));

            loop {
                interval.tick().await;
                self.clean_up_expired().await;
            }
        });
    }
}
