//! # DOM (Document Object Model) Crate
//! 
//! This crate provides the core data structures for representing HTML documents
//! as a tree of nodes. It forms the foundation for all other browser engine
//! components.
//! 
//! ## Design Principles
//! 
//! 1. **Tree Structure**: The DOM is represented as a tree where each node
//!    can have children, parent, and siblings.
//! 
//! 2. **Node Types**: We support different node types (Element, Text, Document)
//!    through an enum-based approach for type safety.
//! 
//! 3. **Ownership**: Each node owns its children, with parent references being
//!    weak to avoid circular references and enable safe tree manipulation.
//! 
//! 4. **Extensibility**: The design allows for easy addition of new node types
//!    and properties as the browser engine evolves.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

// Event system modules
pub mod event_types;
pub mod events;
pub mod delegation;
pub mod element;
pub mod dom_event_integration;

#[cfg(test)]
mod event_tests;

/// Represents different types of nodes in the DOM tree
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    /// The root document node
    Document,
    /// An HTML element with tag name and attributes
    Element {
        tag_name: String,
        attributes: HashMap<String, String>,
    },
    /// Text content within elements
    Text(String),
}

/// A node in the DOM tree
/// 
/// Each node contains:
/// - Its type (Document, Element, or Text)
/// - References to its parent and children
/// - A unique identifier for debugging and manipulation
#[derive(Debug)]
pub struct Node {
    /// The type and data of this node
    pub node_type: NodeType,
    /// Weak reference to parent to avoid circular references
    pub parent: RefCell<Weak<Node>>,
    /// Strong references to children
    pub children: RefCell<Vec<Rc<Node>>>,
    /// Unique identifier for this node
    pub id: u64,
}

impl Node {
    /// Create a new node with the given type and ID
    pub fn new(node_type: NodeType, id: u64) -> Rc<Node> {
        Rc::new(Node {
            node_type,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
            id,
        })
    }

    /// Add a child node to this node
    /// 
    /// This method:
    /// 1. Adds the child to this node's children list
    /// 2. Sets this node as the child's parent
    /// 3. Maintains the tree structure invariants
    pub fn append_child(self: &Rc<Self>, child: &Rc<Node>) {
        // Set this node as the parent of the child
        *child.parent.borrow_mut() = Rc::downgrade(self);
        
        // Add child to this node's children
        self.children.borrow_mut().push(Rc::clone(child));
    }

    /// Get the text content of this node and all its descendants
    /// 
    /// This is useful for extracting all text from a document or element
    /// without the HTML markup.
    pub fn text_content(&self) -> String {
        match &self.node_type {
            NodeType::Text(text) => text.clone(),
            _ => {
                // For non-text nodes, concatenate all descendant text
                self.children
                    .borrow()
                    .iter()
                    .map(|child| child.text_content())
                    .collect::<Vec<_>>()
                    .join("")
            }
        }
    }

    /// Find the first descendant element with the given tag name
    /// 
    /// This implements a depth-first search for elements by tag name.
    /// Returns None if no matching element is found.
    pub fn get_element_by_tag_name(&self, tag_name: &str) -> Option<Rc<Node>> {
        // Check if this node is an element with the matching tag name
        if let NodeType::Element { tag_name: node_tag, .. } = &self.node_type {
            if node_tag == tag_name {
                return Some(Rc::new(Node {
                    node_type: self.node_type.clone(),
                    parent: RefCell::new(Weak::new()),
                    children: RefCell::new(Vec::new()),
                    id: self.id,
                }));
            }
        }

        // Search in children
        for child in self.children.borrow().iter() {
            if let Some(found) = child.get_element_by_tag_name(tag_name) {
                return Some(found);
            }
        }

        None
    }

    /// Get all descendant elements with the given tag name
    /// 
    /// This returns a vector of all matching elements in document order.
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<Rc<Node>> {
        let mut results = Vec::new();

        // Check if this node matches
        if let NodeType::Element { tag_name: node_tag, .. } = &self.node_type {
            if node_tag == tag_name {
                results.push(Rc::new(Node {
                    node_type: self.node_type.clone(),
                    parent: RefCell::new(Weak::new()),
                    children: RefCell::new(Vec::new()),
                    id: self.id,
                }));
            }
        }

        // Search in children
        for child in self.children.borrow().iter() {
            results.extend(child.get_elements_by_tag_name(tag_name));
        }

        results
    }
}

/// A document represents the root of a DOM tree
/// 
/// This is a convenience wrapper around a Document node that provides
/// document-specific operations and maintains a node ID counter.
#[derive(Debug)]
pub struct Document {
    /// The root document node
    pub root: Rc<Node>,
    /// Counter for generating unique node IDs
    next_id: RefCell<u64>,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        let root = Node::new(NodeType::Document, 0);
        Document {
            root,
            next_id: RefCell::new(1),
        }
    }

    /// Create a new node with an automatically assigned ID
    pub fn create_node(&self, node_type: NodeType) -> Rc<Node> {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        Node::new(node_type, id)
    }

    /// Create a new element node with the given tag name
    pub fn create_element(&self, tag_name: &str) -> Rc<Node> {
        self.create_node(NodeType::Element {
            tag_name: tag_name.to_string(),
            attributes: HashMap::new(),
        })
    }

    /// Create a new text node with the given content
    pub fn create_text_node(&self, content: &str) -> Rc<Node> {
        self.create_node(NodeType::Text(content.to_string()))
    }

    /// Get the document element (usually the `<html>` element)
    /// 
    /// This is a convenience method to find the root HTML element
    /// of the document.
    pub fn document_element(&self) -> Option<Rc<Node>> {
        self.root.get_element_by_tag_name("html")
    }

    /// Get the body element of the document
    /// 
    /// This is a convenience method to find the `<body>` element
    /// which typically contains the visible content.
    pub fn body(&self) -> Option<Rc<Node>> {
        self.root.get_element_by_tag_name("body")
    }

    /// Get the next available node ID
    /// 
    /// This is used internally by the HTML parser to assign unique IDs to nodes
    pub fn get_next_id(&self) -> u64 {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let doc = Document::new();
        assert_eq!(doc.root.node_type, NodeType::Document);
    }

    #[test]
    fn test_create_element() {
        let doc = Document::new();
        let element = doc.create_element("div");
        
        if let NodeType::Element { tag_name, .. } = &element.node_type {
            assert_eq!(tag_name, "div");
        } else {
            panic!("Expected Element node type");
        }
    }

    #[test]
    fn test_create_text_node() {
        let doc = Document::new();
        let text_node = doc.create_text_node("Hello, World!");
        
        if let NodeType::Text(content) = &text_node.node_type {
            assert_eq!(content, "Hello, World!");
        } else {
            panic!("Expected Text node type");
        }
    }

    #[test]
    fn test_append_child() {
        let doc = Document::new();
        let parent = doc.create_element("div");
        let child = doc.create_text_node("Hello");
        
        parent.append_child(&child);
        
        assert_eq!(parent.children.borrow().len(), 1);
        assert!(Rc::ptr_eq(&parent.children.borrow()[0], &child));
    }

    #[test]
    fn test_text_content() {
        let doc = Document::new();
        let div = doc.create_element("div");
        let text1 = doc.create_text_node("Hello ");
        let text2 = doc.create_text_node("World!");
        
        div.append_child(&text1);
        div.append_child(&text2);
        
        assert_eq!(div.text_content(), "Hello World!");
    }
}
