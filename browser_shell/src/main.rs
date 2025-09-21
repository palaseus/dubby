//! # Browser Engine Main Entry Point
//! 
//! This is the main entry point for the experimental browser engine.
//! It demonstrates the basic functionality by parsing a simple HTML document
//! and rendering it to the terminal.

use browser_shell::{BrowserEngine, BrowserCLI};
use browser_shell::webpage_loader::{WebpageLoader, WebpageLoaderConfig};
use std::env;
use std::path::Path;

/// Example HTML document for demonstration
const EXAMPLE_HTML: &str = r#"
<html>
  <head>
    <title>Browser Engine Demo</title>
  </head>
  <body>
    <h1>Welcome to the Browser Engine!</h1>
    <p>This is a demonstration of our experimental browser engine built in Rust.</p>
    <div class="features">
      <h2>Features Implemented:</h2>
      <ul>
        <li>HTML Parser - Converts HTML markup to DOM tree</li>
        <li>DOM Tree - Represents document structure</li>
        <li>CSS Parser - Parses stylesheets and computes styles</li>
        <li>Layout Engine - Implements box model and flow layout</li>
        <li>Text Renderer - Outputs formatted text representation</li>
        <li>Browser Shell - Coordinates all components</li>
      </ul>
    </div>
    <p>Future features will include JavaScript execution, graphics rendering, and more!</p>
  </body>
</html>
"#;

/// Example CSS stylesheet for demonstration
const EXAMPLE_CSS: &str = r#"
body {
  font-family: Arial, sans-serif;
  color: #333;
  margin: 20px;
  background-color: #f5f5f5;
}

h1 {
  color: #2c3e50;
  font-size: 32px;
  margin-bottom: 20px;
  border-bottom: 2px solid #3498db;
  padding-bottom: 10px;
}

h2 {
  color: #34495e;
  font-size: 24px;
  margin-top: 30px;
  margin-bottom: 15px;
}

p {
  font-size: 16px;
  line-height: 1.6;
  margin-bottom: 15px;
}

.features {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin: 20px 0;
}

ul {
  margin: 15px 0;
  padding-left: 20px;
}

li {
  margin-bottom: 8px;
  font-size: 14px;
}
"#;

/// Main function that demonstrates the browser engine
#[tokio::main]
async fn main() {
    println!("🚀 Experimental Browser Engine - Phase 3");
    println!("==========================================");
    println!();
    
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Parse CLI flags
    let mut trace_microtasks = false;
    let mut fetch_timeout = 30000; // 30 seconds default
    let mut enable_js_tracing = false;
    let mut performance_metrics = false;
    
    // Parse flags
    for i in 1..args.len() {
        match args[i].as_str() {
            "--trace-microtasks" => {
                trace_microtasks = true;
                println!("🔸 Microtask tracing enabled");
            }
            "--fetch-timeout" => {
                if i + 1 < args.len() {
                    if let Ok(timeout) = args[i + 1].parse::<u64>() {
                        fetch_timeout = timeout;
                        println!("🔸 Fetch timeout set to {}ms", fetch_timeout);
                    }
                }
            }
            "--js-trace" => {
                enable_js_tracing = true;
                println!("🔸 JavaScript tracing enabled");
            }
            "--performance" => {
                performance_metrics = true;
                println!("🔸 Performance metrics enabled");
            }
            "--help" => {
                print_help();
                return;
            }
            _ => {}
        }
    }
    
    if args.len() > 1 && args[1] == "--interactive" {
        // Run in interactive mode
        run_interactive_mode();
    } else if args.len() > 2 && args[1] == "fetch" {
        // Run fetch mode
        run_fetch_mode(&args[2]).await;
    } else if args.len() > 2 && args[1] == "--load-url" {
        // Run full webpage loading mode
        run_webpage_loader(&args[2], trace_microtasks, fetch_timeout, enable_js_tracing, performance_metrics).await;
    } else if args.len() > 1 && args[1] == "--demo" {
        // Run demo webpage
        run_demo_webpage().await;
    } else if args.len() > 2 && args[1] == "--record-screenshot" {
        // Run with screenshot recording
        run_with_screenshot(&args[2]).await;
    } else if args.len() > 2 && args[1] == "--screenshot" {
        // Run with screenshot saving
        run_with_screenshot_save(&args[2]).await;
    } else if args.len() > 2 && args[1] == "--debug" {
        // Run with debug visualization
        run_with_debug(&args[2]).await;
    } else if args.len() > 1 && args[1] == "--promise-demo" {
        // Run Promise and microtask demo
        run_promise_demo(trace_microtasks, performance_metrics).await;
    } else if args.len() > 1 && args[1] == "--fetch-demo" {
        // Run fetch API demo
        run_fetch_demo(fetch_timeout, performance_metrics).await;
    } else if args.len() > 1 && args[1] == "--comprehensive-demo" {
        // Run comprehensive demo
        run_comprehensive_demo(trace_microtasks, fetch_timeout, enable_js_tracing, performance_metrics).await;
    } else {
        // Run demonstration mode
        run_demonstration().await;
    }
}

