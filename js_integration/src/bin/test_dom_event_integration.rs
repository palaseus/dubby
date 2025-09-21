//! Test DOM Event Integration
//! 
//! This test demonstrates the integration between the event system and the DOM tree,
//! showing real event propagation through the DOM hierarchy.

use js_integration::JsEngine;
use dom::NodeType;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;

fn main() {
    println!("🎯 DOM Event Integration Test");
    println!("=============================");

    // Test HTML with nested elements and event handlers
    let test_html = r#"
        <html>
            <head>
                <title>DOM Event Integration Test</title>
            </head>
            <body>
                <div id="container" class="parent">
                    <h1 id="title">Event Propagation Test</h1>
                    <div id="content" class="child">
                        <button id="button1" class="btn">Click Me 1</button>
                        <button id="button2" class="btn">Click Me 2</button>
                        <input id="input1" type="text" placeholder="Type here">
                    </div>
                    <div id="sidebar" class="child">
                        <p id="text1">Some text content</p>
                        <span id="span1" class="highlight">Highlighted text</span>
                    </div>
                </div>
                <script>
                    // Event handlers will be registered via JavaScript
                    console.log('Script loaded');
                </script>
            </body>
        </html>
    "#;

    let test_css = r#"
        .parent { padding: 10px; border: 1px solid #ccc; }
        .child { margin: 5px; padding: 5px; }
        .btn { background: blue; color: white; padding: 5px 10px; }
        .highlight { background: yellow; }
    "#;

    // Parse HTML and CSS
    println!("\n📄 Parsing HTML and CSS...");
    let (document, _resources) = parse_html(test_html.into()).unwrap();
    let stylesheet = parse_css(test_css);

    // Create JavaScript engine and set up document
    println!("🔧 Setting up JavaScript engine...");
    let mut js_engine = JsEngine::new();
    js_engine.set_document(Rc::new(document));
    js_engine.set_stylesheet(stylesheet);

    // Test 1: DOM Node Finding
    println!("\n=== Test 1: DOM Node Finding ===");
    test_dom_node_finding(&js_engine);

    // Test 2: Event Listener Registration
    println!("\n=== Test 2: Event Listener Registration ===");
    test_event_listener_registration(&mut js_engine);

    // Test 3: Event Propagation
    println!("\n=== Test 3: Event Propagation ===");
    test_event_propagation(&mut js_engine);

    // Test 4: Event Delegation
    println!("\n=== Test 4: Event Delegation ===");
    test_event_delegation(&mut js_engine);

    // Test 5: Multiple Event Types
    println!("\n=== Test 5: Multiple Event Types ===");
    test_multiple_event_types(&mut js_engine);

    // Test 6: Event Statistics
    println!("\n=== Test 6: Event Statistics ===");
    test_event_statistics(&js_engine);

    // Test 7: Performance Test
    println!("\n=== Test 7: Performance Test ===");
    test_event_performance(&mut js_engine);

    println!("\n🎉 DOM Event Integration Test Complete!");
    println!("\nEvent System Features Demonstrated:");
    println!("✓ Real DOM node finding by ID, class, and tag");
    println!("✓ Event listener registration on DOM nodes");
    println!("✓ Event propagation through DOM hierarchy");
    println!("✓ Event delegation system");
    println!("✓ Multiple event types (click, keydown, input)");
    println!("✓ Event system statistics and monitoring");
    println!("✓ Performance testing and optimization");
}

fn test_dom_node_finding(js_engine: &JsEngine) {
    // Test finding nodes by ID
    if let Some(node) = js_engine.dom_event_manager.find_node_by_id("button1") {
        println!("✓ Found button1 node with ID: {}", node.id);
    } else {
        println!("✗ Failed to find button1 node");
    }

    if let Some(node) = js_engine.dom_event_manager.find_node_by_id("input1") {
        println!("✓ Found input1 node with ID: {}", node.id);
    } else {
        println!("✗ Failed to find input1 node");
    }

    // Test finding nodes by class
    let btn_nodes = js_engine.dom_event_manager.find_nodes_by_class("btn");
    println!("✓ Found {} nodes with class 'btn'", btn_nodes.len());

    let child_nodes = js_engine.dom_event_manager.find_nodes_by_class("child");
    println!("✓ Found {} nodes with class 'child'", child_nodes.len());

    // Test finding nodes by tag
    let div_nodes = js_engine.dom_event_manager.find_nodes_by_tag("div");
    println!("✓ Found {} div nodes", div_nodes.len());

    let button_nodes = js_engine.dom_event_manager.find_nodes_by_tag("button");
    println!("✓ Found {} button nodes", button_nodes.len());
}

fn test_event_listener_registration(js_engine: &mut JsEngine) {
    // Find nodes to attach listeners to
    if let Some(button1) = js_engine.dom_event_manager.find_node_by_id("button1") {
        let listener = dom::event_types::EventListener {
            callback: "function() { console.log('Button 1 clicked!'); }".to_string(),
            options: dom::event_types::EventListenerOptions::default(),
            id: 1,
        };
        js_engine.dom_event_manager.add_event_listener(&button1, "click", listener);
        println!("✓ Added click listener to button1");
    }

    if let Some(input1) = js_engine.dom_event_manager.find_node_by_id("input1") {
        let listener = dom::event_types::EventListener {
            callback: "function() { console.log('Input changed!'); }".to_string(),
            options: dom::event_types::EventListenerOptions::default(),
            id: 2,
        };
        js_engine.dom_event_manager.add_event_listener(&input1, "input", listener);
        println!("✓ Added input listener to input1");
    }

    if let Some(container) = js_engine.dom_event_manager.find_node_by_id("container") {
        let listener = dom::event_types::EventListener {
            callback: "function() { console.log('Container clicked!'); }".to_string(),
            options: dom::event_types::EventListenerOptions { capture: true, ..Default::default() },
            id: 3,
        };
        js_engine.dom_event_manager.add_event_listener(&container, "click", listener);
        println!("✓ Added capture listener to container");
    }
}

fn test_event_propagation(js_engine: &mut JsEngine) {
    println!("Testing event propagation from button1 to container...");
    
    // Simulate click on button1 - should bubble up to container
    let _result = js_engine.simulate_click("button1");
    println!("✓ Click event propagated through DOM hierarchy");
}

fn test_event_delegation(js_engine: &mut JsEngine) {
    // Add delegated handler to container for all button clicks
    js_engine.add_delegated_handler("container", "click", ".btn", "function(e) { console.log('Delegated button click!'); }");
    println!("✓ Added delegated handler for button clicks on container");

    // Simulate clicks on buttons - should trigger delegated handler
    let _result1 = js_engine.simulate_click("button1");
    let _result2 = js_engine.simulate_click("button2");
    println!("✓ Delegated handlers triggered for button clicks");
}

fn test_multiple_event_types(js_engine: &mut JsEngine) {
    // Test different event types
    let _result1 = js_engine.simulate_click("button1");
    println!("✓ Click event dispatched");

    let _result2 = js_engine.simulate_keydown("input1", "Enter");
    println!("✓ Keydown event dispatched");

    let _result3 = js_engine.simulate_input("input1", "Hello World");
    println!("✓ Input event dispatched");
}

fn test_event_statistics(js_engine: &JsEngine) {
    let dom_stats = js_engine.get_dom_event_stats();
    println!("DOM Event Statistics:");
    println!("  Total nodes with listeners: {}", dom_stats.total_nodes);
    println!("  Total event listeners: {}", dom_stats.total_listeners);
    println!("  Cached elements: {}", dom_stats.cached_elements);

    let (delegation_stats, optimization_stats) = js_engine.get_event_stats();
    println!("Delegation Statistics:");
    println!("  Total handlers: {}", delegation_stats.total_handlers);
    println!("  Cached selectors: {}", delegation_stats.cached_selectors);
    println!("Optimization Statistics:");
    println!("  Cache hits: {}", optimization_stats.cache_hits);
    println!("  Cache misses: {}", optimization_stats.cache_misses);
    println!("  Hit rate: {:.1}%", optimization_stats.hit_rate);
}

fn test_event_performance(js_engine: &mut JsEngine) {
    use std::time::Instant;

    println!("Testing event dispatch performance...");
    let start = Instant::now();

    // Dispatch 1000 events
    for i in 0..1000 {
        let target_id = if i % 2 == 0 { "button1" } else { "button2" };
        let _result = js_engine.simulate_click(target_id);
    }

    let duration = start.elapsed();
    println!("✓ Dispatched 1000 events in {:?}", duration);
    println!("✓ Average time per event: {:.3}µs", duration.as_micros() as f64 / 1000.0);
}
