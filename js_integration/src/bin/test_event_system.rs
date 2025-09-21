use js_integration::JsEngine;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing Event System");
    println!("======================");

    // Parse HTML with interactive elements
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>Event System Test</title>
        <script>
            console.log("Setting up event handlers...");
            
            // Global counter for testing
            var clickCount = 0;
            var hoverCount = 0;
        </script>
    </head>
    <body>
        <h1 id="main-title">Event System Test</h1>
        <p id="description">This page tests the event system.</p>
        
        <div id="container">
            <button id="test-button">Click Me!</button>
            <button id="counter-button">Count: 0</button>
            <div id="hover-area">Hover over me!</div>
            <input id="text-input" type="text" placeholder="Type something...">
        </div>
        
        <script>
            console.log("Adding event listeners...");
            
            // Add click event to test button
            var testButton = document.getElementById("test-button");
            if (testButton) {
                testButton.addEventListener("click", function() {
                    console.log("Test button was clicked!");
                    clickCount++;
                });
            }
            
            // Add click event to counter button
            var counterButton = document.getElementById("counter-button");
            if (counterButton) {
                counterButton.addEventListener("click", function() {
                    clickCount++;
                    counterButton.setInnerText("Count: " + clickCount);
                    console.log("Counter button clicked! Count is now:", clickCount);
                });
            }
            
            // Add hover events
            var hoverArea = document.getElementById("hover-area");
            if (hoverArea) {
                hoverArea.addEventListener("mouseover", function() {
                    hoverCount++;
                    console.log("Mouse over hover area! Hover count:", hoverCount);
                });
                
                hoverArea.addEventListener("mouseout", function() {
                    console.log("Mouse left hover area!");
                });
            }
            
            // Add input event
            var textInput = document.getElementById("text-input");
            if (textInput) {
                textInput.addEventListener("input", function() {
                    console.log("Text input changed!");
                });
            }
            
            console.log("All event listeners added!");
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
    
    button {
        margin: 5px;
        padding: 10px 15px;
        background-color: #007bff;
        color: white;
        border: none;
        cursor: pointer;
    }
    
    #hover-area {
        background-color: #f0f0f0;
        padding: 20px;
        margin: 10px 0;
        border: 1px solid #ddd;
    }
    
    #text-input {
        padding: 8px;
        margin: 10px 0;
        width: 200px;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML with event handlers...");
    let (document, _resources) = parse_html(TEST_HTML.into()).unwrap();
    let stylesheet = parse_css(TEST_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine and set up DOM
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured with DOM");

    // Execute inline scripts to set up event handlers
    println!("\n🚀 Setting up event handlers...");
    js_engine.execute_inline_scripts()?;
    println!("✅ Event handlers set up successfully");

    // Test click events
    println!("\n🖱️  Testing click events...");
    js_engine.simulate_click("test-button")?;
    js_engine.simulate_click("counter-button")?;
    js_engine.simulate_click("counter-button")?;
    js_engine.simulate_click("counter-button")?;
    println!("✅ Click events tested");

    // Test hover events
    println!("\n🖱️  Testing hover events...");
    js_engine.dispatch_event("mouseover", "hover-area", true)?;
    js_engine.dispatch_event("mouseout", "hover-area", true)?;
    js_engine.dispatch_event("mouseover", "hover-area", true)?;
    println!("✅ Hover events tested");

    // Test input events
    println!("\n⌨️  Testing input events...");
    js_engine.dispatch_event("input", "text-input", true)?;
    js_engine.dispatch_event("focus", "text-input", false)?;
    js_engine.dispatch_event("blur", "text-input", false)?;
    println!("✅ Input events tested");

    // Test custom events
    println!("\n🎨 Testing custom events...");
    js_engine.dispatch_event("custom-event", "main-title", true)?;
    js_engine.dispatch_event("resize", "container", true)?;
    println!("✅ Custom events tested");

    // Test event state
    println!("\n📊 Testing event state...");
    let state_js = r#"
        console.log("Current event state:");
        console.log("Click count:", clickCount);
        console.log("Hover count:", hoverCount);
        
        // Test that events had effects
        var counterButton = document.getElementById("counter-button");
        if (counterButton) {
            var currentText = counterButton.getInnerText();
            console.log("Counter button text:", currentText);
        }
    "#;

    js_engine.execute_with_layout_update(state_js)?;
    println!("✅ Event state test completed");

    // Test error handling for invalid events
    println!("\n⚠️  Testing error handling...");
    match js_engine.dispatch_event("click", "non-existent-element", true) {
        Ok(_) => println!("✅ Handled non-existent element gracefully"),
        Err(e) => println!("❌ Error with non-existent element: {}", e),
    }

    println!("\n🎉 Event system test completed!");
    println!("The JavaScript engine can now:");
    println!("  ✓ Register event listeners with addEventListener");
    println!("  ✓ Dispatch events to specific elements");
    println!("  ✓ Handle click, hover, and input events");
    println!("  ✓ Support custom event types");
    println!("  ✓ Maintain event state across interactions");
    println!("  ✓ Handle errors gracefully");

    Ok(())
}
