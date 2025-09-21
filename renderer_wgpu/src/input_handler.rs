//! Real User Input Handler
//! 
//! This module provides real mouse and keyboard event capture that integrates
//! with the event system and GPU rendering pipeline.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use winit::{
    event::{WindowEvent, MouseButton, ElementState},
    keyboard::{KeyCode, PhysicalKey},
};
use dom::dom_event_integration::DomEventManager;

/// Real-time input handler for mouse and keyboard events
pub struct InputHandler {
    /// DOM event manager for dispatching events
    dom_event_manager: DomEventManager,
    /// Mouse position tracking
    mouse_position: (f64, f64),
    /// Mouse button states
    mouse_buttons: HashMap<MouseButton, bool>,
    /// Keyboard key states
    keyboard_keys: HashMap<KeyCode, bool>,
    /// Input event statistics
    input_stats: InputStats,
    /// Event callback registry
    event_callbacks: HashMap<String, Vec<Box<dyn Fn(&InputEvent) + Send + Sync>>>,
    /// Running state
    is_running: Arc<AtomicBool>,
}

/// Input event data structure
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub position: Option<(f64, f64)>,
    pub button: Option<MouseButton>,
    pub key: Option<KeyCode>,
    pub modifiers: KeyModifiers,
    pub timestamp: std::time::Instant,
}

/// Types of input events
#[derive(Debug, Clone, PartialEq)]
pub enum InputEventType {
    MouseMove,
    MouseDown,
    MouseUp,
    MouseClick,
    MouseDoubleClick,
    MouseWheel,
    KeyDown,
    KeyUp,
    KeyPress,
    Focus,
    Blur,
    Resize,
}

/// Keyboard modifier keys
#[derive(Debug, Clone, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

/// Input handling statistics
#[derive(Debug, Default)]
pub struct InputStats {
    pub mouse_events: u64,
    pub keyboard_events: u64,
    pub total_events: u64,
    pub last_event_type: Option<InputEventType>,
    pub last_timestamp: Option<std::time::Instant>,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            dom_event_manager: DomEventManager::new(),
            mouse_position: (0.0, 0.0),
            mouse_buttons: HashMap::new(),
            keyboard_keys: HashMap::new(),
            input_stats: InputStats::default(),
            event_callbacks: HashMap::new(),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Set the DOM event manager
    pub fn set_dom_event_manager(&mut self, manager: DomEventManager) {
        self.dom_event_manager = manager;
    }

    /// Get DOM event manager reference
    pub fn get_dom_event_manager(&self) -> &DomEventManager {
        &self.dom_event_manager
    }

    /// Add an event callback
    pub fn add_event_callback<F>(&mut self, event_type: &str, callback: F)
    where
        F: Fn(&InputEvent) + Send + Sync + 'static,
    {
        self.event_callbacks
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Handle window events from winit
    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        let mut input_event = None;

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x, position.y);
                input_event = Some(InputEvent {
                    event_type: InputEventType::MouseMove,
                    position: Some(self.mouse_position),
                    button: None,
                    key: None,
                    modifiers: self.get_current_modifiers(),
                    timestamp: std::time::Instant::now(),
                });
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                self.mouse_buttons.insert(*button, is_pressed);

                let event_type = if is_pressed {
                    InputEventType::MouseDown
                } else {
                    InputEventType::MouseUp
                };

                input_event = Some(InputEvent {
                    event_type: event_type.clone(),
                    position: Some(self.mouse_position),
                    button: Some(*button),
                    key: None,
                    modifiers: self.get_current_modifiers(),
                    timestamp: std::time::Instant::now(),
                });

