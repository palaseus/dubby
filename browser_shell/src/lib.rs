//! # Browser Shell Crate
//! 
//! This crate provides the main application shell for the browser engine.
//! It coordinates between the different components (HTML parser, DOM, renderer)
//! and provides a high-level interface for browser functionality.
//! 
//! ## Design Principles
//! 
//! 1. **Component Coordination**: The shell acts as the main coordinator
//!    between different browser engine components.
//! 
//! 2. **Event Loop**: Provides the main event loop for handling user input
//!    and browser events.
//! 
//! 3. **Extensibility**: Designed to be easily extended with new features
//!    like networking, JavaScript execution, etc.
//! 
//! 4. **Error Handling**: Provides robust error handling and recovery
//!    mechanisms.

use dom::Document;
use html_parser::parse_html;
use css_parser::{parse_css, Stylesheet};
use layout::{LayoutEngine, LayoutBox};
use renderer::{render_as_text, extract_text_content, render_layout_box};
use networking::HttpClient;
use renderer_wgpu::render_layout_tree;
// use js_integration::JsEngine;
use std::io::{self, Write};
use std::rc::Rc;

pub mod webpage_loader;
pub mod gpu_webpage_renderer;

/// The main browser engine that coordinates all components
/// 
/// This struct represents the core browser engine and provides methods
/// for loading and rendering web pages. It's designed to be the main
/// interface for browser functionality.
pub struct BrowserEngine {
    /// The current document being displayed
    current_document: Option<Rc<Document>>,
    /// The current stylesheet
    current_stylesheet: Option<Stylesheet>,
    /// The current layout tree
    current_layout: Option<LayoutBox>,
    /// HTTP client for fetching resources
    http_client: HttpClient,
    // /// JavaScript engine for executing scripts
    // js_engine: JsEngine,
    /// Whether the browser is running
    is_running: bool,
}

impl BrowserEngine {
    /// Create a new browser engine instance
    pub fn new() -> Self {
        BrowserEngine {
            current_document: None,
            current_stylesheet: None,
            current_layout: None,
            http_client: HttpClient::new(),
            // js_engine: JsEngine::new(),
            is_running: false,
        }
    }
    
    /// Load HTML content and parse it into a DOM tree
    /// 
    /// This method takes HTML content, parses it using the HTML parser,
    /// and stores the resulting DOM tree as the current document.
    /// 
    /// # Arguments
    /// 
    /// * `html_content` - The HTML content to load
    /// 
    /// # Returns
    /// 
    /// `true` if the HTML was successfully loaded and parsed, `false` otherwise
    pub fn load_html(&mut self, html_content: &str) -> bool {
        match self.parse_html_safely(html_content) {
            Ok(document) => {
                let document_rc = Rc::new(document);
                self.current_document = Some(Rc::clone(&document_rc));
                // self.js_engine.set_document(document_rc);
                // Clear layout when HTML changes
                self.current_layout = None;
                true
            }
            Err(e) => {
                eprintln!("Error parsing HTML: {}", e);
                false
            }
        }
    }
    
    /// Load CSS content and parse it into a stylesheet
    /// 
    /// This method takes CSS content, parses it using the CSS parser,
    /// and stores the resulting stylesheet for styling the document.
    /// 
    /// # Arguments
    /// 
    /// * `css_content` - The CSS content to load
    /// 
    /// # Returns
    /// 
    /// `true` if the CSS was successfully loaded and parsed, `false` otherwise
    pub fn load_css(&mut self, css_content: &str) -> bool {
        match self.parse_css_safely(css_content) {
            Ok(stylesheet) => {
                self.current_stylesheet = Some(stylesheet.clone());
                // self.js_engine.set_stylesheet(stylesheet);
                // Clear layout when CSS changes
                self.current_layout = None;
                true
            }
            Err(e) => {
                eprintln!("Error parsing CSS: {}", e);
                false
            }
        }
    }
    
    /// Fetch HTML content from a URL
    /// 
    /// This method fetches HTML content from the given URL and loads it
    /// into the browser engine.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL to fetch
    /// 
    /// # Returns
    /// 
    /// `true` if the URL was successfully fetched and loaded, `false` otherwise
    pub async fn fetch_url(&mut self, url: &str) -> bool {
        match self.http_client.fetch_html(url).await {
            Ok(html_content) => {
                println!("Fetched {} bytes from {}", html_content.len(), url);
                self.load_html(&html_content)
            }
            Err(e) => {
                eprintln!("Error fetching URL {}: {}", url, e);
                false
            }
        }
    }
    
