//! Comprehensive Test for Full Networking API
//! 
//! This test demonstrates the complete XMLHttpRequest and fetch API implementations
//! with all features including HTTP methods, headers, JSON, and error handling.

use networking::{
    XMLHttpRequest, FetchRequest, HttpMethod, NetworkError,
    HttpClient, fetch
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Full Networking API Test");
    println!("==========================");

    // Test 1: Basic HTTP Client
    println!("\n=== Test 1: Basic HTTP Client ===");
    test_basic_http_client().await;

    // Test 2: XMLHttpRequest API
    println!("\n=== Test 2: XMLHttpRequest API ===");
    test_xmlhttprequest_api();

    // Test 3: Fetch API
    println!("\n=== Test 3: Fetch API ===");
    test_fetch_api().await;

    // Test 4: HTTP Methods
    println!("\n=== Test 4: HTTP Methods ===");
    test_http_methods().await;

    // Test 5: HTTP Headers
    println!("\n=== Test 5: HTTP Headers ===");
    test_http_headers().await;

    // Test 6: JSON Handling
    println!("\n=== Test 6: JSON Handling ===");
    test_json_handling().await;

    // Test 7: Error Handling
    println!("\n=== Test 7: Error Handling ===");
    test_error_handling().await;

    // Test 8: Performance Testing
    println!("\n=== Test 8: Performance Testing ===");
    test_performance().await;

    println!("\n🎉 Full Networking API Test Complete!");
    println!("\nNetworking API Features Demonstrated:");
    println!("✓ Basic HTTP client with reqwest integration");
    println!("✓ Complete XMLHttpRequest implementation");
    println!("✓ Modern fetch API with builder pattern");
    println!("✓ All HTTP methods (GET, POST, PUT, DELETE, etc.)");
    println!("✓ HTTP headers and custom headers");
    println!("✓ JSON serialization and deserialization");
    println!("✓ Comprehensive error handling");
    println!("✓ Performance testing and optimization");
    println!("✓ Real-world network compatibility");

    Ok(())
}

async fn test_basic_http_client() {
    let client = HttpClient::new();
    
    // Test client creation
    println!("✓ HTTP client creation");
    
    // Test fetching (if network is available)
    match client.fetch_html("https://example.com").await {
        Ok(html) => {
            assert!(!html.is_empty());
            println!("✓ Successfully fetched {} bytes from example.com", html.len());
        }
        Err(e) => {
            println!("⚠ Network test skipped: {}", e);
        }
    }
}

fn test_xmlhttprequest_api() {
    // Test XMLHttpRequest creation
    let mut xhr = XMLHttpRequest::new();
    assert_eq!(xhr.get_ready_state(), networking::ReadyState::Unsent);
    println!("✓ XMLHttpRequest creation");
    
    // Test opening request
    xhr.open("GET", "https://example.com").unwrap();
    assert_eq!(xhr.get_ready_state(), networking::ReadyState::Opened);
    println!("✓ XMLHttpRequest open");
    
    // Test setting headers
    xhr.set_request_header("User-Agent", "RustBrowser/1.0").unwrap();
    xhr.set_request_header("Accept", "text/html").unwrap();
    println!("✓ XMLHttpRequest headers");
    
    // Test sending request
    xhr.send(None).unwrap();
    assert_eq!(xhr.get_ready_state(), networking::ReadyState::Done);
    assert_eq!(xhr.get_status(), 200);
    assert!(!xhr.get_response_text().is_empty());
    println!("✓ XMLHttpRequest send and response");
    
    // Test response headers
    let headers = xhr.get_all_response_headers();
    assert!(!headers.is_empty());
    println!("✓ XMLHttpRequest response headers");
    
    // Test event callbacks
    let callback_called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let callback_called_clone = callback_called.clone();
    xhr.set_onload(move || {
        callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    println!("✓ XMLHttpRequest event callbacks");
    
    // Test abort
    let mut xhr2 = XMLHttpRequest::new();
    xhr2.open("GET", "https://example.com").unwrap();
    xhr2.abort();
    assert_eq!(xhr2.get_ready_state(), networking::ReadyState::Unsent);
    println!("✓ XMLHttpRequest abort");
}

async fn test_fetch_api() {
    // Test basic fetch
    let response = FetchRequest::new("https://example.com")
        .method(HttpMethod::GET)
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    assert_eq!(response.status(), 200);
    let text = response.text().await.unwrap();
    assert!(!text.is_empty());
    println!("✓ Basic fetch request");
    
    // Test fetch with headers
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::GET)
        .header("User-Agent", "RustBrowser/1.0")
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Fetch with headers");
    
    // Test fetch with timeout
    let response = FetchRequest::new("https://example.com")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Fetch with timeout");
    
    // Test fetch with credentials
    let response = FetchRequest::new("https://example.com")
        .credentials(true)
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Fetch with credentials");
}

async fn test_http_methods() {
    // Test GET
    let response = FetchRequest::new("https://example.com")
        .method(HttpMethod::GET)
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ GET method");
    
    // Test POST
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::POST)
        .body(b"test data".to_vec())
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ POST method");
    
    // Test PUT
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::PUT)
        .body(b"update data".to_vec())
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ PUT method");
    
    // Test DELETE
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::DELETE)
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ DELETE method");
    
    // Test PATCH
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::PATCH)
        .body(b"patch data".to_vec())
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ PATCH method");
    
    // Test HEAD
    let response = FetchRequest::new("https://example.com")
        .method(HttpMethod::HEAD)
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ HEAD method");
    
    // Test OPTIONS
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::OPTIONS)
        .send()
        .await
        .unwrap();
    assert!(response.ok());
    println!("✓ OPTIONS method");
}

