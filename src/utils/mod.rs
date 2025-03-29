pub mod ascii_art;
pub mod crypto;
pub mod output;

/// Get the current timestamp as an ISO 8601 formatted string
pub fn get_iso_timestamp() -> String {
    use chrono::Utc;
    Utc::now().to_rfc3339()
}

/// Generate a random string of specified length
pub fn random_string(length: usize) -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Check if a URL is valid
pub fn is_valid_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(url) => url.scheme() == "http" || url.scheme() == "https",
        Err(_) => false,
    }
}

/// Returns the user's home directory
pub fn get_home_dir() -> Option<std::path::PathBuf> {
    dirs::home_dir()
}

/// Generate a UUID string
pub fn generate_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
} 