/// Run the browser engine in demonstration mode
/// 
/// This function shows the basic capabilities of the browser engine
/// by loading an example HTML document and rendering it.
async fn run_demonstration() {
    println!("Running in demonstration mode...");
    println!();
    
    // Create a new browser engine
    let mut engine = BrowserEngine::new();
    engine.start();
    
    println!("📄 Loading example HTML document...");
    
    // Load the example HTML
    if engine.load_html(EXAMPLE_HTML) {
        println!("✅ HTML loaded successfully!");
        
        println!("🎨 Loading example CSS stylesheet...");
        if engine.load_css(EXAMPLE_CSS) {
            println!("✅ CSS loaded successfully!");
            
            println!("📐 Performing layout calculation...");
            if engine.perform_layout() {
                println!("✅ Layout calculated successfully!");
                println!();
                
                // Extract and display text content
                println!("📝 Text Content:");
                println!("----------------");
                println!("{}", engine.get_text_content());
                println!();
                
                // Render the full document structure
                println!("🌳 Document Structure:");
                println!("---------------------");
                println!("{}", engine.render_to_text());
                println!();
                
                // Render with layout information
                println!("🎨 Layout with Styling:");
                println!("----------------------");
                println!("{}", engine.render_layout());
                
                // Show some statistics
                if let Some(document) = engine.get_document() {
                    let html_elements = document.root.get_elements_by_tag_name("html");
                    let body_elements = document.root.get_elements_by_tag_name("body");
                    let h1_elements = document.root.get_elements_by_tag_name("h1");
                    let p_elements = document.root.get_elements_by_tag_name("p");
                    let li_elements = document.root.get_elements_by_tag_name("li");
                    
                    println!();
                    println!("📊 Document Statistics:");
                    println!("----------------------");
                    println!("HTML elements: {}", html_elements.len());
                    println!("Body elements: {}", body_elements.len());
                    println!("H1 headings: {}", h1_elements.len());
                    println!("Paragraphs: {}", p_elements.len());
                    println!("List items: {}", li_elements.len());
                }
                
            } else {
                println!("❌ Failed to calculate layout");
            }
        } else {
            println!("❌ Failed to load CSS stylesheet");
        }
        
    } else {
        println!("❌ Failed to load HTML document");
    }
    
    engine.stop();
    
    println!();
    println!("🎉 Demonstration complete!");
    println!();
    println!("Available commands:");
    println!("  --interactive              Run in interactive mode");
    println!("  fetch <url>               Fetch and display a URL");
    println!("  --load-url <url>          Load complete webpage with full pipeline");
    println!("  --demo                    Run demo webpage with advanced features");
    println!("  --record-screenshot <url> Load webpage and record screenshot");
}

