//! Test Event-Driven GPU Rendering
//! 
//! This test demonstrates the integration between the event system and GPU rendering,
//! showing how DOM events can trigger visual updates.

use renderer_wgpu::event_driven_renderer::EventDrivenRenderer;
use html_parser::parse_html;
use css_parser::parse_css;
use layout::LayoutEngine;
use std::rc::Rc;

fn main() {
    println!("🎨 Event-Driven GPU Rendering Test");
    println!("===================================");

    // Test HTML with interactive elements
    let test_html = r#"
        <html>
            <head>
                <title>Event-Driven Rendering Test</title>
            </head>
            <body>
                <div id="container" class="main-container">
                    <h1 id="title">Interactive Elements</h1>
                    <div id="controls" class="control-panel">
                        <button id="btn-red" class="color-btn">Red</button>
                        <button id="btn-green" class="color-btn">Green</button>
                        <button id="btn-blue" class="color-btn">Blue</button>
                    </div>
                    <div id="display" class="display-area">
                        <p id="text-display">Click a button to change my color!</p>
                    </div>
                    <input id="text-input" type="text" placeholder="Type something...">
                </div>
            </body>
        </html>
    "#;

    let test_css = r#"
        .main-container { padding: 20px; background: #f0f0f0; }
        .control-panel { margin: 10px 0; }
        .color-btn { margin: 5px; padding: 10px 20px; background: #007bff; color: white; }
        .display-area { margin: 20px 0; padding: 15px; background: white; border: 1px solid #ccc; }
        .display-area.red { background: #ffebee; color: #c62828; }
        .display-area.green { background: #e8f5e8; color: #2e7d32; }
        .display-area.blue { background: #e3f2fd; color: #1565c0; }
    "#;

    // Parse HTML and CSS
    println!("\n📄 Parsing HTML and CSS...");
    let document = parse_html(test_html);
    let stylesheet = parse_css(test_css);

    // Create layout engine and compute layout
    println!("🔧 Computing layout...");
    let layout_engine = LayoutEngine::new(stylesheet);
    let layout_tree = layout_engine.layout_document(&document);

    // Create event-driven renderer
    println!("🎨 Creating event-driven renderer...");
    let mut renderer = EventDrivenRenderer::new();
    renderer.set_document(Rc::new(document));
    renderer.set_layout_tree(layout_tree);

    // Test 1: Basic Event Handling
    println!("\n=== Test 1: Basic Event Handling ===");
    test_basic_event_handling(&mut renderer);

    // Test 2: Render Listener Integration
    println!("\n=== Test 2: Render Listener Integration ===");
    test_render_listener_integration(&mut renderer);

    // Test 3: Batch Event Processing
    println!("\n=== Test 3: Batch Event Processing ===");
    test_batch_event_processing(&mut renderer);

    // Test 4: Event Statistics
    println!("\n=== Test 4: Event Statistics ===");
    test_event_statistics(&renderer);

    // Test 5: Performance Testing
    println!("\n=== Test 5: Performance Testing ===");
    test_performance(&mut renderer);

    // Test 6: DOM Integration
    println!("\n=== Test 6: DOM Integration ===");
    test_dom_integration(&renderer);

    println!("\n🎉 Event-Driven GPU Rendering Test Complete!");
    println!("\nEvent-Driven Rendering Features Demonstrated:");
    println!("✓ Event handling and DOM integration");
    println!("✓ Render listener system");
    println!("✓ Batch event processing");
    println!("✓ Event statistics and monitoring");
    println!("✓ Performance testing and optimization");
    println!("✓ DOM node finding and event propagation");
}

fn test_basic_event_handling(renderer: &mut EventDrivenRenderer) {
    // Test click events
    let click_result1 = renderer.simulate_click("btn-red");
    let click_result2 = renderer.simulate_click("btn-green");
    let click_result3 = renderer.simulate_click("btn-blue");
    
    println!("✓ Click events processed: red={}, green={}, blue={}", 
             click_result1, click_result2, click_result3);

    // Test input events
    let input_result = renderer.simulate_input("text-input");
    println!("✓ Input event processed: {}", input_result);

    // Test keydown events
    let keydown_result = renderer.simulate_keydown("text-input");
    println!("✓ Keydown event processed: {}", keydown_result);

    // Test invalid target
    let invalid_result = renderer.simulate_click("non-existent");
    println!("✓ Invalid target handling: {}", !invalid_result);
}

fn test_render_listener_integration(renderer: &mut EventDrivenRenderer) {
    // Add render listeners for different event types
    renderer.add_render_listener("click", || {
        println!("  → Click event triggered render update");
    });

    renderer.add_render_listener("input", || {
        println!("  → Input event triggered render update");
    });

    renderer.add_render_listener("keydown", || {
        println!("  → Keydown event triggered render update");
    });

    println!("✓ Added render listeners for click, input, and keydown events");

    // Test that listeners are triggered
    renderer.simulate_click("btn-red");
    renderer.simulate_input("text-input");
    renderer.simulate_keydown("text-input");

    // Test listener count
    let click_listeners = renderer.get_render_listeners("click");
    let input_listeners = renderer.get_render_listeners("input");
    let keydown_listeners = renderer.get_render_listeners("keydown");
    
    println!("✓ Render listener counts: click={}, input={}, keydown={}", 
             click_listeners, input_listeners, keydown_listeners);
}

fn test_batch_event_processing(renderer: &mut EventDrivenRenderer) {
    // Create a batch of events
    let events = vec![
        ("click".to_string(), "btn-red".to_string()),
        ("click".to_string(), "btn-green".to_string()),
        ("click".to_string(), "btn-blue".to_string()),
        ("input".to_string(), "text-input".to_string()),
        ("keydown".to_string(), "text-input".to_string()),
    ];

    println!("Processing batch of {} events...", events.len());
    let processed = renderer.process_event_batch(events);
    println!("✓ Processed {} events in batch", processed);
}

fn test_event_statistics(renderer: &EventDrivenRenderer) {
    let stats = renderer.get_stats();
    
    println!("Event-Driven Renderer Statistics:");
    println!("  Has layout tree: {}", stats.has_layout_tree);
    println!("  Has document: {}", stats.has_document);
    println!("  Needs re-render: {}", stats.needs_rerender);
    println!("  Render listeners: {}", stats.render_listeners);
    
    println!("Event Statistics:");
    println!("  Events processed: {}", stats.event_stats.events_processed);
    println!("  Re-renders triggered: {}", stats.event_stats.rerenders_triggered);
    println!("  Last event type: {:?}", stats.event_stats.last_event_type);
    println!("  Last target ID: {:?}", stats.event_stats.last_target_id);
    
    println!("DOM Event Statistics:");
    println!("  Total nodes: {}", stats.dom_event_stats.total_nodes);
    println!("  Total listeners: {}", stats.dom_event_stats.total_listeners);
    println!("  Cached elements: {}", stats.dom_event_stats.cached_elements);
}

fn test_performance(renderer: &mut EventDrivenRenderer) {
    use std::time::Instant;
    
    println!("Testing event processing performance...");
    let start = Instant::now();

    // Process 1000 events
    for i in 0..1000 {
        let target_id = match i % 4 {
            0 => "btn-red",
            1 => "btn-green", 
            2 => "btn-blue",
            _ => "text-input",
        };
        
        let event_type = if i % 3 == 0 { "click" } else { "input" };
        renderer.handle_dom_event(event_type, target_id);
    }

    let duration = start.elapsed();
    println!("✓ Processed 1000 events in {:?}", duration);
    println!("✓ Average time per event: {:.3}µs", duration.as_micros() as f64 / 1000.0);
    
    let stats = renderer.get_stats();
    println!("✓ Total events processed: {}", stats.event_stats.events_processed);
    println!("✓ Total re-renders triggered: {}", stats.event_stats.rerenders_triggered);
}

fn test_dom_integration(renderer: &EventDrivenRenderer) {
    let dom_manager = renderer.get_dom_event_manager();
    
    // Test node finding
    if let Some(node) = dom_manager.find_node_by_id("btn-red") {
        println!("✓ Found button node with ID: {}", node.id);
    } else {
        println!("✗ Failed to find button node");
    }

    // Test class-based finding
    let color_buttons = dom_manager.find_nodes_by_class("color-btn");
    println!("✓ Found {} color button nodes", color_buttons.len());

    // Test tag-based finding
    let button_nodes = dom_manager.find_nodes_by_tag("button");
    println!("✓ Found {} button nodes", button_nodes.len());

    let div_nodes = dom_manager.find_nodes_by_tag("div");
    println!("✓ Found {} div nodes", div_nodes.len());

    // Test DOM event statistics
    let dom_stats = dom_manager.get_stats();
    println!("✓ DOM event manager statistics:");
    println!("  Total nodes: {}", dom_stats.total_nodes);
    println!("  Total listeners: {}", dom_stats.total_listeners);
    println!("  Cached elements: {}", dom_stats.cached_elements);
}
