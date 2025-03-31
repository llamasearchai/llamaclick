# Contributing to LlamaClick

First off, thank you for considering contributing to LlamaClick! It's people like you that make LlamaClick such a great tool.

## Code of Conduct

By participating in this project, you are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to [info@llamasearch.ai](mailto:info@llamasearch.ai).

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report for LlamaClick. Following these guidelines helps maintainers understand your report, reproduce the behavior, and find related reports.

* Use the GitHub issue search — check if the issue has already been reported.
* Check if the issue has been fixed — try to reproduce it using the latest `main` or `develop` branch.
* Isolate the problem — create a reduced test case and a live example.

### Feature Requests

This section guides you through submitting a feature request for LlamaClick, including completely new features and minor improvements to existing functionality.

* Use the GitHub discussion board for feature requests.
* Provide a clear and detailed explanation of the feature you want and why it's important.
* Include code samples if applicable to show how you would expect the feature to work.

### Pull Requests

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Style Guides

#### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* Consider starting the commit message with an applicable emoji:
    * 🎨 `:art:` when improving the format/structure of the code
    * 🐎 `:racehorse:` when improving performance
    * 🚱 `:non-potable_water:` when plugging memory leaks
    * 📝 `:memo:` when writing docs
    * 🐛 `:bug:` when fixing a bug
    * 🔥 `:fire:` when removing code or files
    * 💚 `:green_heart:` when fixing the CI build
    * ✅ `:white_check_mark:` when adding tests
    * 🔒 `:lock:` when dealing with security
    * ⬆️ `:arrow_up:` when upgrading dependencies
    * ⬇️ `:arrow_down:` when downgrading dependencies

#### Rust Style Guide

* We follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
* Run `cargo fmt` before committing
* Run `cargo clippy` and address all warnings before committing
* Add documentation comments for all public items
* Use meaningful variable names and add comments for complex operations
* Follow the Rust naming conventions:
    * `snake_case` for variables and functions
    * `CamelCase` for types
    * `SCREAMING_SNAKE_CASE` for constants

## Development Environment

### Prerequisites

* Rust (latest stable)
* Cargo
* Git

### Setting Up

1. Clone the repository
```bash
git clone https://github.com/llamasearch/llamaclick.git
cd llamaclick
```

2. Install dependencies
```bash
cargo build
```

3. Run tests
```bash
cargo test
```

### Development Workflow

1. Create a branch for your work
2. Make your changes
3. Write tests for your changes
4. Run `cargo fmt` and `cargo clippy` to ensure code quality
5. Submit a pull request

## Releasing

1. Update the version in `Cargo.toml`
2. Update the `CHANGELOG.md`
3. Create a new GitHub release
4. Publish to crates.io with `cargo publish`

## Resources

* [Rust Book](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Project Structure

Understanding the project structure will help you make effective contributions:

```
llamaclick/
├── src/
│   ├── cli/              # Command-line interface
│   │   ├── commands.rs   # Command definitions
│   │   ├── ui.rs         # Terminal UI components
│   │   └── mod.rs        # Module exports
│   ├── config/           # Configuration management
│   │   ├── settings.rs   # Settings structure
│   │   └── mod.rs        # Module exports
│   ├── core/             # Core automation functionality
│   │   ├── browser.rs    # Browser automation
│   │   ├── session.rs    # Session management
│   │   └── mod.rs        # Module exports
│   ├── error/            # Error handling
│   │   ├── types.rs      # Error type definitions
│   │   └── mod.rs        # Module exports
│   ├── linkedin/         # LinkedIn-specific automation
│   │   ├── auth.rs       # Authentication
│   │   ├── jobs.rs       # Job search and application
│   │   └── mod.rs        # Module exports
│   ├── llms/             # LLM integration
│   │   ├── openai.rs     # OpenAI API integration
│   │   ├── anthropic.rs  # Anthropic API integration
│   │   ├── ollama.rs     # Ollama integration
│   │   └── mod.rs        # Module exports
│   ├── utils/            # Utility functions
│   │   ├── ascii_art.rs  # ASCII art generation
│   │   ├── crypto.rs     # Cryptography utilities
│   │   └── mod.rs        # Module exports
│   ├── error.rs          # Central error definitions
│   └── main.rs           # Application entry point
├── tests/               # Integration tests
├── docs/                # Documentation
└── examples/            # Example use cases
```

## Pull Request Process

1. **Fill out the pull request template** completely
2. **Ensure all tests pass** and your code meets our style guidelines
3. **Link to any relevant issues** in the PR description
4. **Describe your changes** in detail
5. **Wait for code review** and address any feedback

Pull requests require approval from at least one maintainer before being merged.

## Style Guidelines

We follow standard Rust style guidelines:

- **Code formatting**: We use `rustfmt` with default settings
- **Linting**: We use `clippy` for linting
- **Documentation**: All public API items should have documentation comments
- **Tests**: New features should include tests

## Documentation

When adding new features, please update the relevant documentation:

- **Code comments**: Document your code, especially public API
- **README**: Update if adding major features or changing usage
- **Command help**: Update command help text if changing CLI behavior

## Release Process

Our release process follows these steps:

1. **Version bump** in Cargo.toml according to [Semantic Versioning](https://semver.org/)
2. **CHANGELOG.md update** with notable changes
3. **Git tag** for the version
4. **Cargo publish** to crates.io
5. **GitHub release** with release notes

## Getting Help

If you need help with anything, you can:

- **Open an issue** with a question
- **Join our community Discord** for real-time discussion
- **Email us** at contributors@llamasearch.ai

## License

By contributing to LlamaClick, you agree that your contributions will be licensed under the project's [MIT License](LICENSE). 