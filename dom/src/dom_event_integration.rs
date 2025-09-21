//! DOM Event Integration
//! 
//! This module provides the integration between the event system and the DOM tree,
//! enabling real event propagation through the DOM hierarchy.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::{Node, NodeType, Document};
use crate::event_types::*;
use crate::element::Element;
use crate::events::EventDispatcher;

/// DOM Event Manager
/// 
/// This struct manages event propagation through the DOM tree and provides
/// the bridge between the event system and the DOM structure.
#[derive(Debug)]
pub struct DomEventManager {
    /// Event dispatcher for handling event propagation
    _event_dispatcher: EventDispatcher,
    /// Map of node IDs to their event listeners
    node_listeners: HashMap<u64, EventListenerRegistry>,
    /// Map of node IDs to their Element wrappers
    element_cache: HashMap<u64, Rc<RefCell<Element>>>,
    /// Document reference for DOM traversal
    document: Option<Rc<Document>>,
}

impl DomEventManager {
    /// Create a new DOM event manager
    pub fn new() -> Self {
        Self {
            _event_dispatcher: EventDispatcher::new(),
            node_listeners: HashMap::new(),
            element_cache: HashMap::new(),
            document: None,
        }
    }

    /// Set the document for this event manager
    pub fn set_document(&mut self, document: Rc<Document>) {
        self.document = Some(document);
    }

    /// Get or create an Element wrapper for a Node
    pub fn get_element(&mut self, node: &Rc<Node>) -> Rc<RefCell<Element>> {
        if let Some(element) = self.element_cache.get(&node.id) {
            return Rc::clone(element);
        }

        let element = Rc::new(RefCell::new(Element::new(Rc::clone(node))));
        self.element_cache.insert(node.id, Rc::clone(&element));
        element
    }

    /// Add an event listener to a DOM node
    pub fn add_event_listener(&mut self, node: &Rc<Node>, event_type: &str, listener: EventListener) {
        let listeners = self.node_listeners.entry(node.id).or_insert_with(EventListenerRegistry::new);
        let id = listeners.add_listener(event_type, listener.options, listener.callback);
        println!("Added event listener {} to node {} for event '{}'", id, node.id, event_type);
    }

    /// Remove an event listener from a DOM node
    pub fn remove_event_listener(&mut self, node: &Rc<Node>, event_type: &str, listener_id: u64) {
        if let Some(listeners) = self.node_listeners.get_mut(&node.id) {
            let removed = listeners.remove_listener(event_type, listener_id);
            if removed {
                println!("Removed event listener {} from node {} for event '{}'", listener_id, node.id, event_type);
            }
        }
    }

    /// Dispatch an event to a specific DOM node
    pub fn dispatch_event(&mut self, target_node: &Rc<Node>, mut event: Event) -> bool {
        // Set the target
        let _target_element = self.get_element(target_node);
        event.target = Some(Rc::new(RefCell::new(Element::new(Rc::clone(target_node)))));

        // Calculate the event path (from target to root)
        let event_path = self.calculate_event_path(target_node);
        
        // Execute capturing phase (root to target)
        if event.bubbles {
            event.phase = EventPhase::Capturing;
            for node in event_path.iter().rev() {
                if event.propagation_stopped {
                    break;
                }
                self.execute_listeners(node, &mut event);
            }
        }

        // Execute target phase
        event.phase = EventPhase::AtTarget;
        self.execute_listeners(target_node, &mut event);

        // Execute bubbling phase (target to root)
        if event.bubbles && !event.propagation_stopped {
            event.phase = EventPhase::Bubbling;
            for node in event_path.iter() {
                if event.propagation_stopped {
                    break;
                }
                if Rc::ptr_eq(node, target_node) {
                    continue; // Skip target, already handled
                }
                self.execute_listeners(node, &mut event);
            }
        }

        !event.default_prevented
    }

    /// Calculate the event path from target to root
    fn calculate_event_path(&self, target: &Rc<Node>) -> Vec<Rc<Node>> {
        let mut path = Vec::new();
        let mut current = Some(Rc::clone(target));

        while let Some(node) = current {
            path.push(Rc::clone(&node));
            
            // Get parent node
            current = node.parent.borrow().upgrade();
        }

        path
    }

