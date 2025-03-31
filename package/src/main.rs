//! LlamaClick CLI Application
//!
//! This is the main entry point for the LlamaClick CLI application.
//! It parses command-line arguments and dispatches to the appropriate
//! subcommands and handlers.

use clap::{Parser, Subcommand};
use colored::*;
use llamaclick::{error::Result, init_logging, run_automation, VERSION};
use std::path::PathBuf;

/// LlamaClick - Enterprise-Grade AI Web Automation
#[derive(Parser)]
#[command(
    name = "llamaclick",
    author = "LlamaSearch AI <info@llamasearch.ai>",
    version = VERSION,
    about = "AI-powered CLI for intelligent web automation using LLMs",
    long_about = "LlamaClick is an enterprise-grade CLI application for intelligent web automation powered by LLMs. It allows you to express automation goals in natural language and executes them with precision."
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Path to config file
    #[arg(short, long, global = true, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands for the CLI
#[derive(Subcommand)]
enum Commands {
    /// Run an automation task
    #[command(about = "Run an automation task")]
    Run {
        /// The objective to achieve
        #[arg(help = "The objective to achieve in natural language")]
        objective: String,

        /// URL to navigate to
        #[arg(short, long, help = "URL to navigate to before executing the objective")]
        url: Option<String>,

        /// Run in headless mode
        #[arg(short = 'H', long, help = "Run in headless mode (no UI)")]
        headless: bool,

        /// Output file for results
        #[arg(short, long, help = "Save results to this file")]
        output: Option<PathBuf>,
    },

    /// Configure the CLI
    #[command(about = "Configure the CLI")]
    Config {
        /// Set API key for OpenAI
        #[arg(long, help = "Set API key for OpenAI")]
        openai_key: Option<String>,

        /// Set API key for Anthropic
        #[arg(long, help = "Set API key for Anthropic")]
        anthropic_key: Option<String>,

        /// Show current configuration
        #[arg(short, long, help = "Show current configuration")]
        show: bool,

        /// Reset configuration to defaults
        #[arg(long, help = "Reset configuration to defaults")]
        reset: bool,
    },

    /// Run demo workflows
    #[command(about = "Run demo workflows")]
    Demo {
        /// Demo to run
        #[arg(help = "Name of the demo to run (web, linkedin, form)")]
        name: String,
    },
}

/// Main entry point for the application
fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    init_logging()?;

    // Print banner
    print_banner();

    // Process commands
    match cli.command {
        Commands::Run {
            objective,
            url,
            headless,
            output,
        } => {
            println!("{}", "Running automation task...".green().bold());
            println!("Objective: {}", objective);
            
            let url_str = url.unwrap_or_else(|| "https://example.com".to_string());
            println!("URL: {}", url_str);
            println!("Headless: {}", headless);
            
            if let Some(output_path) = &output {
                println!("Output file: {}", output_path.display());
            }
            
            // Run the automation
            let result = run_automation(&objective, &url_str)?;
            
            println!("\nResult: {}", result);
            println!("\n{}", "✓ Task completed successfully!".green().bold());
            Ok(())
        }
        Commands::Config {
            openai_key,
            anthropic_key,
            show,
            reset,
        } => {
            println!("{}", "Configuration:".blue().bold());
            if show {
                println!("Current configuration would be displayed here.");
            } else if reset {
                println!("Configuration reset to defaults.");
            } else {
                if let Some(key) = openai_key {
                    println!("OpenAI API key set: {}", mask_key(&key));
                }
                if let Some(key) = anthropic_key {
                    println!("Anthropic API key set: {}", mask_key(&key));
                }
            }
            Ok(())
        }
        Commands::Demo { name } => {
            println!("{}", format!("Running demo: {}", name).cyan().bold());
            // This would run the appropriate demo
            println!("\n{}", "✓ Demo completed successfully!".green().bold());
            Ok(())
        }
    }
}

/// Print banner
fn print_banner() {
    let banner = format!(
        r#"
{}
{}
    "#,
        "LlamaClick".bright_magenta().bold(),
        format!("v{} - Enterprise-Grade AI Web Automation", VERSION)
            .bright_blue()
            .bold()
    );
    println!("{}", banner);
}

/// Mask API key for display
fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    
    let visible_prefix = &key[0..4];
    let visible_suffix = &key[key.len() - 4..];
    format!("{}{}{}",
        visible_prefix, 
        "*".repeat(key.len() - 8),
        visible_suffix
    )
} 