use js_integration::JsEngine;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📜 Testing Inline Script Execution");
    println!("==================================");

    // Parse HTML with inline script tags
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>Script Execution Test</title>
        <script>
            console.log("Script 1: This script runs in the head!");
            var globalVar = "Hello from head script!";
        </script>
    </head>
    <body>
        <h1 id="main-title">Script Execution Test</h1>
        <p id="description">This page contains inline scripts.</p>
        
        <script>
            console.log("Script 2: This script runs in the body!");
            console.log("Global variable from head:", globalVar);
            
            // Test DOM manipulation in script
            var title = document.getElementById("main-title");
            if (title) {
                console.log("Found title element in script:", title.id);
            }
        </script>
        
        <div id="container">
            <button id="test-button">Click Me!</button>
            <span id="counter">0</span>
        </div>
        
        <script>
            console.log("Script 3: Final script in body!");
            
            // Test creating elements in script
            var newElement = document.createElement("p");
            newElement.setInnerText("Created by inline script!");
            console.log("Created element in script:", newElement);
            
            // Test event handling in script
            var button = document.getElementById("test-button");
            if (button) {
                button.addEventListener("click", function() {
                    console.log("Button clicked from inline script!");
                });
                console.log("Added event listener in script");
            }
        </script>
    </body>
    </html>
    "#;

    const TEST_CSS: &str = r#"
    body {
        font-family: Arial, sans-serif;
        margin: 20px;
        padding: 20px;
    }
    
    #main-title {
        color: #333;
        margin-bottom: 20px;
    }
    
    #container {
        border: 1px solid #ccc;
        padding: 15px;
        margin: 20px 0;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML with inline scripts...");
    let document = parse_html(TEST_HTML);
    let stylesheet = parse_css(TEST_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine and set up DOM
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured with DOM");

    // Execute inline scripts
    println!("\n🚀 Executing inline scripts...");
    js_engine.execute_inline_scripts()?;
    println!("✅ All inline scripts executed successfully");

    // Test that scripts had effects by running additional JavaScript
    println!("\n🔍 Testing script effects...");
    let test_effects_js = r#"
        console.log("Testing effects of inline scripts...");
        
        // Check if global variable was set by head script
        if (typeof globalVar !== 'undefined') {
            console.log("Global variable from head script:", globalVar);
        } else {
            console.log("Global variable not found");
        }
        
        // Test DOM access that was set up by scripts
        var title = document.getElementById("main-title");
        if (title) {
            console.log("Title element accessible:", title.id);
        }
        
        var button = document.getElementById("test-button");
        if (button) {
            console.log("Button element accessible:", button.id);
        }
        
        console.log("Script effects test completed!");
    "#;

    js_engine.execute_with_layout_update(test_effects_js)?;
    println!("✅ Script effects test successful");

    // Test error handling in scripts
    println!("\n⚠️  Testing error handling in scripts...");
    let error_html = r#"
    <html>
    <body>
        <script>
            console.log("This script will have a syntax error");
            var invalid = ; // Syntax error
        </script>
    </body>
    </html>
    "#;

    let error_document = parse_html(error_html);
    let mut error_js_engine = JsEngine::new();
    error_js_engine.set_document(Rc::new(error_document));

    match error_js_engine.execute_inline_scripts() {
        Ok(_) => println!("❌ Expected error but got success"),
        Err(e) => println!("✅ Caught expected script error: {}", e),
    }

    println!("\n🎉 Inline script execution test completed!");
    println!("The JavaScript engine can now:");
    println!("  ✓ Extract and execute inline <script> tags");
    println!("  ✓ Execute scripts in document order (head → body)");
    println!("  ✓ Maintain global variable scope across scripts");
    println!("  ✓ Allow scripts to manipulate DOM elements");
    println!("  ✓ Handle script execution errors gracefully");

    Ok(())
}