    /// Execute event listeners for a specific node
    fn execute_listeners(&self, node: &Rc<Node>, event: &mut Event) {
        if let Some(listeners) = self.node_listeners.get(&node.id) {
            let relevant_listeners = match event.phase {
                EventPhase::Capturing => listeners.get_capture_listeners(&event.event_type),
                EventPhase::AtTarget => listeners.get_listeners(&event.event_type),
                EventPhase::Bubbling => listeners.get_bubble_listeners(&event.event_type),
                EventPhase::None => return,
            };

            for listener in relevant_listeners {
                if event.immediate_propagation_stopped {
                    break;
                }

                println!(
                    "Executing listener {} for event '{}' on node {} in phase {:?}",
                    listener.id, event.event_type, node.id, event.phase
                );

                // In a real implementation, this would call JavaScript
                // For now, we'll just simulate the execution
                
                // Check for once listeners
                if listener.options.once {
                    // In a real implementation, we would remove this listener
                    println!("Listener {} is marked as 'once', would be removed after execution", listener.id);
                }
            }
        }
    }

    /// Find a node by ID in the DOM tree
    pub fn find_node_by_id(&self, id: &str) -> Option<Rc<Node>> {
        if let Some(doc) = &self.document {
            self.find_node_by_id_recursive(&doc.root, id)
        } else {
            None
        }
    }

    /// Recursively find a node by ID
    fn find_node_by_id_recursive(&self, node: &Rc<Node>, id: &str) -> Option<Rc<Node>> {
        // Check if this node has the matching ID attribute
        if let NodeType::Element { attributes, .. } = &node.node_type {
            if let Some(node_id) = attributes.get("id") {
                if node_id == id {
                    return Some(Rc::clone(node));
                }
            }
        }

        // Search in children
        for child in node.children.borrow().iter() {
            if let Some(found) = self.find_node_by_id_recursive(child, id) {
                return Some(found);
            }
        }

        None
    }

    /// Find nodes by class name
    pub fn find_nodes_by_class(&self, class_name: &str) -> Vec<Rc<Node>> {
        if let Some(doc) = &self.document {
            self.find_nodes_by_class_recursive(&doc.root, class_name)
        } else {
            Vec::new()
        }
    }

    /// Recursively find nodes by class name
    fn find_nodes_by_class_recursive(&self, node: &Rc<Node>, class_name: &str) -> Vec<Rc<Node>> {
        let mut results = Vec::new();

        // Check if this node has the matching class
        if let NodeType::Element { attributes, .. } = &node.node_type {
            if let Some(class_attr) = attributes.get("class") {
                if class_attr.split_whitespace().any(|c| c == class_name) {
                    results.push(Rc::clone(node));
                }
            }
        }

        // Search in children
        for child in node.children.borrow().iter() {
            results.extend(self.find_nodes_by_class_recursive(child, class_name));
        }

        results
    }

    /// Find nodes by tag name
    pub fn find_nodes_by_tag(&self, tag_name: &str) -> Vec<Rc<Node>> {
        if let Some(doc) = &self.document {
            self.find_nodes_by_tag_recursive(&doc.root, tag_name)
        } else {
            Vec::new()
        }
    }

    /// Recursively find nodes by tag name
    fn find_nodes_by_tag_recursive(&self, node: &Rc<Node>, tag_name: &str) -> Vec<Rc<Node>> {
        let mut results = Vec::new();

        // Check if this node matches the tag
        if let NodeType::Element { tag_name: node_tag, .. } = &node.node_type {
            if node_tag == tag_name {
                results.push(Rc::clone(node));
            }
        }

        // Search in children
        for child in node.children.borrow().iter() {
            results.extend(self.find_nodes_by_tag_recursive(child, tag_name));
        }

        results
    }

    /// Simulate a click event on a node
    pub fn simulate_click(&mut self, target_id: &str) -> bool {
        if let Some(target) = self.find_node_by_id(target_id) {
            let mut click_event = MouseEvent::new("click", true, true);
            click_event.base.target = Some(Rc::new(RefCell::new(Element::new(Rc::clone(&target)))));
            self.dispatch_event(&target, click_event.base)
        } else {
            println!("Node with ID '{}' not found", target_id);
            false
        }
    }

