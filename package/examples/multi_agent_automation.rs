//! Multi-Agent Automation Example
//!
//! This example demonstrates how to use the LlamaClick multi-agent system
//! to automate a complex web task using AI-driven navigation and interaction.

use llamaclick::agent::{Agent, AgentConfig, AgentManager, AgentType};
use llamaclick::error::Result;
use llamaclick::llms::{create_provider, LlmProviderConfig, LlmProviderType};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    llamaclick::init_logging()?;
    
    println!("LlamaClick Multi-Agent Automation Example");
    println!("----------------------------------------");
    
    // Get API key from environment
    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable not set");
    
    // Create LLM provider configuration for each agent
    let planner_config = LlmProviderConfig::new(
        LlmProviderType::OpenAi,
        "gpt-4-1106-preview",
        &openai_api_key,
    );
    
    let navigator_config = LlmProviderConfig::new(
        LlmProviderType::OpenAi,
        "gpt-4-1106-preview",
        &openai_api_key,
    );
    
    let interactor_config = LlmProviderConfig::new(
        LlmProviderType::OpenAi,
        "gpt-4-1106-preview",
        &openai_api_key,
    );
    
    let verifier_config = LlmProviderConfig::new(
        LlmProviderType::OpenAi,
        "gpt-4-1106-preview",
        &openai_api_key,
    );
    
    let recovery_config = LlmProviderConfig::new(
        LlmProviderType::OpenAi,
        "gpt-4-1106-preview",
        &openai_api_key,
    );
    
    // Create providers for each agent
    let planner_provider = create_provider(planner_config)?;
    let navigator_provider = create_provider(navigator_config)?;
    let interactor_provider = create_provider(interactor_config)?;
    let verifier_provider = create_provider(verifier_config)?;
    let recovery_provider = create_provider(recovery_config)?;
    
    // Create agent configurations
    let planner_agent_config = AgentConfig::new(AgentType::Planner)
        .with_temperature(0.2)
        .with_system_message("You are a strategic planning agent that breaks down complex web automation tasks into specific, actionable steps. Focus on creating a clear sequence of actions that will achieve the overall objective efficiently.");
    
    let navigator_agent_config = AgentConfig::new(AgentType::Navigator)
        .with_temperature(0.3)
        .with_system_message("You are a web navigation specialist that understands page structure and can identify the optimal elements to interact with. Your expertise is in analyzing HTML and finding the most reliable selectors for elements.");
    
    let interactor_agent_config = AgentConfig::new(AgentType::Interactor)
        .with_temperature(0.1)
        .with_system_message("You are a precise interaction agent that executes browser interactions flawlessly. You handle clicks, form filling, scrolling, and other interactions with websites.");
    
    let verifier_agent_config = AgentConfig::new(AgentType::Verifier)
        .with_temperature(0.2)
        .with_system_message("You are a verification specialist that confirms actions have produced the expected outcomes. You analyze page content and state to determine if objectives have been met.");
    
    let recovery_agent_config = AgentConfig::new(AgentType::Recovery)
        .with_temperature(0.4)
        .with_system_message("You are a recovery expert that implements alternative strategies when primary approaches fail. You're resourceful and can find creative solutions to overcome obstacles.");
    
    // Create agents
    let planner_agent = Agent::new(planner_agent_config, planner_provider);
    let navigator_agent = Agent::new(navigator_agent_config, navigator_provider);
    let interactor_agent = Agent::new(interactor_agent_config, interactor_provider);
    let verifier_agent = Agent::new(verifier_agent_config, verifier_provider);
    let recovery_agent = Agent::new(recovery_agent_config, recovery_provider);
    
    // Create agent manager and add agents
    let mut agent_manager = AgentManager::new();
    agent_manager.add_agent(planner_agent);
    agent_manager.add_agent(navigator_agent);
    agent_manager.add_agent(interactor_agent);
    agent_manager.add_agent(verifier_agent);
    agent_manager.add_agent(recovery_agent);
    
    // Define a complex web automation objective
    let objective = "Search for the latest articles about artificial intelligence on TechCrunch, \
                     find the most recent article that mentions 'large language models', \
                     extract the title, author, date, and a brief summary of the article.";
    
    println!("\nObjective: {}", objective);
    println!("\nExecuting task with multi-agent system...\n");
    
    // Execute the task using the multi-agent system
    let result = agent_manager.execute_task(objective).await?;
    
    println!("\nTask completed. Result:");
    println!("----------------------");
    println!("{}", result);
    
    // You can also access the history of each agent to see their thought process
    println!("\nPlanner Agent History:");
    println!("--------------------");
    if let Some(agent) = agent_manager.get_agent(AgentType::Planner) {
        for (i, (prompt, response)) in agent.history().iter().enumerate() {
            println!("Step {}", i + 1);
            println!("Prompt: {}", prompt);
            println!("Response: {}", response);
            println!();
        }
    }
    
    Ok(())
} 