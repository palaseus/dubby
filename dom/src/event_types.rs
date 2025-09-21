use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Event propagation phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    None = 0,
    Capturing = 1,
    AtTarget = 2,
    Bubbling = 3,
}

/// Event listener options
#[derive(Debug, Clone, PartialEq)]
pub struct EventListenerOptions {
    pub capture: bool,
    pub once: bool,
    pub passive: bool,
}

impl Default for EventListenerOptions {
    fn default() -> Self {
        Self {
            capture: false,
            once: false,
            passive: false,
        }
    }
}

/// Event listener entry
#[derive(Debug, Clone)]
pub struct EventListener {
    pub callback: String, // JavaScript function reference
    pub options: EventListenerOptions,
    pub id: u64,
}

/// Event target interface
pub trait EventTarget {
    fn add_event_listener(&mut self, event_type: &str, listener: EventListener);
    fn remove_event_listener(&mut self, event_type: &str, listener_id: u64);
    fn dispatch_event(&mut self, event: &mut Event) -> bool;
    fn get_event_listeners(&self, event_type: &str) -> Vec<EventListener>;
}

/// Base event structure
#[derive(Clone)]
pub struct Event {
    pub event_type: String,
    pub target: Option<Rc<RefCell<dyn EventTarget>>>,
    pub current_target: Option<Rc<RefCell<dyn EventTarget>>>,
    pub phase: EventPhase,
    pub bubbles: bool,
    pub cancelable: bool,
    pub default_prevented: bool,
    pub propagation_stopped: bool,
    pub immediate_propagation_stopped: bool,
    pub timestamp: u64,
    pub is_trusted: bool,
}

impl Event {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            event_type: event_type.to_string(),
            target: None,
            current_target: None,
            phase: EventPhase::None,
            bubbles,
            cancelable,
            default_prevented: false,
            propagation_stopped: false,
            immediate_propagation_stopped: false,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            is_trusted: false,
        }
    }

    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    pub fn stop_immediate_propagation(&mut self) {
        self.propagation_stopped = true;
        self.immediate_propagation_stopped = true;
    }
}

/// Mouse event
#[derive(Clone)]
pub struct MouseEvent {
    pub base: Event,
    pub client_x: f64,
    pub client_y: f64,
    pub screen_x: f64,
    pub screen_y: f64,
    pub button: i32,
    pub buttons: u32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl MouseEvent {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            base: Event::new(event_type, bubbles, cancelable),
            client_x: 0.0,
            client_y: 0.0,
            screen_x: 0.0,
            screen_y: 0.0,
            button: -1,
            buttons: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        }
    }
}

/// Keyboard event
#[derive(Clone)]
pub struct KeyboardEvent {
    pub base: Event,
    pub key: String,
    pub code: String,
    pub key_code: u32,
    pub char_code: u32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub repeat: bool,
}

impl KeyboardEvent {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            base: Event::new(event_type, bubbles, cancelable),
            key: String::new(),
            code: String::new(),
            key_code: 0,
            char_code: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            repeat: false,
        }
    }
}

/// Input event
#[derive(Clone)]
pub struct InputEvent {
    pub base: Event,
    pub data: Option<String>,
    pub input_type: String,
    pub is_composing: bool,
}

impl InputEvent {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            base: Event::new(event_type, bubbles, cancelable),
            data: None,
            input_type: String::new(),
            is_composing: false,
        }
    }
}

/// Focus event
#[derive(Clone)]
pub struct FocusEvent {
    pub base: Event,
    pub related_target: Option<Rc<RefCell<dyn EventTarget>>>,
}

impl FocusEvent {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            base: Event::new(event_type, bubbles, cancelable),
            related_target: None,
        }
    }
}

/// Custom event
#[derive(Clone)]
pub struct CustomEvent {
    pub base: Event,
    pub detail: Option<String>,
}

impl CustomEvent {
    pub fn new(event_type: &str, bubbles: bool, cancelable: bool) -> Self {
        Self {
            base: Event::new(event_type, bubbles, cancelable),
            detail: None,
        }
    }
}

/// Event listener registry for efficient lookup
#[derive(Debug, Default)]
pub struct EventListenerRegistry {
    listeners: HashMap<String, Vec<EventListener>>,
    next_id: u64,
}

impl EventListenerRegistry {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_listener(&mut self, event_type: &str, options: EventListenerOptions, callback: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let listener = EventListener {
            callback,
            options,
            id,
        };

        self.listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(listener);

        id
    }

    pub fn remove_listener(&mut self, event_type: &str, listener_id: u64) -> bool {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            if let Some(pos) = listeners.iter().position(|l| l.id == listener_id) {
                listeners.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn get_listeners(&self, event_type: &str) -> Vec<EventListener> {
        self.listeners
            .get(event_type)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_capture_listeners(&self, event_type: &str) -> Vec<EventListener> {
        self.get_listeners(event_type)
            .into_iter()
            .filter(|l| l.options.capture)
            .collect()
    }

    pub fn get_bubble_listeners(&self, event_type: &str) -> Vec<EventListener> {
        self.get_listeners(event_type)
            .into_iter()
            .filter(|l| !l.options.capture)
            .collect()
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    pub fn total_listener_count(&self) -> usize {
        self.listeners.values().map(|v| v.len()).sum()
    }
}