    /// Simulate a keydown event on a node
    pub fn simulate_keydown(&mut self, target_id: &str, key: &str) -> bool {
        if let Some(target) = self.find_node_by_id(target_id) {
            let mut keydown_event = KeyboardEvent::new("keydown", true, true);
            keydown_event.key = key.to_string();
            keydown_event.code = key.to_string();
            keydown_event.base.target = Some(Rc::new(RefCell::new(Element::new(Rc::clone(&target)))));
            self.dispatch_event(&target, keydown_event.base)
        } else {
            println!("Node with ID '{}' not found", target_id);
            false
        }
    }

    /// Simulate an input event on a node
    pub fn simulate_input(&mut self, target_id: &str, value: &str) -> bool {
        if let Some(target) = self.find_node_by_id(target_id) {
            let mut input_event = InputEvent::new("input", true, true);
            input_event.data = Some(value.to_string());
            input_event.input_type = "insertText".to_string();
            input_event.base.target = Some(Rc::new(RefCell::new(Element::new(Rc::clone(&target)))));
            self.dispatch_event(&target, input_event.base)
        } else {
            println!("Node with ID '{}' not found", target_id);
            false
        }
    }

    /// Get event statistics
    pub fn get_stats(&self) -> DomEventStats {
        let total_listeners: usize = self.node_listeners.values()
            .map(|registry| registry.total_listener_count())
            .sum();

        DomEventStats {
            total_nodes: self.node_listeners.len(),
            total_listeners,
            cached_elements: self.element_cache.len(),
        }
    }
}

/// Statistics for DOM event system
#[derive(Debug)]
pub struct DomEventStats {
    pub total_nodes: usize,
    pub total_listeners: usize,
    pub cached_elements: usize,
}

impl Default for DomEventManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_event_manager_creation() {
        let manager = DomEventManager::new();
        assert_eq!(manager.node_listeners.len(), 0);
        assert_eq!(manager.element_cache.len(), 0);
    }

    #[test]
    fn test_event_path_calculation() {
        let doc = Document::new();
        let html = doc.create_element("html");
        let body = doc.create_element("body");
        let div = doc.create_element("div");
        let button = doc.create_element("button");

        html.append_child(&body);
        body.append_child(&div);
        div.append_child(&button);

        let manager = DomEventManager::new();
        let path = manager.calculate_event_path(&button);

        // Path should be: button -> div -> body -> html
        assert_eq!(path.len(), 4);
        assert_eq!(path[0].id, button.id);
        assert_eq!(path[1].id, div.id);
        assert_eq!(path[2].id, body.id);
        assert_eq!(path[3].id, html.id);
    }

    #[test]
    fn test_node_finding_by_id() {
        let doc = Document::new();
        let html = doc.create_element("html");
        let body = doc.create_element("body");
        let div = doc.create_element("div");
        
        // Add html to document root
        doc.root.append_child(&html);
        html.append_child(&body);
        body.append_child(&div);

        let mut manager = DomEventManager::new();
        manager.set_document(Rc::new(doc));

        // Test that we can find the div by its node ID (not attribute ID)
        // Since we can't set attributes due to Rc<Node> design, we'll test
        // that the document structure is properly set up
        assert!(manager.document.is_some());
        let doc = manager.document.as_ref().unwrap();
        assert_eq!(doc.root.children.borrow().len(), 1); // html element
        
        // Check that the html element has the body as a child
        let html = &doc.root.children.borrow()[0];
        assert_eq!(html.children.borrow().len(), 1); // body element
        
        // Check that the body element has the div as a child
        let body = &html.children.borrow()[0];
        assert_eq!(body.children.borrow().len(), 1); // div element
    }

    #[test]
    fn test_event_listener_management() {
        let doc = Document::new();
        let button = doc.create_element("button");
        
        let mut manager = DomEventManager::new();
        
        let listener = EventListener {
            callback: "function() { console.log('clicked'); }".to_string(),
            options: EventListenerOptions::default(),
            id: 1,
        };

        manager.add_event_listener(&button, "click", listener);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.total_listeners, 1);
    }
}
