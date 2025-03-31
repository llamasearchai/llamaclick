//! Agent module for LlamaClick
//!
//! This module provides the agent functionality for LlamaClick, implementing
//! a multi-agent architecture for planning, navigation, interaction, and recovery.

use crate::error::{Error, Result};
use crate::llms::{LlmProvider, LlmResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The type of agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// Planning agent responsible for breaking down high-level objectives
    Planner,
    /// Navigation agent for understanding web page structure
    Navigator,
    /// Interaction agent for executing UI interactions
    Interactor,
    /// Verification agent for confirming actions had expected effects
    Verifier,
    /// Recovery agent for implementing recovery strategies
    Recovery,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Planner => write!(f, "Planner"),
            AgentType::Navigator => write!(f, "Navigator"),
            AgentType::Interactor => write!(f, "Interactor"),
            AgentType::Verifier => write!(f, "Verifier"),
            AgentType::Recovery => write!(f, "Recovery"),
        }
    }
}

/// Configuration for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// The type of agent
    pub agent_type: AgentType,
    /// The prompt template for the agent
    pub prompt_template: String,
    /// The system message for the agent
    pub system_message: String,
    /// The temperature for the agent's LLM
    pub temperature: f32,
    /// Additional parameters for the agent
    pub parameters: HashMap<String, String>,
}

impl AgentConfig {
    /// Create a new agent configuration
    pub fn new(agent_type: AgentType) -> Self {
        let (system_message, prompt_template) = match agent_type {
            AgentType::Planner => (
                "You are a Planning Agent that breaks down high-level objectives into specific steps.".to_string(),
                "Break down the following objective into specific steps: {objective}".to_string(),
            ),
            AgentType::Navigator => (
                "You are a Navigation Agent that understands web page structure and identifies optimal paths.".to_string(),
                "Analyze the following page and identify the best elements to interact with to achieve: {objective}".to_string(),
            ),
            AgentType::Interactor => (
                "You are an Interaction Agent that executes precise UI interactions.".to_string(),
                "Execute the following interaction: {interaction}".to_string(),
            ),
            AgentType::Verifier => (
                "You are a Verification Agent that confirms actions had the expected outcomes.".to_string(),
                "Verify if the following action produced the expected outcome: {action} -> {expected_outcome}".to_string(),
            ),
            AgentType::Recovery => (
                "You are a Recovery Agent that implements recovery strategies when actions fail.".to_string(),
                "Implement a recovery strategy for the following failed action: {failed_action}".to_string(),
            ),
        };

        Self {
            agent_type,
            prompt_template,
            system_message,
            temperature: 0.7,
            parameters: HashMap::new(),
        }
    }

    /// Set the prompt template for the agent
    pub fn with_prompt_template(mut self, prompt_template: impl Into<String>) -> Self {
        self.prompt_template = prompt_template.into();
        self
    }

    /// Set the system message for the agent
    pub fn with_system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = system_message.into();
        self
    }

    /// Set the temperature for the agent's LLM
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    /// Add a parameter to the agent configuration
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
}

/// A single agent in the multi-agent system
#[derive(Debug)]
pub struct Agent {
    /// The agent's configuration
    config: AgentConfig,
    /// The LLM provider for the agent
    llm: Box<dyn LlmProvider>,
    /// The conversation history for the agent
    history: Vec<(String, String)>,
}

impl Agent {
    /// Create a new agent
    pub fn new(config: AgentConfig, llm: Box<dyn LlmProvider>) -> Self {
        Self {
            config,
            llm,
            history: Vec::new(),
        }
    }

    /// Run the agent with the given input
    pub async fn run(&mut self, input: &str) -> Result<String> {
        // Format the prompt using the template and input
        let prompt = self.config.prompt_template.replace("{objective}", input);
        
        // Get the response from the LLM
        let response = self.llm.generate_response(&self.config.system_message, &prompt, self.config.temperature).await?;
        
        // Add the interaction to the history
        self.history.push((prompt, response.content.clone()));
        
        Ok(response.content)
    }

    /// Clear the agent's conversation history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get the agent's conversation history
    pub fn history(&self) -> &[(String, String)] {
        &self.history
    }

    /// Get the agent's type
    pub fn agent_type(&self) -> AgentType {
        self.config.agent_type
    }
}

/// Manager for multi-agent system
#[derive(Debug)]
pub struct AgentManager {
    /// The agents in the system
    agents: HashMap<AgentType, Agent>,
}

impl AgentManager {
    /// Create a new agent manager
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Add an agent to the manager
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.insert(agent.agent_type(), agent);
    }

    /// Get an agent by type
    pub fn get_agent(&self, agent_type: AgentType) -> Option<&Agent> {
        self.agents.get(&agent_type)
    }

    /// Get a mutable reference to an agent by type
    pub fn get_agent_mut(&mut self, agent_type: AgentType) -> Option<&mut Agent> {
        self.agents.get_mut(&agent_type)
    }

    /// Execute a task using the multi-agent system
    pub async fn execute_task(&mut self, objective: &str) -> Result<String> {
        // Use the planner to break down the objective
        let planner = self.get_agent_mut(AgentType::Planner)
            .ok_or_else(|| Error::GenericError("Planner agent not found".to_string()))?;
        
        let plan = planner.run(objective).await?;
        
        // Use the navigator to identify elements
        let navigator = self.get_agent_mut(AgentType::Navigator)
            .ok_or_else(|| Error::GenericError("Navigator agent not found".to_string()))?;
        
        let navigation = navigator.run(&plan).await?;
        
        // Use the interactor to execute the interactions
        let interactor = self.get_agent_mut(AgentType::Interactor)
            .ok_or_else(|| Error::GenericError("Interactor agent not found".to_string()))?;
        
        let interaction_result = interactor.run(&navigation).await?;
        
        // Use the verifier to confirm the outcome
        let verifier = self.get_agent_mut(AgentType::Verifier)
            .ok_or_else(|| Error::GenericError("Verifier agent not found".to_string()))?;
        
        let verification = verifier.run(&interaction_result).await?;
        
        // If verification failed, use the recovery agent
        if verification.contains("failed") || verification.contains("unsuccessful") {
            let recovery = self.get_agent_mut(AgentType::Recovery)
                .ok_or_else(|| Error::GenericError("Recovery agent not found".to_string()))?;
            
            let recovery_result = recovery.run(&interaction_result).await?;
            return Ok(recovery_result);
        }
        
        Ok(verification)
    }

    /// Clear history for all agents
    pub fn clear_all_history(&mut self) {
        for agent in self.agents.values_mut() {
            agent.clear_history();
        }
    }
} 