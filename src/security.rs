use tracing::{event, Level};

pub fn login_attempt(username: &str) {
    event!(target: "security", Level::WARN, kind = "auth", action = "login_attempt", username);
}

pub fn login_success(user_id: &str, role: &str, session_id: &str) {
    event!(target: "security", Level::INFO, kind = "auth", action = "login_success", user_id, role, session_id);
}

pub fn login_failure(username: &str) {
    event!(target: "security", Level::WARN, kind = "auth", action = "login_failure", username);
}

pub fn logout(user_id: &str, role: &str, session_id: &str) {
    event!(target: "security", Level::INFO, kind = "auth", action = "logout", user_id, role, session_id);
}

pub fn session_created(user_id: &str, role: &str, session_id: &str) {
    event!(target: "security", Level::INFO, kind = "session", action = "created", user_id, role, session_id);
}

pub fn session_removed(session_id: &str) {
    event!(target: "security", Level::INFO, kind = "session", action = "removed", session_id);
}

pub fn session_expired(session_id: &str) {
    event!(target: "security", Level::WARN, kind = "session", action = "expired", session_id);
}

pub fn permission_denied(user_id: &str, role: &str, permission: &str) {
    event!(target: "security", Level::WARN, kind = "authz", action = "permission_denied", user_id, role, permission);
}

pub fn account_created(new_user_id: &str, role: &str) {
    event!(target: "security", Level::INFO, kind = "account", action = "created", user_id = new_user_id, role);
}

pub fn activation_code_issued(issuer_id: &str, new_account_type: &str) {
    event!(target: "security", Level::INFO, kind = "activation", action = "issued", issuer_id, new_account_type);
}

pub fn activation_code_used(user_id: &str, account_type: &str) {
    event!(target: "security", Level::INFO, kind = "activation", action = "used", user_id, account_type);
}

pub fn db_error(operation: &str, error_message: &str) {
    event!(target: "security", Level::ERROR, kind = "db", action = "error", operation, error = %error_message);
}