    // /// Execute JavaScript code
    // /// 
    // /// This method executes JavaScript code and optionally triggers layout
    // /// recalculation if the DOM is modified.
    // /// 
    // /// # Arguments
    // /// 
    // /// * `code` - The JavaScript code to execute
    // /// 
    // /// # Returns
    // /// 
    // /// `true` if the JavaScript was successfully executed, `false` otherwise
    // pub fn execute_javascript(&mut self, code: &str) -> bool {
    //     match self.js_engine.execute(code) {
    //         Ok(_) => {
    //             println!("JavaScript executed successfully");
    //             true
    //         }
    //         Err(e) => {
    //             eprintln!("JavaScript execution error: {}", e);
    //             false
    //         }
    //     }
    // }
    
    // /// Execute JavaScript code with layout update
    // /// 
    // /// This method executes JavaScript code and automatically triggers
    // /// layout recalculation if the DOM is modified.
    // /// 
    // /// # Arguments
    // /// 
    // /// * `code` - The JavaScript code to execute
    // /// 
    // /// # Returns
    // /// 
    // /// `true` if the JavaScript was successfully executed and layout updated, `false` otherwise
    // pub fn execute_javascript_with_layout(&mut self, code: &str) -> bool {
    //     match self.js_engine.execute_with_layout_update(code) {
    //         Ok(Some(layout)) => {
    //             self.current_layout = Some(layout);
    //             println!("JavaScript executed and layout updated");
    //             true
    //         }
    //         Ok(None) => {
    //             println!("JavaScript executed (no layout update needed)");
    //             true
    //         }
    //         Err(e) => {
    //             eprintln!("JavaScript execution error: {}", e);
    //             false
    //         }
    //     }
    // }
    
    /// Perform layout calculation on the current document
    /// 
    /// This method computes styles and layout for the current document
    /// using the current stylesheet. It must be called after loading
    /// both HTML and CSS content.
    /// 
    /// # Returns
    /// 
    /// `true` if layout was successfully calculated, `false` otherwise
    pub fn perform_layout(&mut self) -> bool {
        if let (Some(document), Some(stylesheet)) = (&self.current_document, &self.current_stylesheet) {
            let layout_engine = LayoutEngine::new(stylesheet.clone());
            let layout = layout_engine.layout_document(document);
            self.current_layout = Some(layout);
            true
        } else {
            eprintln!("Cannot perform layout: missing document or stylesheet");
            false
        }
    }
    
    /// Safely parse HTML content with error handling
    /// 
    /// This method wraps the HTML parsing in error handling to provide
    /// graceful degradation when encountering malformed HTML.
    fn parse_html_safely(&self, html_content: &str) -> Result<Document, String> {
        // For now, we'll use a simple approach - in a real browser,
        // this would include more sophisticated error recovery
        if html_content.trim().is_empty() {
            return Err("Empty HTML content".to_string());
        }
        
        match html_parser::parse_html_string(html_content) {
            Ok((document, _resources)) => Ok(document),
            Err(e) => Err(format!("HTML parsing error: {}", e)),
        }
    }
    
    /// Safely parse CSS content with error handling
    /// 
    /// This method wraps the CSS parsing in error handling to provide
    /// graceful degradation when encountering malformed CSS.
    fn parse_css_safely(&self, css_content: &str) -> Result<Stylesheet, String> {
        // For now, we'll use a simple approach - in a real browser,
        // this would include more sophisticated error recovery
        if css_content.trim().is_empty() {
            return Err("Empty CSS content".to_string());
        }
        
        Ok(parse_css(css_content))
    }
    
    /// Render the current document to text
    /// 
    /// This method renders the current document using the text renderer
    /// and returns the formatted output.
    /// 
    /// # Returns
    /// 
    /// The rendered text output, or an error message if no document is loaded
    pub fn render_to_text(&self) -> String {
        match &self.current_document {
            Some(document) => render_as_text(document),
            None => "No document loaded".to_string(),
        }
    }
    
