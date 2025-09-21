//! # Renderer Crate
//! 
//! This crate provides functionality to render DOM trees to various output formats.
//! Currently, it supports text-based rendering to the terminal, but it's designed
//! to be extensible for future graphics backends.
//! 
//! ## Design Principles
//! 
//! 1. **Backend Abstraction**: The renderer is designed with a pluggable backend
//!    system to support different output formats (terminal, graphics, etc.).
//! 
//! 2. **Tree Traversal**: Rendering is done by traversing the DOM tree and
//!    generating appropriate output for each node type.
//! 
//! 3. **Extensibility**: The design allows for easy addition of new rendering
//!    backends and output formats.
//! 
//! 4. **Performance**: The renderer is designed to be efficient and avoid
//!    unnecessary allocations where possible.

use dom::{Document, Node, NodeType};
use layout::{LayoutBox, ComputedStyles, DisplayType};
use std::rc::Rc;

/// A trait for different rendering backends
/// 
/// This trait allows the renderer to work with different output formats
/// without being tied to a specific implementation. Future backends could
/// include graphics rendering, PDF generation, or other formats.
pub trait RenderBackend {
    /// Start rendering a document
    fn start_document(&mut self);
    
    /// End rendering a document
    fn end_document(&mut self);
    
    /// Render an element node
    fn render_element(&mut self, node: &Rc<Node>, depth: usize);
    
    /// Render a text node
    fn render_text(&mut self, node: &Rc<Node>, depth: usize);
    
    /// Get the rendered output
    fn get_output(&self) -> String;
}

/// A text-based renderer that outputs to a string
/// 
/// This renderer creates a text representation of the DOM tree that's
/// suitable for display in a terminal or text file. It includes indentation
/// to show the tree structure.
pub struct TextRenderer {
    output: String,
    indent_string: String,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new() -> Self {
        TextRenderer {
            output: String::new(),
            indent_string: "  ".to_string(), // 2 spaces per level
        }
    }
    
    /// Create a new text renderer with custom indentation
    pub fn with_indent(indent: &str) -> Self {
        TextRenderer {
            output: String::new(),
            indent_string: indent.to_string(),
        }
    }
    
    /// Add indentation to the output
    fn add_indent(&mut self, depth: usize) {
        for _ in 0..depth {
            self.output.push_str(&self.indent_string);
        }
    }
    
    /// Add a line to the output with proper indentation
    fn add_line(&mut self, content: &str, depth: usize) {
        self.add_indent(depth);
        self.output.push_str(content);
        self.output.push('\n');
    }
    
    /// Recursively render a node and its children
    fn render_node_recursive(&mut self, node: &Rc<Node>, depth: usize) {
        match &node.node_type {
            NodeType::Element { .. } => {
                self.render_element(node, depth);
            }
            NodeType::Text(_) => {
                self.render_text(node, depth);
            }
            NodeType::Document => {
                // Document nodes are handled specially - render their children
                for child in node.children.borrow().iter() {
                    self.render_node_recursive(child, depth);
                }
            }
        }
    }
}

impl RenderBackend for TextRenderer {
    fn start_document(&mut self) {
        self.output.push_str("Document:\n");
    }
    
    fn end_document(&mut self) {
        // Nothing to do at the end
    }
    
    fn render_element(&mut self, node: &Rc<Node>, depth: usize) {
        if let NodeType::Element { tag_name, attributes } = &node.node_type {
            // Create attribute string
            let attr_string = if attributes.is_empty() {
                String::new()
            } else {
                let attrs: Vec<String> = attributes
                    .iter()
                    .map(|(k, v)| {
                        if v.is_empty() {
                            k.clone()
                        } else {
                            format!("{}=\"{}\"", k, v)
                        }
                    })
                    .collect();
                format!(" {}", attrs.join(" "))
            };
            
            self.add_line(&format!("<{}{}>", tag_name, attr_string), depth);
            
            // Render children
            for child in node.children.borrow().iter() {
                self.render_node_recursive(child, depth + 1);
            }
            
            self.add_line(&format!("</{}>", tag_name), depth);
        }
    }
    
