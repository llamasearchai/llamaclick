# 🦙 LlamaClick

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI/CD](https://img.shields.io/badge/CI/CD-GitHub%20Actions-brightgreen)](https://github.com/llamasearch/llamaclick/actions)
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue)](https://crates.io/crates/llamaclick)

A colorful, intelligent CLI application for web automation powered by LLMs. LlamaClick helps you automate web interactions using natural language objectives.

<div align="center">
  <img src="docs/images/llamaclick_banner.png" alt="LlamaClick Banner" width="800">
</div>

## 🌟 Features

- **🌈 Beautiful CLI Interface**: Colorful, intuitive terminal UI with helpful visual feedback
- **🧠 LLM-Powered Automation**: Uses AI to understand and interact with web pages
- **🖱️ Natural Language Control**: Describe what you want to accomplish in plain English
- **🛠️ Multi-Browser Support**: Works with Chrome, Firefox, and Edge via PlayWright
- **🔄 Session Management**: Save and resume automation sessions
- **🔒 Secure Credential Handling**: Securely store and manage credentials
- **🎮 Interactive Demo Mode**: Try features without affecting real sites
- **💼 LinkedIn Automation**: Special features for professional networking and job applications
- **📊 Detailed Reporting**: Generate reports of automation runs for audit or improvement

## 🚀 Installation

### From Crates.io

```bash
cargo install llamaclick
```

### From Source

```bash
# Clone the repository
git clone https://github.com/llamasearch/llamaclick.git
cd llamaclick

# Build the project
cargo build --release

# Run the binary
./target/release/llamaclick
```

## 📋 Requirements

- **Rust** 1.60 or higher
- **Internet connection** for LLM API calls
- **Browser**: Chrome, Firefox, or Edge

## 🔧 Usage

### Installing Dependencies

```bash
llamaclick install
```

Use the `--force` flag to force reinstallation:

```bash
llamaclick install --force
```

### Running LlamaClick

```bash
llamaclick run --url https://example.com --objective "Find the contact information"
```

Options:
- `--url` or `-u`: The URL to navigate to
- `--objective` or `-o`: The objective to accomplish
- `--headless` or `-h`: Run in headless mode (no browser UI)

### Interactive Demo

```bash
llamaclick demo
```

Options:
- `--port` or `-p`: Port for the demo server (default: 8000)

### Running Tests

```bash
llamaclick test
```

Options:
- `--pattern` or `-p`: Pattern to match test names

### Managing Configuration

Show current configuration:
```bash
llamaclick config show
```

Edit configuration file:
```bash
llamaclick config edit
```

Set a configuration value:
```bash
llamaclick config set llm.api_key YOUR_API_KEY
```

### Managing API Keys

```bash
llamaclick api-keys
```

## 💼 LinkedIn Automation

LlamaClick includes specialized LinkedIn automation features:

### Authentication

```bash
llamaclick linkedin login
llamaclick linkedin logout
```

### Profile Management

```bash
llamaclick linkedin profile
```

### Job Applications

```bash
llamaclick linkedin jobs
llamaclick linkedin jobs search
llamaclick linkedin jobs apply
```

### Network Management

```bash
llamaclick linkedin network
llamaclick linkedin network search
llamaclick linkedin network requests
```

### Post Management

```bash
llamaclick linkedin post
```

## ⚙️ Configuration

LlamaClick's configuration is stored in a TOML file at:
- Linux/macOS: `~/.config/llamaclick/config.toml`
- Windows: `C:\Users\{USERNAME}\AppData\Roaming\llamaclick\config.toml`

### LLM Settings

```toml
[llm]
provider = "openai"
model = "gpt-4"
api_key = "YOUR_API_KEY"
anthropic_api_key = ""
ollama_url = "http://localhost:11434"
anthropic_model = "claude-3-haiku-20240307"
ollama_model = "llama3"
```

### Browser Settings

```toml
[browser]
timeout_seconds = 30
headless = false
screenshots = true
driver_type = "playwright"
```

### Agent Settings

```toml
[agent]
time_between_actions_ms = 500
max_steps = 50
```

## 🔍 Examples

### Basic Web Interaction

```bash
# Fill out a contact form
llamaclick run --url "https://example.com/contact" --objective "Fill out the contact form with name 'John Doe', email 'john@example.com', and message 'Hello, I'm interested in your services'"
```

### LinkedIn Job Application

```bash
# Search and apply for jobs
llamaclick linkedin jobs search --keywords "Rust Developer" --location "Remote"
llamaclick linkedin jobs apply --id "job123456" --cover-letter "templates/cover_letter.txt"
```

### Web Scraping

```bash
# Extract product information
llamaclick run --url "https://example.com/products" --objective "Extract all product names and prices on the page and save to products.json"
```

## 📚 Documentation

- [User Guide](https://github.com/llamasearch/llamaclick/wiki/User-Guide)
- [API Documentation](https://docs.rs/llamaclick)
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## 🛠️ Architecture

```
llamaclick/
├── src/
│   ├── cli/          # Command-line interface
│   ├── config/       # Configuration management
│   ├── core/         # Core automation functionality
│   ├── error/        # Error handling
│   ├── linkedin/     # LinkedIn-specific automation
│   ├── llms/         # LLM integration
│   └── utils/        # Utility functions
├── docs/             # Documentation
└── tests/            # Test suite
```

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- [Clap](https://github.com/clap-rs/clap) for CLI argument parsing
- [Tokio](https://github.com/tokio-rs/tokio) for async runtime
- [PlayWright](https://playwright.dev/) for browser automation
- [OpenAI](https://openai.com/) and [Anthropic](https://www.anthropic.com/) for LLM APIs
- [Dialoguer](https://github.com/console-rs/dialoguer) for interactive prompts // Improved documentation
// Improved documentation
