// Simple Browser Automation Example
//
// This example demonstrates basic browser automation capabilities of LlamaClick.
// It shows how to:
// 1. Initialize a browser session
// 2. Navigate to a website
// 3. Find and interact with elements
// 4. Extract text and take screenshots
// 5. Close the session gracefully

use llamaclick::browser::{BrowserController, BrowserOptions, ChromiumBrowser};
use llamaclick::config::Config;
use llamaclick::error::Result;
use llamaclick::utils::file_system::{ensure_dir, write_to_file};

use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== LlamaClick Simple Browser Example ===");
    
    // 1. Load or create configuration
    let config = Config::default()
        .with_browser_options(BrowserOptions {
            headless: false,                   // Set to true for headless operation
            user_agent: Some(String::from(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
            )),
            proxy: None,
            timeout: Duration::from_secs(30),
            screenshot_dir: Some(String::from("./screenshots")),
            .. Default::default()
        });
    
    // 2. Create browser instance
    println!("Initializing browser...");
    let mut browser = ChromiumBrowser::new(&config.browser)?;
    
    // 3. Navigate to a website
    println!("Navigating to example.com...");
    browser.navigate("https://example.com")?;
    
    // Wait for page to load completely
    thread::sleep(Duration::from_secs(2));
    
    // 4. Find main elements on the page
    println!("Finding main elements...");
    let heading = browser.find_element("h1")?;
    let heading_text = heading.text()?;
    println!("Found heading: {}", heading_text);
    
    let paragraph = browser.find_element("p")?;
    let paragraph_text = paragraph.text()?;
    println!("Found paragraph: {}", paragraph_text);
    
    // 5. Take a screenshot
    println!("Taking screenshot...");
    ensure_dir("./screenshots")?;
    let screenshot_path = browser.take_screenshot(Some("example_page.png"))?;
    println!("Screenshot saved to: {}", screenshot_path.display());
    
    // 6. Click on a link
    println!("Finding and clicking on a link...");
    let link = browser.find_element("a")?;
    println!("Link text: {}", link.text()?);
    println!("Link href: {}", link.get_attribute("href")?);
    
    // Click the link (commented out to avoid leaving the example page)
    // link.click()?;
    // thread::sleep(Duration::from_secs(3));
    
    // 7. Execute JavaScript
    println!("Executing JavaScript...");
    let title = browser.execute_script("return document.title;")?;
    println!("Page title from JS: {}", title.as_string().unwrap_or_default());
    
    // Change the page heading using JavaScript
    browser.execute_script("document.querySelector('h1').textContent = 'Modified by LlamaClick';")?;
    
    // Verify the change
    thread::sleep(Duration::from_secs(1));
    let new_heading = browser.find_element("h1")?.text()?;
    println!("Modified heading: {}", new_heading);
    
    // Take another screenshot showing the change
    browser.take_screenshot(Some("example_page_modified.png"))?;
    
    // 8. Extract all links from the page
    println!("\nExtracting all links from the page:");
    let links = browser.find_elements("a")?;
    for (i, link) in links.iter().enumerate() {
        println!("Link {}: {} ({})", 
            i + 1, 
            link.text()?,
            link.get_attribute("href").unwrap_or_default()
        );
    }
    
    // 9. Save page content
    println!("\nSaving page content...");
    let page_content = browser.page_source()?;
    write_to_file("./example_page.html", &page_content)?;
    println!("Page content saved to: ./example_page.html");
    
    // 10. Close the browser session
    println!("\nClosing browser session...");
    browser.close()?;
    
    println!("\n=== Example completed successfully ===");
    Ok(())
} 