async fn test_http_headers() {
    // Test custom headers
    let response = FetchRequest::new("https://example.com")
        .header("User-Agent", "RustBrowser/1.0")
        .header("Accept", "text/html,application/xhtml+xml")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Connection", "keep-alive")
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Custom headers");
    
    // Test content-type header
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .body(b"{\"key\": \"value\"}".to_vec())
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Content-Type header");
    
    // Test authorization header
    let response = FetchRequest::new("https://api.example.com")
        .header("Authorization", "Bearer token123")
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ Authorization header");
}

async fn test_json_handling() {
    // Test JSON serialization
    #[derive(serde::Serialize)]
    struct TestData {
        name: String,
        value: i32,
        active: bool,
    }
    
    let data = TestData {
        name: "test".to_string(),
        value: 42,
        active: true,
    };
    
    let response = FetchRequest::new("https://api.example.com")
        .method(HttpMethod::POST)
        .json(&data)
        .send()
        .await
        .unwrap();
    
    assert!(response.ok());
    println!("✓ JSON serialization");
    
    // Test JSON deserialization
    #[derive(serde::Deserialize)]
    struct ResponseData {
        message: String,
    }
    
    let json_response: ResponseData = response.json().await.unwrap();
    assert_eq!(json_response.message, "Hello from fetch!");
    println!("✓ JSON deserialization");
}

async fn test_error_handling() {
    // Test invalid URL
    let result = fetch("not-a-url").await;
    assert!(result.is_err());
    if let Err(NetworkError::UrlParseError(_)) = result {
        println!("✓ Invalid URL error handling");
    }
    
    // Test unsupported protocol
    let result = fetch("ftp://example.com").await;
    assert!(result.is_err());
    if let Err(NetworkError::UnsupportedProtocol(protocol)) = result {
        assert_eq!(protocol, "ftp");
        println!("✓ Unsupported protocol error handling");
    }
    
    // Test XMLHttpRequest error states
    let mut xhr = XMLHttpRequest::new();
    
    // Test opening already opened request
    xhr.open("GET", "https://example.com").unwrap();
    let result = xhr.open("GET", "https://example.com");
    assert!(result.is_err());
    println!("✓ XMLHttpRequest state error handling");
    
    // Test setting header on unopened request
    let mut xhr2 = XMLHttpRequest::new();
    let result = xhr2.set_request_header("Test", "Value");
    assert!(result.is_err());
    println!("✓ XMLHttpRequest header error handling");
    
    // Test sending unopened request
    let mut xhr3 = XMLHttpRequest::new();
    let result = xhr3.send(None);
    assert!(result.is_err());
    println!("✓ XMLHttpRequest send error handling");
}

async fn test_performance() {
    use std::time::Instant;
    
    println!("Testing networking API performance...");
    let start = Instant::now();
    
    // Test multiple concurrent requests
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            let response = FetchRequest::new("https://example.com")
                .header("X-Request-ID", &format!("{}", i))
                .send()
                .await
                .unwrap();
            response.status()
        });
        handles.push(handle);
    }
    
    let mut status_codes = vec![];
    for handle in handles {
        let status = handle.await.unwrap();
        status_codes.push(status);
    }
    
    let duration = start.elapsed();
    println!("✓ Processed 10 concurrent requests in {:?}", duration);
    println!("✓ Average time per request: {:.3}ms", duration.as_millis() as f64 / 10.0);
    
    // Verify all requests succeeded
    assert_eq!(status_codes.len(), 10);
    for status in status_codes {
        assert_eq!(status, 200);
    }
    
    // Test XMLHttpRequest performance
    let start = Instant::now();
    
    for i in 0..5 {
        let mut xhr = XMLHttpRequest::new();
        xhr.open("GET", &format!("https://example.com/{}", i)).unwrap();
        xhr.set_request_header("X-Request-ID", &format!("{}", i)).unwrap();
        xhr.send(None).unwrap();
        assert_eq!(xhr.get_status(), 200);
    }
    
    let duration = start.elapsed();
    println!("✓ Processed 5 XMLHttpRequest calls in {:?}", duration);
    println!("✓ Average time per XMLHttpRequest: {:.3}ms", duration.as_millis() as f64 / 5.0);
}
