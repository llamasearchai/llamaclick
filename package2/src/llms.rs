//! LLM Provider module for LlamaClick
//!
//! This module provides interfaces and implementations for interacting with
//! various LLM providers like OpenAI, Anthropic, and local models.

use crate::error::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// Response from an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// The content of the response
    pub content: String,
    /// The model used for the response
    pub model: String,
    /// The duration of the request
    pub duration: Duration,
    /// The token usage
    pub token_usage: Option<TokenUsage>,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// The number of prompt tokens
    pub prompt_tokens: usize,
    /// The number of completion tokens
    pub completion_tokens: usize,
    /// The total number of tokens
    pub total_tokens: usize,
}

/// LLM provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync + fmt::Debug {
    /// Generate a response from the LLM
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse>;
    
    /// Get the model name
    fn model_name(&self) -> &str;
    
    /// Get the provider name
    fn provider_name(&self) -> &str;
}

/// LLM provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LlmProviderType {
    /// OpenAI provider
    OpenAi,
    /// Anthropic provider
    Anthropic,
    /// Local provider
    Local,
    /// Hugging Face provider
    HuggingFace,
    /// Azure OpenAI provider
    AzureOpenAi,
}

impl fmt::Display for LlmProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmProviderType::OpenAi => write!(f, "OpenAI"),
            LlmProviderType::Anthropic => write!(f, "Anthropic"),
            LlmProviderType::Local => write!(f, "Local"),
            LlmProviderType::HuggingFace => write!(f, "HuggingFace"),
            LlmProviderType::AzureOpenAi => write!(f, "Azure OpenAI"),
        }
    }
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    /// The provider type
    pub provider_type: LlmProviderType,
    /// The model name
    pub model: String,
    /// The API key
    pub api_key: String,
    /// The API endpoint
    pub api_endpoint: Option<String>,
    /// Additional configuration options
    pub options: HashMap<String, String>,
}

impl LlmProviderConfig {
    /// Create a new LLM provider configuration
    pub fn new(provider_type: LlmProviderType, model: &str, api_key: &str) -> Self {
        Self {
            provider_type,
            model: model.to_string(),
            api_key: api_key.to_string(),
            api_endpoint: None,
            options: HashMap::new(),
        }
    }
    
    /// Set the API endpoint
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.api_endpoint = Some(endpoint.to_string());
        self
    }
    
    /// Add an option
    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }
}

/// Create an LLM provider from a configuration
pub fn create_provider(config: LlmProviderConfig) -> Result<Box<dyn LlmProvider>> {
    match config.provider_type {
        LlmProviderType::OpenAi => Ok(Box::new(OpenAiProvider::new(config)?)),
        LlmProviderType::Anthropic => Ok(Box::new(AnthropicProvider::new(config)?)),
        LlmProviderType::Local => Ok(Box::new(LocalProvider::new(config)?)),
        LlmProviderType::HuggingFace => Ok(Box::new(HuggingFaceProvider::new(config)?)),
        LlmProviderType::AzureOpenAi => Ok(Box::new(AzureOpenAiProvider::new(config)?)),
    }
}

/// OpenAI provider
#[derive(Debug)]
pub struct OpenAiProvider {
    /// The configuration
    config: LlmProviderConfig,
    /// The HTTP client
    client: reqwest::Client,
}

impl OpenAiProvider {
    /// Create a new OpenAI provider
    pub fn new(config: LlmProviderConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::AuthenticationError("OpenAI API key is required".to_string()));
        }
        
        Ok(Self {
            config,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse> {
        let start = std::time::Instant::now();
        
        // Build the request payload
        let payload = serde_json::json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": "system",
                    "content": system
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": temperature,
        });
        
        // Get the API endpoint
        let endpoint = self.config.api_endpoint.as_deref().unwrap_or("https://api.openai.com/v1/chat/completions");
        
        // Send the request
        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Error::LlmError(format!("OpenAI API error: {}", error_text)));
        }
        
        // Parse the response
        let response_json: serde_json::Value = response.json().await?;
        
        // Extract the content
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::LlmError("Failed to extract content from OpenAI response".to_string()))?
            .to_string();
        
        // Extract token usage if available
        let token_usage = if let Some(usage) = response_json["usage"].as_object() {
            Some(TokenUsage {
                prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as usize,
                completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as usize,
                total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as usize,
            })
        } else {
            None
        };
        
        let duration = start.elapsed();
        
        Ok(LlmResponse {
            content,
            model: self.config.model.clone(),
            duration,
            token_usage,
        })
    }
    
    fn model_name(&self) -> &str {
        &self.config.model
    }
    
    fn provider_name(&self) -> &str {
        "OpenAI"
    }
}

/// Anthropic provider
#[derive(Debug)]
pub struct AnthropicProvider {
    /// The configuration
    config: LlmProviderConfig,
    /// The HTTP client
    client: reqwest::Client,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(config: LlmProviderConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::AuthenticationError("Anthropic API key is required".to_string()));
        }
        
        Ok(Self {
            config,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse> {
        let start = std::time::Instant::now();
        
        // Build the request payload
        let payload = serde_json::json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": "system",
                    "content": system
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": temperature,
            "max_tokens": 1024,
        });
        
        // Get the API endpoint
        let endpoint = self.config.api_endpoint.as_deref().unwrap_or("https://api.anthropic.com/v1/messages");
        
        // Send the request
        let response = self.client
            .post(endpoint)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Error::LlmError(format!("Anthropic API error: {}", error_text)));
        }
        
        // Parse the response
        let response_json: serde_json::Value = response.json().await?;
        
        // Extract the content
        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| Error::LlmError("Failed to extract content from Anthropic response".to_string()))?
            .to_string();
        
        let duration = start.elapsed();
        
        Ok(LlmResponse {
            content,
            model: self.config.model.clone(),
            duration,
            token_usage: None, // Anthropic doesn't provide token usage in the same way
        })
    }
    
    fn model_name(&self) -> &str {
        &self.config.model
    }
    
    fn provider_name(&self) -> &str {
        "Anthropic"
    }
}

