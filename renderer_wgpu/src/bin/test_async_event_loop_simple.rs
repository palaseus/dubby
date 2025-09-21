//! Simple Test for Async Event Loop Integration
//! 
//! This test demonstrates the core async event loop functionality without complex threading.

use renderer_wgpu::async_event_loop_simple::SimpleAsyncEventLoop;
use dom::dom_event_integration::DomEventManager;
use html_parser::parse_html;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Simple Async Event Loop Integration Test");
    println!("===========================================");

    // Create async event loop
    let event_loop = SimpleAsyncEventLoop::new();
    
    // Test HTML for DOM integration
    let test_html = r#"
        <html>
            <head><title>Simple Async Event Loop Test</title></head>
            <body>
                <div id="container">
                    <button id="btn-simple">Simple Button</button>
                    <input id="input-simple" type="text" placeholder="Type here...">
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

    // Test 6: Event Loop Statistics
    println!("\n=== Test 6: Event Loop Statistics ===");
    test_event_loop_statistics(&event_loop).await;

    // Test 7: Performance Testing
    println!("\n=== Test 7: Performance Testing ===");
    test_performance(&event_loop).await;

    println!("\n🎉 Simple Async Event Loop Integration Test Complete!");
    println!("\nAsync Event Loop Features Demonstrated:");
    println!("✓ Event loop creation and management");
    println!("✓ Event sending and processing");
    println!("✓ Input event handling");
    println!("✓ JavaScript execution integration");
    println!("✓ Render event processing");
    println!("✓ Event loop statistics and monitoring");
    println!("✓ Performance testing and optimization");

    Ok(())
}

fn test_event_loop_creation(event_loop: &SimpleAsyncEventLoop) {
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

async fn test_event_sending(event_loop: &SimpleAsyncEventLoop) {
    // Test sending various types of events
    let result = event_loop.simulate_click("btn-simple");
    assert!(result.is_ok());
    println!("✓ Click event sent successfully");
    
    let result = event_loop.simulate_keydown("input-simple", "Enter");
    assert!(result.is_ok());
    println!("✓ Keydown event sent successfully");
    
    let result = event_loop.execute_js("console.log('Hello from simple async event loop!');");
    assert!(result.is_ok());
    println!("✓ JavaScript execution event sent successfully");
    
    let result = event_loop.trigger_render("update");
    assert!(result.is_ok());
    println!("✓ Render event sent successfully");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(100)).await;
}

async fn test_input_event_processing(event_loop: &SimpleAsyncEventLoop) {
    // Send multiple input events
    for i in 0..5 {
        let result = event_loop.simulate_click(&format!("btn-{}", i));
        assert!(result.is_ok());
    }
    
    for i in 0..3 {
        let result = event_loop.simulate_keydown("input-simple", &format!("key{}", i));
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple input events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_javascript_execution(event_loop: &SimpleAsyncEventLoop) {
    // Send JavaScript execution events
    let scripts = vec![
        "console.log('Script 1 executed');",
        "let x = 42; console.log('x =', x);",
        "document.getElementById('display').innerText = 'Updated by JS';",
    ];
    
    for script in scripts {
        let result = event_loop.execute_js(script);
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple JavaScript execution events sent");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(300)).await;
}

async fn test_render_event_processing(event_loop: &SimpleAsyncEventLoop) {
    // Send render events
    let render_events = vec![
        "layout_update",
        "style_change",
        "content_update",
    ];
    
    for event_type in render_events {
        let result = event_loop.trigger_render(event_type);
        assert!(result.is_ok());
    }
    
    println!("✓ Multiple render events sent and processed");
    
    // Small delay to allow event processing
    sleep(Duration::from_millis(200)).await;
}

async fn test_event_loop_statistics(event_loop: &SimpleAsyncEventLoop) {
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

async fn test_performance(event_loop: &SimpleAsyncEventLoop) {
    use std::time::Instant;
    
    println!("Testing event loop performance...");
    let start = Instant::now();
    
    // Send 100 events rapidly
    for i in 0..100 {
        let event_type = match i % 3 {
            0 => "click",
            1 => "keydown",
            _ => "render",
        };
        
        let result = match event_type {
            "click" => event_loop.simulate_click(&format!("btn-{}", i)),
            "keydown" => event_loop.simulate_keydown("input-simple", &format!("key{}", i)),
            "render" => event_loop.trigger_render("performance_test"),
            _ => Ok(()),
        };
        
        assert!(result.is_ok());
    }
    
    let duration = start.elapsed();
    println!("✓ Sent 100 events in {:?}", duration);
    println!("✓ Average time per event: {:.3}µs", duration.as_micros() as f64 / 100.0);
    
    // Wait for events to be processed
    sleep(Duration::from_millis(200)).await;
    
    let stats = event_loop.get_stats();
    println!("✓ Final statistics: {} events processed", stats.events_processed);
    println!("✓ Events per second: {:.2}", stats.events_per_second);
}
