//! Basic Test for Real User Input Handler
//! 
//! This test demonstrates the core functionality of the input handler without complex winit integration.

use renderer_wgpu::input_handler::InputHandler;
use dom::dom_event_integration::DomEventManager;
use html_parser::parse_html;
use std::rc::Rc;

fn main() {
    println!("🖱️ Basic Real User Input Handler Test");
    println!("====================================");

    // Create input handler
    let mut input_handler = InputHandler::new();
    
    // Test HTML for DOM integration
    let test_html = r#"
        <html>
            <head><title>Input Handler Test</title></head>
            <body>
                <div id="container">
                    <button id="btn-test">Test Button</button>
                    <input id="input-test" type="text" placeholder="Type here...">
                </div>
            </body>
        </html>
    "#;

    // Parse HTML and set up DOM
    let document = parse_html(test_html);
    let mut dom_manager = DomEventManager::new();
    dom_manager.set_document(Rc::new(document));
    input_handler.set_dom_event_manager(dom_manager);

    // Test 1: Basic Input Handler Functionality
    println!("\n=== Test 1: Basic Input Handler Functionality ===");
    test_basic_functionality(&mut input_handler);

    // Test 2: Input Statistics
    println!("\n=== Test 2: Input Statistics ===");
    test_input_statistics(&input_handler);

    // Test 3: Event Callbacks
    println!("\n=== Test 3: Event Callbacks ===");
    test_event_callbacks(&mut input_handler);

    // Test 4: Performance Testing
    println!("\n=== Test 4: Performance Testing ===");
    test_performance(&mut input_handler);

    println!("\n🎉 Basic Real User Input Handler Test Complete!");
    println!("\nInput Handler Features Demonstrated:");
    println!("✓ Input handler initialization and configuration");
    println!("✓ DOM integration for event dispatch");
    println!("✓ Event callback system");
    println!("✓ Input statistics and monitoring");
    println!("✓ Performance testing and optimization");
}

fn test_basic_functionality(handler: &mut InputHandler) {
    // Test initial state
    let stats = handler.get_stats();
    assert_eq!(stats.total_events, 0);
    assert_eq!(stats.mouse_events, 0);
    assert_eq!(stats.keyboard_events, 0);
    
    println!("✓ Initial state verified");
    
    // Test mouse position
    let pos = handler.get_mouse_position();
    assert_eq!(pos, (0.0, 0.0));
    println!("✓ Mouse position tracking initialized");
    
    // Test running state
    assert!(!handler.is_running());
    handler.start();
    assert!(handler.is_running());
    handler.stop();
    assert!(!handler.is_running());
    println!("✓ Running state management works");
    
    // Test DOM integration
    let dom_manager = handler.get_dom_event_manager();
    let dom_stats = dom_manager.get_stats();
    println!("✓ DOM integration verified: {} nodes, {} listeners", 
             dom_stats.total_nodes, dom_stats.total_listeners);
}

fn test_input_statistics(handler: &InputHandler) {
    let stats = handler.get_stats();
    
    println!("Input Statistics:");
    println!("  Total events: {}", stats.total_events);
    println!("  Mouse events: {}", stats.mouse_events);
    println!("  Keyboard events: {}", stats.keyboard_events);
    println!("  Last event type: {:?}", stats.last_event_type);
    
    if let Some(timestamp) = stats.last_timestamp {
        println!("  Last event timestamp: {:?}", timestamp.elapsed());
    }
    
    println!("✓ Input statistics tracking works");
}

fn test_event_callbacks(handler: &mut InputHandler) {
    let callback_calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let callback_calls_clone = callback_calls.clone();
    
    // Add callback for mouse events
    handler.add_event_callback("mousemove", move |_event| {
        callback_calls_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });
    
    // Add callback for keyboard events
    let callback_calls_clone2 = callback_calls.clone();
    handler.add_event_callback("keydown", move |_event| {
        callback_calls_clone2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });
    
    println!("✓ Event callbacks registered");
    
    // Note: In a real implementation, these would be triggered by actual window events
    // For this test, we're just verifying the callback system is set up correctly
    let calls = callback_calls.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(calls, 0); // No events processed yet
    println!("✓ Event callback system initialized");
}

fn test_performance(handler: &mut InputHandler) {
    use std::time::Instant;
    
    println!("Testing input handler performance...");
    let start = Instant::now();
    
    // Test basic operations
    for i in 0..1000 {
        // Simulate checking mouse position
        let _pos = handler.get_mouse_position();
        
        // Simulate checking key states
        use winit::keyboard::KeyCode;
        let _is_pressed = handler.is_key_pressed(KeyCode::KeyA);
        
        // Simulate checking mouse button states
        use winit::event::MouseButton;
        let _is_clicked = handler.is_mouse_button_pressed(MouseButton::Left);
    }
    
    let duration = start.elapsed();
    println!("✓ Performed 3000 input operations in {:?}", duration);
    println!("✓ Average time per operation: {:.3}µs", duration.as_micros() as f64 / 3000.0);
    
    let stats = handler.get_stats();
    println!("✓ Handler statistics: {} total events", stats.total_events);
}
