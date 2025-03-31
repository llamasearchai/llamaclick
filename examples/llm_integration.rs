// LLM Integration Example
//
// This example demonstrates how to integrate with LLMs in LlamaClick.
// It shows how to:
// 1. Configure different LLM providers
// 2. Generate completions and chat responses
// 3. Process and use the responses
// 4. Handle errors and rate limiting

use llamaclick::config::Config;
use llamaclick::error::Result;
use llamaclick::llms::{
    ChatMessage, 
    LlmConfig, 
    LlmProvider, 
    OpenAIProvider, 
    AnthropicProvider,
    OllamaProvider,
    Role
};

use std::env;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== LlamaClick LLM Integration Example ===\n");
    
    // Load API keys from environment (or use a .env file in real applications)
    // Uncomment and set these in your environment to test with real APIs
    // let openai_api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "demo_key".to_string());
    // let anthropic_api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "demo_key".to_string());
    
    // For this example, we'll simulate responses for demo purposes
    let simulation_mode = true;
    
    // 1. Create a configuration with LLM settings
    let config = Config::default()
        .with_llm_config(LlmConfig {
            primary_provider: "openai".to_string(),
            openai: OpenAIProvider::default_config()
                .with_api_key("sk-...") // Replace with real key in production
                .with_model("gpt-4")
                .with_temperature(0.7)
                .with_max_tokens(150),
            anthropic: AnthropicProvider::default_config()
                .with_api_key("sk-ant-...") // Replace with real key in production
                .with_model("claude-3-sonnet-20240229")
                .with_temperature(0.7)
                .with_max_tokens(150),
            ollama: OllamaProvider::default_config()
                .with_host("http://localhost:11434")
                .with_model("llama3")
                .with_temperature(0.7)
                .with_max_tokens(150),
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
            cache_results: true,
            .. Default::default()
        });
    
    // 2. Initialize providers
    // In production, you would select one based on configuration
    println!("Initializing LLM providers...");
    
    // Simulated providers for the example
    let mut openai = if simulation_mode {
        create_simulated_openai_provider()
    } else {
        OpenAIProvider::new(&config.llm)?
    };
    
    let mut anthropic = if simulation_mode {
        create_simulated_anthropic_provider()
    } else {
        AnthropicProvider::new(&config.llm)?
    };
    
    let mut ollama = if simulation_mode {
        create_simulated_ollama_provider()
    } else {
        OllamaProvider::new(&config.llm)?
    };
    
    // 3. Text completion example
    println!("\n--- Text Completion Example ---");
    
    let prompt = "Explain the concept of browser automation in three sentences.";
    println!("Prompt: {}", prompt);
    
    // Get completions from different providers
    println!("\nOpenAI response:");
    let openai_response = openai.complete(prompt)?;
    println!("{}", openai_response);
    
    println!("\nAnthropic response:");
    let anthropic_response = anthropic.complete(prompt)?;
    println!("{}", anthropic_response);
    
    println!("\nOllama (local) response:");
    let ollama_response = ollama.complete(prompt)?;
    println!("{}", ollama_response);
    
    // 4. Chat completion example
    println!("\n--- Chat Completion Example ---");
    
    let messages = vec![
        ChatMessage {
            role: Role::System,
            content: "You are a helpful assistant that provides concise responses about web automation.".to_string(),
        },
        ChatMessage {
            role: Role::User,
            content: "What are three common challenges in web automation and how can they be solved?".to_string(),
        },
    ];
    
    println!("User message: {}", messages[1].content);
    
    // Get chat completions
    println!("\nOpenAI chat response:");
    let openai_chat_response = openai.chat(&messages)?;
    println!("{}", openai_chat_response);
    
    println!("\nAnthropic chat response:");
    let anthropic_chat_response = anthropic.chat(&messages)?;
    println!("{}", anthropic_chat_response);
    
    // 5. Using embeddings
    println!("\n--- Embeddings Example ---");
    
    let text_to_embed = "Browser automation can help with testing and data collection.";
    println!("Text: {}", text_to_embed);
    
    let embeddings = openai.embed(text_to_embed)?;
    println!("Generated {} dimensional embedding vector.", embeddings.len());
    println!("First 5 dimensions: {:?}", &embeddings[..5.min(embeddings.len())]);
    
    // 6. Practical application: Extracting structured data
    println!("\n--- Practical Application: Extracting Structured Data ---");
    
    let webpage_content = r#"
    <div class="product-info">
        <h1>Ergonomic Office Chair</h1>
        <div class="price">$299.99</div>
        <div class="rating">4.7/5 (129 reviews)</div>
        <div class="description">
            High-quality ergonomic office chair with lumbar support,
            adjustable height, and breathable mesh back.
        </div>
        <ul class="features">
            <li>Adjustable height: 18-22 inches</li>
            <li>Weight capacity: 300 lbs</li>
            <li>360-degree swivel</li>
            <li>5-year warranty</li>
        </ul>
    </div>
    "#;
    
    let extraction_prompt = format!(
        "Extract the following information from this product webpage HTML as JSON: 
        product name, price, rating, and key features. HTML content: {}",
        webpage_content
    );
    
    println!("Extracting structured data using LLM...");
    let structured_data = openai.complete(&extraction_prompt)?;
    println!("\nExtracted JSON data:\n{}", structured_data);
    
    // 7. Error handling example
    println!("\n--- Error Handling Example ---");
    
    // Simulating an error scenario
    if simulation_mode {
        println!("Simulating a rate limit error scenario...");
        println!("Error response: Rate limit exceeded. Please try again in 20s.");
        println!("Implementing retry with exponential backoff...");
        println!("Retry 1: Waiting 1s...");
        println!("Retry 2: Waiting 2s...");
        println!("Retry 3: Waiting 4s...");
        println!("Successfully received response after retries.");
    } else {
        // In real applications, this would be handled by the provider's retry logic
        println!("Skipping real error testing in demo mode.");
    }
    
    println!("\n=== Example completed successfully ===");
    Ok(())
}

