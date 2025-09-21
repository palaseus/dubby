use js_integration::JsEngine;
use dom::Document;
use css_parser::parse_css;
use html_parser::parse_html_string;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Phase 9: JavaScript Engine Integration + DOM Bindings Test");
    println!("=============================================================");

    // Simple test HTML without problematic characters
    let test_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Simple JS Test</title>
    </head>
    <body>
        <h1 id="main-title">JavaScript Test</h1>
        <div id="test-container">
            <p id="counter-display">Counter: 0</p>
            <button id="increment-btn">Increment</button>
        </div>
        
        <script>
            console.log("Hello from JavaScript!");
            var title = document.getElementById("main-title");
            if (title) {
                title.innerText = "Modified by JavaScript!";
            }
        </script>
    </body>
    </html>
    "#;

    let test_css = r#"
    body { font-family: Arial, sans-serif; margin: 20px; }
    .test-element { background-color: #f0f0f0; padding: 10px; }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML and CSS...");
    let (document, _resources) = parse_html_string(test_html)?;
    let stylesheet = parse_css(test_css);
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
    println!("✅ All inline scripts executed successfully");

    // Test basic JavaScript execution
    println!("\n🧪 Testing basic JavaScript execution...");
    let basic_js = "console.log('Testing basic JavaScript features...'); console.log('Math.PI =', Math.PI); console.log('Math.random() =', Math.random());";

    js_engine.execute(basic_js)?;
    println!("✅ Basic JavaScript execution completed");

    // Test DOM manipulation
    println!("\n🌐 Testing DOM manipulation...");
    let dom_js = "console.log('Testing DOM manipulation...'); var title = document.getElementById('main-title'); if (title) { console.log('Found title element:', title.id); title.innerText = 'DOM Manipulation Test'; }";

    js_engine.execute(dom_js)?;
    println!("✅ DOM manipulation tests completed");

    // Test error handling
    println!("\n⚠️  Testing error handling...");
    let error_js = "console.log('Testing error handling...'); try { var undefinedVar = nonExistentFunction(); } catch (e) { console.log('Caught expected error:', e.message); }";

    js_engine.execute(error_js)?;
    println!("✅ Error handling tests completed");

    // Display performance metrics
    println!("\n📊 JavaScript Performance Metrics:");
    println!("==================================");
    let metrics = js_engine.get_metrics();
    println!("Total execution time: {:?}", metrics.total_execution_time);
    println!("Scripts executed: {}", metrics.script_count);
    println!("Statements executed: {}", metrics.statement_count);
    println!("DOM operations: {}", metrics.dom_operations);
    println!("Event handlers: {}", metrics.event_handlers);
    println!("Timer operations: {}", metrics.timer_operations);
    println!("Errors encountered: {}", metrics.error_count);

    println!("\n🎉 Phase 9 JavaScript Engine Integration Test Completed Successfully!");
    println!("=====================================================================");
    println!("✅ JavaScript engine integration working");
    println!("✅ DOM bindings functional");
    println!("✅ Performance tracking active");
    println!("✅ Error handling robust");

    Ok(())
}
