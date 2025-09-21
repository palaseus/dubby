//! Async Event Loop Integration
//! 
//! This module provides an asynchronous event loop that integrates the event system
//! with GPU rendering, DOM updates, and JavaScript execution.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};
use crate::event_driven_renderer::EventDrivenRenderer;
use crate::input_handler::InputHandler;
use dom::dom_event_integration::DomEventManager;

/// Async event loop that coordinates all browser engine components
pub struct AsyncEventLoop {
    /// Event queue for processing events
    event_queue: Arc<Mutex<VecDeque<EventLoopEvent>>>,
    /// Event-driven renderer
    renderer: Arc<Mutex<EventDrivenRenderer>>,
    /// Input handler
    input_handler: Arc<Mutex<InputHandler>>,
    /// JavaScript engine (placeholder for now)
    js_engine: Arc<Mutex<()>>,
    /// DOM event manager
    dom_event_manager: Arc<Mutex<DomEventManager>>,
    /// Running state
    is_running: Arc<AtomicBool>,
    /// Event processing statistics
    stats: Arc<Mutex<EventLoopStats>>,
    /// Event channels for async communication
    event_sender: mpsc::UnboundedSender<EventLoopEvent>,
    event_receiver: mpsc::UnboundedReceiver<EventLoopEvent>,
}

/// Events that can be processed by the event loop
#[derive(Debug, Clone)]
pub enum EventLoopEvent {
    /// User input events
    InputEvent {
        event_type: String,
        target_id: String,
        data: Option<String>,
    },
    /// DOM events
    DomEvent {
        event_type: String,
        target_id: String,
        bubbles: bool,
    },
    /// JavaScript execution events
    JsEvent {
        script: String,
        context: String,
    },
    /// Rendering events
    RenderEvent {
        event_type: String,
        data: Option<String>,
    },
    /// Timer events
    TimerEvent {
        timer_id: u32,
        callback: String,
    },
    /// Network events
    NetworkEvent {
        url: String,
        method: String,
        data: Option<String>,
    },
    /// Custom events
    CustomEvent {
        event_name: String,
        data: Option<String>,
    },
}

/// Statistics for the event loop
#[derive(Debug, Default)]
pub struct EventLoopStats {
    pub events_processed: u64,
    pub events_per_second: f64,
    pub average_processing_time: Duration,
    pub last_event_time: Option<Instant>,
    pub queue_size: usize,
    pub render_updates: u64,
    pub dom_updates: u64,
    pub js_executions: u64,
    pub input_events: u64,
}

