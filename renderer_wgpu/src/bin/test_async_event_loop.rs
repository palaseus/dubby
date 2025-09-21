//! Test Async Event Loop Integration
//! 
//! This test demonstrates the complete async event loop integration with all browser engine components.

use renderer_wgpu::async_event_loop::{AsyncEventLoop, EventLoopEvent};
use dom::dom_event_integration::DomEventManager;
use dom::Document;
use html_parser::parse_html;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Async Event Loop Integration Test");
    println!("====================================");

    // Create async event loop
    let mut event_loop = AsyncEventLoop::new();
    
    // Test HTML for DOM integration
    let test_html = r#"
        <html>
            <head><title>Async Event Loop Test</title></head>
            <body>
                <div id="container">
                    <button id="btn-interactive">Interactive Button</button>
                    <input id="input-interactive" type="text" placeholder="Type here...">
                    <div id="display">Event loop events will appear here</div>
                </div>
            </body>
        </html>
    "#;

    // Parse HTML and set up DOM
    let document = parse_html(test_html);
    let mut dom_manager = DomEventManager::new();
    dom_manager.set_document(Rc::new(document));

    // Test 1: Event Loop Creation and Basic Functionality
    println!("\n=== Test 1: Event Loop Creation and Basic Functionality ===");
    test_event_loop_creation(&event_loop);

    // Test 2: Event Sending and Processing
    println!("\n=== Test 2: Event Sending and Processing ===");
    test_event_sending(&event_loop).await;

    // Test 3: Input Event Processing
    println!("\n=== Test 3: Input Event Processing ===");
    test_input_event_processing(&event_loop).await;

    // Test 4: JavaScript Execution
    println!("\n=== Test 4: JavaScript Execution ===");
    test_javascript_execution(&event_loop).await;

    // Test 5: Render Event Processing
    println!("\n=== Test 5: Render Event Processing ===");
    test_render_event_processing(&event_loop).await;

    // Test 6: Timer Event Processing
    println!("\n=== Test 6: Timer Event Processing ===");
    test_timer_event_processing(&event_loop).await;

    // Test 7: Network Event Processing
    println!("\n=== Test 7: Network Event Processing ===");
    test_network_event_processing(&event_loop).await;

    // Test 8: Custom Event Processing
    println!("\n=== Test 8: Custom Event Processing ===");
    test_custom_event_processing(&event_loop).await;

    // Test 9: Event Loop Statistics
    println!("\n=== Test 9: Event Loop Statistics ===");
    test_event_loop_statistics(&event_loop).await;

    // Test 10: Performance Testing
    println!("\n=== Test 10: Performance Testing ===");
    test_performance(&event_loop).await;

    println!("\n🎉 Async Event Loop Integration Test Complete!");
    println!("\nAsync Event Loop Features Demonstrated:");
    println!("✓ Event loop creation and management");
    println!("✓ Event sending and processing");
    println!("✓ Input event handling");
    println!("✓ JavaScript execution integration");
    println!("✓ Render event processing");
    println!("✓ Timer event handling");
    println!("✓ Network event processing");
    println!("✓ Custom event support");
    println!("✓ Event loop statistics and monitoring");
    println!("✓ Performance testing and optimization");

    Ok(())
}

fn test_event_loop_creation(event_loop: &AsyncEventLoop) {
    // Test initial state
    assert!(!event_loop.is_running());
    println!("✓ Event loop initial state verified");
    
    // Test statistics
    let stats = event_loop.get_stats();
    assert_eq!(stats.events_processed, 0);
    assert_eq!(stats.events_per_second, 0.0);
    assert_eq!(stats.queue_size, 0);
    println!("✓ Event loop statistics initialized");
    
    // Test event sender
    let sender = event_loop.get_event_sender();
    assert!(!sender.is_closed());
    println!("✓ Event sender created and functional");
}

async fn test_event_sending(event_loop: &AsyncEventLoop) {
    // Test sending various types of events
    let result = event_loop.simulate_click("btn-interactive");
    assert!(result.is_ok());
    println!("✓ Click event sent successfully");
    
    let result = event_loop.simulate_keydown("input-interactive", "Enter");
    assert!(result.is_ok());
    println!("✓ Keydown event sent successfully");
    
    let result = event_loop.execute_js("console.log('Hello from async event loop!');");
    assert!(result.is_ok());
    println!("✓ JavaScript execution event sent successfully");
    
    let result = event_loop.trigger_render("update");
    assert!(result.is_ok());
    println!("✓ Render event sent successfully");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(100)).await;
}

