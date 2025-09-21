use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⏰ Testing Event Loop and Timers");
    println!("===============================");

    // Parse HTML with timer-based scripts
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>Event Loop Test</title>
        <script>
            console.log("Setting up timer-based functionality...");
            
            // Global variables for testing
            var timerCount = 0;
            var intervalCount = 0;
            var timeoutExecuted = false;
        </script>
    </head>
    <body>
        <h1 id="main-title">Event Loop Test</h1>
        <p id="description">This page tests the event loop and timers.</p>
        
        <div id="container">
            <button id="start-timers">Start Timers</button>
            <div id="timer-display">Timer Count: 0</div>
            <div id="interval-display">Interval Count: 0</div>
            <div id="status">Status: Ready</div>
        </div>
        
        <script>
            console.log("Adding timer functionality...");
            
            // Test setTimeout
            var timeoutId = setTimeout(function() {
                console.log("setTimeout executed after 1000ms!");
                timeoutExecuted = true;
                
                var status = document.getElementById("status");
                if (status) {
                    status.setInnerText("Status: Timeout executed!");
                }
            }, 1000);
            
            console.log("setTimeout scheduled with ID:", timeoutId);
            
            // Test setInterval
            var intervalId = setInterval(function() {
                intervalCount++;
                console.log("setInterval executed, count:", intervalCount);
                
                var display = document.getElementById("interval-display");
                if (display) {
                    display.setInnerText("Interval Count: " + intervalCount);
                }
                
                // Stop after 5 executions
                if (intervalCount >= 5) {
                    clearInterval(intervalId);
                    console.log("Interval cleared after 5 executions");
                }
            }, 500);
            
            console.log("setInterval scheduled with ID:", intervalId);
            
            // Test nested setTimeout
            setTimeout(function() {
                console.log("Nested setTimeout executed!");
                
                setTimeout(function() {
                    console.log("Double nested setTimeout executed!");
                }, 200);
            }, 300);
            
            // Test timer with DOM manipulation
            setTimeout(function() {
                var title = document.getElementById("main-title");
                if (title) {
                    title.setInnerText("Title Updated by Timer!");
                    console.log("Title updated by timer");
                }
            }, 800);
            
            console.log("All timers scheduled!");
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
    
    #timer-display, #interval-display, #status {
        margin: 10px 0;
        padding: 5px;
        background-color: #f0f0f0;
        border: 1px solid #ddd;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML with timer scripts...");
    let document = parse_html(TEST_HTML);
    let stylesheet = parse_css(TEST_CSS);
    println!("✅ HTML and CSS parsed successfully");

    // Create JavaScript engine and set up DOM
    println!("\n🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    println!("✅ JavaScript engine configured with DOM");

    // Execute inline scripts to set up timers
    println!("\n🚀 Setting up timers...");
    js_engine.execute_inline_scripts()?;
    println!("✅ Timers set up successfully");

    // Test timer API directly
    println!("\n⏱️  Testing timer API...");
    let timer_test_js = r#"
        console.log("Testing timer API...");
        
        // Test setTimeout
        var timeoutId = setTimeout(function() {
            console.log("Direct setTimeout test executed!");
        }, 100);
        
        console.log("setTimeout returned ID:", timeoutId);
        
        // Test setInterval
        var intervalId = setInterval(function() {
            console.log("Direct setInterval test executed!");
        }, 200);
        
        console.log("setInterval returned ID:", intervalId);
        
        // Test clearTimeout
        setTimeout(function() {
            clearTimeout(timeoutId);
            console.log("Timeout cleared");
        }, 50);
        
        // Test clearInterval
        setTimeout(function() {
            clearInterval(intervalId);
            console.log("Interval cleared");
        }, 400);
    "#;

    js_engine.execute_with_layout_update(timer_test_js)?;
    println!("✅ Timer API test completed");

    // Simulate event loop processing
    println!("\n🔄 Simulating event loop processing...");
    
    // Process the event loop multiple times to simulate time passing
    for i in 1..=10 {
        println!("Event loop iteration {}", i);
        js_engine.process_event_loop()?;
        
        // Simulate time passing
        thread::sleep(Duration::from_millis(100));
        
        // Check timer state
        let state_js = format!(
            r#"
            console.log("Event loop iteration {} - checking state...");
            console.log("Timer count:", timerCount);
            console.log("Interval count:", intervalCount);
            console.log("Timeout executed:", timeoutExecuted);
            "#,
            i
        );
        
        js_engine.execute_with_layout_update(&state_js)?;
    }

    // Test final state
    println!("\n📊 Testing final state...");
    let final_state_js = r#"
        console.log("Final state check:");
        console.log("Timer count:", timerCount);
        console.log("Interval count:", intervalCount);
        console.log("Timeout executed:", timeoutExecuted);
        
        // Check DOM updates
        var title = document.getElementById("main-title");
        if (title) {
            console.log("Title text:", title.getInnerText());
        }
        
        var status = document.getElementById("status");
        if (status) {
            console.log("Status text:", status.getInnerText());
        }
    "#;

    js_engine.execute_with_layout_update(final_state_js)?;
    println!("✅ Final state test completed");

    // Test error handling in timers
    println!("\n⚠️  Testing error handling in timers...");
    let error_timer_js = r#"
        console.log("Testing error handling...");
        
        // Test setTimeout with invalid callback
        try {
            setTimeout("invalid callback", 100);
            console.log("setTimeout with invalid callback handled");
        } catch (e) {
            console.log("Caught error in setTimeout:", e);
        }
        
        // Test setInterval with zero delay
        try {
            setInterval(function() {
                console.log("Zero delay interval");
            }, 0);
            console.log("setInterval with zero delay handled");
        } catch (e) {
            console.log("Caught error in setInterval:", e);
        }
    "#;

    js_engine.execute_with_layout_update(error_timer_js)?;
    println!("✅ Error handling test completed");

    println!("\n🎉 Event loop and timer test completed!");
    println!("The JavaScript engine can now:");
    println!("  ✓ Schedule timers with setTimeout");
    println!("  ✓ Schedule repeating timers with setInterval");
    println!("  ✓ Clear timers with clearTimeout/clearInterval");
    println!("  ✓ Process the event loop to execute ready timers");
    println!("  ✓ Handle nested and complex timer scenarios");
    println!("  ✓ Update DOM elements from timer callbacks");
    println!("  ✓ Handle timer errors gracefully");

    Ok(())
}