    /// Render the current layout with styling information
    /// 
    /// This method renders the current layout tree with computed styles
    /// and dimensions, showing how the browser engine has styled and
    /// positioned each element.
    /// 
    /// # Returns
    /// 
    /// The rendered layout output, or an error message if no layout is available
    pub fn render_layout(&self) -> String {
        match &self.current_layout {
            Some(layout) => render_layout_box(layout),
            None => "No layout available. Load HTML, CSS, and perform layout first.".to_string(),
        }
    }
    
    /// Extract text content from the current document
    /// 
    /// This method extracts only the text content from the current document,
    /// ignoring all HTML markup.
    /// 
    /// # Returns
    /// 
    /// The text content, or an error message if no document is loaded
    pub fn get_text_content(&self) -> String {
        match &self.current_document {
            Some(document) => extract_text_content(document),
            None => "No document loaded".to_string(),
        }
    }
    
    /// Get a reference to the current document
    /// 
    /// This method provides access to the current DOM tree for advanced
    /// operations that require direct DOM manipulation.
    /// 
    /// # Returns
    /// 
    /// A reference to the current document, or `None` if no document is loaded
    pub fn get_document(&self) -> Option<&Rc<Document>> {
        self.current_document.as_ref()
    }
    
    /// Check if a document is currently loaded
    /// 
    /// # Returns
    /// 
    /// `true` if a document is loaded, `false` otherwise
    pub fn has_document(&self) -> bool {
        self.current_document.is_some()
    }
    
    /// Check if a stylesheet is currently loaded
    /// 
    /// # Returns
    /// 
    /// `true` if a stylesheet is loaded, `false` otherwise
    pub fn has_stylesheet(&self) -> bool {
        self.current_stylesheet.is_some()
    }
    
    /// Check if layout has been calculated
    /// 
    /// # Returns
    /// 
    /// `true` if layout is available, `false` otherwise
    pub fn has_layout(&self) -> bool {
        self.current_layout.is_some()
    }
    
    /// Get a reference to the current layout
    /// 
    /// # Returns
    /// 
    /// A reference to the current layout, or `None` if no layout is available
    pub fn get_layout(&self) -> Option<&LayoutBox> {
        self.current_layout.as_ref()
    }

    /// Render the current layout using GPU
    pub async fn render_with_gpu(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(layout) = &self.current_layout {
            render_layout_tree(layout).await?;
            Ok(())
        } else {
            Err("No layout available. Load HTML, CSS, and perform layout first.".into())
        }
    }
    
    /// Start the browser engine
    /// 
    /// This method initializes the browser engine and prepares it for
    /// processing documents and handling events.
    pub fn start(&mut self) {
        self.is_running = true;
        println!("Browser engine started");
    }
    
    /// Stop the browser engine
    /// 
    /// This method shuts down the browser engine and cleans up resources.
    pub fn stop(&mut self) {
        self.is_running = false;
        self.current_document = None;
        self.current_stylesheet = None;
        self.current_layout = None;
        println!("Browser engine stopped");
    }
    
    /// Check if the browser engine is running
    /// 
    /// # Returns
    /// 
    /// `true` if the browser is running, `false` otherwise
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Default for BrowserEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple command-line interface for the browser engine
/// 
/// This struct provides a basic command-line interface for interacting
/// with the browser engine. It's useful for testing and demonstration
/// purposes.
pub struct BrowserCLI {
    engine: BrowserEngine,
}

impl BrowserCLI {
    /// Create a new browser CLI
    pub fn new() -> Self {
        BrowserCLI {
            engine: BrowserEngine::new(),
        }
    }
    
