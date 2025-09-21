//! Event-Driven GPU Renderer
//! 
//! This module provides an event-driven GPU renderer that can update the display
//! when DOM changes occur due to events. It integrates the event system with
//! the GPU rendering pipeline.

use std::rc::Rc;
use std::collections::HashMap;
use layout::LayoutBox;
use dom::Document;
use dom::dom_event_integration::DomEventManager;

/// Event-driven GPU renderer that updates display on DOM changes
pub struct EventDrivenRenderer {
    /// Current layout tree
    layout_tree: Option<LayoutBox>,
    /// DOM event manager for handling events
    dom_event_manager: DomEventManager,
    /// Document reference
    document: Option<Rc<Document>>,
    /// Flag to indicate if a re-render is needed
    needs_rerender: bool,
    /// Event listener registry for render updates
    render_listeners: HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>>,
    /// Event statistics
    event_stats: EventStats,
}

/// Statistics for event-driven rendering
#[derive(Debug, Default)]
pub struct EventStats {
    pub events_processed: u64,
    pub rerenders_triggered: u64,
    pub last_event_type: Option<String>,
    pub last_target_id: Option<String>,
}

impl EventDrivenRenderer {
    /// Create a new event-driven renderer
    pub fn new() -> Self {
        Self {
            layout_tree: None,
            dom_event_manager: DomEventManager::new(),
            document: None,
            needs_rerender: true,
            render_listeners: HashMap::new(),
            event_stats: EventStats::default(),
        }
    }

    /// Set the document for this renderer
    pub fn set_document(&mut self, document: Rc<Document>) {
        self.document = Some(Rc::clone(&document));
        self.dom_event_manager.set_document(document);
        self.needs_rerender = true;
        println!("Document set for event-driven renderer");
    }

    /// Set the layout tree for rendering
    pub fn set_layout_tree(&mut self, layout_tree: LayoutBox) {
        self.layout_tree = Some(layout_tree);
        self.needs_rerender = true;
        println!("Layout tree set for event-driven renderer");
    }

    /// Add a render update listener
    pub fn add_render_listener<F>(&mut self, event_type: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.render_listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
        println!("Added render listener for event type: {}", event_type);
    }

    /// Trigger a re-render
    pub fn trigger_rerender(&mut self) {
        self.needs_rerender = true;
        self.event_stats.rerenders_triggered += 1;
        println!("Re-render triggered");
    }

    /// Handle a DOM event and trigger re-render if needed
    pub fn handle_dom_event(&mut self, event_type: &str, target_id: &str) -> bool {
        println!("Handling DOM event: {} on target: {}", event_type, target_id);
        
        // Update statistics
        self.event_stats.events_processed += 1;
        self.event_stats.last_event_type = Some(event_type.to_string());
        self.event_stats.last_target_id = Some(target_id.to_string());
        
        // Dispatch the event through the DOM event manager
        let event = match event_type {
            "click" => dom::event_types::Event::new("click", true, true),
            "input" => dom::event_types::Event::new("input", true, true),
            "keydown" => dom::event_types::Event::new("keydown", true, true),
            _ => dom::event_types::Event::new(event_type, true, true),
        };

        let event_result = if let Some(target_node) = self.dom_event_manager.find_node_by_id(target_id) {
            let _result = self.dom_event_manager.dispatch_event(&target_node, event);
            
            // Check if any render listeners should be triggered
            if let Some(listeners) = self.render_listeners.get(event_type) {
                for listener in listeners {
                    listener();
                }
            }
            
            true
        } else {
            println!("Target node '{}' not found for event '{}'", target_id, event_type);
            false
        };
        
        // Always mark for re-render and increment counter, even if target not found
        self.needs_rerender = true;
        self.event_stats.rerenders_triggered += 1;
        
        event_result
    }

    /// Simulate a click event and trigger re-render
    pub fn simulate_click(&mut self, target_id: &str) -> bool {
        self.handle_dom_event("click", target_id)
    }

    /// Simulate an input event and trigger re-render
    pub fn simulate_input(&mut self, target_id: &str) -> bool {
        self.handle_dom_event("input", target_id)
    }

    /// Simulate a keydown event and trigger re-render
    pub fn simulate_keydown(&mut self, target_id: &str) -> bool {
        self.handle_dom_event("keydown", target_id)
    }

