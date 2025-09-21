use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::event_types::*;
use crate::Node;

/// Event dispatcher for handling event propagation
#[derive(Debug)]
pub struct EventDispatcher {
    /// Performance metrics
    pub dispatch_count: u64,
    pub total_dispatch_time: u64,
    pub max_dispatch_time: u64,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            dispatch_count: 0,
            total_dispatch_time: 0,
            max_dispatch_time: 0,
        }
    }

    /// Dispatch an event through the DOM tree with proper propagation
    pub fn dispatch_event(
        &mut self,
        target: Rc<RefCell<dyn EventTarget>>,
        mut event: Event,
        document_root: Option<Rc<RefCell<Node>>>,
    ) -> bool {
        let start_time = std::time::Instant::now();
        self.dispatch_count += 1;

        // Set the target
        event.target = Some(target.clone());
        event.current_target = Some(target.clone());

        // Build the event path (capturing phase path)
        let event_path = self.build_event_path(&target, document_root);
        
        // Phase 1: Capturing phase (root → target)
        if !event.propagation_stopped {
            event.phase = EventPhase::Capturing;
            for (i, node) in event_path.iter().enumerate().rev() {
                if i == 0 { break; } // Skip target in capturing phase
                event.current_target = Some(node.clone());
                
                if self.execute_listeners(node, &mut event) {
                    break; // stop_immediate_propagation called
                }
            }
        }

        // Phase 2: Target phase
        if !event.propagation_stopped {
            event.phase = EventPhase::AtTarget;
            event.current_target = Some(target.clone());
            self.execute_listeners(&target, &mut event);
        }

        // Phase 3: Bubbling phase (target → root)
        if event.bubbles && !event.propagation_stopped {
            event.phase = EventPhase::Bubbling;
            for node in event_path.iter().skip(1) {
                event.current_target = Some(node.clone());
                
                if self.execute_listeners(node, &mut event) {
                    break; // stop_immediate_propagation called
                }
            }
        }

        // Update performance metrics
        let dispatch_time = start_time.elapsed().as_micros() as u64;
        self.total_dispatch_time += dispatch_time;
        self.max_dispatch_time = self.max_dispatch_time.max(dispatch_time);

        !event.default_prevented
    }

    /// Build the event path from target to root
    fn build_event_path(
        &self,
        target: &Rc<RefCell<dyn EventTarget>>,
        document_root: Option<Rc<RefCell<Node>>>,
    ) -> Vec<Rc<RefCell<dyn EventTarget>>> {
        let mut path = Vec::new();
        
        // Try to get the path through the DOM tree
        if let Some(root) = document_root {
            if let Ok(_target_node) = target.clone().try_borrow() {
                // This is a simplified path building - in a real implementation,
                // we'd traverse the DOM tree from target to root
                path.push(target.clone());
                
                // For now, we'll add a mock parent path
                // In a real implementation, this would traverse the actual DOM tree
                if let Some(parent) = self.find_parent_node(&target, &root) {
                    path.push(parent);
                }
            }
        } else {
            // Fallback: just the target
            path.push(target.clone());
        }

        path
    }

    /// Find parent node in the DOM tree
    fn find_parent_node(
        &self,
        _target: &Rc<RefCell<dyn EventTarget>>,
        _root: &Rc<RefCell<Node>>,
    ) -> Option<Rc<RefCell<dyn EventTarget>>> {
        // This is a simplified implementation
        // In a real browser, we'd traverse the DOM tree to find the parent
        // For now, we'll return None to keep the path simple
        None
    }

    /// Execute event listeners for a given target
    fn execute_listeners(
        &self,
        target: &Rc<RefCell<dyn EventTarget>>,
        event: &mut Event,
    ) -> bool {
        let listeners = target.borrow().get_event_listeners(&event.event_type);
        
        for listener in listeners {
            // Filter listeners based on current phase
            let should_execute = match event.phase {
                EventPhase::Capturing => listener.options.capture,
                EventPhase::AtTarget => true, // Execute all listeners on target
                EventPhase::Bubbling => !listener.options.capture,
                EventPhase::None => false,
            };

            if should_execute {
                // Execute the listener (in a real implementation, this would call JavaScript)
                self.execute_listener_callback(&listener, event);
                
                if event.immediate_propagation_stopped {
                    return true;
                }
            }
        }

        false
    }

    /// Execute a single listener callback
    fn execute_listener_callback(&self, listener: &EventListener, event: &mut Event) {
        // In a real implementation, this would:
        // 1. Convert the event to a JavaScript Event object
        // 2. Call the JavaScript function with the event
        // 3. Handle any exceptions
        // 4. Update the event object based on JavaScript modifications
        
        println!(
            "Executing listener {} for event {} in phase {:?}",
            listener.id, event.event_type, event.phase
        );

        // Simulate JavaScript execution
        if listener.options.once {
            // Mark for removal after execution
            println!("Listener {} marked for removal (once: true)", listener.id);
        }
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> EventDispatchStats {
        EventDispatchStats {
            dispatch_count: self.dispatch_count,
            total_dispatch_time: self.total_dispatch_time,
            max_dispatch_time: self.max_dispatch_time,
            avg_dispatch_time: if self.dispatch_count > 0 {
                self.total_dispatch_time / self.dispatch_count
            } else {
                0
            },
        }
    }
}

