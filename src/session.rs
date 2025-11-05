use std::time::{SystemTime, Duration};
use crate::db::queries;
use rusqlite::Connection;
use rand::RngCore;
use crate::access_control::{Role, Permission};

/*
Securely track logged-in users.
Associate each session with a unique token.
Support session expiration (time-based).
Store active sessions in memory
*/

//struct for sessoin
#[derive(Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub role : String,
    pub create_time: SystemTime,
    pub exp_time: Duration,
}
//impl methods for session
impl Session {
    // Check if session has expired
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
    //-------------------------Session Management Methods-------------------------//
    // Create a new session and persist it in the DB
    pub fn create_session(
        &self, 
        conn: &Connection, 
        user_id: String, 
        role: String) -> rusqlite::Result<String> {
        //check if user already has an active session
        // if so remove it
        if let Some(existing_session) = queries::get_session(conn, &user_id)? {
            queries::remove_session(conn, &existing_session.session_id)?;
        };

        // Generate a random session token
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let session_id = hex::encode(bytes);

        // Create session
        let session = Session {
            session_id: session_id.clone(),
            user_id,
            role,
            create_time: SystemTime::now(),
            exp_time: Duration::from_secs(60 * 60), // 1 hour
        };

        // Store directly in DB (no async)
        queries::add_session_to_db(conn, &session)?;
        Ok(session_id)
    }
    // Retrieve a session by username
    pub fn get_session_by_username(&self, 
    conn: &Connection, 
    user_id: &str) -> Option<Session> {
        match queries::get_session(conn, user_id) {
            Ok(Some(session)) if !session.is_expired() => Some(session),
            _ => None,
        }
    }
    // Retrieve a session by ID
    pub fn get_session_by_id(
        &self, 
        conn: &Connection, 
        session_id: &str) -> Option<Session> {
        match queries::get_session_by_id(conn, session_id) {
            Ok(Some(session)) if !session.is_expired() => Some(session),
            _ => None,
        }
    }
    // Remove a session manually
    pub fn remove_session(
    &self,
    conn: &Connection,
    session_id: &str,
) -> rusqlite::Result<()> {
    match queries::get_session_by_id(conn, session_id)? {
        Some(session) => {
            if session.is_expired() {
                println!("Session already expired");
            } else {
                println!("Removing active session");
            }
            queries::remove_session(conn, session_id)?;
        }
        None => {
            println!("Session not found");
        }
    }
    Ok(())
}
    // Periodic cleanup task (removes expired sessions)
    pub fn cleanup_expired_sessions(
        &self, 
        conn: &Connection) -> rusqlite::Result<()> {
        // Call the query to remove expired sessions
        queries::remove_expired_sessions(conn)
    }

    // Run cleanup in a background thread every 60 seconds
    pub fn run_cleanup(&self, db_path: &str) {
        //clone db_path for thread
        let db_path = db_path.to_string();
        //create a new thread to rmove expired sessions
        std::thread::spawn(move || loop {
            //open a new connection to the db
            match Connection::open(&db_path) {
                Ok(conn) => {
                    //remove expired sessions by calling remove_expired_sessions
                    if let Err(e) = queries::remove_expired_sessions(&conn) {
                        eprintln!("Failed to cleanup expired sessions: {:?}", e);
                    }
                }
                Err(e) => eprintln!("Failed to open DB connection for cleanup: {:?}", e),
            }
            std::thread::sleep(Duration::from_secs(60));
        });
    }
    //-------------------------Access Control Methods-------------------------//
    /* Access managed 
    through session manager
    Check user permissions
    */
    //check access permissions for a given session and role
    pub fn check_access(
    &self,
    conn: &Connection,
    session_id: &str,
    ) -> bool {
        match queries::get_session_by_id(conn, session_id) {
            Ok(Some(session)) => {
                // Check if session expired
                if session.is_expired() {
                    println!("Session expired");
                    queries::remove_session(conn, session_id).ok(); 
                    return false;
                }

                // Build Role from trusted session role string
                let role = Role::new(&session.role, "");
                if role.permissions.is_empty() {
                    println!("Unknown role '{}'", session.role);
                    return false;
                }
                //check if role has any permissions
                if role.permissions.is_empty() {
                    println!("Role '{}' has no permissions", role.name);
                    return false;
                }
                // Valid session and role
                true
            }
            Ok(None) => {
                println!("Invalid or missing session");
                false
            }
            Err(e) => {
                eprintln!("Database error checking session: {}", e);
                false
            }
        }
    }

    //check if user has specific permission for action
    pub fn has_permission(
        &self,
        conn: &Connection,
        session_id: &str,
        req_permission: Permission,
        ) -> bool {
            match queries::get_session_by_id(conn, session_id) {
            Ok(Some(session)) => {
                if session.is_expired() {
                    println!("Session expired");
                    queries::remove_session(conn, session_id).ok();
                    return false;
                }
                // Create role from session role string (trusted source)
                let role = Role::new(&session.role, "");
                role.has_permission(&req_permission)
            }
            Ok(None) => {
                println!("Invalid or missing session");
                false
            }
            Err(e) => {
                eprintln!("Database error checking session: {}", e);
                false
            }
        }
    }

    //check if the user has the rights to complete action
    pub fn check_permissions(
    &self,
    conn: &Connection,
    session_id: &str,
    req_permission: Permission,
    ) -> bool {
        match queries::get_session_by_id(conn, session_id) {
            Ok(Some(session)) => {
                if session.is_expired() {
                    println!("Session expired");
                    return false;
                }

                // Create role from session role string (trusted source)
                let role = Role::new(&session.role, "");
                role.has_permission(&req_permission)
            }
            Ok(None) => {
                println!("Invalid or missing session");
                false
            }
            Err(e) => {
                eprintln!("Database error checking session: {}", e);
                false
            }
        }
    }
}