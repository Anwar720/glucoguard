use std::time::{SystemTime, Duration};
use crate::db::queries;
use rusqlite::Connection;
use rand::RngCore;

/*
Securely track logged-in users.
Associate each session with a unique token.
Support session expiration (time-based).
Store active sessions in memory (or optionally persist to disk)
*/

//struct for sessoin
#[derive(Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub create_time: SystemTime,
    pub exp_time: Duration,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.create_time.elapsed().unwrap_or_default() > self.exp_time
    }
}

//session manager to manage session creation and cleanup
#[derive(Clone)]
pub struct SessionManager;

impl SessionManager {
    pub fn new() -> Self {
        Self
    }

    // Create a new session and persist it in the DB
    pub fn create_session(&self, conn: &Connection, user_id: String) -> rusqlite::Result<String> {
        // Generate a random session token
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let session_id = hex::encode(bytes);

        // Create session
        let session = Session {
            session_id: session_id.clone(),
            user_id,
            create_time: SystemTime::now(),
            exp_time: Duration::from_secs(60 * 60), // 1 hour
        };

        // Store directly in DB (no async)
        queries::add_session_to_db(conn, &session)?;

        Ok(session_id)
    }
    // Retrieve a session by username
    pub fn get_session_by_username(&self, conn: &Connection, user_id: &str) -> Option<Session> {
        match queries::get_session(conn, user_id) {
            Ok(Some(session)) if !session.is_expired() => Some(session),
            _ => None,
        }
    }

    // Retrieve a session by ID
    //if the session exists it is returned
    //if the session doesn't exist or expired
    //None is returned
    pub fn get_session_by_id(&self, conn: &Connection, session_id: &str) -> Option<Session> {
        match queries::get_session_by_id(conn, session_id) {
            Ok(Some(session)) if !session.is_expired() => Some(session),
            _ => None,
        }
    }

    // Remove a session manually
    pub fn remove_session(&self, conn: &Connection, session_id: &str) -> rusqlite::Result<()> {
        queries::remove_session(conn, session_id)
    }

    // Periodic cleanup task (removes expired sessions)

    pub fn cleanup_expired_sessions(&self, conn: &Connection) -> rusqlite::Result<()> {
        queries::remove_expired_sessions(conn)
    }

    // Run cleanup in a background thread every 60 seconds
    pub fn run_cleanup(&self, db_path: &str) {
        let db_path = db_path.to_string();
        std::thread::spawn(move || loop {
            match Connection::open(&db_path) {
                Ok(conn) => {
                    if let Err(e) = queries::remove_expired_sessions(&conn) {
                        eprintln!("Failed to cleanup expired sessions: {:?}", e);
                    }
                }
                Err(e) => eprintln!("Failed to open DB connection for cleanup: {:?}", e),
            }
            std::thread::sleep(Duration::from_secs(60));
        });
    }
}