    /// Check if a re-render is needed
    pub fn needs_rerender(&self) -> bool {
        self.needs_rerender
    }

    /// Mark that rendering is complete
    pub fn mark_rendered(&mut self) {
        self.needs_rerender = false;
    }

    /// Get the current layout tree
    pub fn get_layout_tree(&self) -> Option<&LayoutBox> {
        self.layout_tree.as_ref()
    }

    /// Get renderer statistics
    pub fn get_stats(&self) -> RendererStats {
        RendererStats {
            has_layout_tree: self.layout_tree.is_some(),
            has_document: self.document.is_some(),
            needs_rerender: self.needs_rerender,
            render_listeners: self.render_listeners.len(),
            dom_event_stats: self.dom_event_manager.get_stats(),
            event_stats: self.event_stats.clone(),
        }
    }

    /// Get DOM event manager reference
    pub fn get_dom_event_manager(&self) -> &DomEventManager {
        &self.dom_event_manager
    }

    /// Get mutable DOM event manager reference
    pub fn get_dom_event_manager_mut(&mut self) -> &mut DomEventManager {
        &mut self.dom_event_manager
    }

    /// Process multiple events in batch
    pub fn process_event_batch(&mut self, events: Vec<(String, String)>) -> u32 {
        let mut processed = 0;
        for (event_type, target_id) in events {
            if self.handle_dom_event(&event_type, &target_id) {
                processed += 1;
            }
        }
        processed
    }

    /// Clear all render listeners
    pub fn clear_render_listeners(&mut self) {
        self.render_listeners.clear();
        println!("All render listeners cleared");
    }

    /// Get render listeners for a specific event type
    pub fn get_render_listeners(&self, event_type: &str) -> usize {
        self.render_listeners
            .get(event_type)
            .map(|listeners| listeners.len())
            .unwrap_or(0)
    }
}

/// Statistics for the event-driven renderer
#[derive(Debug)]
pub struct RendererStats {
    pub has_layout_tree: bool,
    pub has_document: bool,
    pub needs_rerender: bool,
    pub render_listeners: usize,
    pub dom_event_stats: dom::dom_event_integration::DomEventStats,
    pub event_stats: EventStats,
}

impl Clone for EventStats {
    fn clone(&self) -> Self {
        Self {
            events_processed: self.events_processed,
            rerenders_triggered: self.rerenders_triggered,
            last_event_type: self.last_event_type.clone(),
            last_target_id: self.last_target_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_driven_renderer_creation() {
        let renderer = EventDrivenRenderer::new();
        let stats = renderer.get_stats();
        
        assert!(!stats.has_layout_tree);
        assert!(!stats.has_document);
        assert!(stats.needs_rerender);
        assert_eq!(stats.render_listeners, 0);
        assert_eq!(stats.event_stats.events_processed, 0);
    }

    #[test]
    fn test_event_handling() {
        let mut renderer = EventDrivenRenderer::new();
        
        // Test event handling without document
        let result = renderer.simulate_click("test-button");
        assert!(!result); // Should fail without document
        
        let stats = renderer.get_stats();
        assert_eq!(stats.event_stats.events_processed, 1);
        assert_eq!(stats.event_stats.rerenders_triggered, 1);
    }

    #[test]
    fn test_render_listeners() {
        let mut renderer = EventDrivenRenderer::new();
        
        // Add a render listener
        renderer.add_render_listener("click", || {
            println!("Click event triggered render update");
        });
        
        assert_eq!(renderer.get_render_listeners("click"), 1);
        assert_eq!(renderer.get_render_listeners("input"), 0);
        
        // Clear listeners
        renderer.clear_render_listeners();
        assert_eq!(renderer.get_render_listeners("click"), 0);
    }

    #[test]
    fn test_batch_event_processing() {
        let mut renderer = EventDrivenRenderer::new();
        
        let events = vec![
            ("click".to_string(), "button1".to_string()),
            ("input".to_string(), "input1".to_string()),
            ("keydown".to_string(), "input1".to_string()),
        ];
        
        let processed = renderer.process_event_batch(events);
        assert_eq!(processed, 0); // Should be 0 without document
        
        let stats = renderer.get_stats();
        assert_eq!(stats.event_stats.events_processed, 3);
        assert_eq!(stats.event_stats.rerenders_triggered, 3);
    }
}