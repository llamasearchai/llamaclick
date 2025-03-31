//! Browser automation module for LlamaClick
//!
//! This module provides interfaces and implementations for browser automation
//! using headless browsers and various drivers.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

/// Browser type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserType {
    /// Chrome browser
    Chrome,
    /// Firefox browser
    Firefox,
    /// Edge browser
    Edge,
    /// Safari browser
    Safari,
}

impl fmt::Display for BrowserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserType::Chrome => write!(f, "Chrome"),
            BrowserType::Firefox => write!(f, "Firefox"),
            BrowserType::Edge => write!(f, "Edge"),
            BrowserType::Safari => write!(f, "Safari"),
        }
    }
}

/// Browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Browser type
    pub browser_type: BrowserType,
    /// Whether to run in headless mode
    pub headless: bool,
    /// Default timeout for operations
    pub timeout: Duration,
    /// User agent
    pub user_agent: Option<String>,
    /// Proxy URL
    pub proxy: Option<String>,
    /// Whether to ignore HTTPS errors
    pub ignore_https_errors: bool,
    /// Whether to block images
    pub block_images: bool,
    /// Window width
    pub window_width: u32,
    /// Window height
    pub window_height: u32,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser_type: BrowserType::Chrome,
            headless: true,
            timeout: Duration::from_secs(30),
            user_agent: None,
            proxy: None,
            ignore_https_errors: false,
            block_images: false,
            window_width: 1280,
            window_height: 800,
        }
    }
}

impl BrowserConfig {
    /// Create a new browser configuration
    pub fn new(browser_type: BrowserType) -> Self {
        Self {
            browser_type,
            ..Default::default()
        }
    }
    
    /// Set whether to run in headless mode
    pub fn with_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }
    
    /// Set the default timeout for operations
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    /// Set the proxy URL
    pub fn with_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    
    /// Set whether to ignore HTTPS errors
    pub fn with_ignore_https_errors(mut self, ignore_https_errors: bool) -> Self {
        self.ignore_https_errors = ignore_https_errors;
        self
    }
    
    /// Set whether to block images
    pub fn with_block_images(mut self, block_images: bool) -> Self {
        self.block_images = block_images;
        self
    }
    
    /// Set the window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }
}

/// Element selector
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Selector {
    /// CSS selector
    Css(String),
    /// XPath selector
    XPath(String),
    /// Text selector
    Text(String),
    /// ID selector
    Id(String),
    /// Class selector
    Class(String),
    /// Name selector
    Name(String),
    /// Semantic selector using AI for matching
    Semantic(String),
}

impl Selector {
    /// Create a new CSS selector
    pub fn css(selector: impl Into<String>) -> Self {
        Self::Css(selector.into())
    }
    
    /// Create a new XPath selector
    pub fn xpath(selector: impl Into<String>) -> Self {
        Self::XPath(selector.into())
    }
    
    /// Create a new text selector
    pub fn text(selector: impl Into<String>) -> Self {
        Self::Text(selector.into())
    }
    
    /// Create a new ID selector
    pub fn id(selector: impl Into<String>) -> Self {
        Self::Id(selector.into())
    }
    
    /// Create a new class selector
    pub fn class(selector: impl Into<String>) -> Self {
        Self::Class(selector.into())
    }
    
    /// Create a new name selector
    pub fn name(selector: impl Into<String>) -> Self {
        Self::Name(selector.into())
    }
    
    /// Create a new semantic selector
    pub fn semantic(selector: impl Into<String>) -> Self {
        Self::Semantic(selector.into())
    }
}

/// Browser interface
pub trait Browser: Send + Sync {
    /// Get the browser type
    fn browser_type(&self) -> BrowserType;
    
    /// Navigate to a URL
    fn navigate(&mut self, url: &str) -> Result<()>;
    
    /// Get the current URL
    fn current_url(&self) -> Result<String>;
    
    /// Click on an element
    fn click(&mut self, selector: &Selector) -> Result<()>;
    
    /// Type text into an element
    fn type_text(&mut self, selector: &Selector, text: &str) -> Result<()>;
    
    /// Get text from an element
    fn get_text(&self, selector: &Selector) -> Result<String>;
    
    /// Get attributes of an element
    fn get_attributes(&self, selector: &Selector) -> Result<std::collections::HashMap<String, String>>;
    
    /// Check if an element exists
    fn element_exists(&self, selector: &Selector) -> Result<bool>;
    
    /// Wait for an element to be visible
    fn wait_for_element(&mut self, selector: &Selector, timeout: Duration) -> Result<()>;
    
    /// Wait for navigation to complete
    fn wait_for_navigation(&mut self, timeout: Duration) -> Result<()>;
    
    /// Take a screenshot
    fn take_screenshot(&self, path: &str) -> Result<()>;
    
    /// Execute JavaScript
    fn execute_js(&mut self, script: &str) -> Result<serde_json::Value>;
    
    /// Get page HTML
    fn get_html(&self) -> Result<String>;
    
    /// Close the browser
    fn close(&mut self) -> Result<()>;
}

/// Browser session
pub struct BrowserSession {
    /// The browser
    browser: Box<dyn Browser>,
    /// The configuration
    config: BrowserConfig,
}

impl BrowserSession {
    /// Create a new browser session
    pub fn new(browser: Box<dyn Browser>, config: BrowserConfig) -> Self {
        Self { browser, config }
    }
    
    /// Navigate to a URL
    pub fn navigate(&mut self, url: &str) -> Result<()> {
        self.browser.navigate(url)
    }
    
    /// Get the current URL
    pub fn current_url(&self) -> Result<String> {
        self.browser.current_url()
    }
    
    /// Click on an element
    pub fn click(&mut self, selector: &Selector) -> Result<()> {
        self.browser.click(selector)
    }
    
    /// Type text into an element
    pub fn type_text(&mut self, selector: &Selector, text: &str) -> Result<()> {
        self.browser.type_text(selector, text)
    }
    
    /// Get text from an element
    pub fn get_text(&self, selector: &Selector) -> Result<String> {
        self.browser.get_text(selector)
    }
    
    /// Get attributes of an element
    pub fn get_attributes(&self, selector: &Selector) -> Result<std::collections::HashMap<String, String>> {
        self.browser.get_attributes(selector)
    }
    
    /// Check if an element exists
    pub fn element_exists(&self, selector: &Selector) -> Result<bool> {
        self.browser.element_exists(selector)
    }
    
    /// Wait for an element to be visible with the default timeout
    pub fn wait_for_element(&mut self, selector: &Selector) -> Result<()> {
        self.browser.wait_for_element(selector, self.config.timeout)
    }
    
    /// Wait for navigation to complete with the default timeout
    pub fn wait_for_navigation(&mut self) -> Result<()> {
        self.browser.wait_for_navigation(self.config.timeout)
    }
    
    /// Take a screenshot
    pub fn take_screenshot(&self, path: &str) -> Result<()> {
        self.browser.take_screenshot(path)
    }
    
    /// Execute JavaScript
    pub fn execute_js(&mut self, script: &str) -> Result<serde_json::Value> {
        self.browser.execute_js(script)
    }
    
    /// Get page HTML
    pub fn get_html(&self) -> Result<String> {
        self.browser.get_html()
    }
    
    /// Close the browser
    pub fn close(&mut self) -> Result<()> {
        self.browser.close()
    }
} 