                // Handle click events (down + up sequence)
                if !is_pressed && self.mouse_buttons.get(button).copied().unwrap_or(false) {
                    let click_event = InputEvent {
                        event_type: InputEventType::MouseClick,
                        position: Some(self.mouse_position),
                        button: Some(*button),
                        key: None,
                        modifiers: self.get_current_modifiers(),
                        timestamp: std::time::Instant::now(),
                    };
                    self.process_input_event(click_event);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = &event.physical_key {
                    let is_pressed = event.state == ElementState::Pressed;
                    self.keyboard_keys.insert(*keycode, is_pressed);

                    let event_type = if is_pressed {
                        InputEventType::KeyDown
                    } else {
                        InputEventType::KeyUp
                    };

                    input_event = Some(InputEvent {
                        event_type: event_type.clone(),
                        position: None,
                        button: None,
                        key: Some(*keycode),
                        modifiers: self.get_current_modifiers(),
                        timestamp: std::time::Instant::now(),
                    });

                    // Handle key press events (for printable characters)
                    if is_pressed && self.is_printable_key(*keycode) {
                        let press_event = InputEvent {
                            event_type: InputEventType::KeyPress,
                            position: None,
                            button: None,
                            key: Some(*keycode),
                            modifiers: self.get_current_modifiers(),
                            timestamp: std::time::Instant::now(),
                        };
                        self.process_input_event(press_event);
                    }
                }
            }
            WindowEvent::Focused(focused) => {
                let event_type = if *focused {
                    InputEventType::Focus
                } else {
                    InputEventType::Blur
                };

                input_event = Some(InputEvent {
                    event_type,
                    position: None,
                    button: None,
                    key: None,
                    modifiers: self.get_current_modifiers(),
                    timestamp: std::time::Instant::now(),
                });
            }
            WindowEvent::Resized(_) => {
                input_event = Some(InputEvent {
                    event_type: InputEventType::Resize,
                    position: None,
                    button: None,
                    key: None,
                    modifiers: self.get_current_modifiers(),
                    timestamp: std::time::Instant::now(),
                });
            }
            _ => {}
        }

        if let Some(event) = input_event {
            self.process_input_event(event);
            true
        } else {
            false
        }
    }

    /// Process an input event
    fn process_input_event(&mut self, event: InputEvent) {
        // Update statistics
        self.input_stats.total_events += 1;
        self.input_stats.last_event_type = Some(event.event_type.clone());
        self.input_stats.last_timestamp = Some(event.timestamp);

        match event.event_type {
            InputEventType::MouseMove | InputEventType::MouseDown | InputEventType::MouseUp | InputEventType::MouseClick => {
                self.input_stats.mouse_events += 1;
            }
            InputEventType::KeyDown | InputEventType::KeyUp | InputEventType::KeyPress => {
                self.input_stats.keyboard_events += 1;
            }
            _ => {}
        }

        // Dispatch to DOM event manager
        self.dispatch_to_dom(&event);

        // Call registered callbacks
        self.call_event_callbacks(&event);
    }

    /// Dispatch input event to DOM event manager
    fn dispatch_to_dom(&mut self, event: &InputEvent) {
        let dom_event_type = match event.event_type {
            InputEventType::MouseClick => "click",
            InputEventType::MouseDown => "mousedown",
            InputEventType::MouseUp => "mouseup",
            InputEventType::MouseMove => "mousemove",
            InputEventType::KeyDown => "keydown",
            InputEventType::KeyUp => "keyup",
            InputEventType::KeyPress => "keypress",
            InputEventType::Focus => "focus",
            InputEventType::Blur => "blur",
            _ => return,
        };

        // Find target element at mouse position (simplified)
        if let Some(position) = event.position {
            if let Some(target_id) = self.find_element_at_position(position) {
                let dom_event = dom::event_types::Event::new(dom_event_type, true, true);
                if let Some(target_node) = self.dom_event_manager.find_node_by_id(&target_id) {
                    let _result = self.dom_event_manager.dispatch_event(&target_node, dom_event);
                }
            }
        }
    }

    /// Find element at mouse position (simplified implementation)
    fn find_element_at_position(&self, _position: (f64, f64)) -> Option<String> {
        // This is a simplified implementation
        // In a real browser, this would involve hit testing against the layout tree
        Some("btn-red".to_string()) // Placeholder
    }

    /// Call registered event callbacks
    fn call_event_callbacks(&self, event: &InputEvent) {
        let event_type_str = match event.event_type {
            InputEventType::MouseMove => "mousemove",
            InputEventType::MouseDown => "mousedown",
            InputEventType::MouseUp => "mouseup",
            InputEventType::MouseClick => "click",
            InputEventType::MouseDoubleClick => "dblclick",
            InputEventType::MouseWheel => "wheel",
            InputEventType::KeyDown => "keydown",
            InputEventType::KeyUp => "keyup",
            InputEventType::KeyPress => "keypress",
            InputEventType::Focus => "focus",
            InputEventType::Blur => "blur",
            InputEventType::Resize => "resize",
        };

        if let Some(callbacks) = self.event_callbacks.get(event_type_str) {
            for callback in callbacks {
                callback(event);
            }
        }
    }

    /// Get current keyboard modifiers
    fn get_current_modifiers(&self) -> KeyModifiers {
        KeyModifiers {
            ctrl: self.keyboard_keys.get(&KeyCode::ControlLeft).copied().unwrap_or(false) ||
                  self.keyboard_keys.get(&KeyCode::ControlRight).copied().unwrap_or(false),
            alt: self.keyboard_keys.get(&KeyCode::AltLeft).copied().unwrap_or(false) ||
                 self.keyboard_keys.get(&KeyCode::AltRight).copied().unwrap_or(false),
            shift: self.keyboard_keys.get(&KeyCode::ShiftLeft).copied().unwrap_or(false) ||
                   self.keyboard_keys.get(&KeyCode::ShiftRight).copied().unwrap_or(false),
            meta: self.keyboard_keys.get(&KeyCode::SuperLeft).copied().unwrap_or(false) ||
                  self.keyboard_keys.get(&KeyCode::SuperRight).copied().unwrap_or(false),
        }
    }

    /// Check if a key is printable
    fn is_printable_key(&self, key: KeyCode) -> bool {
        matches!(key,
            KeyCode::KeyA | KeyCode::KeyB | KeyCode::KeyC | KeyCode::KeyD |
            KeyCode::KeyE | KeyCode::KeyF | KeyCode::KeyG | KeyCode::KeyH |
            KeyCode::KeyI | KeyCode::KeyJ | KeyCode::KeyK | KeyCode::KeyL |
            KeyCode::KeyM | KeyCode::KeyN | KeyCode::KeyO | KeyCode::KeyP |
            KeyCode::KeyQ | KeyCode::KeyR | KeyCode::KeyS | KeyCode::KeyT |
            KeyCode::KeyU | KeyCode::KeyV | KeyCode::KeyW | KeyCode::KeyX |
            KeyCode::KeyY | KeyCode::KeyZ |
            KeyCode::Digit0 | KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 |
            KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 |
            KeyCode::Digit8 | KeyCode::Digit9 |
            KeyCode::Space | KeyCode::Enter | KeyCode::Tab
        )
    }

    /// Get input statistics
    pub fn get_stats(&self) -> &InputStats {
        &self.input_stats
    }

    /// Get mouse position
    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    /// Check if a mouse button is pressed
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.get(&button).copied().unwrap_or(false)
    }

    /// Check if a key is pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard_keys.get(&key).copied().unwrap_or(false)
    }

    /// Start the input handler (for testing)
    pub fn start(&mut self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    /// Stop the input handler
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Check if the input handler is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_handler_creation() {
        let handler = InputHandler::new();
        let stats = handler.get_stats();
        
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.mouse_events, 0);
        assert_eq!(stats.keyboard_events, 0);
        assert!(!handler.is_running());
    }

    #[test]
    fn test_mouse_position_tracking() {
        let handler = InputHandler::new();
        assert_eq!(handler.get_mouse_position(), (0.0, 0.0));
    }

    #[test]
    fn test_key_modifiers() {
        let handler = InputHandler::new();
        let modifiers = handler.get_current_modifiers();
        
        assert!(!modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.shift);
        assert!(!modifiers.meta);
    }

    #[test]
    fn test_printable_key_detection() {
        let handler = InputHandler::new();
        
        assert!(handler.is_printable_key(KeyCode::KeyA));
        assert!(handler.is_printable_key(KeyCode::Space));
        assert!(!handler.is_printable_key(KeyCode::Escape));
        assert!(!handler.is_printable_key(KeyCode::F1));
    }
}