    /// Run the interactive command-line interface
    /// 
    /// This method starts an interactive loop where users can enter
    /// commands to control the browser engine.
    pub fn run(&mut self) {
        self.engine.start();
        
        println!("Browser Engine CLI");
        println!("Commands:");
        println!("  load <html>     - Load HTML content");
        println!("  css <css>       - Load CSS content");
        println!("  fetch <url>     - Fetch HTML from URL");
        println!("  layout          - Perform layout calculation");
        println!("  render          - Render current document");
        println!("  layout-render   - Render with layout information");
        println!("  gpu-render      - Render with GPU (opens window)");
        // println!("  js <code>       - Execute JavaScript code");
        println!("  text            - Extract text content");
        println!("  help            - Show this help");
        println!("  quit            - Exit the browser");
        println!();
        
        loop {
            print!("browser> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            let input = input.trim();
            if input.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            let command = parts[0];
            let args = if parts.len() > 1 { parts[1] } else { "" };
            
            match command {
                "load" => {
                    if args.is_empty() {
                        println!("Usage: load <html>");
                    } else {
                        if self.engine.load_html(args) {
                            println!("HTML loaded successfully");
                        } else {
                            println!("Failed to load HTML");
                        }
                    }
                }
                "css" => {
                    if args.is_empty() {
                        println!("Usage: css <css>");
                    } else {
                        if self.engine.load_css(args) {
                            println!("CSS loaded successfully");
                        } else {
                            println!("Failed to load CSS");
                        }
                    }
                }
                "fetch" => {
                    if args.is_empty() {
                        println!("Usage: fetch <url>");
                    } else {
                        println!("Fetching URL: {}", args);
                        println!("Note: Fetch command requires async runtime. Use 'cargo run -- fetch <url>' instead.");
                    }
                }
                "layout" => {
                    if self.engine.perform_layout() {
                        println!("Layout calculated successfully");
                    } else {
                        println!("Failed to calculate layout");
                    }
                }
                "render" => {
                    println!("{}", self.engine.render_to_text());
                }
                "layout-render" => {
                    println!("{}", self.engine.render_layout());
                }
                "gpu-render" => {
                    if let Some(_layout) = self.engine.get_layout() {
                        println!("Opening GPU renderer window...");
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        if let Err(e) = rt.block_on(self.engine.render_with_gpu()) {
                            eprintln!("GPU renderer error: {}", e);
                        }
                    } else {
                        println!("No layout available. Load HTML, CSS, and perform layout first.");
                    }
                }
                // "js" => {
                //     if args.is_empty() {
                //         println!("Usage: js <javascript_code>");
                //     } else {
                //         if self.engine.execute_javascript(args) {
                //             println!("JavaScript executed successfully");
                //         } else {
                //             println!("Failed to execute JavaScript");
                //         }
                //     }
                // }
                "text" => {
                    println!("{}", self.engine.get_text_content());
                }
                "help" => {
                    self.show_help();
                }
                "quit" | "exit" => {
                    break;
                }
                _ => {
                    println!("Unknown command: {}. Type 'help' for available commands.", command);
                }
            }
        }
        
        self.engine.stop();
    }
    
    /// Show help information
    fn show_help(&self) {
        println!("Available commands:");
        println!("  load <html>      - Load HTML content and parse it into DOM");
        println!("  css <css>        - Load CSS content and parse it into stylesheet");
        println!("  fetch <url>      - Fetch HTML content from a URL");
        println!("  layout           - Perform layout calculation on current document");
        println!("  render           - Render the current document as formatted text");
        println!("  layout-render    - Render with layout and styling information");
        println!("  gpu-render       - Render with GPU acceleration (opens window)");
        // println!("  js <code>        - Execute JavaScript code");
        println!("  text             - Extract only the text content from the document");
        println!("  help             - Show this help message");
        println!("  quit/exit        - Exit the browser");
    }
}

impl Default for BrowserCLI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_engine_creation() {
        let engine = BrowserEngine::new();
        assert!(!engine.is_running());
        assert!(!engine.has_document());
    }

    #[test]
    fn test_load_html() {
        let mut engine = BrowserEngine::new();
        let html = r#"<html><body><h1>Test</h1></body></html>"#;
        
        assert!(engine.load_html(html));
        assert!(engine.has_document());
    }

    #[test]
    fn test_render_to_text() {
        let mut engine = BrowserEngine::new();
        let html = r#"<html><body><h1>Hello World</h1></body></html>"#;
        
        engine.load_html(html);
        let output = engine.render_to_text();
        
        assert!(output.contains("Hello World"));
        assert!(output.contains("<h1>"));
    }

    #[test]
    fn test_get_text_content() {
        let mut engine = BrowserEngine::new();
        let html = r#"<html><body><h1>Hello World</h1></body></html>"#;
        
        engine.load_html(html);
        let text = engine.get_text_content();
        
        assert_eq!(text.trim(), "Hello World");
    }

    #[test]
    fn test_engine_lifecycle() {
        let mut engine = BrowserEngine::new();
        
        assert!(!engine.is_running());
        
        engine.start();
        assert!(engine.is_running());
        
        engine.stop();
        assert!(!engine.is_running());
        assert!(!engine.has_document());
    }
}