// Helper function to create a simulated OpenAI provider for the example
fn create_simulated_openai_provider() -> OpenAIProvider {
    struct SimulatedOpenAI;
    
    impl LlmProvider for SimulatedOpenAI {
        fn complete(&mut self, prompt: &str) -> Result<String> {
            // Simulated response based on prompt
            if prompt.contains("browser automation") {
                Ok("Browser automation enables programmatic control of web browsers to simulate user interactions. It allows for automated testing, data scraping, and workflow automation without manual intervention. Modern browser automation tools provide cross-browser compatibility and can interact with complex web applications using DOM manipulation.".to_string())
            } else if prompt.contains("Extract") {
                Ok(r#"{
  "product_name": "Ergonomic Office Chair",
  "price": "$299.99",
  "rating": "4.7/5 (129 reviews)",
  "key_features": [
    "Adjustable height: 18-22 inches",
    "Weight capacity: 300 lbs",
    "360-degree swivel",
    "5-year warranty"
  ]
}"#.to_string())
            } else {
                Ok("This is a simulated response from OpenAI for demonstration purposes.".to_string())
            }
        }
        
        fn chat(&mut self, messages: &[ChatMessage]) -> Result<String> {
            // Check if any message mentions web automation challenges
            if messages.iter().any(|m| m.content.contains("challenges in web automation")) {
                Ok("1. Dynamic content: Use explicit waits instead of fixed sleep times to wait for elements to appear. 2. Flaky elements: Implement robust selectors and retry mechanisms for intermittent failures. 3. Browser differences: Use cross-browser testing frameworks and normalize behavior with abstraction layers.".to_string())
            } else {
                Ok("This is a simulated chat response from OpenAI.".to_string())
            }
        }
        
        fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
            // Generate a simulated embedding vector (normally 1536 dimensions for OpenAI)
            Ok(vec![0.02, -0.01, 0.03, 0.01, -0.02, 0.04, -0.03, 0.01, 0.02, -0.01])
        }
    }
    
    OpenAIProvider::from_provider(Box::new(SimulatedOpenAI))
}

