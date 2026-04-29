use web_sys::{Storage, Window};

/// Get access to the session storage
pub fn get_session_storage() -> Option<Storage> {
    web_sys::window()
        .and_then(|w: Window| w.session_storage().ok())
        .flatten()
}

/// Get a value from session storage
pub fn get_from_session(key: &str) -> Option<String> {
    get_session_storage()
        .and_then(|storage| storage.get(key).ok())
        .flatten()
}

/// Set a value in session storage
pub fn set_in_session(key: &str, value: &str) {
    if let Some(storage) = get_session_storage() {
        storage.set(key, value).ok();
    }
}

/// Remove a value from session storage
pub fn remove_from_session(key: &str) {
    if let Some(storage) = get_session_storage() {
        storage.delete(key).ok();
    }
}

/// Clear all session storage
pub fn clear_session() {
    if let Some(storage) = get_session_storage() {
        storage.clear().ok();
    }
}
