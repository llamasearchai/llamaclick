use clap::{Parser, Subcommand, Args};

/// LlamaClick - A colorful, intelligent CLI application for web automation powered by LLMs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for LlamaClick
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install or update dependencies required by LlamaClick
    Install {
        /// Force reinstallation even if dependencies are already installed
        #[arg(short, long)]
        force: bool,
    },

    /// Run a web automation task with a natural language objective
    Run {
        /// The URL to navigate to
        #[arg(short, long)]
        url: String,

        /// The objective to accomplish, described in natural language
        #[arg(short, long)]
        objective: String,

        /// Run in headless mode (no browser UI)
        #[arg(short, long)]
        headless: bool,
    },

    /// Run an interactive demo of LlamaClick
    Demo {
        /// Port for the demo server
        #[arg(short, long, default_value_t = 8000)]
        port: u16,
    },

    /// Run tests to verify LlamaClick is working correctly
    Test {
        /// Pattern to match test names
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Manage LlamaClick configuration
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },

    /// Configure API keys for LLM providers
    ApiKeys,

    /// LinkedIn-specific automation commands
    LinkedIn {
        #[command(subcommand)]
        subcommand: LinkedInSubcommand,
    },
}

/// Subcommands for configuration management
#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    /// Show current configuration
    Show,

    /// Edit configuration file in a text editor
    Edit,

    /// Set a specific configuration value
    Set {
        /// Configuration key (e.g., llm.api_key)
        key: String,

        /// Value to set
        value: String,
    },

    /// Reset configuration to default values
    Reset {
        /// Confirm reset without prompt
        #[arg(short, long)]
        force: bool,
    },
}

/// Subcommands for LinkedIn automation
#[derive(Subcommand, Debug)]
pub enum LinkedInSubcommand {
    /// Log in to LinkedIn
    Login,

    /// Log out from LinkedIn
    Logout,

    /// Manage LinkedIn profile
    Profile {
        #[command(subcommand)]
        command: LinkedInProfileCommand,
    },

    /// Search and apply for jobs
    Jobs {
        #[command(subcommand)]
        command: LinkedInJobsCommand,
    },

    /// Manage network connections
    Network {
        #[command(subcommand)]
        command: LinkedInNetworkCommand,
    },

    /// Create and manage posts
    Post {
        #[command(subcommand)]
        command: LinkedInPostCommand,
    },
}

/// Subcommands for LinkedIn profile management
#[derive(Subcommand, Debug)]
pub enum LinkedInProfileCommand {
    /// View profile information
    View,

    /// Edit profile information
    Edit {
        /// Section to edit
        #[arg(short, long)]
        section: Option<String>,
    },

    /// Generate profile improvements using AI
    Improve,
}

/// Subcommands for LinkedIn job search and application
#[derive(Subcommand, Debug)]
pub enum LinkedInJobsCommand {
    /// Search for jobs
    Search {
        /// Keywords for job search
        #[arg(short, long)]
        keywords: String,

        /// Location for job search
        #[arg(short, long)]
        location: Option<String>,

        /// Maximum number of job results to return
        #[arg(short, long, default_value_t = 10)]
        limit: u32,
    },

    /// Apply to a job
    Apply {
        /// Job ID to apply for
        #[arg(short, long)]
        id: String,

        /// Path to cover letter template
        #[arg(short, long)]
        cover_letter: Option<String>,

        /// Skip application preview and apply immediately
        #[arg(short, long)]
        no_preview: bool,
    },

    /// View saved jobs
    Saved,
}

/// Subcommands for LinkedIn network management
#[derive(Subcommand, Debug)]
pub enum LinkedInNetworkCommand {
    /// Search for people
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// Maximum number of results to return
        #[arg(short, long, default_value_t = 10)]
        limit: u32,
    },

    /// Manage connection requests
    Requests {
        /// View incoming or outgoing requests
        #[arg(short, long, default_value = "incoming")]
        direction: String,
    },

    /// View connections
    Connections,
}

/// Subcommands for LinkedIn post management
#[derive(Subcommand, Debug)]
pub enum LinkedInPostCommand {
    /// Create a new post
    Create {
        /// Post content
        #[arg(short, long)]
        content: Option<String>,

        /// Path to content file
        #[arg(short, long)]
        file: Option<String>,

        /// Generate content with AI
        #[arg(short, long)]
        generate: bool,
    },

    /// Generate post content using AI
    Generate {
        /// Topic for generated post
        #[arg(short, long)]
        topic: String,

        /// Tone for generated post (professional, casual, etc.)
        #[arg(short, long, default_value = "professional")]
        tone: String,
    },

    /// Manage draft posts
    Drafts,

    /// Schedule a post
    Schedule {
        /// Post content
        #[arg(short, long)]
        content: Option<String>,

        /// Path to content file
        #[arg(short, long)]
        file: Option<String>,

        /// Date and time for scheduled post (YYYY-MM-DD HH:MM)
        #[arg(short, long)]
        datetime: String,
    },
} 