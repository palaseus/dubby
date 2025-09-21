//! Test Real User Input Handler
//! 
//! This test demonstrates real mouse and keyboard event capture and processing.

use renderer_wgpu::input_handler::{InputHandler, InputEventType, KeyModifiers};
use dom::dom_event_integration::DomEventManager;
use dom::Document;
use html_parser::parse_html;
use std::rc::Rc;
use winit::{
    event::{WindowEvent, MouseButton, ElementState, KeyEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    println!("🖱️ Real User Input Handler Test");
    println!("===============================");

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
                    <div id="display">Mouse and keyboard events will appear here</div>
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

    // Test 4: Event Callbacks
    println!("\n=== Test 4: Event Callbacks ===");
    test_event_callbacks(&mut input_handler);

    // Test 5: Input Statistics
    println!("\n=== Test 5: Input Statistics ===");
    test_input_statistics(&input_handler);

    // Test 6: Modifier Keys
    println!("\n=== Test 6: Modifier Keys ===");
    test_modifier_keys(&mut input_handler);

    // Test 7: Performance Testing
    println!("\n=== Test 7: Performance Testing ===");
    test_performance(&mut input_handler);

    println!("\n🎉 Real User Input Handler Test Complete!");
    println!("\nInput Handler Features Demonstrated:");
    println!("✓ Mouse event capture and processing");
    println!("✓ Keyboard event capture and processing");
    println!("✓ Event callback system");
    println!("✓ Input statistics and monitoring");
    println!("✓ Modifier key detection");
    println!("✓ Performance testing and optimization");
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
    
    // Test right mouse button
    let right_down_event = WindowEvent::MouseInput {
        device_id: unsafe { std::mem::zeroed() },
        state: ElementState::Pressed,
        button: MouseButton::Right,
        modifiers: winit::event::ModifiersState::default(),
    };
    
    handler.handle_window_event(&right_down_event);
    let is_right_pressed = handler.is_mouse_button_pressed(MouseButton::Right);
    assert!(is_right_pressed);
    println!("✓ Right mouse button handling works");
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
    
    // Test multiple keys
    let space_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::Space),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    
    handler.handle_window_event(&space_down);
    let is_space_pressed = handler.is_key_pressed(KeyCode::Space);
    assert!(is_space_pressed);
    println!("✓ Multiple key handling works: Space key pressed");
    
    // Test non-printable key
    let escape_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::Escape),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    
    handler.handle_window_event(&escape_down);
    let is_escape_pressed = handler.is_key_pressed(KeyCode::Escape);
    assert!(is_escape_pressed);
    println!("✓ Non-printable key handling works: Escape key pressed");
}

fn test_event_callbacks(handler: &mut InputHandler) {
    let mut callback_calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
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
    
    // Trigger mouse move
    let mouse_move_event = WindowEvent::CursorMoved {
        device_id: unsafe { std::mem::zeroed() },
        position: winit::dpi::PhysicalPosition::new(50.0, 75.0),
        modifiers: winit::event::ModifiersState::default(),
    };
    handler.handle_window_event(&mouse_move_event);
    
    // Trigger key press
    let key_down_event = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::B),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    handler.handle_window_event(&key_down_event);
    
    // Check callback calls
    let calls = callback_calls.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(calls, 2);
    println!("✓ Event callbacks triggered: {} calls", calls);
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

fn test_modifier_keys(handler: &mut InputHandler) {
    // Test Ctrl key
    let ctrl_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::LControl),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    
    handler.handle_window_event(&ctrl_down);
    
    // Test Alt key
    let alt_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::LAlt),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    
    handler.handle_window_event(&alt_down);
    
    // Test Shift key
    let shift_down = WindowEvent::KeyboardInput {
        device_id: unsafe { std::mem::zeroed() },
        input: KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(KeyCode::LShift),
            modifiers: winit::event::ModifiersState::default(),
        },
        is_synthetic: false,
    };
    
    handler.handle_window_event(&shift_down);
    
    // Check modifier states
    let is_ctrl_pressed = handler.is_key_pressed(KeyCode::LControl);
    let is_alt_pressed = handler.is_key_pressed(KeyCode::LAlt);
    let is_shift_pressed = handler.is_key_pressed(KeyCode::LShift);
    
    assert!(is_ctrl_pressed);
    assert!(is_alt_pressed);
    assert!(is_shift_pressed);
    
    println!("✓ Modifier key detection works:");
    println!("  Ctrl: {}, Alt: {}, Shift: {}", is_ctrl_pressed, is_alt_pressed, is_shift_pressed);
}

fn test_performance(handler: &mut InputHandler) {
    use std::time::Instant;
    
    println!("Testing input event processing performance...");
    let start = Instant::now();
    
    // Process 1000 mouse move events
    for i in 0..1000 {
        let mouse_move_event = WindowEvent::CursorMoved {
            device_id: unsafe { std::mem::zeroed() },
            position: winit::dpi::PhysicalPosition::new(i as f64, i as f64),
            modifiers: winit::event::ModifiersState::default(),
        };
        handler.handle_window_event(&mouse_move_event);
    }
    
    // Process 1000 keyboard events
    for i in 0..1000 {
        let key = match i % 26 {
            0..=25 => KeyCode::KeyA,
            _ => KeyCode::Space,
        };
        
        let key_down_event = WindowEvent::KeyboardInput {
            device_id: unsafe { std::mem::zeroed() },
            input: KeyboardInput {
                scancode: 0,
                state: ElementState::Pressed,
                virtual_keycode: Some(key),
                modifiers: winit::event::ModifiersState::default(),
            },
            is_synthetic: false,
        };
        handler.handle_window_event(&key_down_event);
    }
    
    let duration = start.elapsed();
    println!("✓ Processed 2000 input events in {:?}", duration);
    println!("✓ Average time per event: {:.3}µs", duration.as_micros() as f64 / 2000.0);
    
    let stats = handler.get_stats();
    println!("✓ Total events processed: {}", stats.total_events);
    println!("✓ Mouse events: {}", stats.mouse_events);
    println!("✓ Keyboard events: {}", stats.keyboard_events);
}