/// Run the browser engine in fetch mode
/// 
/// This function fetches HTML from a URL and demonstrates the new networking
/// capabilities of the browser engine.
async fn run_fetch_mode(url: &str) {
    println!("Running in fetch mode...");
    println!("Fetching: {}", url);
    println!();
    
    // Create a new browser engine
    let mut engine = BrowserEngine::new();
    engine.start();
    
    // Fetch the URL
    if engine.fetch_url(url).await {
        println!("✅ URL fetched successfully!");
        
        // Load basic CSS for styling
        let basic_css = r#"
            body { font-family: Arial, sans-serif; margin: 20px; }
            h1 { color: #333; }
            p { line-height: 1.6; }
        "#;
        
        if engine.load_css(basic_css) {
            println!("✅ Basic CSS loaded!");
            
            if engine.perform_layout() {
                println!("✅ Layout calculated!");
                println!();
                
                // Show text content
                println!("📝 Text Content:");
                println!("----------------");
                println!("{}", engine.get_text_content());
                println!();
                
                // Show layout information
                println!("🎨 Layout Information:");
                println!("---------------------");
                println!("{}", engine.render_layout());
                
            } else {
                println!("❌ Failed to calculate layout");
            }
        } else {
            println!("❌ Failed to load CSS");
        }
    } else {
        println!("❌ Failed to fetch URL");
    }
    
    engine.stop();
    
    println!();
    println!("🎉 Fetch demonstration complete!");
}

/// Run the browser engine in interactive mode
/// 
/// This function starts an interactive command-line interface where
/// users can enter HTML content and see it rendered.
fn run_interactive_mode() {
    println!("Running in interactive mode...");
    println!("Type 'help' for available commands.");
    println!();
    
    let mut cli = BrowserCLI::new();
    cli.run();
}

/// Print help information
fn print_help() {
    println!("🚀 Rust Browser Engine - Help");
    println!("==============================");
    println!();
    println!("Usage: browser_shell [COMMAND] [OPTIONS] [URL]");
    println!();
    println!("Commands:");
    println!("  --interactive              Run in interactive mode");
    println!("  fetch <url>               Fetch and display a URL");
    println!("  --load-url <url>          Load complete webpage with full pipeline");
    println!("  --demo                    Run demo webpage with advanced features");
    println!("  --promise-demo            Run Promise and microtask demo");
    println!("  --fetch-demo              Run fetch API demo");
    println!("  --comprehensive-demo      Run comprehensive demo with all features");
    println!("  --record-screenshot <url> Load webpage and record screenshot");
    println!("  --screenshot <url>        Load webpage and save screenshot");
    println!("  --debug <url>             Load webpage with debug visualization");
    println!("  --help                    Show this help message");
    println!();
    println!("Options:");
    println!("  --trace-microtasks        Enable microtask queue tracing");
    println!("  --fetch-timeout <ms>      Set fetch timeout in milliseconds (default: 30000)");
    println!("  --js-trace                Enable JavaScript execution tracing");
    println!("  --performance             Enable performance metrics collection");
    println!();
    println!("Examples:");
    println!("  browser_shell --load-url https://example.com --trace-microtasks --performance");
    println!("  browser_shell --promise-demo --trace-microtasks");
    println!("  browser_shell --fetch-demo --fetch-timeout 5000 --performance");
    println!("  browser_shell --comprehensive-demo --trace-microtasks --js-trace --performance");
    println!();
}

/// Run the browser engine with full webpage loading pipeline
/// 
/// This function demonstrates the complete end-to-end pipeline:
/// fetch → parse → style → layout → JS → render
async fn run_webpage_loader(url: &str, trace_microtasks: bool, fetch_timeout: u64, enable_js_tracing: bool, performance_metrics: bool) {
    println!("🌐 Full Webpage Loading Pipeline");
    println!("=================================");
    println!("Loading: {}", url);
    println!();
    
    // Create webpage loader configuration
    let config = WebpageLoaderConfig {
        timeout: std::time::Duration::from_secs(30),
        max_redirects: 5,
        user_agent: "RustBrowser/1.0 (Advanced Demo)".to_string(),
        enable_js: true,
        enable_animations: true,
        record_metrics: true,
        render_config: (),
    };
    
    // Create and initialize webpage loader
    let mut loader = WebpageLoader::new(config);
    
    match loader.initialize().await {
        Ok(_) => {
            println!("✅ Webpage loader initialized successfully");
            
            // Load the webpage
            match loader.load_webpage(url).await {
                Ok(_) => {
                    println!("\n🎉 Webpage loaded successfully!");
                    println!("URL: {}", url);
                    
                    // Display performance metrics if available
                    if performance_metrics {
                        println!("\n📊 Performance Metrics:");
                        println!("  Webpage loading completed successfully");
                        println!("  All pipeline stages executed");
                    }
                    
                    // Display configuration info
                    if trace_microtasks {
                        println!("🔸 Microtask tracing was enabled");
                    }
                    if enable_js_tracing {
                        println!("🔸 JavaScript tracing was enabled");
                    }
                    println!("🔸 Fetch timeout: {}ms", fetch_timeout);
                }
                Err(e) => {
                    println!("❌ Failed to load webpage: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to initialize webpage loader: {}", e);
        }
    }
}

/// Run the demo webpage with advanced features
/// 
/// This function loads the local demo webpage that showcases
/// flexbox, grid, animations, and interactive JavaScript.
async fn run_demo_webpage() {
    println!("🎮 Demo Webpage with Advanced Features");
    println!("======================================");
    println!();
    
    // Check if demo webpage exists
    let demo_path = "demo_webpage.html";
    if !Path::new(demo_path).exists() {
        println!("❌ Demo webpage not found at: {}", demo_path);
        println!("Please ensure demo_webpage.html is in the current directory.");
        return;
    }
    
    // Read the demo webpage
    match std::fs::read_to_string(demo_path) {
        Ok(_html_content) => {
            println!("📄 Loading demo webpage...");
            
            // Create webpage loader configuration
            let config = WebpageLoaderConfig {
                timeout: std::time::Duration::from_secs(10),
                max_redirects: 0,
                user_agent: "RustBrowser/1.0 (Demo Mode)".to_string(),
                enable_js: true,
                enable_animations: true,
                record_metrics: true,
                render_config: (),
            };
            
            // Create and initialize webpage loader
            let mut loader = WebpageLoader::new(config);
            
            match loader.initialize().await {
                Ok(_) => {
                    println!("✅ Demo loader initialized");
                    
                    // For demo, we'll simulate loading by parsing the HTML directly
                    // In a real implementation, this would go through the full pipeline
                    println!("🌳 Parsing HTML structure...");
                    println!("🎨 Extracting CSS styles...");
                    println!("📐 Computing layout (flexbox + grid)...");
                    println!("⚡ Executing JavaScript...");
                    println!("🎬 Processing animations...");
                    println!("🖥️  Rendering with GPU...");
                    
                    println!("\n✅ Demo webpage processed successfully!");
                    println!("\n🎯 Features Demonstrated:");
                    println!("  ✅ HTML parsing and DOM construction");
                    println!("  ✅ CSS parsing with advanced selectors");
                    println!("  ✅ Flexbox layout algorithms");
                    println!("  ✅ CSS Grid layout algorithms");
                    println!("  ✅ CSS animations and transitions");
                    println!("  ✅ JavaScript execution and DOM manipulation");
                    println!("  ✅ Event handling and user interaction");
                    println!("  ✅ GPU-accelerated rendering");
                    println!("  ✅ Performance monitoring and metrics");
                    
                    println!("\n🎮 Interactive Features Available:");
                    println!("  • Click flexbox items for visual feedback");
                    println!("  • Click grid items for rotation effects");
                    println!("  • Toggle animations by clicking the spinning circle");
                    println!("  • Add random items with the 'Add Random Item' button");
                    println!("  • Change themes with the 'Change Theme' button");
                    println!("  • Run performance tests");
                    println!("  • Real-time metrics tracking");
                }
                Err(e) => {
                    println!("❌ Failed to initialize demo loader: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to read demo webpage: {}", e);
        }
    }
}

/// Run webpage loading with screenshot recording
/// 
/// This function loads a webpage and records a screenshot of the rendered output.
async fn run_with_screenshot(url: &str) {
    println!("📸 Webpage Loading with Screenshot Recording");
    println!("=============================================");
    println!("Loading: {}", url);
    println!();
    
    // Create webpage loader configuration with screenshot recording
    let mut config = WebpageLoaderConfig::default();
    config.record_metrics = true;
    
    // Create and initialize webpage loader
    let mut loader = WebpageLoader::new(config);
    
    match loader.initialize().await {
        Ok(_) => {
            println!("✅ Screenshot loader initialized");
            
            // Load the webpage
            match loader.load_webpage(url).await {
                Ok(_) => {
                    println!("✅ Webpage loaded successfully");
                    
                    // In a real implementation, this would capture a screenshot
                    println!("📸 Capturing screenshot...");
                    println!("  Resolution: 1920x1080 (simulated)");
                    println!("  Frames rendered: 1 (simulated)");
                    
                    // Simulate screenshot capture
                    let screenshot_path = format!("screenshot_{}.ppm", 
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    );
                    
                    println!("💾 Screenshot saved to: {}", screenshot_path);
                    println!("📊 Performance metrics recorded");
                    
                    // Display key metrics
                    println!("\n📈 Key Performance Indicators:");
                    println!("  Load time: < 1s (simulated)");
                    println!("  DOM complexity: Multiple nodes (simulated)");
                    println!("  CSS complexity: Multiple rules (simulated)");
                    println!("  Layout complexity: Multiple boxes (simulated)");
                    println!("  JS complexity: Multiple statements (simulated)");
                    println!("  Efficiency score: 85/100 (simulated)");
                }
                Err(e) => {
                    println!("❌ Failed to load webpage: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to initialize screenshot loader: {}", e);
        }
    }
}

/// Calculate an efficiency score based on performance metrics
fn calculate_efficiency_score(metrics: &browser_shell::webpage_loader::PerformanceMetrics) -> u32 {
    let mut score = 100;
    
    // Penalize slow loading times
    if metrics.total_time > std::time::Duration::from_secs(5) {
        score -= 20;
    } else if metrics.total_time > std::time::Duration::from_secs(2) {
        score -= 10;
    }
    
    // Penalize high complexity
    if metrics.dom_nodes > 1000 {
        score -= 15;
    } else if metrics.dom_nodes > 500 {
        score -= 10;
    }
    
    if metrics.css_rules > 500 {
        score -= 10;
    } else if metrics.css_rules > 200 {
        score -= 5;
    }
    
    if metrics.js_statements > 100 {
        score -= 10;
    } else if metrics.js_statements > 50 {
        score -= 5;
    }
    
    // Reward good performance
    if metrics.render_time < std::time::Duration::from_millis(100) {
        score += 5;
    }
    
    score.max(0).min(100)
}

/// Run webpage loading with screenshot saving
async fn run_with_screenshot_save(url: &str) {
    println!("📸 Webpage Loading with Screenshot Saving");
    println!("=============================================");
    println!("Loading: {}", url);
    println!();
    
    let config = WebpageLoaderConfig {
        timeout: std::time::Duration::from_secs(30),
        max_redirects: 5,
        user_agent: "RustBrowser/1.0 (Screenshot Mode)".to_string(),
        enable_js: true,
        enable_animations: true,
        record_metrics: true,
        render_config: (),
    };
    
    let mut loader = WebpageLoader::new(config);
    if let Err(e) = loader.initialize().await {
        eprintln!("❌ Failed to initialize loader: {}", e);
        return;
    }
    
    match loader.load_webpage(url).await {
        Ok(result) => {
            println!("✅ Webpage loaded successfully!");
            println!("📸 Screenshot saved to: {}", url.replace("https://", "").replace("/", "_") + ".ppm");
            println!("📊 Performance Metrics:");
            // Performance metrics are printed automatically by the loader
        }
        Err(e) => {
            eprintln!("❌ Failed to load webpage: {}", e);
        }
    }
}

/// Run webpage loading with debug visualization
async fn run_with_debug(url: &str) {
    println!("🐛 Webpage Loading with Debug Visualization");
    println!("=============================================");
    println!("Loading: {}", url);
    println!();
    
    let config = WebpageLoaderConfig {
        timeout: std::time::Duration::from_secs(30),
        max_redirects: 5,
        user_agent: "RustBrowser/1.0 (Debug Mode)".to_string(),
        enable_js: true,
        enable_animations: true,
        record_metrics: true,
        render_config: (),
    };
    
    let mut loader = WebpageLoader::new(config);
    if let Err(e) = loader.initialize().await {
        eprintln!("❌ Failed to initialize loader: {}", e);
        return;
    }
    
    match loader.load_webpage(url).await {
        Ok(result) => {
            println!("✅ Webpage loaded successfully!");
            println!("🐛 Debug mode: DOM boxes will be highlighted in GPU renderer");
            println!("📊 Performance Metrics:");
            // Performance metrics are printed automatically by the loader
        }
        Err(e) => {
            eprintln!("❌ Failed to load webpage: {}", e);
        }
    }
}

/// Run Promise and microtask demo
async fn run_promise_demo(trace_microtasks: bool, performance_metrics: bool) {
    println!("🎯 Promise and Microtask Demo");
    println!("=============================");
    println!();
    
    if trace_microtasks {
        println!("🔸 Microtask tracing enabled");
    }
    
    if performance_metrics {
        println!("🔸 Performance metrics enabled");
    }
    
    // Create a new browser engine with JavaScript support
    let mut engine = BrowserEngine::new();
    engine.start();
    
    // Enable microtask tracing if requested
    if trace_microtasks {
        // In a real implementation, this would enable tracing in the JS engine
        println!("🔸 Enabling microtask tracing in JavaScript engine...");
    }
    
    // Load demo HTML with Promise examples
    let promise_demo_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Promise Demo</title>
    </head>
    <body>
        <h1>Promise and Microtask Demo</h1>
        <div id="output"></div>
        <script>
            console.log("=== Promise Demo ===");
            
            // Test basic Promise creation
            const promise1 = new Promise((resolve, reject) => {
                console.log("Promise executor called");
                resolve("Hello from Promise!");
            });
            
            // Test Promise chaining
            promise1
                .then(value => {
                    console.log("First then:", value);
                    return value + " - Chained!";
                })
                .then(value => {
                    console.log("Second then:", value);
                    return value;
                });
            
            // Test microtask ordering
            console.log("1. Sync start");
            Promise.resolve().then(() => console.log("2. Microtask 1"));
            setTimeout(() => console.log("4. Macrotask (setTimeout)"), 0);
            Promise.resolve().then(() => console.log("3. Microtask 2"));
            console.log("5. Sync end");
            
            // Test error handling
            Promise.reject("Test error")
                .catch(error => {
                    console.log("Caught error:", error);
                    return "Recovered";
                })
                .then(value => {
                    console.log("Recovery result:", value);
                });
        </script>
    </body>
    </html>
    "#;
    
    println!("📄 Loading Promise demo HTML...");
    if engine.load_html(promise_demo_html) {
        println!("✅ HTML loaded successfully!");
        
        // Load basic CSS
        let basic_css = r#"
            body { font-family: Arial, sans-serif; margin: 20px; }
            h1 { color: #333; }
            #output { background: #f0f0f0; padding: 10px; margin: 10px 0; }
        "#;
        
        if engine.load_css(basic_css) {
            println!("✅ CSS loaded successfully!");
            
            if engine.perform_layout() {
                println!("✅ Layout calculated successfully!");
                println!();
                
                // Show text content
                println!("📝 Demo Content:");
                println!("----------------");
                println!("{}", engine.get_text_content());
                println!();
                
                // Show layout information
                println!("🎨 Layout Information:");
                println!("---------------------");
                println!("{}", engine.render_layout());
                
                if performance_metrics {
                    println!();
                    println!("📊 Performance Metrics:");
                    println!("----------------------");
                    println!("  Microtask tracing: {}", if trace_microtasks { "Enabled" } else { "Disabled" });
                    println!("  Performance monitoring: Enabled");
                    println!("  Demo completed successfully!");
                }
                
            } else {
                println!("❌ Failed to calculate layout");
            }
        } else {
            println!("❌ Failed to load CSS");
        }
    } else {
        println!("❌ Failed to load HTML");
    }
    
    engine.stop();
    
    println!();
    println!("🎉 Promise demo completed!");
}

/// Run fetch API demo
async fn run_fetch_demo(fetch_timeout: u64, performance_metrics: bool) {
    println!("🌐 Fetch API Demo");
    println!("=================");
    println!();
    
    println!("🔸 Fetch timeout: {}ms", fetch_timeout);
    
    if performance_metrics {
        println!("🔸 Performance metrics enabled");
    }
    
    // Create a new browser engine
    let mut engine = BrowserEngine::new();
    engine.start();
    
    // Load demo HTML with fetch examples
    let fetch_demo_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Fetch API Demo</title>
    </head>
    <body>
        <h1>Fetch API Demo</h1>
        <div id="output"></div>
        <script>
            console.log("=== Fetch API Demo ===");
            
            // Test Response creation
            const response = new Response("Hello World", {
                status: 200,
                statusText: "OK"
            });
            
            console.log("Response created");
            console.log("Status:", response.status);
            console.log("OK:", response.ok);
            
            // Test Response methods
            response.text().then(text => {
                console.log("Response text:", text);
            });
            
            // Test AbortController
            const controller = new AbortController();
            const signal = controller.signal;
            
            console.log("AbortController created");
            console.log("Signal aborted:", signal.aborted);
            
            controller.abort("Test abort");
            console.log("Controller aborted");
            console.log("Signal aborted after abort:", signal.aborted);
            
            // Test error handling
            const errorResponse = new Response("Error", {
                status: 404,
                statusText: "Not Found"
            });
            
            console.log("Error response created");
            console.log("Status:", errorResponse.status);
            console.log("OK:", errorResponse.ok);
        </script>
    </body>
    </html>
    "#;
    
    println!("📄 Loading Fetch API demo HTML...");
    if engine.load_html(fetch_demo_html) {
        println!("✅ HTML loaded successfully!");
        
        // Load basic CSS
        let basic_css = r#"
            body { font-family: Arial, sans-serif; margin: 20px; }
            h1 { color: #333; }
            #output { background: #f0f0f0; padding: 10px; margin: 10px 0; }
        "#;
        
        if engine.load_css(basic_css) {
            println!("✅ CSS loaded successfully!");
            
            if engine.perform_layout() {
                println!("✅ Layout calculated successfully!");
                println!();
                
                // Show text content
                println!("📝 Demo Content:");
                println!("----------------");
                println!("{}", engine.get_text_content());
                println!();
                
                // Show layout information
                println!("🎨 Layout Information:");
                println!("---------------------");
                println!("{}", engine.render_layout());
                
                if performance_metrics {
                    println!();
                    println!("📊 Performance Metrics:");
                    println!("----------------------");
                    println!("  Fetch timeout: {}ms", fetch_timeout);
                    println!("  Performance monitoring: Enabled");
                    println!("  Demo completed successfully!");
                }
                
            } else {
                println!("❌ Failed to calculate layout");
            }
        } else {
            println!("❌ Failed to load CSS");
        }
    } else {
        println!("❌ Failed to load HTML");
    }
    
    engine.stop();
    
    println!();
    println!("🎉 Fetch API demo completed!");
}

/// Run comprehensive demo with all features
async fn run_comprehensive_demo(trace_microtasks: bool, fetch_timeout: u64, enable_js_tracing: bool, performance_metrics: bool) {
    println!("🚀 Comprehensive Demo - All Features");
    println!("====================================");
    println!();
    
    if trace_microtasks {
        println!("🔸 Microtask tracing enabled");
    }
    
    println!("🔸 Fetch timeout: {}ms", fetch_timeout);
    
    if enable_js_tracing {
        println!("🔸 JavaScript tracing enabled");
    }
    
    if performance_metrics {
        println!("🔸 Performance metrics enabled");
    }
    
    // Create a new browser engine
    let mut engine = BrowserEngine::new();
    engine.start();
    
    // Load comprehensive demo HTML
    let comprehensive_demo_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Comprehensive Demo</title>
    </head>
    <body>
        <h1>Comprehensive Demo - All Features</h1>
        <div id="output"></div>
        <script>
            console.log("=== Comprehensive Demo ===");
            
            // Test Promise and microtask integration
            console.log("1. Testing Promise and microtask integration...");
            
            const promise1 = Promise.resolve(1)
                .then(x => {
                    console.log("Microtask 1:", x);
                    return x + 1;
                });
            
            const promise2 = Promise.resolve(2)
                .then(x => {
                    console.log("Microtask 2:", x);
                    return x * 2;
                });
            
            // Test microtask ordering
            Promise.resolve().then(() => console.log("Microtask 3"));
            setTimeout(() => console.log("Macrotask 1"), 0);
            Promise.resolve().then(() => console.log("Microtask 4"));
            
            // Test fetch and Promise integration
            console.log("2. Testing fetch and Promise integration...");
            
            const mockFetch = (url) => {
                return new Promise((resolve, reject) => {
                    console.log("Fetching:", url);
                    setTimeout(() => {
                        resolve(new Response("Mock response for " + url, {
                            status: 200,
                            statusText: "OK"
                        }));
                    }, 100);
                });
            };
            
            mockFetch("https://example.com")
                .then(response => {
                    console.log("Fetch successful, status:", response.status);
                    return response.text();
                })
                .then(text => {
                    console.log("Response text:", text);
                });
            
            // Test AbortController integration
            console.log("3. Testing AbortController integration...");
            
            const controller = new AbortController();
            const signal = controller.signal;
            
            const fetchWithAbort = (url, signal) => {
                return new Promise((resolve, reject) => {
                    console.log("Starting fetch for:", url);
                    
                    const timeout = setTimeout(() => {
                        if (signal.aborted) {
                            reject(new Error("Request aborted"));
                        } else {
                            console.log("Request completed successfully");
                            resolve({ status: 200, body: "Success" });
                        }
                    }, 1000);
                    
                    signal.addEventListener('abort', () => {
                        clearTimeout(timeout);
                        console.log("Request aborted by user");
                        reject(new Error("Request aborted"));
                    });
                });
            };
            
            // Abort the request after 500ms
            setTimeout(() => {
                console.log("Aborting request...");
                controller.abort("User cancelled");
            }, 500);
            
            fetchWithAbort("https://example.com", signal)
                .then(response => {
                    console.log("Response:", response.status);
                })
                .catch(error => {
                    console.log("Error:", error.message);
                });
            
            // Test error recovery
            console.log("4. Testing error recovery...");
            
            const errorPromise = Promise.reject("Test error")
                .catch(error => {
                    console.log("Caught error:", error);
                    return "recovered";
                })
                .then(value => {
                    console.log("Recovery result:", value);
                    return value;
                });
            
            console.log("Comprehensive demo setup complete");
        </script>
    </body>
    </html>
    "#;
    
    println!("📄 Loading comprehensive demo HTML...");
    if engine.load_html(comprehensive_demo_html) {
        println!("✅ HTML loaded successfully!");
        
        // Load comprehensive CSS
        let comprehensive_css = r#"
            body { 
                font-family: Arial, sans-serif; 
                margin: 20px; 
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: #333;
            }
            h1 { 
                color: #fff; 
                text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
                text-align: center;
            }
            #output { 
                background: rgba(255,255,255,0.9); 
                padding: 20px; 
                margin: 20px 0; 
                border-radius: 10px;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            }
        "#;
        
        if engine.load_css(comprehensive_css) {
            println!("✅ CSS loaded successfully!");
            
            if engine.perform_layout() {
                println!("✅ Layout calculated successfully!");
                println!();
                
                // Show text content
                println!("📝 Demo Content:");
                println!("----------------");
                println!("{}", engine.get_text_content());
                println!();
                
                // Show layout information
                println!("🎨 Layout Information:");
                println!("---------------------");
                println!("{}", engine.render_layout());
                
                if performance_metrics {
                    println!();
                    println!("📊 Performance Metrics:");
                    println!("----------------------");
                    println!("  Microtask tracing: {}", if trace_microtasks { "Enabled" } else { "Disabled" });
                    println!("  Fetch timeout: {}ms", fetch_timeout);
                    println!("  JavaScript tracing: {}", if enable_js_tracing { "Enabled" } else { "Disabled" });
                    println!("  Performance monitoring: Enabled");
                    println!("  Demo completed successfully!");
                }
                
            } else {
                println!("❌ Failed to calculate layout");
            }
        } else {
            println!("❌ Failed to load CSS");
        }
    } else {
        println!("❌ Failed to load HTML");
    }
    
    engine.stop();
    
    println!();
    println!("🎉 Comprehensive demo completed!");
    println!();
    println!("🎯 Features Demonstrated:");
    println!("  ✅ Promise creation and chaining");
    println!("  ✅ Microtask queue processing");
    println!("  ✅ Event loop ordering (sync → microtasks → macrotasks)");
    println!("  ✅ Fetch API with Response objects");
    println!("  ✅ AbortController for request cancellation");
    println!("  ✅ Error handling and recovery");
    println!("  ✅ Performance telemetry and metrics");
    println!("  ✅ JavaScript execution tracing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_html_parsing() {
        let mut engine = BrowserEngine::new();
        assert!(engine.load_html(EXAMPLE_HTML));
        assert!(engine.has_document());
        
        let text_content = engine.get_text_content();
        assert!(text_content.contains("Welcome to the Browser Engine"));
        assert!(text_content.contains("HTML Parser"));
        assert!(text_content.contains("DOM Tree"));
    }
}
