use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 JavaScript Engine Integration Demo");
    println!("====================================");

    // Simple HTML with JavaScript
    const DEMO_HTML: &str = r#"
    <html>
    <head>
        <title>Simple Demo</title>
        <script>
            console.log("Demo initialized!");
            var count = 0;
        </script>
    </head>
    <body>
        <h1 id="title">Simple Demo</h1>
        <button id="button">Click Me!</button>
        <p id="counter">Count: 0</p>
        
        <script>
            var button = document.getElementById("button");
            if (button) {
                button.addEventListener("click", function() {
                    count++;
                    var counter = document.getElementById("counter");
                    if (counter) {
                        counter.setInnerText("Count: " + count);
                    }
                    console.log("Button clicked! Count:", count);
                });
            }
        </script>
    </body>
    </html>
    "#;

    const DEMO_CSS: &str = r#"
    body {
        font-family: Arial, sans-serif;
        margin: 20px;
        padding: 20px;
    }
    
    h1 {
        color: #333;
        text-align: center;
    }
    
    button {
        display: block;
        margin: 20px auto;
        padding: 10px 20px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }
    
    p {
        text-align: center;
        font-size: 18px;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML and CSS...");
    let (document, _resources) = parse_html(DEMO_HTML.into()).unwrap();
    let stylesheet = parse_css(DEMO_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured");

    // Execute inline scripts
    println!("\n🚀 Executing inline scripts...");
    js_engine.execute_inline_scripts()?;
    println!("✅ Scripts executed successfully");

    // Simulate button clicks
    println!("\n🎮 Simulating interactions...");
    for i in 1..=5 {
        js_engine.simulate_click("button")?;
        println!("  Click {} completed", i);
    }

    // Test layout integration
    println!("\n📐 Testing layout integration...");
    let layout_js = r#"
        var title = document.getElementById("title");
        if (title) {
            title.setInnerText("Layout Test Complete!");
        }
        console.log("Layout test completed");
    "#;

    let layout_result = js_engine.execute_with_layout_update(layout_js)?;
    if let Some(_layout) = layout_result {
        println!("✅ Layout integration successful");
    } else {
        println!("⚠️  Layout integration completed");
    }

    // Final state
    println!("\n📊 Final state check...");
    let final_js = r#"
        console.log("Final state:");
        console.log("Count:", count);
        
        var counter = document.getElementById("counter");
        if (counter) {
            console.log("Counter text:", counter.getInnerText());
        }
        
        var title = document.getElementById("title");
        if (title) {
            console.log("Title text:", title.getInnerText());
        }
    "#;

    js_engine.execute_with_layout_update(final_js)?;
    println!("✅ Final state check completed");

    println!("\n🎉 JavaScript Integration Demo Completed!");
    println!("=========================================");
    println!("✅ Successfully demonstrated:");
    println!("  • JavaScript execution with Boa engine");
    println!("  • DOM element access and manipulation");
    println!("  • Inline script execution");
    println!("  • Event listener registration and dispatch");
    println!("  • Layout integration");
    println!("  • Interactive functionality");
    println!();
    println!("🚀 The browser engine now has JavaScript integration!");

    Ok(())
}
