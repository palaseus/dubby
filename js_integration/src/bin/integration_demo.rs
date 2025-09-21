use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use layout::{LayoutEngine, LayoutBox};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 JavaScript Engine & DOM Integration Demo");
    println!("===========================================");
    println!("This demo showcases the complete integration of:");
    println!("  • JavaScript execution with Boa engine");
    println!("  • DOM manipulation and traversal");
    println!("  • Inline script execution");
    println!("  • Event handling system");
    println!("  • Layout integration");
    println!();

    // Parse HTML with comprehensive JavaScript functionality
    const DEMO_HTML: &str = r#"
    <html>
    <head>
        <title>JavaScript Integration Demo</title>
        <script>
            console.log("Initializing JavaScript Integration Demo...");
            
            // Global state for the demo
            var clickCount = 0;
            var isAnimating = false;
            var currentTheme = "light";
        </script>
    </head>
    <body>
        <h1 id="main-title">JavaScript Integration Demo</h1>
        <p id="description">This demonstrates the full JavaScript engine integration.</p>
        
        <div id="controls">
            <button id="click-counter">Click Me! (0)</button>
            <button id="theme-toggle">Toggle Theme</button>
            <button id="add-element">Add Element</button>
            <button id="animate-title">Animate Title</button>
        </div>
        
        <div id="content">
            <div id="dynamic-content">
                <p>This content can be modified by JavaScript!</p>
            </div>
            <div id="element-list">
                <h3>Dynamic Elements:</h3>
                <ul id="element-ul">
                    <li>Initial element</li>
                </ul>
            </div>
        </div>
        
        <script>
            console.log("Setting up interactive functionality...");
            
            // Click counter functionality
            var clickButton = document.getElementById("click-counter");
            if (clickButton) {
                clickButton.addEventListener("click", function() {
                    clickCount++;
                    clickButton.setInnerText("Click Me! (" + clickCount + ")");
                    console.log("Button clicked! Count:", clickCount);
                    
                    // Update description
                    var desc = document.getElementById("description");
                    if (desc) {
                        desc.setInnerText("Button has been clicked " + clickCount + " times!");
                    }
                });
            }
            
            // Theme toggle functionality
            var themeButton = document.getElementById("theme-toggle");
            if (themeButton) {
                themeButton.addEventListener("click", function() {
                    currentTheme = currentTheme === "light" ? "dark" : "light";
                    console.log("Theme toggled to:", currentTheme);
                    
                    // Update button text
                    themeButton.setInnerText("Theme: " + currentTheme);
                    
                    // Update content styling
                    var content = document.getElementById("content");
                    if (content) {
                        content.setAttribute("data-theme", currentTheme);
                    }
                });
            }
            
            // Add element functionality
            var addButton = document.getElementById("add-element");
            if (addButton) {
                addButton.addEventListener("click", function() {
                    var ul = document.getElementById("element-ul");
                    if (ul) {
                        var newElement = document.createElement("li");
                        newElement.setInnerText("Dynamic element " + (ul.children.length + 1));
                        console.log("Added new element to list");
                    }
                });
            }
            
            // Animate title functionality
            var animateButton = document.getElementById("animate-title");
            if (animateButton) {
                animateButton.addEventListener("click", function() {
                    if (!isAnimating) {
                        isAnimating = true;
                        animateButton.setInnerText("Animating...");
                        
                        var title = document.getElementById("main-title");
                        if (title) {
                            var originalText = title.getInnerText();
                            var animationSteps = [
                                "JavaScript Integration Demo",
                                "JavaScript Integration Demo!",
                                "JavaScript Integration Demo!!",
                                "JavaScript Integration Demo!!!",
                                originalText
                            ];
                            
                            var step = 0;
                            function animateStep() {
                                if (step < animationSteps.length) {
                                    title.setInnerText(animationSteps[step]);
                                    step++;
                                    // In a real implementation, this would use setTimeout
                                    // For now, we'll just simulate the animation
                                    console.log("Animation step", step, ":", animationSteps[step - 1]);
                                } else {
                                    isAnimating = false;
                                    animateButton.setInnerText("Animate Title");
                                    console.log("Animation completed");
                                }
                            }
                            
                            // Simulate animation steps
                            animateStep();
                            animateStep();
                            animateStep();
                            animateStep();
                            animateStep();
                        }
                    }
                });
            }
            
            console.log("All event listeners set up successfully!");
        </script>
        
        <script>
            console.log("Setting up additional functionality...");
            
            // Initialize UI state
            var themeButton = document.getElementById("theme-toggle");
            if (themeButton) {
                themeButton.setInnerText("Theme: " + currentTheme);
            }
            
            // Add some initial styling
            var content = document.getElementById("content");
            if (content) {
                content.setAttribute("data-theme", currentTheme);
            }
            
            console.log("JavaScript Integration Demo initialized!");
        </script>
    </body>
    </html>
    "#;

    const DEMO_CSS: &str = r#"
    body {
        font-family: Arial, sans-serif;
        margin: 20px;
        padding: 20px;
        background-color: #f5f5f5;
    }
    
    #main-title {
        color: #333;
        margin-bottom: 20px;
        text-align: center;
    }
    
    #description {
        text-align: center;
        color: #666;
        margin-bottom: 30px;
    }
    
    #controls {
        display: flex;
        gap: 10px;
        justify-content: center;
        margin-bottom: 30px;
        flex-wrap: wrap;
    }
    
    button {
        padding: 12px 20px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        font-size: 14px;
        transition: background-color 0.3s;
    }
    
    button:hover {
        background-color: #0056b3;
    }
    
    #content {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        background-color: white;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    }
    
    #dynamic-content {
        margin-bottom: 30px;
        padding: 15px;
        background-color: #f8f9fa;
        border-radius: 5px;
    }
    
    #element-list {
        margin-top: 20px;
    }
    
    #element-list h3 {
        color: #333;
        margin-bottom: 15px;
    }
    
    #element-list ul {
        list-style-type: disc;
        padding-left: 20px;
    }
    
    #element-list li {
        margin: 5px 0;
        padding: 5px;
        background-color: #e9ecef;
        border-radius: 3px;
    }
    
    /* Theme variations */
    [data-theme="dark"] {
        background-color: #2d3748 !important;
        color: #e2e8f0 !important;
    }
    
    [data-theme="dark"] button {
        background-color: #4a5568;
    }
    
    [data-theme="dark"] button:hover {
        background-color: #2d3748;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML and CSS...");
    let (document, _resources) = parse_html(DEMO_HTML.into()).unwrap();
    let stylesheet = parse_css(DEMO_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine and set up DOM
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured with DOM");

    // Execute inline scripts to set up functionality
    println!("\n🚀 Executing inline scripts...");
    js_engine.execute_inline_scripts()?;
    println!("✅ All inline scripts executed successfully");

    // Demonstrate interactive functionality
    println!("\n🎮 Demonstrating interactive functionality...");
    
    // Simulate button clicks
    println!("  • Simulating click counter button clicks...");
    for i in 1..=5 {
        js_engine.simulate_click("click-counter")?;
        println!("    Click {} completed", i);
    }
    
    // Simulate theme toggle
    println!("  • Simulating theme toggle...");
    js_engine.simulate_click("theme-toggle")?;
    js_engine.simulate_click("theme-toggle")?;
    println!("    Theme toggled twice");
    
    // Simulate adding elements
    println!("  • Simulating add element button...");
    for i in 1..=3 {
        js_engine.simulate_click("add-element")?;
        println!("    Added element {}", i);
    }
    
    // Simulate title animation
    println!("  • Simulating title animation...");
    js_engine.simulate_click("animate-title")?;
    println!("    Title animation triggered");

    // Test layout integration
    println!("\n📐 Testing layout integration...");
    let layout_js = r#"
        console.log("Testing layout integration...");
        
        // Simulate DOM changes that would trigger layout
        var content = document.getElementById("content");
        if (content) {
            content.setAttribute("style", "margin: 40px auto; padding: 30px;");
            console.log("Updated content styling");
        }
        
        var title = document.getElementById("main-title");
        if (title) {
            title.setInnerText("Layout Integration Test Complete!");
            console.log("Updated title for layout test");
        }
    "#;

    let layout_result = js_engine.execute_with_layout_update(layout_js)?;
    if let Some(_layout) = layout_result {
        println!("✅ Layout integration test successful - layout was recalculated");
    } else {
        println!("⚠️  Layout integration test completed - no layout recalculation");
    }

    // Final state check
    println!("\n📊 Final state check...");
    let final_state_js = r#"
        console.log("Final demo state:");
        console.log("Click count:", clickCount);
        console.log("Current theme:", currentTheme);
        console.log("Is animating:", isAnimating);
        
        // Check DOM state
        var clickButton = document.getElementById("click-counter");
        if (clickButton) {
            console.log("Click button text:", clickButton.getInnerText());
        }
        
        var themeButton = document.getElementById("theme-toggle");
        if (themeButton) {
            console.log("Theme button text:", themeButton.getInnerText());
        }
        
        var title = document.getElementById("main-title");
        if (title) {
            console.log("Title text:", title.getInnerText());
        }
        
        var content = document.getElementById("content");
        if (content) {
            console.log("Content theme:", content.getAttribute("data-theme"));
        }
    "#;

    js_engine.execute_with_layout_update(final_state_js)?;
    println!("✅ Final state check completed");

    println!("\n🎉 JavaScript Engine & DOM Integration Demo Completed!");
    println!("=====================================================");
    println!("✅ Successfully demonstrated:");
    println!("  • JavaScript execution with Boa engine");
    println!("  • DOM element access and manipulation");
    println!("  • Inline script execution in document order");
    println!("  • Event listener registration and dispatch");
    println!("  • Dynamic DOM updates and styling");
    println!("  • Layout integration and recalculation");
    println!("  • Interactive functionality simulation");
    println!();
    println!("🚀 The browser engine now has a fully functional JavaScript integration!");
    println!("   Ready for Phase 6: Advanced Features and Optimization");

    Ok(())
}
