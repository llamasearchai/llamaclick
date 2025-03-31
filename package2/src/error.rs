//! Error handling for LlamaClick
//!
//! This module defines the error types used throughout the LlamaClick library.

use std::fmt;
use std::io;
use thiserror::Error;

/// Result type for LlamaClick operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for LlamaClick
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Browser error
    #[error("Browser error: {0}")]
    BrowserError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// LLM error
    #[error("LLM error: {0}")]
    LlmError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// Timeout error
    #[error("Timeout error: {0}")]
    TimeoutError(String),

    /// Permission error
    #[error("Permission error: {0}")]
    PermissionError(String),

    /// Agent error
    #[error("Agent error: {0}")]
    AgentError(String),

    /// Workflow error
    #[error("Workflow error: {0}")]
    WorkflowError(String),

    /// UI error
    #[error("UI error: {0}")]
    UiError(String),

    /// CLI error
    #[error("CLI error: {0}")]
    CliError(String),

    /// Script error
    #[error("Script error: {0}")]
    ScriptError(String),

    /// Module not found
    #[error("Module not found: {0}")]
    ModuleNotFound(String),
    
    /// Generic error
    #[error("{0}")]
    GenericError(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::DeserializationError(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::GenericError(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::GenericError(err)
    }
}

/// Helper functions for error handling
pub mod helpers {
    use super::{Error, Result};
    
    /// Convert any error that implements `std::fmt::Display` to an `Error`
    pub fn to_error<E: std::fmt::Display>(err: E) -> Error {
        Error::GenericError(err.to_string())
    }
    
    /// Convert any error that implements `std::fmt::Display` to a `Result`
    pub fn to_result<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
        result.map_err(|e| Error::GenericError(e.to_string()))
    }
}

/// Create a new configuration error
pub fn config_error<T: Into<String>>(message: T) -> Error {
    Error::ConfigurationError(message.into())
}

/// Create a new browser error
pub fn browser_error<T: Into<String>>(message: T) -> Error {
    Error::BrowserError(message.into())
}

/// Create a new LLM error
pub fn llm_error<T: Into<String>>(message: T) -> Error {
    Error::LlmError(message.into())
}

/// Create a new generic error
pub fn generic_error<T: Into<String>>(message: T) -> Error {
    Error::GenericError(message.into())
} 