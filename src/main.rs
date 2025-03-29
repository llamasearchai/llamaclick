mod cli;
mod config;
mod core;
mod error;
mod linkedin;
mod llms;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use error::Result;
use utils::ascii_art;
use utils::output::{print_error, print_info, print_rainbow};

/// The main entry point for the LlamaClick application
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    
    // Display colorful LlamaClick header
    ascii_art::print_llama_ascii_art();
    print_rainbow("🦙 LlamaClick - The Intelligent Web Automation Tool 🌈");
    println!();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Load settings
    let settings = match config::load_settings() {
        Ok(s) => s,
        Err(e) => {
            print_error(&format!("Failed to load settings: {}", e));
            print_info("Using default settings.");
            config::settings::Settings::default()
        }
    };
    
    // Check if this is the first run and no API keys are set
    let first_run = is_first_run(&settings);
    if first_run {
        print_info("Welcome to LlamaClick! It looks like this is your first time running the application.");
        print_info("Let's set up your LLM API keys and configuration.");
        cli::apikeys::run(&settings).await?;
    }
    
    // Match and execute the appropriate command
    match cli.command {
        Commands::Install { force } => {
            cli::install::run(force).await?;
        },
        Commands::Run { url, objective, headless } => {
            // Check if we have an API key for the configured provider
            if !has_valid_llm_config(&settings) {
                print_error("No valid LLM configuration found. Please configure your API keys first.");
                print_info("Running API Key Management interface...");
                cli::apikeys::run(&settings).await?;
                
                // Reload settings after configuration
                let updated_settings = config::load_settings()?;
                cli::run::run(url, objective, headless, &updated_settings).await?;
            } else {
                cli::run::run(url, objective, headless, &settings).await?;
            }
        },
        Commands::Demo { port } => {
            cli::demo::run(port, &settings).await?;
        },
        Commands::Test { pattern } => {
            cli::test::run(pattern).await?;
        },
        Commands::Config { subcommand } => {
            cli::config::run(subcommand, &settings).await?;
        },
        Commands::ApiKeys => {
            cli::apikeys::run(&settings).await?;
        },
        Commands::LinkedIn { subcommand } => {
            cli::linkedin::run(subcommand, &settings).await?;
        },
    }
    
    println!();
    print_rainbow("Thanks for using LlamaClick! 🦙");
    
    Ok(())
}

/// Determines if this is the first run of the application
fn is_first_run(settings: &config::settings::Settings) -> bool {
    // Check if we have any API keys configured
    let no_api_keys = settings.llm.api_key.is_empty() && settings.llm.anthropic_api_key.is_empty();
    
    // For simplicity, we'll consider it a first run if no API keys are set
    // We could check for Ollama availability, but that requires async
    no_api_keys
}

/// Checks if we have a valid LLM configuration
fn has_valid_llm_config(settings: &config::settings::Settings) -> bool {
    match settings.llm.provider.to_lowercase().as_str() {
        "openai" => !settings.llm.api_key.is_empty(),
        "anthropic" => !settings.llm.anthropic_api_key.is_empty(),
        "ollama" => true, // Ollama doesn't require an API key
        _ => false,
    }
} 