use web_sys::{Location, Storage, Window};
use log;

pub const JWT_STORAGE_KEY: &str = "kyc_portal_jwt";

/// Extract JWT token from URL query parameters or hash fragment
/// Stores it in SessionStorage if found
pub fn extract_jwt_from_url() -> Option<String> {
    let window: Window = match web_sys::window() {
        Some(w) => w,
        None => {
            log::warn!("No window object available");
            return None;
        }
    };

    // First try query parameters (?jwt=...)
    let location: Location = window.location();
    let href = location.href().unwrap_or_default();
    
    if let Some(token) = extract_token_from_url(&href) {
        store_jwt(&token);
        // Clean up the URL by removing the jwt parameter
        clean_url(&location, &href);
        return Some(token);
    }

    // Then try hash fragment (#jwt=...)
    let hash = location.hash().unwrap_or_default();
    if !hash.is_empty() {
        let url_with_hash = format!("{}{}", href.split('#').next().unwrap_or(&href), hash);
        if let Some(token) = extract_token_from_url(&url_with_hash) {
            store_jwt(&token);
            location.set_hash("").ok();
            return Some(token);
        }
    }

    // Return stored token if available
    get_stored_jwt()
}

fn extract_token_from_url(url: &str) -> Option<String> {
    // Parse query string for jwt parameter
    if let Some(query_start) = url.find('?') {
        let query = &url[query_start + 1..];
        let params: Vec<&str> = query.split('&').collect();
        for param in params {
            if let Some((key, value)) = param.split_once('=') {
                if key == "jwt" && !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    // Parse hash fragment for jwt parameter
    if let Some(hash_start) = url.find('#') {
        let hash = &url[hash_start + 1..];
        let params: Vec<&str> = hash.split('&').collect();
        for param in params {
            if let Some((key, value)) = param.split_once('=') {
                if key == "jwt" && !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    None
}

fn store_jwt(token: &str) {
    if let Some(storage) = get_session_storage() {
        storage.set(JWT_STORAGE_KEY, token).ok();
        log::info!("JWT token stored in SessionStorage");
    }
}

fn get_stored_jwt() -> Option<String> {
    get_session_storage().and_then(|storage| {
        storage.get(JWT_STORAGE_KEY).ok().and_then(|v| {
            if v.is_some() {
                v
            } else {
                None
            }
        })
    })
}

fn clean_url(location: &Location, original_url: &str) {
    if let Some(base_url) = original_url.split('?').next() {
        location.set_href(base_url).ok();
    }
}

fn get_session_storage() -> Option<Storage> {
    web_sys::window()
        .and_then(|w| w.session_storage().ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_token_from_query() {
        let url = "https://example.com/page?jwt=abc123&other=value";
        assert_eq!(extract_token_from_url(url), Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_token_from_hash() {
        let url = "https://example.com/page#jwt=xyz789&foo=bar";
        assert_eq!(extract_token_from_url(url), Some("xyz789".to_string()));
    }

    #[test]
    fn test_no_jwt_in_url() {
        let url = "https://example.com/page?other=value";
        assert_eq!(extract_token_from_url(url), None);
    }
}
