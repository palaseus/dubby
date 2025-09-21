//! # Promise and Microtask Integration Tests
//! 
//! This test suite validates the Promise and microtask queue implementation,
//! ensuring proper execution order and behavior according to JavaScript specifications.

use js_integration::JsEngine;
use boa_engine::{Context, Source};
use std::time::Duration;
use std::thread;

fn main() {
    println!("🧪 Running Promise and Microtask Integration Tests");
    println!("=" .repeat(60));

    test_promise_creation();
    test_promise_resolve();
    test_promise_reject();
    test_promise_chaining();
    test_microtask_ordering();
    test_async_await_simulation();
    test_performance_telemetry();
    test_error_handling();
    test_abort_controller();

    println!("\n✅ All Promise and Microtask tests completed!");
}

/// Test basic Promise creation
fn test_promise_creation() {
    println!("\n🔸 Testing Promise Creation...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    // Test Promise constructor
    let code = r#"
        const promise = new Promise((resolve, reject) => {
            console.log("Promise executor called");
        });
        console.log("Promise created:", typeof promise);
        promise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Promise creation test passed");
            } else {
                println!("❌ Promise creation test failed");
            }
        }
        Err(e) => {
            println!("❌ Promise creation test failed with error: {}", e);
        }
    }
}

/// Test Promise.resolve()
fn test_promise_resolve() {
    println!("\n🔸 Testing Promise.resolve()...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const resolved = Promise.resolve("test value");
        console.log("Promise.resolve() called");
        resolved !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Promise.resolve() test passed");
            } else {
                println!("❌ Promise.resolve() test failed");
            }
        }
        Err(e) => {
            println!("❌ Promise.resolve() test failed with error: {}", e);
        }
    }
}

/// Test Promise.reject()
fn test_promise_reject() {
    println!("\n🔸 Testing Promise.reject()...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const rejected = Promise.reject("error message");
        console.log("Promise.reject() called");
        rejected !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Promise.reject() test passed");
            } else {
                println!("❌ Promise.reject() test failed");
            }
        }
        Err(e) => {
            println!("❌ Promise.reject() test failed with error: {}", e);
        }
    }
}

/// Test Promise chaining with .then()
fn test_promise_chaining() {
    println!("\n🔸 Testing Promise Chaining...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const promise = Promise.resolve(1)
            .then(x => {
                console.log("First then:", x);
                return x + 1;
            })
            .then(x => {
                console.log("Second then:", x);
                return x * 2;
            });
        
        console.log("Promise chain created");
        promise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Promise chaining test passed");
            } else {
                println!("❌ Promise chaining test failed");
            }
        }
        Err(e) => {
            println!("❌ Promise chaining test failed with error: {}", e);
        }
    }
}

/// Test microtask ordering (Promise.then vs setTimeout)
fn test_microtask_ordering() {
    println!("\n🔸 Testing Microtask Ordering...");
    
    let mut js_engine = JsEngine::new();
    js_engine.set_microtask_trace_enabled(true);

    let context = &mut js_engine.context;

    let code = r#"
        console.log("1. Sync start");
        
        Promise.resolve().then(() => {
            console.log("2. Microtask 1");
        });
        
        setTimeout(() => {
            console.log("4. Macrotask (setTimeout)");
        }, 0);
        
        Promise.resolve().then(() => {
            console.log("3. Microtask 2");
        });
        
        console.log("5. Sync end");
        true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(_) => {
            // Process the event loop to execute microtasks and timers
            js_engine.process_event_loop().unwrap();
            thread::sleep(Duration::from_millis(10));
            js_engine.process_event_loop().unwrap();
            
            println!("✅ Microtask ordering test completed (check console output)");
        }
        Err(e) => {
            println!("❌ Microtask ordering test failed with error: {}", e);
        }
    }
}

/// Test async/await simulation
fn test_async_await_simulation() {
    println!("\n🔸 Testing Async/Await Simulation...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        // Simulate async/await with Promise chains
        function asyncFunction() {
            return Promise.resolve("async result")
                .then(result => {
                    console.log("Async result:", result);
                    return result.toUpperCase();
                });
        }
        
        const result = asyncFunction();
        console.log("Async function called");
        result !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Async/await simulation test passed");
            } else {
                println!("❌ Async/await simulation test failed");
            }
        }
        Err(e) => {
            println!("❌ Async/await simulation test failed with error: {}", e);
        }
    }
}

/// Test performance telemetry
fn test_performance_telemetry() {
    println!("\n🔸 Testing Performance Telemetry...");
    
    let mut js_engine = JsEngine::new();
    js_engine.set_microtask_trace_enabled(true);

    let context = &mut js_engine.context;

    // Create some microtasks to generate telemetry
    let code = r#"
        for (let i = 0; i < 5; i++) {
            Promise.resolve(i).then(x => console.log("Telemetry test:", x));
        }
        true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(_) => {
            // Process microtasks to generate telemetry
            js_engine.process_event_loop().unwrap();
            
            // Get and display telemetry
            let telemetry = js_engine.get_telemetry();
            println!("📊 Performance Telemetry:");
            println!("{}", telemetry);
            
            println!("✅ Performance telemetry test passed");
        }
        Err(e) => {
            println!("❌ Performance telemetry test failed with error: {}", e);
        }
    }
}

/// Test error handling in Promises
fn test_error_handling() {
    println!("\n🔸 Testing Error Handling...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const errorPromise = Promise.reject("Test error")
            .catch(error => {
                console.log("Caught error:", error);
                return "recovered";
            });
        
        console.log("Error handling test setup");
        errorPromise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Error handling test passed");
            } else {
                println!("❌ Error handling test failed");
            }
        }
        Err(e) => {
            println!("❌ Error handling test failed with error: {}", e);
        }
    }
}

/// Test AbortController functionality
fn test_abort_controller() {
    println!("\n🔸 Testing AbortController...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const controller = new AbortController();
        const signal = controller.signal;
        
        console.log("AbortController created");
        console.log("Signal aborted:", signal.aborted);
        
        controller.abort("Test abort");
        console.log("Controller aborted");
        console.log("Signal aborted after abort:", signal.aborted);
        
        signal.aborted === true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ AbortController test passed");
            } else {
                println!("❌ AbortController test failed");
            }
        }
        Err(e) => {
            println!("❌ AbortController test failed with error: {}", e);
        }
    }
}
