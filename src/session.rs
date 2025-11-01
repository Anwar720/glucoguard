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
use tokio::time;

use crate::user::User;

#[derive(Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub user: User,
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
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        session_id
    }

    //get a session
    pub async fn get_session(&self, session_id: &str) -> Option<User> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get(session_id) {
            if session.is_expired() {
                sessions.remove(session_id);
                None
            } else {
                Some(session.user.clone())
            }
        } else {
            None
        }
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
    /// Invalidate a single session (logout)
    pub fn invalidate_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Invalidate all sessions for a given user (e.g., password change)
    pub fn invalidate_user_sessions(&self, username: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, s| s.user.name != username);
    }

    /// Invalidate all sessions (e.g., system-wide security breach)
    pub fn invalidate_all(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.clear();
    }

    /// Cleanup expired or idle sessions
    pub fn cleanup_expired_and_idle(&self, max_idle: Duration) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, s| !s.is_expired() && !s.is_inactive(max_idle));
    }
}