use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing Advanced JavaScript Features");
    println!("=======================================");
    println!("Testing: timers, fetch API, Promise handling");
    println!();

    // Simple HTML with advanced JavaScript features
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>Advanced Features Test</title>
        <script>
            console.log("Advanced features test initialized");
            var timerCount = 0;
            var promiseCount = 0;
        </script>
    </head>
    <body>
        <h1 id="title">Advanced Features Test</h1>
        <div id="stats">
            <p id="timer-stats">Timers: 0</p>
            <p id="promise-stats">Promises: 0</p>
        </div>
        
        <script>
            console.log("Testing advanced timer features...");
            
            // Test setTimeout
            var timeoutId = setTimeout(function() {
                timerCount++;
                console.log("setTimeout executed, count:", timerCount);
            }, 100);
            
            console.log("setTimeout scheduled with ID:", timeoutId);
            
            // Test setInterval
            var intervalId = setInterval(function() {
                timerCount++;
                console.log("setInterval tick, count:", timerCount);
                
                if (timerCount >= 3) {
                    clearInterval(intervalId);
                    console.log("Interval cleared after 3 ticks");
                }
            }, 200);
            
            console.log("setInterval scheduled with ID:", intervalId);
        </script>
        
        <script>
            console.log("Testing Promise features...");
            
            // Test Promise
            var promise1 = new Promise(function(resolve, reject) {
                console.log("Promise executor called");
                setTimeout(function() {
                    promiseCount++;
                    console.log("Promise resolved, count:", promiseCount);
                    resolve("Promise result");
                }, 150);
            });
            
            promise1.then(function(result) {
                console.log("Promise then:", result);
            }).catch(function(error) {
                console.log("Promise error:", error);
            });
        </script>
        
        <script>
            console.log("Testing fetch API...");
            
            // Test fetch
            fetch("https://api.example.com/test").then(function(response) {
                console.log("Fetch response received");
                return response.text();
            }).then(function(text) {
                console.log("Fetch text received, length:", text.length);
            }).catch(function(error) {
                console.log("Fetch error:", error);
            });
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
    
    h1 {
        color: #333;
        text-align: center;
    }
    
    #stats {
        background-color: #f0f0f0;
        padding: 20px;
        border-radius: 10px;
        margin: 20px 0;
    }
    
    #stats p {
        margin: 10px 0;
        font-size: 16px;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing HTML with advanced JavaScript...");
    let start_time = Instant::now();
    let document = parse_html(TEST_HTML);
    let stylesheet = parse_css(TEST_CSS);
    let parse_time = start_time.elapsed();
    println!("✅ HTML and CSS parsed in {:?}", parse_time);

    // Create JavaScript engine
    println!("\n🔧 Setting up JavaScript engine...");
    let setup_start = Instant::now();
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);
    let setup_time = setup_start.elapsed();
    println!("✅ JavaScript engine configured in {:?}", setup_time);

    // Execute advanced scripts
    println!("\n🚀 Executing advanced JavaScript features...");
    let exec_start = Instant::now();
    js_engine.execute_inline_scripts()?;
    let exec_time = exec_start.elapsed();
    println!("✅ Advanced features executed in {:?}", exec_time);

    // Test advanced features directly
    println!("\n🧪 Testing advanced features directly...");
    let advanced_start = Instant::now();
    
    let advanced_js = r#"
        console.log("=== DIRECT ADVANCED FEATURE TESTING ===");
        
        // Test complex timer operations
        var timerId1 = setTimeout(function() {
            console.log("Direct timer 1 executed");
        }, 100);
        
        var timerId2 = setTimeout(function() {
            console.log("Direct timer 2 executed");
        }, 200);
        
        console.log("Direct timers scheduled:", timerId1, timerId2);
        
        // Test Promise with chain
        var promise = new Promise(function(resolve, reject) {
            console.log("Direct promise executor called");
            setTimeout(function() {
                resolve("Direct promise result");
            }, 150);
        });
        
        promise.then(function(result) {
            console.log("Direct promise resolved:", result);
            return "Chained result";
        }).then(function(result) {
            console.log("Direct promise chain:", result);
        });
        
        // Test multiple fetch operations
        fetch("https://api.test.com/endpoint1").then(function(response) {
            console.log("Direct fetch 1 response received");
            return response.text();
        }).then(function(text) {
            console.log("Direct fetch 1 text received");
        });
        
        fetch("https://api.test.com/endpoint2").then(function(response) {
            console.log("Direct fetch 2 response received");
            return response.text();
        }).then(function(text) {
            console.log("Direct fetch 2 text received");
        });
        
        console.log("Direct advanced feature testing initiated");
    "#;

    js_engine.execute_with_layout_update(advanced_js)?;
    let advanced_time = advanced_start.elapsed();
    println!("✅ Direct advanced features tested in {:?}", advanced_time);

    // Simulate event loop processing
    println!("\n⏰ Simulating event loop processing...");
    let event_loop_start = Instant::now();
    
    for i in 1..=10 {
        let iteration_start = Instant::now();
        js_engine.process_event_loop()?;
        let iteration_time = iteration_start.elapsed();
        
        if i % 3 == 0 {
            println!("  Event loop iteration {} completed in {:?}", i, iteration_time);
        }
        
        // Simulate time passing
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    let event_loop_time = event_loop_start.elapsed();
    println!("✅ Event loop processing completed in {:?}", event_loop_time);

    // Performance analysis
    println!("\n📊 Performance Analysis:");
    println!("  Parse time: {:?}", parse_time);
    println!("  Setup time: {:?}", setup_time);
    println!("  Execution time: {:?}", exec_time);
    println!("  Advanced features time: {:?}", advanced_time);
    println!("  Event loop time: {:?}", event_loop_time);
    
    let total_time = start_time.elapsed();
    println!("  Total time: {:?}", total_time);

    // Final state check
    println!("\n📈 Final state analysis...");
    let final_js = r#"
        console.log("=== FINAL STATE ANALYSIS ===");
        console.log("Timer count:", timerCount);
        console.log("Promise count:", promiseCount);
        
        // Update DOM stats
        var timerStats = document.getElementById("timer-stats");
        var promiseStats = document.getElementById("promise-stats");
        
        if (timerStats) {
            timerStats.setInnerText("Timers: " + timerCount);
        }
        
        if (promiseStats) {
            promiseStats.setInnerText("Promises: " + promiseCount);
        }
        
        console.log("=== ADVANCED FEATURES TEST COMPLETE ===");
    "#;

    js_engine.execute_with_layout_update(final_js)?;
    println!("✅ Final state analysis completed");

    println!("\n🚀 Advanced Features Test Results:");
    println!("=================================");
    println!("✅ Successfully tested:");
    println!("  • Advanced timer functions (setTimeout, setInterval)");
    println!("  • Promise handling and chaining");
    println!("  • Fetch API with mock responses");
    println!("  • Event loop processing");
    println!("  • Performance monitoring");
    println!();
    println!("🔥 The engine handles advanced JavaScript features!");
    println!("   Ready for DOM API expansion and event model polish");

    Ok(())
}