impl AsyncEventLoop {
    /// Create a new async event loop
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        Self {
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            renderer: Arc::new(Mutex::new(EventDrivenRenderer::new())),
            input_handler: Arc::new(Mutex::new(InputHandler::new())),
            js_engine: Arc::new(Mutex::new(())),
            dom_event_manager: Arc::new(Mutex::new(DomEventManager::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            stats: Arc::new(Mutex::new(EventLoopStats::default())),
            event_sender,
            event_receiver,
        }
    }

    /// Start the async event loop
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting Async Event Loop");
        
        self.is_running.store(true, Ordering::SeqCst);
        
        // Spawn event processing task
        let event_queue = Arc::clone(&self.event_queue);
        let renderer = Arc::clone(&self.renderer);
        let input_handler = Arc::clone(&self.input_handler);
        let js_engine = Arc::clone(&self.js_engine);
        let dom_event_manager = Arc::clone(&self.dom_event_manager);
        let stats = Arc::clone(&self.stats);
        let is_running = Arc::clone(&self.is_running);
        
        let mut receiver = std::mem::replace(&mut self.event_receiver, mpsc::unbounded_channel().1);
        
        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                // Process events from the channel
                if let Some(event) = receiver.recv().await {
                    Self::process_event(
                        event,
                        &event_queue,
                        &renderer,
                        &input_handler,
                        &js_engine,
                        &dom_event_manager,
                        &stats,
                    ).await;
                }
                
                // Small delay to prevent busy waiting
                sleep(Duration::from_millis(1)).await;
            }
        });
        
        // Main event loop
        let start_time = Instant::now();
        let mut last_stats_time = start_time;
        let mut event_count = 0;
        
        while self.is_running.load(Ordering::SeqCst) {
            // Process any queued events
            self.process_queued_events().await;
            
            // Update statistics periodically
            let now = Instant::now();
            if now.duration_since(last_stats_time) >= Duration::from_secs(1) {
                self.update_statistics(event_count, now.duration_since(start_time)).await;
                last_stats_time = now;
                event_count = 0;
            }
            
            // Small delay to prevent busy waiting
            sleep(Duration::from_millis(10)).await;
        }
        
        println!("🛑 Async Event Loop Stopped");
        Ok(())
    }

    /// Stop the async event loop
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Send an event to the event loop
    pub fn send_event(&self, event: EventLoopEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.event_sender.send(event)?;
        Ok(())
    }

    /// Process queued events
    async fn process_queued_events(&mut self) {
        let mut queue = self.event_queue.lock().unwrap();
        while let Some(event) = queue.pop_front() {
            Self::process_event(
                event,
                &self.event_queue,
                &self.renderer,
                &self.input_handler,
                &self.js_engine,
                &self.dom_event_manager,
                &self.stats,
            ).await;
        }
    }

    /// Process a single event
    async fn process_event(
        event: EventLoopEvent,
        event_queue: &Arc<Mutex<VecDeque<EventLoopEvent>>>,
        renderer: &Arc<Mutex<EventDrivenRenderer>>,
        input_handler: &Arc<Mutex<InputHandler>>,
        js_engine: &Arc<Mutex<()>>,
        dom_event_manager: &Arc<Mutex<DomEventManager>>,
        stats: &Arc<Mutex<EventLoopStats>>,
    ) {
        let start_time = Instant::now();
        
        match event {
            EventLoopEvent::InputEvent { event_type, target_id, data } => {
                // Process input event (placeholder)
                println!("🖱️ Input event: {} on target: {} with data: {:?}", event_type, target_id, data);
                
                // Update statistics
                if let Ok(mut stats) = stats.lock() {
                    stats.input_events += 1;
                }
            }
            EventLoopEvent::DomEvent { event_type, target_id, bubbles } => {
                // Process DOM event
                if let Ok(mut manager) = dom_event_manager.lock() {
                    // Find target and dispatch event
                    if let Some(target_node) = manager.find_node_by_id(&target_id) {
                        let dom_event = dom::event_types::Event::new(&event_type, bubbles, true);
                        let _result = manager.dispatch_event(&target_node, dom_event);
                    }
                }
                
                // Update statistics
                if let Ok(mut stats) = stats.lock() {
                    stats.dom_updates += 1;
                }
            }
            EventLoopEvent::JsEvent { script, context } => {
                // Execute JavaScript (placeholder)
                println!("📜 JavaScript execution: {} in context: {}", script, context);
                
                // Update statistics
                if let Ok(mut stats) = stats.lock() {
                    stats.js_executions += 1;
                }
            }
            EventLoopEvent::RenderEvent { event_type, data } => {
                // Trigger render update
                if let Ok(mut renderer) = renderer.lock() {
                    renderer.trigger_rerender();
                }
                
                // Update statistics
                if let Ok(mut stats) = stats.lock() {
                    stats.render_updates += 1;
                }
            }
            EventLoopEvent::TimerEvent { timer_id, callback } => {
                // Execute timer callback (placeholder)
                println!("⏰ Timer {} executed: {}", timer_id, callback);
            }
            EventLoopEvent::NetworkEvent { url, method, data } => {
                // Handle network request (stubbed)
                println!("🌐 Network request: {} {} with data: {:?}", method, url, data);
            }
            EventLoopEvent::CustomEvent { event_name, data } => {
                // Handle custom event
                println!("🎯 Custom event: {} with data: {:?}", event_name, data);
            }
        }
        
        // Update processing statistics
        let processing_time = start_time.elapsed();
        if let Ok(mut stats) = stats.lock() {
            stats.events_processed += 1;
            stats.last_event_time = Some(start_time);
            stats.average_processing_time = Duration::from_micros(
                ((stats.average_processing_time.as_micros() + processing_time.as_micros()) / 2) as u64
            );
        }
    }

    /// Update event loop statistics
    async fn update_statistics(&self, event_count: u64, total_time: Duration) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.events_per_second = event_count as f64 / total_time.as_secs_f64();
            stats.queue_size = self.event_queue.lock().unwrap().len();
        }
    }

    /// Get event loop statistics
    pub fn get_stats(&self) -> EventLoopStats {
        self.stats.lock().unwrap().clone()
    }

    /// Check if the event loop is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Get the event sender for external use
    pub fn get_event_sender(&self) -> mpsc::UnboundedSender<EventLoopEvent> {
        self.event_sender.clone()
    }

    /// Simulate a click event
    pub fn simulate_click(&self, target_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_event(EventLoopEvent::InputEvent {
            event_type: "click".to_string(),
            target_id: target_id.to_string(),
            data: None,
        })
    }

    /// Simulate a keydown event
    pub fn simulate_keydown(&self, target_id: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_event(EventLoopEvent::InputEvent {
            event_type: "keydown".to_string(),
            target_id: target_id.to_string(),
            data: Some(key.to_string()),
        })
    }

    /// Execute JavaScript code
    pub fn execute_js(&self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_event(EventLoopEvent::JsEvent {
            script: script.to_string(),
            context: "main".to_string(),
        })
    }

    /// Trigger a render update
    pub fn trigger_render(&self, event_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_event(EventLoopEvent::RenderEvent {
            event_type: event_type.to_string(),
            data: None,
        })
    }
}

impl Clone for EventLoopStats {
    fn clone(&self) -> Self {
        Self {
            events_processed: self.events_processed,
            events_per_second: self.events_per_second,
            average_processing_time: self.average_processing_time,
            last_event_time: self.last_event_time,
            queue_size: self.queue_size,
            render_updates: self.render_updates,
            dom_updates: self.dom_updates,
            js_executions: self.js_executions,
            input_events: self.input_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_loop_creation() {
        let event_loop = AsyncEventLoop::new();
        assert!(!event_loop.is_running());
        
        let stats = event_loop.get_stats();
        assert_eq!(stats.events_processed, 0);
    }

    #[tokio::test]
    async fn test_event_sending() {
        let event_loop = AsyncEventLoop::new();
        
        let result = event_loop.simulate_click("test-button");
        assert!(result.is_ok());
        
        let result = event_loop.execute_js("console.log('test');");
        assert!(result.is_ok());
    }
}