/// Local provider for LLMs
#[derive(Debug)]
pub struct LocalProvider {
    /// The configuration
    config: LlmProviderConfig,
}

impl LocalProvider {
    /// Create a new local provider
    pub fn new(config: LlmProviderConfig) -> Result<Self> {
        if config.api_endpoint.is_none() {
            return Err(Error::ConfigurationError("Local provider requires an API endpoint".to_string()));
        }
        
        Ok(Self {
            config,
        })
    }
}

#[async_trait]
impl LlmProvider for LocalProvider {
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse> {
        let start = std::time::Instant::now();
        
        // For demonstration purposes, we're just echoing back the prompt
        // In a real implementation, this would connect to a local LLM server
        let content = format!("Local LLM response to: {}", prompt);
        
        let duration = start.elapsed();
        
        Ok(LlmResponse {
            content,
            model: self.config.model.clone(),
            duration,
            token_usage: None,
        })
    }
    
    fn model_name(&self) -> &str {
        &self.config.model
    }
    
    fn provider_name(&self) -> &str {
        "Local"
    }
}

/// HuggingFace provider
#[derive(Debug)]
pub struct HuggingFaceProvider {
    /// The configuration
    config: LlmProviderConfig,
    /// The HTTP client
    client: reqwest::Client,
}

impl HuggingFaceProvider {
    /// Create a new HuggingFace provider
    pub fn new(config: LlmProviderConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::AuthenticationError("HuggingFace API key is required".to_string()));
        }
        
        Ok(Self {
            config,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl LlmProvider for HuggingFaceProvider {
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse> {
        let start = std::time::Instant::now();
        
        // Build the request payload
        let full_prompt = format!("{}\n{}", system, prompt);
        let payload = serde_json::json!({
            "inputs": full_prompt,
            "parameters": {
                "temperature": temperature,
                "max_length": 1024,
            }
        });
        
        // Get the API endpoint
        let endpoint = self.config.api_endpoint.as_deref().unwrap_or(
            &format!("https://api-inference.huggingface.co/models/{}", self.config.model)
        );
        
        // Send the request
        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Error::LlmError(format!("HuggingFace API error: {}", error_text)));
        }
        
        // Parse the response
        let response_json: serde_json::Value = response.json().await?;
        
        // Extract the content - this depends on the specific model being used
        let content = match response_json.as_array() {
            Some(array) if !array.is_empty() => {
                array[0]["generated_text"]
                    .as_str()
                    .ok_or_else(|| Error::LlmError("Failed to extract content from HuggingFace response".to_string()))?
                    .to_string()
            },
            _ => response_json["generated_text"]
                .as_str()
                .ok_or_else(|| Error::LlmError("Failed to extract content from HuggingFace response".to_string()))?
                .to_string(),
        };
        
        let duration = start.elapsed();
        
        Ok(LlmResponse {
            content,
            model: self.config.model.clone(),
            duration,
            token_usage: None, // HuggingFace doesn't provide token usage
        })
    }
    
    fn model_name(&self) -> &str {
        &self.config.model
    }
    
    fn provider_name(&self) -> &str {
        "HuggingFace"
    }
}

/// Azure OpenAI provider
#[derive(Debug)]
pub struct AzureOpenAiProvider {
    /// The configuration
    config: LlmProviderConfig,
    /// The HTTP client
    client: reqwest::Client,
}

impl AzureOpenAiProvider {
    /// Create a new Azure OpenAI provider
    pub fn new(config: LlmProviderConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::AuthenticationError("Azure OpenAI API key is required".to_string()));
        }
        
        if config.api_endpoint.is_none() {
            return Err(Error::ConfigurationError("Azure OpenAI provider requires an API endpoint".to_string()));
        }
        
        Ok(Self {
            config,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl LlmProvider for AzureOpenAiProvider {
    async fn generate_response(&self, system: &str, prompt: &str, temperature: f32) -> Result<LlmResponse> {
        let start = std::time::Instant::now();
        
        // Build the request payload
        let payload = serde_json::json!({
            "messages": [
                {
                    "role": "system",
                    "content": system
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": temperature,
            "max_tokens": 800,
        });
        
        // Get the deployment name and endpoint
        let endpoint = self.config.api_endpoint.as_ref().unwrap();
        let deployment_name = &self.config.model;
        
        // Construct the full URL
        let url = format!("{}/openai/deployments/{}/chat/completions?api-version=2023-05-15", endpoint, deployment_name);
        
        // Send the request
        let response = self.client
            .post(&url)
            .header("api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Error::LlmError(format!("Azure OpenAI API error: {}", error_text)));
        }
        
        // Parse the response
        let response_json: serde_json::Value = response.json().await?;
        
        // Extract the content
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::LlmError("Failed to extract content from Azure OpenAI response".to_string()))?
            .to_string();
        
        // Extract token usage if available
        let token_usage = if let Some(usage) = response_json["usage"].as_object() {
            Some(TokenUsage {
                prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as usize,
                completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as usize,
                total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as usize,
            })
        } else {
            None
        };
        
        let duration = start.elapsed();
        
        Ok(LlmResponse {
            content,
            model: self.config.model.clone(),
            duration,
            token_usage,
        })
    }
    
    fn model_name(&self) -> &str {
        &self.config.model
    }
    
    fn provider_name(&self) -> &str {
        "Azure OpenAI"
    }
} 