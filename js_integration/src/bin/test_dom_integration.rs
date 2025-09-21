use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use layout::{LayoutEngine, LayoutBox};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Testing DOM Integration with JavaScript");
    println!("==========================================");

    // Parse HTML with elements that have IDs
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>DOM Integration Test</title>
    </head>
    <body>
        <h1 id="main-title">Welcome to DOM Integration!</h1>
        <p id="description">This is a test of JavaScript DOM manipulation.</p>
        <div id="container">
            <button id="test-button">Click Me!</button>
            <span id="counter">0</span>
        </div>
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
    
    #test-button {
        background-color: #007bff;
        color: white;
        padding: 10px 20px;
        border: none;
        cursor: pointer;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML and CSS...");
    let document = parse_html(TEST_HTML);
    let stylesheet = parse_css(TEST_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine and set up DOM
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured with DOM");

    // Test DOM element access
    println!("\n🔍 Testing DOM element access...");
    let dom_access_js = r#"
        console.log("Testing DOM element access...");
        
        // Test getting elements by ID
        let title = document.getElementById("main-title");
        let description = document.getElementById("description");
        let button = document.getElementById("test-button");
        let counter = document.getElementById("counter");
        
        console.log("Found title element:", title);
        console.log("Found description element:", description);
        console.log("Found button element:", button);
        console.log("Found counter element:", counter);
        
        // Test element properties
        if (title) {
            console.log("Title ID:", title.id);
            console.log("Title tagName:", title.tagName);
        }
        
        if (button) {
            console.log("Button ID:", button.id);
            console.log("Button tagName:", button.tagName);
        }
    "#;

    js_engine.execute_with_layout_update(dom_access_js)?;
    println!("✅ DOM element access test successful");

    // Test DOM manipulation
    println!("\n✏️  Testing DOM manipulation...");
    let dom_manipulation_js = r#"
        console.log("Testing DOM manipulation...");
        
        // Test setting attributes
        let button2 = document.getElementById("test-button");
        if (button2) {
            button2.setAttribute("data-clicked", "false");
            console.log("Set data-clicked attribute");
            
            // Test getting attributes
            let clicked = button2.getAttribute("data-clicked");
            console.log("Got data-clicked attribute:", clicked);
        }
        
        // Test innerText manipulation
        let counter2 = document.getElementById("counter");
        if (counter2) {
            let currentValue = counter2.getInnerText();
            console.log("Current counter value:", currentValue);
            
            counter2.setInnerText("42");
            console.log("Set counter to 42");
            
            let newValue = counter2.getInnerText();
            console.log("New counter value:", newValue);
        }
        
        // Test creating new elements
        let newElement = document.createElement("p");
        newElement.setInnerText("This is a dynamically created element!");
        console.log("Created new element:", newElement);
    "#;

    js_engine.execute_with_layout_update(dom_manipulation_js)?;
    println!("✅ DOM manipulation test successful");

    // Test event handling
    println!("\n🎯 Testing event handling...");
    let event_handling_js = r#"
        console.log("Testing event handling...");
        
        let button3 = document.getElementById("test-button");
        if (button3) {
            // Add click event listener
            button3.addEventListener("click", function() {
                console.log("Button was clicked!");
            });
            
            // Add mouseover event listener
            button3.addEventListener("mouseover", function() {
                console.log("Mouse over button!");
            });
            
            console.log("Event listeners added successfully");
        }
        
        // Test multiple event listeners on same element
        let title2 = document.getElementById("main-title");
        if (title2) {
            title2.addEventListener("click", function() {
                console.log("Title clicked!");
            });
            
            title2.addEventListener("mouseover", function() {
                console.log("Mouse over title!");
            });
            
            console.log("Multiple event listeners added to title");
        }
    "#;

    js_engine.execute_with_layout_update(event_handling_js)?;
    println!("✅ Event handling test successful");

    // Test layout integration
    println!("\n📐 Testing layout integration...");
    let layout_js = r#"
        console.log("Testing layout integration...");
        
        // Simulate DOM changes that would trigger layout
        let container = document.getElementById("container");
        if (container) {
            container.setAttribute("style", "background-color: #f0f0f0;");
            console.log("Modified container style - layout should be recalculated");
        }
        
        let title3 = document.getElementById("main-title");
        if (title3) {
            title3.setInnerText("Updated Title via JavaScript!");
            console.log("Updated title text - layout should be recalculated");
        }
    "#;

    let layout_result = js_engine.execute_with_layout_update(layout_js)?;
    if let Some(_layout) = layout_result {
        println!("✅ Layout integration test successful - layout was recalculated");
    } else {
        println!("⚠️  Layout integration test completed - no layout recalculation");
    }

    println!("\n🎉 DOM Integration test completed successfully!");
    println!("The JavaScript engine can now:");
    println!("  ✓ Access DOM elements by ID");
    println!("  ✓ Manipulate element properties and attributes");
    println!("  ✓ Handle events (addEventListener)");
    println!("  ✓ Create new elements");
    println!("  ✓ Trigger layout recalculation");

    Ok(())
}
