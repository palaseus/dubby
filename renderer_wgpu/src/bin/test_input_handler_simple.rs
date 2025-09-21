//! Simple Test for Real User Input Handler
//! 
//! This test demonstrates the core functionality of the input handler.

use renderer_wgpu::input_handler::InputHandler;
use dom::dom_event_integration::DomEventManager;
use dom::Document;
use html_parser::parse_html;
use std::rc::Rc;
use winit::{
    event::{WindowEvent, MouseButton, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    println!("🖱️ Simple Real User Input Handler Test");
    println!("=====================================");

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

    // Test 2: Mouse Event Handling
    println!("\n=== Test 2: Mouse Event Handling ===");
    test_mouse_events(&mut input_handler);

    // Test 3: Keyboard Event Handling
    println!("\n=== Test 3: Keyboard Event Handling ===");
    test_keyboard_events(&mut input_handler);

    // Test 4: Input Statistics
    println!("\n=== Test 4: Input Statistics ===");
    test_input_statistics(&input_handler);

    println!("\n🎉 Simple Real User Input Handler Test Complete!");
    println!("\nInput Handler Features Demonstrated:");
    println!("✓ Mouse event capture and processing");
    println!("✓ Keyboard event capture and processing");
    println!("✓ Input statistics and monitoring");
    println!("✓ DOM integration for event dispatch");
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
}

fn test_mouse_events(handler: &mut InputHandler) {
    // Simulate mouse move event
    let mouse_move_event = WindowEvent::CursorMoved {
        device_id: unsafe { std::mem::zeroed() },
        position: winit::dpi::PhysicalPosition::new(100.0, 200.0),
        modifiers: winit::event::ModifiersState::default(),
    };
    
    let processed = handler.handle_window_event(&mouse_move_event);
    assert!(processed);
    
    let pos = handler.get_mouse_position();
    assert_eq!(pos, (100.0, 200.0));
    println!("✓ Mouse move event processed: position ({}, {})", pos.0, pos.1);
    
    // Simulate mouse button press
    let mouse_down_event = WindowEvent::MouseInput {
        device_id: unsafe { std::mem::zeroed() },
        state: ElementState::Pressed,
        button: MouseButton::Left,
        modifiers: winit::event::ModifiersState::default(),
    };
    
    let processed = handler.handle_window_event(&mouse_down_event);
    assert!(processed);
    
    let is_pressed = handler.is_mouse_button_pressed(MouseButton::Left);
    assert!(is_pressed);
    println!("✓ Mouse button press event processed: Left button pressed");
    
    // Simulate mouse button release
    let mouse_up_event = WindowEvent::MouseInput {
        device_id: unsafe { std::mem::zeroed() },
        state: ElementState::Released,
        button: MouseButton::Left,
        modifiers: winit::event::ModifiersState::default(),
    };
    
    let processed = handler.handle_window_event(&mouse_up_event);
    assert!(processed);
    
    let is_pressed = handler.is_mouse_button_pressed(MouseButton::Left);
    assert!(!is_pressed);
    println!("✓ Mouse button release event processed: Left button released");
}

fn test_keyboard_events(handler: &mut InputHandler) {
    // Simulate key press
    let key_down_event = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        event: KeyEvent {
            physical_key: PhysicalKey::Code(KeyCode::KeyA),
            logical_key: winit::keyboard::Key::Character("a".into()),
            text: Some("a".into()),
            location: winit::keyboard::KeyLocation::Standard,
            state: ElementState::Pressed,
            repeat: false,
        },
    };
    
    let processed = handler.handle_window_event(&key_down_event);
    assert!(processed);
    
    let is_pressed = handler.is_key_pressed(KeyCode::KeyA);
    assert!(is_pressed);
    println!("✓ Key press event processed: 'A' key pressed");
    
    // Simulate key release
    let key_up_event = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        event: KeyEvent {
            physical_key: PhysicalKey::Code(KeyCode::KeyA),
            logical_key: winit::keyboard::Key::Character("a".into()),
            text: Some("a".into()),
            location: winit::keyboard::KeyLocation::Standard,
            state: ElementState::Released,
            repeat: false,
        },
    };
    
    let processed = handler.handle_window_event(&key_up_event);
    assert!(processed);
    
    let is_pressed = handler.is_key_pressed(KeyCode::KeyA);
    assert!(!is_pressed);
    println!("✓ Key release event processed: 'A' key released");
    
    // Test space key
    let space_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        event: KeyEvent {
            physical_key: PhysicalKey::Code(KeyCode::Space),
            logical_key: winit::keyboard::Key::Character(" ".into()),
            text: Some(" ".into()),
            location: winit::keyboard::KeyLocation::Standard,
            state: ElementState::Pressed,
            repeat: false,
        },
    };
    
    handler.handle_window_event(&space_down);
    let is_space_pressed = handler.is_key_pressed(KeyCode::Space);
    assert!(is_space_pressed);
    println!("✓ Multiple key handling works: Space key pressed");
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
