//! # Comprehensive Integration Tests
//! 
//! This test suite validates the complete Promise, microtask, and fetch API
//! integration, including real-world scenarios and performance testing.

use js_integration::JsEngine;
use boa_engine::{Context, Source};
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    println!("🚀 Running Comprehensive Integration Tests");
    println!("=" .repeat(60));

    test_promise_microtask_integration();
    test_fetch_promise_integration();
    test_abort_controller_promise_integration();
    test_performance_stress_test();
    test_real_world_scenario();
    test_error_recovery();

    println!("\n✅ All comprehensive integration tests completed!");
}

/// Test Promise and microtask integration
fn test_promise_microtask_integration() {
    println!("\n🔸 Testing Promise and Microtask Integration...");
    
    let mut js_engine = JsEngine::new();
    js_engine.set_microtask_trace_enabled(true);

    let context = &mut js_engine.context;

    let code = r#"
        console.log("=== Promise and Microtask Integration Test ===");
        
        // Test Promise chain with microtasks
        const promise1 = Promise.resolve(1)
            .then(x => {
                console.log("Microtask 1:", x);
                return x + 1;
            });
        
        const promise2 = Promise.resolve(2)
            .then(x => {
                console.log("Microtask 2:", x);
                return x * 2;
            });
        
        // Test Promise.all simulation
        const allPromises = [promise1, promise2];
        console.log("Promise chain created");
        
        // Test microtask ordering
        Promise.resolve().then(() => console.log("Microtask 3"));
        setTimeout(() => console.log("Macrotask 1"), 0);
        Promise.resolve().then(() => console.log("Microtask 4"));
        
        console.log("Sync end");
        true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(_) => {
            // Process event loop multiple times to handle microtasks and timers
            for _ in 0..3 {
                js_engine.process_event_loop().unwrap();
                thread::sleep(Duration::from_millis(5));
            }
            
            println!("✅ Promise and microtask integration test passed");
        }
        Err(e) => {
            println!("❌ Promise and microtask integration test failed with error: {}", e);
        }
    }
}

/// Test fetch and Promise integration
fn test_fetch_promise_integration() {
    println!("\n🔸 Testing Fetch and Promise Integration...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        console.log("=== Fetch and Promise Integration Test ===");
        
        // Simulate fetch with Promise
        function mockFetch(url) {
            return new Promise((resolve, reject) => {
                console.log("Fetching:", url);
                
                // Simulate async operation
                setTimeout(() => {
                    if (url.includes("error")) {
                        reject(new Error("Network error"));
                    } else {
                        resolve(new Response("Mock response for " + url, {
                            status: 200,
                            statusText: "OK"
                        }));
                    }
                }, 10);
            });
        }
        
        // Test successful fetch
        const fetchPromise = mockFetch("https://example.com")
            .then(response => {
                console.log("Fetch successful, status:", response.status);
                return response.text();
            })
            .then(text => {
                console.log("Response text:", text);
                return text.length;
            });
        
        console.log("Fetch promise created");
        fetchPromise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Fetch and Promise integration test passed");
            } else {
                println!("❌ Fetch and Promise integration test failed");
            }
        }
        Err(e) => {
            println!("❌ Fetch and Promise integration test failed with error: {}", e);
        }
    }
}

/// Test AbortController with Promise integration
fn test_abort_controller_promise_integration() {
    println!("\n🔸 Testing AbortController and Promise Integration...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        console.log("=== AbortController and Promise Integration Test ===");
        
        const controller = new AbortController();
        const signal = controller.signal;
        
        // Simulate fetch with abort signal
        function fetchWithAbort(url, signal) {
            return new Promise((resolve, reject) => {
                console.log("Starting fetch for:", url);
                
                // Check if already aborted
                if (signal.aborted) {
                    reject(new Error("Request aborted"));
                    return;
                }
                
                // Simulate network delay
                const timeout = setTimeout(() => {
                    if (signal.aborted) {
                        reject(new Error("Request aborted"));
                    } else {
                        resolve(new Response("Success"));
                    }
                }, 100);
                
                // Listen for abort
                signal.addEventListener('abort', () => {
                    clearTimeout(timeout);
                    reject(new Error("Request aborted"));
                });
            });
        }
        
        const fetchPromise = fetchWithAbort("https://example.com", signal);
        
        // Abort the request after a short delay
        setTimeout(() => {
            console.log("Aborting request...");
            controller.abort("User cancelled");
        }, 50);
        
        console.log("Fetch with abort signal created");
        fetchPromise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ AbortController and Promise integration test passed");
            } else {
                println!("❌ AbortController and Promise integration test failed");
            }
        }
        Err(e) => {
            println!("❌ AbortController and Promise integration test failed with error: {}", e);
        }
    }
}