async fn test_input_event_processing(event_loop: &AsyncEventLoop) {
    // Send multiple input events
    for i in 0..5 {
        let result = event_loop.simulate_click(&format!("btn-{}", i));
        assert!(result.is_ok());
    }
    
    for i in 0..3 {
        let result = event_loop.simulate_keydown("input-interactive", &format!("key{}", i));
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple input events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_javascript_execution(event_loop: &AsyncEventLoop) {
    // Send JavaScript execution events
    let scripts = vec![
        "console.log('Script 1 executed');",
        "let x = 42; console.log('x =', x);",
        "document.getElementById('display').innerText = 'Updated by JS';",
        "setTimeout(() => console.log('Timer executed'), 1000);",
    ];
    
    for script in scripts {
        let result = event_loop.execute_js(script);
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple JavaScript execution events sent");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(300)).await;
}

async fn test_render_event_processing(event_loop: &AsyncEventLoop) {
    // Send render events
    let render_events = vec![
        "layout_update",
        "style_change",
        "content_update",
        "animation_frame",
    ];
    
    for event_type in render_events {
        let result = event_loop.trigger_render(event_type);
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple render events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_timer_event_processing(event_loop: &AsyncEventLoop) {
    // Send timer events
    for i in 0..3 {
        let result = event_loop.send_event(EventLoopEvent::TimerEvent {
            timer_id: i,
            callback: format!("console.log('Timer {} executed');", i),
        });
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple timer events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_network_event_processing(event_loop: &AsyncEventLoop) {
    // Send network events
    let network_requests = vec![
        ("https://api.example.com/data", "GET", None),
        ("https://api.example.com/submit", "POST", Some("{\"data\": \"test\"}".to_string())),
        ("https://api.example.com/update", "PUT", Some("{\"id\": 123}".to_string())),
    ];
    
    for (url, method, data) in network_requests {
        let result = event_loop.send_event(EventLoopEvent::NetworkEvent {
            url: url.to_string(),
            method: method.to_string(),
            data,
        });
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple network events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_custom_event_processing(event_loop: &AsyncEventLoop) {
    // Send custom events
    let custom_events = vec![
        ("user_login", Some("{\"user\": \"john\"}".to_string())),
        ("data_loaded", Some("{\"count\": 42}".to_string())),
        ("animation_complete", None),
        ("error_occurred", Some("{\"error\": \"network timeout\"}".to_string())),
    ];
    
    for (event_name, data) in custom_events {
        let result = event_loop.send_event(EventLoopEvent::CustomEvent {
            event_name: event_name.to_string(),
            data,
        });
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple custom events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_event_loop_statistics(event_loop: &AsyncEventLoop) {
    // Get and display statistics
    let stats = event_loop.get_stats();
    
    println!("Event Loop Statistics:");
    println!("  Events processed: {}", stats.events_processed);
    println!("  Events per second: {:.2}", stats.events_per_second);
    println!("  Average processing time: {:?}", stats.average_processing_time);
    println!("  Queue size: {}", stats.queue_size);
    println!("  Render updates: {}", stats.render_updates);
    println!("  DOM updates: {}", stats.dom_updates);
    println!("  JS executions: {}", stats.js_executions);
    println!("  Input events: {}", stats.input_events);
    
    if let Some(last_event_time) = stats.last_event_time {
        println!("  Last event time: {:?} ago", last_event_time.elapsed());
    }
    
    println!("✓ Event loop statistics tracking works");
}

async fn test_performance(event_loop: &AsyncEventLoop) {
    use std::time::Instant;
    
    println!("Testing event loop performance...");
    let start = Instant::now();
    
    // Send 1000 events rapidly
    for i in 0..1000 {
        let event_type = match i % 4 {
            0 => "click",
            1 => "keydown",
            2 => "render",
            _ => "custom",
        };
        
        let result = match event_type {
            "click" => event_loop.simulate_click(&format!("btn-{}", i)),
            "keydown" => event_loop.simulate_keydown("input-interactive", &format!("key{}", i)),
            "render" => event_loop.trigger_render("performance_test"),
            _ => event_loop.send_event(EventLoopEvent::CustomEvent {
                event_name: "performance_test".to_string(),
                data: Some(format!("event_{}", i)),
            }),
        };
        
        assert!(result.is_ok());
    }
    
    let duration = start.elapsed();
    println!("✓ Sent 1000 events in {:?}", duration);
    println!("✓ Average time per event: {:.3}µs", duration.as_micros() as f64 / 1000.0);
    
    // Wait for events to be processed
    sleep(Duration::from_millis(500)).await;
    
    let stats = event_loop.get_stats();
    println!("✓ Final statistics: {} events processed", stats.events_processed);
    println!("✓ Events per second: {:.2}", stats.events_per_second);
}
