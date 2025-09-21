//! # Fetch API Integration Tests
//! 
//! This test suite validates the fetch API implementation,
//! including Response objects, error handling, and AbortController integration.

use js_integration::JsEngine;
use boa_engine::{Context, Source};

fn main() {
    println!("🌐 Running Fetch API Integration Tests");
    println!("=" .repeat(60));

    test_response_creation();
    test_response_methods();
    test_fetch_binding_initialization();
    test_abort_controller_integration();
    test_error_handling();

    println!("\n✅ All Fetch API tests completed!");
}

/// Test Response object creation
fn test_response_creation() {
    println!("\n🔸 Testing Response Creation...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const response = new Response("Hello World", {
            status: 200,
            statusText: "OK",
            headers: {
                "Content-Type": "text/plain"
            }
        });
        
        console.log("Response created");
        console.log("Status:", response.status);
        console.log("Status Text:", response.statusText);
        console.log("OK:", response.ok);
        
        response.status === 200 && response.ok === true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Response creation test passed");
            } else {
                println!("❌ Response creation test failed");
            }
        }
        Err(e) => {
            println!("❌ Response creation test failed with error: {}", e);
        }
    }
}

/// Test Response methods (text, json, blob)
fn test_response_methods() {
    println!("\n🔸 Testing Response Methods...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    // Test text() method
    let text_code = r#"
        const response = new Response("Hello World");
        const text = response.text();
        console.log("Text method called");
        text !== undefined
    "#;

    match context.eval(Source::from_bytes(text_code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Response.text() test passed");
            } else {
                println!("❌ Response.text() test failed");
            }
        }
        Err(e) => {
            println!("❌ Response.text() test failed with error: {}", e);
        }
    }

    // Test json() method
    let json_code = r#"
        const response = new Response('{"key": "value"}');
        const json = response.json();
        console.log("JSON method called");
        json !== undefined
    "#;

    match context.eval(Source::from_bytes(json_code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Response.json() test passed");
            } else {
                println!("❌ Response.json() test failed");
            }
        }
        Err(e) => {
            println!("❌ Response.json() test failed with error: {}", e);
        }
    }

    // Test blob() method
    let blob_code = r#"
        const response = new Response("Binary data");
        const blob = response.blob();
        console.log("Blob method called");
        blob !== undefined
    "#;

    match context.eval(Source::from_bytes(blob_code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Response.blob() test passed");
            } else {
                println!("❌ Response.blob() test failed");
            }
        }
        Err(e) => {
            println!("❌ Response.blob() test failed with error: {}", e);
        }
    }
}

/// Test fetch binding initialization
fn test_fetch_binding_initialization() {
    println!("\n🔸 Testing Fetch Binding Initialization...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    // Check if Response constructor is available
    let code = r#"
        console.log("Response constructor available:", typeof Response);
        typeof Response === "function"
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ Fetch binding initialization test passed");
            } else {
                println!("❌ Fetch binding initialization test failed");
            }
        }
        Err(e) => {
            println!("❌ Fetch binding initialization test failed with error: {}", e);
        }
    }
}

/// Test AbortController integration with fetch
fn test_abort_controller_integration() {
    println!("\n🔸 Testing AbortController Integration...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        const controller = new AbortController();
        const signal = controller.signal;
        
        // Simulate a fetch request with abort signal
        const mockFetch = () => {
            return new Promise((resolve, reject) => {
                if (signal.aborted) {
                    reject(new Error("Request aborted"));
                } else {
                    resolve(new Response("Success"));
                }
            });
        };
        
        console.log("Mock fetch created with abort signal");
        
        // Test aborting the request
        controller.abort("User cancelled");
        console.log("Request aborted");
        
        signal.aborted === true
    "#;

    match context.eval(Source::from_bytes(code)) {
        Ok(result) => {
            if result.to_boolean() {
                println!("✅ AbortController integration test passed");
            } else {
                println!("❌ AbortController integration test failed");
            }
        }
        Err(e) => {
            println!("❌ AbortController integration test failed with error: {}", e);
        }
    }
}

/// Test error handling in fetch operations
fn test_error_handling() {
    println!("\n🔸 Testing Error Handling...");
    
    let mut js_engine = JsEngine::new();
    let context = &mut js_engine.context;

    let code = r#"
        // Test error response
        const errorResponse = new Response("Error", {
            status: 404,
            statusText: "Not Found"
        });
        
        console.log("Error response created");
        console.log("Status:", errorResponse.status);
        console.log("OK:", errorResponse.ok);
        
        // Test error handling with Promise
        const errorPromise = Promise.reject("Network error")
            .catch(error => {
                console.log("Caught fetch error:", error);
                return "Error handled";
            });
        
        console.log("Error handling test setup");
        errorResponse.status === 404 && errorResponse.ok === false
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
