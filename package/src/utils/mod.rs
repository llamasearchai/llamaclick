//! Utilities module for LlamaClick
//!
//! This module provides basic utility functions.

use crate::error::{Error, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Get a timestamp as seconds since the epoch
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Measure the execution time of a function
pub fn measure_time<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Create a directory if it doesn't exist
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| Error::IoError(e))?;
    }
    Ok(())
}

/// Write content to a file
pub fn write_to_file(path: &Path, content: &str) -> Result<()> {
    let dir = path.parent().ok_or_else(|| Error::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Invalid path",
    )))?;
    
    ensure_dir(dir)?;
    
    let mut file = File::create(path).map_err(|e| Error::IoError(e))?;
    file.write_all(content.as_bytes()).map_err(|e| Error::IoError(e))?;
    
    Ok(())
}

/// Read content from a file
pub fn read_from_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| Error::IoError(e))
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Get the config directory
pub fn config_dir() -> Result<PathBuf> {
    let mut dir = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
    dir.push(".llamaclick");
    ensure_dir(&dir)?;
    Ok(dir)
}

/// Format a duration for human readability
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if seconds == 0 {
        format!("{}ms", millis)
    } else if seconds < 60 {
        format!("{}s {}ms", seconds, millis)
    } else {
        let minutes = seconds / 60;
        let rem_seconds = seconds % 60;
        format!("{}m {}s", minutes, rem_seconds)
    }
} 