// Helper function to create a simulated Anthropic provider for the example
fn create_simulated_anthropic_provider() -> AnthropicProvider {
    struct SimulatedAnthropic;
    
    impl LlmProvider for SimulatedAnthropic {
        fn complete(&mut self, prompt: &str) -> Result<String> {
            // Simulated response based on prompt
            if prompt.contains("browser automation") {
                Ok("Browser automation involves using software to control browser actions programmatically, eliminating the need for manual interaction. It's commonly used for testing websites, scraping data, and automating repetitive tasks across web applications. Modern browser automation frameworks provide APIs to simulate clicks, form submissions, navigation, and can verify elements appear correctly.".to_string())
            } else {
                Ok("This is a simulated response from Anthropic Claude for demonstration purposes.".to_string())
            }
        }
        
        fn chat(&mut self, messages: &[ChatMessage]) -> Result<String> {
            // Check if any message mentions web automation challenges
            if messages.iter().any(|m| m.content.contains("challenges in web automation")) {
                Ok("1. Element locator changes: Use more stable selectors like data attributes instead of CSS classes or XPaths that change frequently, and implement self-healing locators that can adapt to minor changes. 2. Performance issues: Implement headless mode for faster execution and parallel testing for larger test suites. 3. Captchas and anti-bot measures: Use authenticated sessions where possible and specialized services for captcha solving when necessary.".to_string())
            } else {
                Ok("This is a simulated chat response from Anthropic Claude.".to_string())
            }
        }
        
        fn embed(&mut self, _text: &str) -> Result<Vec<f32>> {
            // Anthropic doesn't have embeddings API as of this example
            Err(llamaclick::error::Error::OperationNotSupported(
                "Anthropic doesn't support embeddings".to_string()
            ))
        }
    }
    
    AnthropicProvider::from_provider(Box::new(SimulatedAnthropic))
}

// Helper function to create a simulated Ollama provider for the example
fn create_simulated_ollama_provider() -> OllamaProvider {
    struct SimulatedOllama;
    
    impl LlmProvider for SimulatedOllama {
        fn complete(&mut self, prompt: &str) -> Result<String> {
            // Simulated response based on prompt
            if prompt.contains("browser automation") {
                Ok("Browser automation is the process of controlling a web browser through code to automate repetitive tasks. It allows developers to write scripts that can navigate websites, fill forms, and extract data without manual intervention. This technology is fundamental for web testing, scraping, and creating bots that interact with web services.".to_string())
            } else {
                Ok("This is a simulated response from a local Ollama model for demonstration purposes.".to_string())
            }
        }
        
        fn chat(&mut self, messages: &[ChatMessage]) -> Result<String> {
            // Check if any message mentions web automation challenges
            if messages.iter().any(|m| m.content.contains("challenges in web automation")) {
                Ok("1. Synchronization issues: Implement smart waiting mechanisms that understand the state of the page rather than using fixed timeouts. 2. Iframe and shadow DOM handling: Use specialized techniques to access elements within iframes and shadow DOMs, which traditional selectors can't reach directly. 3. State management: Develop robust mechanisms to handle and restore browser state to make tests more deterministic and less prone to environmental factors.".to_string())
            } else {
                Ok("This is a simulated chat response from a local Ollama model.".to_string())
            }
        }
        
        fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
            // Generate a simulated embedding vector (smaller dimension for demo)
            Ok(vec![0.01, 0.02, -0.03, 0.04, -0.02, 0.01, -0.01, 0.03])
        }
    }
    
    OllamaProvider::from_provider(Box::new(SimulatedOllama))
} 