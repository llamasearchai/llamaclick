# Contributing to LlamaClick

Thank you for your interest in contributing to LlamaClick! This document provides guidelines and instructions to help you get started.

## Code of Conduct

We maintain a friendly and inclusive environment for all contributors. Please be respectful and considerate of others when participating in this project.

## How to Contribute

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/llamaclick.git
   cd llamaclick
   ```
3. **Set up the upstream remote**:
   ```bash
   git remote add upstream https://github.com/llamasearch/llamaclick.git
   ```
4. **Install development dependencies**:
   ```bash
   cargo install cargo-watch cargo-edit cargo-tarpaulin
   ```

### Development Workflow

1. **Create a new branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and write tests for them.

3. **Run tests** to make sure everything works:
   ```bash
   cargo test
   ```

4. **Run the formatter and linter**:
   ```bash
   cargo fmt
   cargo clippy
   ```

5. **Commit your changes** with a clear message:
   ```bash
   git commit -m "feat: add feature XYZ"
   ```
   
   We follow [Conventional Commits](https://www.conventionalcommits.org/) for our commit messages.

6. **Keep your branch up to date** with upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

7. **Push your changes** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

8. **Submit a pull request** to the main repository.

## Development Tips

### Running the Development Version

To run your development version of the application:

```bash
cargo run -- [command] [options]
```

For example:

```bash
cargo run -- run --url https://example.com --objective "Find contact information"
```

### Live Reloading During Development

For a better development experience, you can use cargo-watch:

```bash
cargo watch -x 'run -- [command] [options]'
```

### Testing

We have several types of tests:

1. **Unit tests**: Test individual functions and methods
2. **Integration tests**: Test the interaction between components
3. **End-to-end tests**: Test the entire application (these are marked with `#[ignore]` by default)

To run all tests except ignored ones:

```bash
cargo test
```

To run a specific test:

```bash
cargo test test_name
```

To run ignored tests:

```bash
cargo test -- --ignored
```

### Code Coverage

To check code coverage:

```bash
cargo tarpaulin
```

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