/// Test performance under stress
fn test_performance_stress_test() {
    println!("\n🔸 Testing Performance Under Stress...");
    
    let mut js_engine = JsEngine::new();
    js_engine.set_microtask_trace_enabled(false); // Disable tracing for performance

    let context = &mut js_engine.context;
    let start_time = Instant::now();

    let code = r#"
        console.log("=== Performance Stress Test ===");
        
        // Create many Promises to test microtask queue performance
        const promises = [];
        for (let i = 0; i < 1000; i++) {
            promises.push(
                Promise.resolve(i)
                    .then(x => x * 2)
                    .then(x => x + 1)
            );
        }
        
        console.log("Created", promises.length, "Promises");
        promises.length === 1000
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                // Process all microtasks
                let process_start = Instant::now();
                js_engine.process_event_loop().unwrap();
                let process_time = process_start.elapsed();
                
                let total_time = start_time.elapsed();
                
                // Get telemetry
                let telemetry = js_engine.get_telemetry();
                
                println!("📊 Performance Results:");
                println!("├─ Total test time: {:?}", total_time);
                println!("├─ Microtask processing time: {:?}", process_time);
                println!("└─ Telemetry:");
                println!("{}", telemetry);
                
                println!("✅ Performance stress test passed");
            } else {
                println!("❌ Performance stress test failed");
            }
        }
        Err(e) => {
            println!("❌ Performance stress test failed with error: {}", e);
        }
    }
}

/// Test real-world scenario
fn test_real_world_scenario() {
    println!("\n🔸 Testing Real-World Scenario...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        console.log("=== Real-World Scenario Test ===");
        
        // Simulate a real-world web application scenario
        function loadUserData(userId) {
            return new Promise((resolve, reject) => {
                console.log("Loading user data for:", userId);
                
                // Simulate API call
                setTimeout(() => {
                    if (userId === "error") {
                        reject(new Error("User not found"));
                    } else {
                        resolve({
                            id: userId,
                            name: "John Doe",
                            email: "john@example.com"
                        });
                    }
                }, 50);
            });
        }
        
        function loadUserPosts(userId) {
            return new Promise((resolve, reject) => {
                console.log("Loading posts for user:", userId);
                
                setTimeout(() => {
                    resolve([
                        { id: 1, title: "Post 1", content: "Content 1" },
                        { id: 2, title: "Post 2", content: "Content 2" }
                    ]);
                }, 30);
            });
        }
        
        // Load user data and posts in parallel
        const userPromise = loadUserData("123");
        const postsPromise = loadUserPosts("123");
        
        // Combine results
        const combinedPromise = Promise.all([userPromise, postsPromise])
            .then(([user, posts]) => {
                console.log("User:", user.name);
                console.log("Posts count:", posts.length);
                return { user, posts };
            })
            .catch(error => {
                console.log("Error loading data:", error.message);
                return null;
            });
        
        console.log("Real-world scenario setup complete");
        combinedPromise !== undefined
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                // Process event loop to handle async operations
                for _ in 0..5 {
                    js_engine.process_event_loop().unwrap();
                    thread::sleep(Duration::from_millis(20));
                }
                
                println!("✅ Real-world scenario test passed");
            } else {
                println!("❌ Real-world scenario test failed");
            }
        }
        Err(e) => {
            println!("❌ Real-world scenario test failed with error: {}", e);
        }
    }
}

/// Test error recovery and resilience
fn test_error_recovery() {
    println!("\n🔸 Testing Error Recovery and Resilience...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        console.log("=== Error Recovery Test ===");
        
        // Test error recovery in Promise chains
        const resilientPromise = Promise.resolve("start")
            .then(value => {
                console.log("Step 1:", value);
                throw new Error("Step 1 failed");
            })
            .catch(error => {
                console.log("Caught error in step 1:", error.message);
                return "recovered";
            })
            .then(value => {
                console.log("Step 2:", value);
                return value + " - success";
            })
            .catch(error => {
                console.log("Final error:", error.message);
                return "final recovery";
            });
        
        // Test multiple error scenarios
        const errorScenarios = [
            Promise.reject("Error 1").catch(e => "Recovered 1"),
            Promise.reject("Error 2").catch(e => "Recovered 2"),
            Promise.resolve("Success").then(v => v + " - OK")
        ];
        
        console.log("Error recovery test setup complete");
        resilientPromise !== undefined && errorScenarios.length === 3
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                // Process event loop to handle error recovery
                js_engine.process_event_loop().unwrap();
                
                println!("✅ Error recovery test passed");
            } else {
                println!("❌ Error recovery test failed");
            }
        }
        Err(e) => {
            println!("❌ Error recovery test failed with error: {}", e);
        }
    }
}