    fn render_text(&mut self, node: &Rc<Node>, depth: usize) {
        if let NodeType::Text(content) = &node.node_type {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                self.add_line(&format!("\"{}\"", trimmed), depth);
            }
        }
    }
    
    fn get_output(&self) -> String {
        self.output.clone()
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple text content renderer that extracts only the text
/// 
/// This renderer ignores all HTML markup and returns only the text content
/// of the document. It's useful for extracting plain text from HTML documents.
pub struct TextContentRenderer {
    output: String,
}

impl TextContentRenderer {
    /// Create a new text content renderer
    pub fn new() -> Self {
        TextContentRenderer {
            output: String::new(),
        }
    }
}

impl RenderBackend for TextContentRenderer {
    fn start_document(&mut self) {
        // Nothing to do at the start
    }
    
    fn end_document(&mut self) {
        // Nothing to do at the end
    }
    
    fn render_element(&mut self, node: &Rc<Node>, _depth: usize) {
        // Elements don't contribute to text content directly, but we need to traverse their children
        for child in node.children.borrow().iter() {
            match &child.node_type {
                NodeType::Element { .. } => {
                    self.render_element(child, _depth + 1);
                }
                NodeType::Text(_) => {
                    self.render_text(child, _depth + 1);
                }
                NodeType::Document => {
                    // Document nodes - traverse their children
                    for grandchild in child.children.borrow().iter() {
                        match &grandchild.node_type {
                            NodeType::Element { .. } => {
                                self.render_element(grandchild, _depth + 1);
                            }
                            NodeType::Text(_) => {
                                self.render_text(grandchild, _depth + 1);
                            }
                            NodeType::Document => {
                                // Skip deeply nested documents
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn render_text(&mut self, node: &Rc<Node>, _depth: usize) {
        if let NodeType::Text(content) = &node.node_type {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                if !self.output.is_empty() {
                    self.output.push(' ');
                }
                self.output.push_str(trimmed);
            }
        }
    }
    
    fn get_output(&self) -> String {
        self.output.clone()
    }
}

impl Default for TextContentRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// The main renderer that coordinates rendering with different backends
/// 
/// This struct provides a high-level interface for rendering DOM trees
/// using different backends. It handles the tree traversal and delegates
/// the actual rendering to the backend.
pub struct Renderer<B: RenderBackend> {
    backend: B,
}

impl<B: RenderBackend> Renderer<B> {
    /// Create a new renderer with the given backend
    pub fn new(backend: B) -> Self {
        Renderer { backend }
    }
    
    /// Render a document using the configured backend
    /// 
    /// This method traverses the DOM tree and renders each node using
    /// the appropriate backend method. It handles the tree structure
    /// and delegates node-specific rendering to the backend.
    pub fn render_document(&mut self, document: &Document) -> String {
        self.backend.start_document();
        
        // Render all children of the document root
        for child in document.root.children.borrow().iter() {
            self.render_node(child, 0);
        }
        
        self.backend.end_document();
        self.backend.get_output()
    }
    
    /// Render a single node and its children
    /// 
    /// This method recursively renders a node and all its descendants.
    /// It determines the node type and calls the appropriate backend method.
    fn render_node(&mut self, node: &Rc<Node>, depth: usize) {
        match &node.node_type {
            NodeType::Document => {
                // Document nodes are handled specially - render their children
                for child in node.children.borrow().iter() {
                    self.render_node(child, depth);
                }
            }
            NodeType::Element { .. } => {
                self.backend.render_element(node, depth);
            }
            NodeType::Text(_) => {
                self.backend.render_text(node, depth);
            }
        }
    }
    
    /// Get the backend (useful for accessing backend-specific functionality)
    pub fn backend(&self) -> &B {
        &self.backend
    }
    
    /// Get a mutable reference to the backend
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }
}

/// Convenience function to render a document as text
/// 
/// This function creates a text renderer and renders the document,
/// returning the formatted text output.
pub fn render_as_text(document: &Document) -> String {
    let mut renderer = Renderer::new(TextRenderer::new());
    renderer.render_document(document)
}

/// Convenience function to extract text content from a document
/// 
/// This function creates a text content renderer and extracts only
/// the text content, ignoring all HTML markup.
pub fn extract_text_content(document: &Document) -> String {
    let mut renderer = Renderer::new(TextContentRenderer::new());
    renderer.render_document(document)
}

/// A layout-aware renderer that renders layout boxes with styling information
/// 
/// This renderer takes layout boxes (which include computed styles) and
/// renders them with visual formatting that reflects the CSS styling.
pub struct LayoutRenderer {
    output: String,
    indent_string: String,
}

impl LayoutRenderer {
    /// Create a new layout renderer
    pub fn new() -> Self {
        LayoutRenderer {
            output: String::new(),
            indent_string: "  ".to_string(),
        }
    }
    
    /// Create a new layout renderer with custom indentation
    pub fn with_indent(indent: &str) -> Self {
        LayoutRenderer {
            output: String::new(),
            indent_string: indent.to_string(),
        }
    }
    
    /// Add indentation to the output
    fn add_indent(&mut self, depth: usize) {
        for _ in 0..depth {
            self.output.push_str(&self.indent_string);
        }
    }
    
    /// Add a line to the output with proper indentation
    fn add_line(&mut self, content: &str, depth: usize) {
        self.add_indent(depth);
        self.output.push_str(content);
        self.output.push('\n');
    }
    
    /// Render a layout box and its children
    pub fn render_layout_box(&mut self, layout_box: &LayoutBox, depth: usize) {
        // Skip elements with display: none
        if layout_box.styles.display == DisplayType::None {
            return;
        }
        
        match &layout_box.node.node_type {
            NodeType::Element { tag_name, attributes } => {
                // Create attribute string
                let attr_string = if attributes.is_empty() {
                    String::new()
                } else {
                    let attrs: Vec<String> = attributes
                        .iter()
                        .map(|(k, v)| {
                            if v.is_empty() {
                                k.clone()
                            } else {
                                format!("{}=\"{}\"", k, v)
                            }
                        })
                        .collect();
                    format!(" {}", attrs.join(" "))
                };
                
                // Add style information
                let style_info = self.format_style_info(&layout_box.styles);
                let dimensions_info = self.format_dimensions_info(layout_box);
                
                self.add_line(&format!("<{}{}>", tag_name, attr_string), depth);
                if !style_info.is_empty() {
                    self.add_line(&format!("  Styles: {}", style_info), depth + 1);
                }
                if !dimensions_info.is_empty() {
                    self.add_line(&format!("  Layout: {}", dimensions_info), depth + 1);
                }
                
                // Render children
                for child in &layout_box.children {
                    self.render_layout_box(child, depth + 1);
                }
                
                self.add_line(&format!("</{}>", tag_name), depth);
            }
            NodeType::Text(content) => {
                let trimmed = content.trim();
                if !trimmed.is_empty() {
                    let style_info = self.format_text_style_info(&layout_box.styles);
                    if !style_info.is_empty() {
                        self.add_line(&format!("\"{}\" ({})", trimmed, style_info), depth);
                    } else {
                        self.add_line(&format!("\"{}\"", trimmed), depth);
                    }
                }
            }
            NodeType::Document => {
                // Document nodes - render their children
                for child in &layout_box.children {
                    self.render_layout_box(child, depth);
                }
            }
        }
    }
    
    /// Format style information for display
    fn format_style_info(&self, styles: &ComputedStyles) -> String {
        let mut info = Vec::new();
        
        match styles.display {
            DisplayType::Block => info.push("display:block".to_string()),
            DisplayType::Inline => info.push("display:inline".to_string()),
            DisplayType::InlineBlock => info.push("display:inline-block".to_string()),
            DisplayType::Flex => info.push("display:flex".to_string()),
            DisplayType::Grid => info.push("display:grid".to_string()),
            DisplayType::None => info.push("display:none".to_string()),
        }
        
        if let Some(width) = styles.width {
            info.push(format!("width:{}px", width));
        }
        if let Some(height) = styles.height {
            info.push(format!("height:{}px", height));
        }
        if let Some(ref color) = styles.color {
            info.push(format!("color:{}", color));
        }
        if let Some(ref bg_color) = styles.background_color {
            info.push(format!("background:{}", bg_color));
        }
        if let Some(font_size) = styles.font_size {
            info.push(format!("font-size:{}px", font_size));
        }
        
        info.join(", ")
    }
    
    /// Format text-specific style information
    fn format_text_style_info(&self, styles: &ComputedStyles) -> String {
        let mut info = Vec::new();
        
        if let Some(ref color) = styles.color {
            info.push(format!("color:{}", color));
        }
        if let Some(font_size) = styles.font_size {
            info.push(format!("font-size:{}px", font_size));
        }
        if let Some(ref font_family) = styles.font_family {
            info.push(format!("font-family:{}", font_family));
        }
        if let Some(ref font_weight) = styles.font_weight {
            info.push(format!("font-weight:{}", font_weight));
        }
        
        info.join(", ")
    }
    
    /// Format dimensions information for display
    fn format_dimensions_info(&self, layout_box: &LayoutBox) -> String {
        let mut info = Vec::new();
        
        info.push(format!("x:{}", layout_box.content.x));
        info.push(format!("y:{}", layout_box.content.y));
        info.push(format!("w:{}", layout_box.content.width));
        info.push(format!("h:{}", layout_box.content.height));
        
        if layout_box.styles.margin.top > 0.0 || layout_box.styles.margin.right > 0.0 ||
           layout_box.styles.margin.bottom > 0.0 || layout_box.styles.margin.left > 0.0 {
            info.push(format!("margin:{},{},{},{}", 
                layout_box.styles.margin.top,
                layout_box.styles.margin.right,
                layout_box.styles.margin.bottom,
                layout_box.styles.margin.left));
        }
        
        if layout_box.styles.padding.top > 0.0 || layout_box.styles.padding.right > 0.0 ||
           layout_box.styles.padding.bottom > 0.0 || layout_box.styles.padding.left > 0.0 {
            info.push(format!("padding:{},{},{},{}", 
                layout_box.styles.padding.top,
                layout_box.styles.padding.right,
                layout_box.styles.padding.bottom,
                layout_box.styles.padding.left));
        }
        
        info.join(", ")
    }
    
    /// Get the rendered output
    pub fn get_output(&self) -> String {
        self.output.clone()
    }
}

impl Default for LayoutRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to render a layout box with styling information
/// 
/// This function creates a layout renderer and renders the layout box
/// with visual formatting that shows the computed styles and dimensions.
pub fn render_layout_box(layout_box: &LayoutBox) -> String {
    let mut renderer = LayoutRenderer::new();
    renderer.render_layout_box(layout_box, 0);
    renderer.get_output()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom::Document;

    #[test]
    fn test_text_renderer() {
        let doc = Document::new();
        let html = doc.create_element("html");
        let body = doc.create_element("body");
        let h1 = doc.create_element("h1");
        let text = doc.create_text_node("Hello World");
        
        h1.append_child(&text);
        body.append_child(&h1);
        html.append_child(&body);
        doc.root.append_child(&html);
        
        let output = render_as_text(&doc);
        assert!(output.contains("<html>"));
        assert!(output.contains("<body>"));
        assert!(output.contains("<h1>"));
        assert!(output.contains("Hello World"));
        assert!(output.contains("</h1>"));
        assert!(output.contains("</body>"));
        assert!(output.contains("</html>"));
    }

    #[test]
    fn test_text_content_renderer() {
        let doc = Document::new();
        let html = doc.create_element("html");
        let body = doc.create_element("body");
        let h1 = doc.create_element("h1");
        let text = doc.create_text_node("Hello World");
        
        h1.append_child(&text);
        body.append_child(&h1);
        html.append_child(&body);
        doc.root.append_child(&html);
        
        let output = extract_text_content(&doc);
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_renderer_with_attributes() {
        let doc = Document::new();
        let div = doc.create_node(NodeType::Element {
            tag_name: "div".to_string(),
            attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("class".to_string(), "container".to_string());
                attrs.insert("id".to_string(), "main".to_string());
                attrs
            },
        });
        
        let text = doc.create_text_node("Content");
        div.append_child(&text);
        doc.root.append_child(&div);
        
        let output = render_as_text(&doc);
        assert!(output.contains("class=\"container\""));
        assert!(output.contains("id=\"main\""));
    }
}
