# LlamaClick Examples

This directory contains example scripts demonstrating various capabilities and use cases of the LlamaClick library. These examples are designed to help you understand how to use LlamaClick in your own projects.

## Running Examples

To run an example, use the following command from the project root:

```bash
cargo run --example <example_name>
```

For instance, to run the `simple_browser` example:

```bash
cargo run --example simple_browser
```

## Available Examples

### Basic Examples

1. **[simple_browser.rs](./simple_browser.rs)** - Simple browser automation example
   * Demonstrates basic navigation, element selection, and interaction
   * Shows how to take screenshots and extract text

2. **[llm_integration.rs](./llm_integration.rs)** - Basic LLM integration
   * Shows how to use OpenAI or other LLM providers
   * Demonstrates prompt construction and response parsing

3. **[config_example.rs](./config_example.rs)** - Configuration management
   * Illustrates how to load, modify, and save configurations
   * Shows environment variable integration

### Intermediate Examples

4. **[linkedin_job_search.rs](./linkedin_job_search.rs)** - LinkedIn job search automation
   * Demonstrates how to search for jobs on LinkedIn
   * Shows how to extract job details and save to CSV

5. **[form_automation.rs](./form_automation.rs)** - Complex form automation
   * Shows how to fill out complex forms with multiple field types
   * Demonstrates form validation and error handling

6. **[parallel_browsers.rs](./parallel_browsers.rs)** - Running multiple browser sessions
   * Shows how to manage multiple browser instances in parallel
   * Demonstrates synchronization between sessions

### Advanced Examples

7. **[ai_driven_testing.rs](./ai_driven_testing.rs)** - LLM-powered test automation
   * Uses natural language to specify test scenarios
   * Demonstrates autonomous test execution and reporting

8. **[custom_plugin.rs](./custom_plugin.rs)** - Creating a custom plugin
   * Shows how to extend LlamaClick with custom functionality
   * Demonstrates the plugin registration and lifecycle

9. **[enterprise_workflow.rs](./enterprise_workflow.rs)** - Complex enterprise workflow
   * Shows a complete end-to-end enterprise automation scenario
   * Demonstrates secure credential handling
   * Illustrates error recovery strategies

## Example Structure

Each example follows a similar structure:

1. **Setup** - Configuration and initialization
2. **Main functionality** - Core demonstration of the feature
3. **Cleanup** - Proper resource management and shutdown
4. **Comments** - Detailed explanations of important code sections

## Creating Your Own Examples

Feel free to modify these examples or create your own. If you create an example that might be useful to others, please consider submitting a pull request!

When creating your own examples:

1. Follow the naming convention: `descriptive_name.rs`
2. Include detailed comments explaining what the example demonstrates
3. Handle errors appropriately
4. Clean up resources when done
5. Update this README to include your example

## Troubleshooting

If you encounter issues running the examples:

1. Ensure you have the correct dependencies installed
2. Check that your browser drivers are up to date
3. Verify that your API keys are set correctly for LLM examples
4. Review the logs for detailed error information

For more help, please check the main documentation or open an issue on GitHub. 