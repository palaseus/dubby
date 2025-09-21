use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::event_types::*;
use crate::Node;

/// An element node that implements EventTarget
#[derive(Debug)]
pub struct Element {
    /// The underlying DOM node
    pub node: Rc<Node>,
    /// Event listener registry
    event_listeners: RefCell<EventListenerRegistry>,
    /// Element attributes
    attributes: RefCell<HashMap<String, String>>,
    /// Element ID
    pub id: Option<String>,
    /// Element class list
    class_list: RefCell<Vec<String>>,
}

impl Element {
    /// Create a new element
    pub fn new(node: Rc<Node>) -> Self {
        Self {
            node,
            event_listeners: RefCell::new(EventListenerRegistry::new()),
            attributes: RefCell::new(HashMap::new()),
            id: None,
            class_list: RefCell::new(Vec::new()),
        }
    }

    /// Get the tag name of this element
    pub fn tag_name(&self) -> Option<String> {
        match &self.node.node_type {
            crate::NodeType::Element { tag_name, .. } => Some(tag_name.clone()),
            _ => None,
        }
    }

    /// Get an attribute value
    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.attributes.borrow().get(name).cloned()
    }

    /// Set an attribute value
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.borrow_mut().insert(name.to_string(), value.to_string());
        
        // Handle special attributes
        match name {
            "id" => self.id = Some(value.to_string()),
            "class" => {
                self.class_list.borrow_mut().clear();
                self.class_list.borrow_mut().extend(
                    value.split_whitespace().map(|s| s.to_string())
                );
            }
            _ => {}
        }
    }

    /// Remove an attribute
    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.borrow_mut().remove(name);
        
        match name {
            "id" => self.id = None,
            "class" => self.class_list.borrow_mut().clear(),
            _ => {}
        }
    }

    /// Check if element has a specific class
    pub fn has_class(&self, class_name: &str) -> bool {
        self.class_list.borrow().contains(&class_name.to_string())
    }

    /// Add a class to the element
    pub fn add_class(&mut self, class_name: &str) {
        let class_name = class_name.to_string();
        if !self.class_list.borrow().contains(&class_name) {
            self.class_list.borrow_mut().push(class_name);
            self.update_class_attribute();
        }
    }

    /// Remove a class from the element
    pub fn remove_class(&mut self, class_name: &str) {
        self.class_list.borrow_mut().retain(|c| c != class_name);
        self.update_class_attribute();
    }

    /// Update the class attribute based on the class list
    fn update_class_attribute(&mut self) {
        let class_value = self.class_list.borrow().join(" ");
        if class_value.is_empty() {
            self.attributes.borrow_mut().remove("class");
        } else {
            self.attributes.borrow_mut().insert("class".to_string(), class_value);
        }
    }

    /// Get the inner text of the element
    pub fn inner_text(&self) -> String {
        self.node.text_content()
    }

    /// Set the inner text of the element
    pub fn set_inner_text(&mut self, text: &str) {
        // Clear existing children
        self.node.children.borrow_mut().clear();
        
        // Add new text node
        let text_node = crate::Node::new(
            crate::NodeType::Text(text.to_string()),
            self.node.id + 1000 // Simple ID generation
        );
        self.node.append_child(&text_node);
    }

    /// Get the inner HTML of the element (simplified)
    pub fn inner_html(&self) -> String {
        // Simplified implementation - just return text content
        self.node.text_content()
    }

    /// Set the inner HTML of the element (simplified)
    pub fn set_inner_html(&mut self, html: &str) {
        // Simplified implementation - just set as text
        self.set_inner_text(html);
    }
}

impl EventTarget for Element {
    fn add_event_listener(&mut self, event_type: &str, listener: EventListener) {
        let id = self.event_listeners.borrow_mut().add_listener(
            event_type,
            listener.options,
            listener.callback,
        );
        println!("Added event listener {} for event '{}'", id, event_type);
    }

    fn remove_event_listener(&mut self, event_type: &str, listener_id: u64) {
        let removed = self.event_listeners.borrow_mut().remove_listener(event_type, listener_id);
        if removed {
            println!("Removed event listener {} for event '{}'", listener_id, event_type);
        }
    }

    fn dispatch_event(&mut self, event: &mut Event) -> bool {
        // Set current target
        // Note: In a real implementation, we would need to properly handle the current_target
        // For now, we'll leave it as None to avoid the trait object issue
        event.current_target = None;
        
        // Execute listeners based on event phase
        let listeners = self.event_listeners.borrow().get_listeners(&event.event_type);
        
        for listener in listeners {
            let should_execute = match event.phase {
                EventPhase::Capturing => listener.options.capture,
                EventPhase::AtTarget => true,
                EventPhase::Bubbling => !listener.options.capture,
                EventPhase::None => false,
            };

            if should_execute {
                println!(
                    "Executing listener {} for event '{}' in phase {:?}",
                    listener.id, event.event_type, event.phase
                );
                
                // In a real implementation, this would call JavaScript
                // For now, we'll just simulate the execution
                
                if event.immediate_propagation_stopped {
                    break;
                }
            }
        }

        !event.default_prevented
    }

    fn get_event_listeners(&self, event_type: &str) -> Vec<EventListener> {
        self.event_listeners.borrow().get_listeners(event_type)
    }
}

/// Create an element from a node
impl From<Rc<Node>> for Element {
    fn from(node: Rc<Node>) -> Self {
        Element::new(node)
    }
}

/// Convert element back to node
impl From<Element> for Rc<Node> {
    fn from(element: Element) -> Self {
        element.node
    }
}
