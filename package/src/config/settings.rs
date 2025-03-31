use serde::{Deserialize, Serialize};

/// Settings for LlamaClick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// LLM API settings
    pub llm: LlmSettings,
    /// Browser settings
    pub browser: BrowserSettings,
    /// Agent settings
    pub agent: AgentSettings,
    /// Telemetry settings
    pub telemetry: TelemetrySettings,
}

/// LLM API settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    /// LLM provider to use (openai, anthropic, ollama)
    pub provider: String,
    /// Model name to use
    pub model: String,
    /// API key for OpenAI
    pub api_key: String,
    /// API key for Anthropic
    pub anthropic_api_key: String,
    /// URL for Ollama server
    pub ollama_url: String,
    /// Model for Anthropic
    pub anthropic_model: String,
    /// Model for Ollama
    pub ollama_model: String,
}

/// Browser settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    /// Timeout for browser operations in seconds
    pub timeout_seconds: u32,
    /// Whether to run the browser in headless mode
    pub headless: bool,
    /// Whether to take screenshots
    pub screenshots: bool,
    /// Browser automation driver to use
    pub driver_type: String,
}

/// Agent settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    /// Time between actions in milliseconds
    pub time_between_actions_ms: u32,
    /// Maximum number of steps to take
    pub max_steps: u32,
}

/// Telemetry settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySettings {
    /// Whether to send anonymous telemetry data
    pub enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            llm: LlmSettings::default(),
            browser: BrowserSettings::default(),
            agent: AgentSettings::default(),
            telemetry: TelemetrySettings::default(),
        }
    }
}

impl Default for LlmSettings {
    fn default() -> Self {
        LlmSettings {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            api_key: "".to_string(),
            anthropic_api_key: "".to_string(),
            ollama_url: "http://localhost:11434".to_string(),
            anthropic_model: "claude-3-haiku-20240307".to_string(),
            ollama_model: "llama3".to_string(),
        }
    }
}

impl Default for BrowserSettings {
    fn default() -> Self {
        BrowserSettings {
            timeout_seconds: 30,
            headless: false,
            screenshots: true,
            driver_type: "playwright".to_string(),
        }
    }
}

impl Default for AgentSettings {
    fn default() -> Self {
        AgentSettings {
            time_between_actions_ms: 500,
            max_steps: 50,
        }
    }
}

impl Default for TelemetrySettings {
    fn default() -> Self {
        TelemetrySettings {
            enabled: false,
        }
    }
} 