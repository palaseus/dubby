use js_integration::JsEngine;
use dom::Document;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 STRESS TEST: Advanced JavaScript Features");
    println!("=============================================");
    println!("Testing: async/await, timers, fetch API, Promise handling");
    println!();

    // Complex HTML with advanced JavaScript features
    const STRESS_HTML: &str = r#"
    <html>
    <head>
        <title>Advanced JavaScript Stress Test</title>
        <script>
            console.log("=== STRESS TEST INITIALIZATION ===");
            
            // Global state for stress testing
            var timerCount = 0;
            var promiseCount = 0;
            var fetchCount = 0;
            var asyncOperations = 0;
            var errors = 0;
            
            // Performance tracking
            var startTime = Date.now();
            var operationTimes = [];
        </script>
    </head>
    <body>
        <h1 id="title">Advanced JavaScript Stress Test</h1>
        <div id="stats">
            <p id="timer-stats">Timers: 0</p>
            <p id="promise-stats">Promises: 0</p>
            <p id="fetch-stats">Fetch calls: 0</p>
            <p id="error-stats">Errors: 0</p>
        </div>
        <div id="content"></div>
        
        <script>
            console.log("=== TESTING ADVANCED TIMER FEATURES ===");
            
            // Test complex timer chains
            function testTimerChains() {
                console.log("Starting timer chain test...");
                
                // Chain of timers
                setTimeout(function() {
                    timerCount++;
                    console.log("Timer 1 executed, count:", timerCount);
                    
                    setTimeout(function() {
                        timerCount++;
                        console.log("Timer 2 executed, count:", timerCount);
                        
                        setTimeout(function() {
                            timerCount++;
                            console.log("Timer 3 executed, count:", timerCount);
                            updateStats();
                        }, 50);
                    }, 100);
                }, 200);
                
                // Multiple parallel timers
                for (var i = 0; i < 5; i++) {
                    setTimeout(function(index) {
                        return function() {
                            timerCount++;
                            console.log("Parallel timer", index, "executed, count:", timerCount);
                        };
                    }(i), 300 + (i * 10));
                }
            }
            
            // Test intervals
            function testIntervals() {
                console.log("Starting interval test...");
                
                var intervalId = setInterval(function() {
                    timerCount++;
                    console.log("Interval tick, count:", timerCount);
                    
                    if (timerCount >= 10) {
                        clearInterval(intervalId);
                        console.log("Interval cleared after 10 ticks");
                    }
                }, 100);
            }
            
            // Test Promise handling
            function testPromises() {
                console.log("=== TESTING PROMISE FEATURES ===");
                
                // Basic Promise
                var promise1 = new Promise(function(resolve, reject) {
                    console.log("Promise 1 executor called");
                    setTimeout(function() {
                        promiseCount++;
                        console.log("Promise 1 resolved, count:", promiseCount);
                        resolve("Promise 1 result");
                    }, 150);
                });
                
                promise1.then(function(result) {
                    console.log("Promise 1 then:", result);
                }).catch(function(error) {
                    console.log("Promise 1 error:", error);
                    errors++;
                });
                
                // Promise chain
                var promise2 = new Promise(function(resolve, reject) {
                    console.log("Promise 2 executor called");
                    resolve("Promise 2 initial");
                });
                
                promise2.then(function(result) {
                    console.log("Promise 2 step 1:", result);
                    return "Promise 2 step 2";
                }).then(function(result) {
                    console.log("Promise 2 step 2:", result);
                    promiseCount++;
                    return "Promise 2 final";
                }).then(function(result) {
                    console.log("Promise 2 final:", result);
                });
            }
            
            // Test fetch API
            function testFetchAPI() {
                console.log("=== TESTING FETCH API ===");
                
                // Multiple fetch calls
                var urls = [
                    "https://api.example.com/users",
                    "https://api.example.com/posts",
                    "https://api.example.com/comments",
                    "https://api.example.com/albums"
                ];
                
                urls.forEach(function(url, index) {
                    console.log("Fetching URL", index + 1, ":", url);
                    
                    fetch(url).then(function(response) {
                        fetchCount++;
                        console.log("Fetch", index + 1, "response received, count:", fetchCount);
                        return response.text();
                    }).then(function(text) {
                        console.log("Fetch", index + 1, "text received, length:", text.length);
                    }).catch(function(error) {
                        console.log("Fetch", index + 1, "error:", error);
                        errors++;
                    });
                });
            }
            
            // Test async/await simulation
            function testAsyncAwait() {
                console.log("=== TESTING ASYNC/AWAIT SIMULATION ===");
                
                // Simulate async function
                function asyncFunction() {
                    return new Promise(function(resolve) {
                        setTimeout(function() {
                            asyncOperations++;
                            console.log("Async operation completed, count:", asyncOperations);
                            resolve("Async result");
                        }, 250);
                    });
                }
                
                // Simulate await
                asyncFunction().then(function(result) {
                    console.log("Async result:", result);
                    
                    // Chain more async operations
                    return asyncFunction();
                }).then(function(result) {
                    console.log("Chained async result:", result);
                });
            }
            
            // Test error handling
            function testErrorHandling() {
                console.log("=== TESTING ERROR HANDLING ===");
                
                // Promise rejection
                var errorPromise = new Promise(function(resolve, reject) {
                    setTimeout(function() {
                        reject(new Error("Test error"));
                    }, 100);
                });
                
                errorPromise.then(function(result) {
                    console.log("Error promise resolved:", result);
                }).catch(function(error) {
                    errors++;
                    console.log("Error promise caught:", error.message);
                });
                
                // Timer error
                setTimeout(function() {
                    try {
                        throw new Error("Timer error");
                    } catch (e) {
                        errors++;
                        console.log("Timer error caught:", e.message);
                    }
                }, 200);
            }
            
            // Performance testing
            function testPerformance() {
                console.log("=== TESTING PERFORMANCE ===");
                
                var iterations = 1000;
                var start = Date.now();
                
                for (var i = 0; i < iterations; i++) {
                    // Heavy computation
                    var result = 0;
                    for (var j = 0; j < 100; j++) {
                        result += Math.sqrt(j * i);
                    }
                }
                
                var end = Date.now();
                var duration = end - start;
                operationTimes.push(duration);
                
                console.log("Performance test completed:", iterations, "iterations in", duration, "ms");
            }
            
            // Update statistics
            function updateStats() {
                var timerStats = document.getElementById("timer-stats");
                var promiseStats = document.getElementById("promise-stats");
                var fetchStats = document.getElementById("fetch-stats");
                var errorStats = document.getElementById("error-stats");
                
                if (timerStats) timerStats.setInnerText("Timers: " + timerCount);
                if (promiseStats) promiseStats.setInnerText("Promises: " + promiseCount);
                if (fetchStats) fetchStats.setInnerText("Fetch calls: " + fetchCount);
                if (errorStats) errorStats.setInnerText("Errors: " + errors);
            }
            
            // Run all tests
            function runAllTests() {
                console.log("=== STARTING ALL STRESS TESTS ===");
                
                testTimerChains();
                testIntervals();
                testPromises();
                testFetchAPI();
                testAsyncAwait();
                testErrorHandling();
                testPerformance();
                
                // Final stats update
                setTimeout(function() {
                    updateStats();
                    var totalTime = Date.now() - startTime;
                    console.log("=== STRESS TEST COMPLETED ===");
                    console.log("Total time:", totalTime, "ms");
                    console.log("Final stats - Timers:", timerCount, "Promises:", promiseCount, "Fetch:", fetchCount, "Errors:", errors);
                }, 1000);
            }
            
            // Start the stress test
            runAllTests();
        </script>
    </body>
    </html>
    "#;

    const STRESS_CSS: &str = r#"
    body {
        font-family: 'Courier New', monospace;
        margin: 20px;
        padding: 20px;
        background-color: #1a1a1a;
        color: #00ff00;
    }
    
    h1 {
        color: #ff0000;
        text-align: center;
        margin-bottom: 30px;
    }
    
    #stats {
        background-color: #2a2a2a;
        padding: 20px;
        border-radius: 10px;
        margin-bottom: 20px;
    }
    
    #stats p {
        margin: 10px 0;
        font-size: 16px;
    }
    
    #content {
        background-color: #333;
        padding: 20px;
        border-radius: 10px;
        min-height: 200px;
    }
    "#;

    // Parse HTML and CSS
    println!("📄 Parsing complex HTML with advanced JavaScript...");
    let start_time = Instant::now();
    let document = parse_html(STRESS_HTML);
    let stylesheet = parse_css(STRESS_CSS);
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

    // Execute stress test scripts
    println!("\n🚀 Executing stress test scripts...");
    let exec_start = Instant::now();
    js_engine.execute_inline_scripts()?;
    let exec_time = exec_start.elapsed();
    println!("✅ Stress test scripts executed in {:?}", exec_time);

    // Simulate event loop processing for timers
    println!("\n⏰ Simulating event loop processing...");
    let event_loop_start = Instant::now();
    
    for i in 1..=20 {
        let iteration_start = Instant::now();
        js_engine.process_event_loop()?;
        let iteration_time = iteration_start.elapsed();
        
        if i % 5 == 0 {
            println!("  Event loop iteration {} completed in {:?}", i, iteration_time);
        }
        
        // Simulate time passing
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    let event_loop_time = event_loop_start.elapsed();
    println!("✅ Event loop processing completed in {:?}", event_loop_time);

    // Test advanced JavaScript features directly
    println!("\n🧪 Testing advanced features directly...");
    let advanced_start = Instant::now();
    
    let advanced_js = r#"
        console.log("=== DIRECT ADVANCED FEATURE TESTING ===");
        
        // Test complex timer operations
        var complexTimerId = setTimeout(function() {
            console.log("Complex timer executed");
        }, 1000);
        
        // Test Promise with complex chain
        var complexPromise = new Promise(function(resolve, reject) {
            setTimeout(function() {
                resolve("Complex promise result");
            }, 500);
        });
        
        complexPromise.then(function(result) {
            console.log("Complex promise resolved:", result);
            return new Promise(function(resolve) {
                setTimeout(function() {
                    resolve("Chained promise result");
                }, 200);
            });
        }).then(function(result) {
            console.log("Chained promise resolved:", result);
        });
        
        // Test multiple fetch operations
        var fetchPromises = [];
        for (var i = 0; i < 3; i++) {
            fetchPromises.push(fetch("https://api.test.com/endpoint" + i));
        }
        
        Promise.all(fetchPromises).then(function(responses) {
            console.log("All fetch operations completed:", responses.length);
        }).catch(function(error) {
            console.log("Fetch operations failed:", error);
        });
        
        // Test error handling in async operations
        setTimeout(function() {
            try {
                throw new Error("Async error test");
            } catch (e) {
                console.log("Async error caught:", e.message);
            }
        }, 300);
        
        console.log("Advanced feature testing initiated");
    "#;

    js_engine.execute_with_layout_update(advanced_js)?;
    let advanced_time = advanced_start.elapsed();
    println!("✅ Advanced features tested in {:?}", advanced_time);

    // Performance analysis
    println!("\n📊 Performance Analysis:");
    println!("  Parse time: {:?}", parse_time);
    println!("  Setup time: {:?}", setup_time);
    println!("  Execution time: {:?}", exec_time);
    println!("  Event loop time: {:?}", event_loop_time);
    println!("  Advanced features time: {:?}", advanced_time);
    
    let total_time = start_time.elapsed();
    println!("  Total time: {:?}", total_time);

    // Final state check
    println!("\n📈 Final state analysis...");
    let final_js = r#"
        console.log("=== FINAL STATE ANALYSIS ===");
        console.log("Timer count:", timerCount);
        console.log("Promise count:", promiseCount);
        console.log("Fetch count:", fetchCount);
        console.log("Async operations:", asyncOperations);
        console.log("Error count:", errors);
        console.log("Operation times:", operationTimes.length);
        
        // Calculate average operation time
        if (operationTimes.length > 0) {
            var totalTime = 0;
            for (var i = 0; i < operationTimes.length; i++) {
                totalTime += operationTimes[i];
            }
            var avgTime = totalTime / operationTimes.length;
            console.log("Average operation time:", avgTime, "ms");
        }
        
        // Memory usage simulation
        var memoryUsage = {
            timers: timerCount * 100,
            promises: promiseCount * 200,
            fetch: fetchCount * 300,
            total: timerCount * 100 + promiseCount * 200 + fetchCount * 300
        };
        
        console.log("Simulated memory usage:", memoryUsage);
        console.log("=== STRESS TEST COMPLETE ===");
    "#;

    js_engine.execute_with_layout_update(final_js)?;
    println!("✅ Final state analysis completed");

    println!("\n🔥 STRESS TEST RESULTS:");
    println!("======================");
    println!("✅ Advanced JavaScript features tested:");
    println!("  • Complex timer chains and intervals");
    println!("  • Promise handling and chaining");
    println!("  • Fetch API with multiple requests");
    println!("  • Async/await simulation");
    println!("  • Error handling in async operations");
    println!("  • Performance testing with heavy computation");
    println!("  • Event loop processing");
    println!();
    println!("🚀 The engine handled advanced JavaScript features!");
    println!("   Ready for DOM API expansion and event model polish");

    Ok(())
}
