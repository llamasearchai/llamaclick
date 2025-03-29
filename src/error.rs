use thiserror::Error;

/// Custom result type for LlamaClick operations
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for LlamaClick
#[derive(Error, Debug)]
pub enum Error {
    /// Error with configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Error with browser automation
    #[error("Browser automation error: {0}")]
    BrowserError(String),

    /// Error with LLM API
    #[error("LLM API error: {0}")]
    LlmError(String),

    /// Error with LinkedIn automation
    #[error("LinkedIn automation error: {0}")]
    LinkedInError(String),

    /// Error with file I/O
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error with serialization/deserialization
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Error with TOML serialization/deserialization
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Error with HTTP requests
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// Playwright browser automation error
    #[error("Playwright error: {0}")]
    PlaywrightError(String),
    
    /// Network connectivity error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Rate limiting error
    #[error("Rate limit error: {0}")]
    RateLimitError(String),
    
    /// Security error
    #[error("Security error: {0}")]
    SecurityError(String),

    /// Generic error
    #[error("{0}")]
    GenericError(String),
}

/// Create a new configuration error
pub fn config_error<T: Into<String>>(message: T) -> Error {
    Error::ConfigError(message.into())
}

/// Create a new browser error
pub fn browser_error<T: Into<String>>(message: T) -> Error {
    Error::BrowserError(message.into())
}

/// Create a new LLM API error
pub fn llm_error<T: Into<String>>(message: T) -> Error {
    Error::LlmError(message.into())
}

/// Create a new LinkedIn automation error
pub fn linkedin_error<T: Into<String>>(message: T) -> Error {
    Error::LinkedInError(message.into())
}

/// Create a new generic error
pub fn generic_error<T: Into<String>>(message: T) -> Error {
    Error::GenericError(message.into())
} 