/// Performance statistics for event dispatching
#[derive(Debug, Clone)]
pub struct EventDispatchStats {
    pub dispatch_count: u64,
    pub total_dispatch_time: u64,
    pub max_dispatch_time: u64,
    pub avg_dispatch_time: u64,
}

/// Event delegation manager for efficient listener lookup
pub struct EventDelegationManager {
    /// Delegated event handlers: (parent_id, event_type, selector) -> callback
    delegated_handlers: HashMap<(String, String, String), String>,
}

impl EventDelegationManager {
    pub fn new() -> Self {
        Self {
            delegated_handlers: HashMap::new(),
        }
    }

    /// Register a delegated event handler
    pub fn add_delegated_handler(
        &mut self,
        parent_id: &str,
        event_type: &str,
        selector: &str,
        callback: String,
    ) {
        self.delegated_handlers
            .insert((parent_id.to_string(), event_type.to_string(), selector.to_string()), callback);
    }

    /// Remove a delegated event handler
    pub fn remove_delegated_handler(&mut self, parent_id: &str, event_type: &str) {
        // Remove all handlers for this parent_id and event_type
        let keys_to_remove: Vec<_> = self.delegated_handlers
            .keys()
            .filter(|(pid, et, _)| pid == parent_id && et == event_type)
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            self.delegated_handlers.remove(&key);
        }
    }

    /// Check if an event should be handled by delegation
    pub fn should_delegate(&self, event_type: &str) -> bool {
        self.delegated_handlers
            .keys()
            .any(|(_, et, _)| et == event_type)
    }

    /// Get delegated handlers for an event type
    pub fn get_delegated_handlers(&self, event_type: &str) -> Vec<(String, String)> {
        self.delegated_handlers
            .keys()
            .filter(|(_, et, _)| et == event_type)
            .map(|(_, _, selector)| (selector.clone(), event_type.to_string()))
            .collect()
    }
}

/// Synthetic event factory
pub struct SyntheticEventFactory;

impl SyntheticEventFactory {
    /// Create a synthetic click event
    pub fn create_click_event(x: f64, y: f64, button: i32) -> MouseEvent {
        let mut event = MouseEvent::new("click", true, true);
        event.client_x = x;
        event.client_y = y;
        event.button = button;
        event.buttons = 1 << button.max(0) as u32;
        event
    }

    /// Create a synthetic mouseover event
    pub fn create_mouseover_event(x: f64, y: f64) -> MouseEvent {
        let mut event = MouseEvent::new("mouseover", true, true);
        event.client_x = x;
        event.client_y = y;
        event
    }

    /// Create a synthetic mouseout event
    pub fn create_mouseout_event(x: f64, y: f64) -> MouseEvent {
        let mut event = MouseEvent::new("mouseout", true, true);
        event.client_x = x;
        event.client_y = y;
        event
    }

    /// Create a synthetic input event
    pub fn create_input_event(data: Option<String>, input_type: &str) -> InputEvent {
        let mut event = InputEvent::new("input", true, true);
        event.data = data;
        event.input_type = input_type.to_string();
        event
    }

    /// Create a synthetic focus event
    pub fn create_focus_event() -> FocusEvent {
        FocusEvent::new("focus", false, false)
    }

    /// Create a synthetic blur event
    pub fn create_blur_event() -> FocusEvent {
        FocusEvent::new("blur", false, false)
    }

    /// Create a synthetic keydown event
    pub fn create_keydown_event(key: &str, code: &str, key_code: u32) -> KeyboardEvent {
        let mut event = KeyboardEvent::new("keydown", true, true);
        event.key = key.to_string();
        event.code = code.to_string();
        event.key_code = key_code;
        event
    }

    /// Create a synthetic keyup event
    pub fn create_keyup_event(key: &str, code: &str, key_code: u32) -> KeyboardEvent {
        let mut event = KeyboardEvent::new("keyup", true, true);
        event.key = key.to_string();
        event.code = code.to_string();
        event.key_code = key_code;
        event
    }

    /// Create a custom event
    pub fn create_custom_event(event_type: &str, detail: Option<String>) -> CustomEvent {
        let mut event = CustomEvent::new(event_type, true, true);
        event.detail = detail;
        event
    }
}
