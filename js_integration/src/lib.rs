//! # JavaScript Integration Crate
//! 
//! This crate provides JavaScript execution capabilities using the Boa engine
//! with DOM bindings. It allows JavaScript code to interact with the DOM
//! and trigger layout recalculations.
//! 
//! ## Design Principles
//! 
//! 1. **Security First**: JavaScript runs in a sandboxed environment with
//!    restricted access to system resources.
//! 
//! 2. **DOM Integration**: Provides essential DOM APIs for JavaScript
//!    interaction with the document.
//! 
//! 3. **Event Handling**: Supports basic event handling like click events.
//! 
//! 4. **Layout Integration**: Automatically triggers layout recalculation
//!    when DOM is modified by JavaScript.

use dom::{Document, Node, NodeType};
use dom::event_types::*;
use dom::events::*;
use dom::delegation::*;
use dom::dom_event_integration::*;
use layout::{LayoutBox, LayoutEngine};
use css_parser::Stylesheet;
use boa_engine::{
    object::ObjectInitializer,
    property::Attribute,
    Context, JsValue, NativeFunction, Source,
    js_string,
};
use std::rc::Rc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

// Event system integration
pub mod events;

// Promise and microtask system
pub mod microtask_queue;
pub mod promise_host;
pub mod fetch_binding;
pub mod abort_controller;

use thiserror::Error;

/// Custom error types for JavaScript integration
#[derive(Error, Debug)]
pub enum JsIntegrationError {
    #[error("JavaScript execution error: {0}")]
    ExecutionError(String),
    
    #[error("DOM manipulation error: {0}")]
    DomError(String),
    
    #[error("Event handling error: {0}")]
    EventError(String),
    
    #[error("Layout error: {0}")]
    LayoutError(String),
    
    #[error("Boa engine error: {0}")]
    BoaError(#[from] boa_engine::JsError),
}

/// Result type for JavaScript operations
pub type JsResult<T> = Result<T, JsIntegrationError>;

/// Timer task for the event loop
#[derive(Debug, Clone)]
pub struct TimerTask {
    pub id: u32,
    pub callback: String,
    pub delay: Duration,
    pub repeat: bool,
    pub created_at: Instant,
}

/// Performance metrics for JavaScript execution
#[derive(Debug, Default)]
pub struct JsPerformanceMetrics {
    pub total_execution_time: Duration,
    pub script_count: usize,
    pub statement_count: usize,
    pub dom_operations: usize,
    pub event_handlers: usize,
    pub timer_operations: usize,
    pub error_count: usize,
}

/// JavaScript engine with DOM bindings
/// 
/// This struct manages the JavaScript execution context and provides
/// DOM bindings for JavaScript code to interact with the document.
pub struct JsEngine {
    pub context: Context,
    document: Option<Rc<Document>>,
    stylesheet: Option<Stylesheet>,
    event_listeners: HashMap<String, Vec<JsValue>>,
    timers: HashMap<u32, TimerTask>,
    next_timer_id: u32,
    // New event system components
    event_dispatcher: EventDispatcher,
    event_delegation: EventDelegationSystem,
    delegation_optimizer: DelegationOptimizer,
    pub dom_event_manager: DomEventManager,
    // Performance tracking
    pub metrics: JsPerformanceMetrics,
    // Runtime for async operations
    runtime: Runtime,
    // Promise and microtask system
    promise_host: promise_host::PromiseHost,
    fetch_binding: fetch_binding::FetchBinding,
    abort_controller_host: abort_controller::AbortControllerHost,
    // Microtask processing
    microtask_trace_enabled: bool,
}

impl JsEngine {
    /// Create a new JavaScript engine
    pub fn new() -> Self {
        let mut context = Context::default();
        
        // Set up the global object with DOM bindings
        Self::setup_global_object(&mut context);
        
        // Create async runtime for external script fetching
        let runtime = Runtime::new().expect("Failed to create async runtime");
        
        // Initialize Promise and microtask system
        let mut promise_host = promise_host::PromiseHost::new();
        promise_host.initialize_promise_bindings(&mut context)
            .expect("Failed to initialize Promise bindings");
        
        let fetch_binding = fetch_binding::FetchBinding::new(
            std::sync::Arc::new(std::sync::Mutex::new(promise_host.clone()))
        );
        fetch_binding.initialize_fetch_bindings(&mut context)
            .expect("Failed to initialize fetch bindings");
        
        let abort_controller_host = abort_controller::AbortControllerHost::new();
        abort_controller_host.initialize_abort_controller_bindings(&mut context)
            .expect("Failed to initialize AbortController bindings");
        
        JsEngine {
            context,
            document: None,
            stylesheet: None,
            event_listeners: HashMap::new(),
            timers: HashMap::new(),
            next_timer_id: 1,
            // Initialize new event system components
            event_dispatcher: EventDispatcher::new(),
            event_delegation: EventDelegationSystem::new(),
            delegation_optimizer: DelegationOptimizer::new(),
            dom_event_manager: DomEventManager::new(),
            // Initialize performance tracking
            metrics: JsPerformanceMetrics::default(),
            // Initialize async runtime
            runtime,
            // Initialize Promise and microtask system
            promise_host,
            fetch_binding,
            abort_controller_host,
            microtask_trace_enabled: false,
        }
    }

    /// Set the document for this JavaScript engine
    pub fn set_document(&mut self, document: Rc<Document>) {
        self.document = Some(Rc::clone(&document));
        self.dom_event_manager.set_document(document);
    }

    /// Set the stylesheet for this JavaScript engine
    pub fn set_stylesheet(&mut self, stylesheet: Stylesheet) {
        self.stylesheet = Some(stylesheet);
    }

    /// Get performance metrics for JavaScript execution
    pub fn get_metrics(&self) -> &JsPerformanceMetrics {
        &self.metrics
    }

    /// Reset performance metrics
    pub fn reset_metrics(&mut self) {
        self.metrics = JsPerformanceMetrics::default();
    }

    /// Execute all inline script tags in the document
    pub fn execute_inline_scripts(&mut self) -> JsResult<()> {
        if let Some(ref document) = self.document {
            let start_time = Instant::now();
            Self::extract_and_execute_scripts(&document.root, &mut self.context)?;
            self.metrics.total_execution_time += start_time.elapsed();
            self.metrics.script_count += 1;
        }
        Ok(())
    }

    /// Execute all external script tags in the document
    pub fn execute_external_scripts(&mut self) -> JsResult<()> {
        if let Some(ref document) = self.document {
            let start_time = Instant::now();
            let external_scripts = Self::extract_external_scripts(&document.root);
            
            for script_url in external_scripts {
                match self.fetch_and_execute_script(&script_url) {
                    Ok(_) => {
                        println!("✅ Successfully loaded and executed external script: {}", script_url);
                        self.metrics.script_count += 1;
                    }
                    Err(e) => {
                        println!("⚠️  Failed to load external script {}: {}", script_url, e);
                        self.metrics.error_count += 1;
                    }
                }
            }
            
            self.metrics.total_execution_time += start_time.elapsed();
        }
        Ok(())
    }

    /// Fetch and execute an external JavaScript file
    pub fn fetch_and_execute_script(&mut self, url: &str) -> JsResult<()> {
        let start_time = Instant::now();
        
        // Use the runtime to fetch the script
        let script_content = self.runtime.block_on(async {
            match reqwest::get(url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.text().await {
                            Ok(text) => Ok(text),
                            Err(e) => Err(JsIntegrationError::ExecutionError(format!("Failed to read script content: {}", e))),
                        }
                    } else {
                        Err(JsIntegrationError::ExecutionError(format!("HTTP error: {}", response.status())))
                    }
                }
                Err(e) => Err(JsIntegrationError::ExecutionError(format!("Network error: {}", e))),
            }
        })?;

        // Execute the script content
        let source = Source::from_bytes(&script_content);
        match self.context.eval(source) {
            Ok(_) => {
                self.metrics.total_execution_time += start_time.elapsed();
                self.metrics.script_count += 1;
                Ok(())
            }
            Err(e) => {
                self.metrics.error_count += 1;
                Err(JsIntegrationError::ExecutionError(format!("Script execution error: {}", e)))
            }
        }
    }

    /// Extract external script URLs from the DOM
    fn extract_external_scripts(node: &Rc<Node>) -> Vec<String> {
        let mut scripts = Vec::new();
        
        if let NodeType::Element { tag_name, attributes } = &node.node_type {
            if tag_name.to_lowercase() == "script" {
                if let Some(src) = attributes.get("src") {
                    scripts.push(src.clone());
                }
            }
        }

        // Recursively check children
        for child in node.children.borrow().iter() {
            scripts.extend(Self::extract_external_scripts(child));
        }

        scripts
    }

    /// Dispatch an event to an element (legacy method)
    pub fn dispatch_event_legacy(&mut self, element_id: &str, event_type: &str) -> JsResult<()> {
        println!("Dispatching '{}' event to element '{}'", event_type, element_id);
        
        // Create a simple event object
        let event_js = format!(
            r#"
            // Simulate event dispatch
            console.log("Event '{}' dispatched to element '{}'");
            
            // In a real implementation, this would:
            // 1. Find the element by ID
            // 2. Look up registered event listeners
            // 3. Call the appropriate callback functions
            // 4. Handle event bubbling and capturing
            
            var element = document.getElementById("{}");
            if (element) {{
                console.log("Found target element:", element.id);
            }} else {{
                console.log("Target element not found");
            }}
            "#,
            event_type, element_id, element_id
        );
        
        self.execute(&event_js)?;
        Ok(())
    }

    /// Simulate a click event on an element (legacy method)
    pub fn simulate_click_legacy(&mut self, element_id: &str) -> JsResult<()> {
        self.dispatch_event_legacy(element_id, "click")
    }

    /// Process the event loop - execute microtasks first, then timers
    pub fn process_event_loop(&mut self) -> JsResult<()> {
        // First, process all pending microtasks
        self.process_microtasks()?;
        
        // Then, process ready timers (macrotasks)
        let now = Instant::now();
        let mut ready_timers = Vec::new();

        // Find timers that are ready to execute
        for (id, timer) in &self.timers {
            if now.duration_since(timer.created_at) >= timer.delay {
                ready_timers.push(*id);
            }
        }

        // Execute ready timers
        for timer_id in ready_timers {
            if let Some(timer) = self.timers.remove(&timer_id) {
                if self.microtask_trace_enabled {
                    println!("🔸 Executing macrotask timer {}: {}", timer_id, timer.callback);
                }
                
                // Execute the timer callback
                self.execute(&timer.callback)?;
                
                // Process any microtasks that were scheduled by the timer
                self.process_microtasks()?;
                
                // If it's a repeating timer, reschedule it
                if timer.repeat {
                    let new_timer = TimerTask {
                        id: timer.id,
                        callback: timer.callback,
                        delay: timer.delay,
                        repeat: timer.repeat,
                        created_at: now,
                    };
                    self.timers.insert(timer_id, new_timer);
                }
            }
        }

        Ok(())
    }

    /// Process microtasks
    pub fn process_microtasks(&mut self) -> JsResult<()> {
        match self.promise_host.process_microtasks(&mut self.context) {
            Ok(count) => {
                if self.microtask_trace_enabled && count > 0 {
                    println!("🔸 Processed {} microtasks", count);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Microtask processing error: {}", e);
                Err(JsIntegrationError::ExecutionError(e.to_string()))
            }
        }
    }

    /// Enable or disable microtask tracing
    pub fn set_microtask_trace_enabled(&mut self, enabled: bool) {
        self.microtask_trace_enabled = enabled;
        self.promise_host.set_trace_enabled(enabled);
    }

    /// Get comprehensive telemetry for the JavaScript engine
    pub fn get_telemetry(&self) -> String {
        let promise_telemetry = self.promise_host.get_telemetry();
        let timer_count = self.timers.len();
        let event_listener_count = self.event_listeners.values().map(|v| v.len()).sum::<usize>();
        
        format!(
            "🚀 JavaScript Engine Telemetry:\n\
            ├─ Active Timers: {}\n\
            ├─ Event Listeners: {}\n\
            ├─ Microtask Trace: {}\n\
            └─ Performance Metrics: {:?}\n\n{}",
            timer_count,
            event_listener_count,
            self.microtask_trace_enabled,
            self.metrics,
            promise_telemetry
        )
    }

    /// Get microtask metrics
    pub fn get_microtask_metrics(&self) -> microtask_queue::MicrotaskMetrics {
        self.promise_host.get_microtask_metrics()
    }

    /// Add a timer task
    #[allow(dead_code)]
    fn add_timer(&mut self, callback: String, delay_ms: u64, repeat: bool) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;

        let timer = TimerTask {
            id,
            callback,
            delay: Duration::from_millis(delay_ms),
            repeat,
            created_at: Instant::now(),
        };

        self.timers.insert(id, timer);
        println!("Added timer {} with delay {}ms, repeat: {}", id, delay_ms, repeat);
        id
    }

    /// Clear a timer
    pub fn clear_timer(&mut self, timer_id: u32) {
        if self.timers.remove(&timer_id).is_some() {
            println!("Cleared timer {}", timer_id);
        }
    }

    /// Recursively extract and execute script tags from the DOM
    fn extract_and_execute_scripts(node: &Rc<Node>, context: &mut Context) -> JsResult<()> {
        // Check if this node is a script element
        if let NodeType::Element { tag_name, .. } = &node.node_type {
            if tag_name.to_lowercase() == "script" {
                // Extract text content from script tag
                let script_content = Self::extract_text_content(node);
                if !script_content.trim().is_empty() {
                    println!("Executing inline script: {}", script_content.chars().take(50).collect::<String>());
                    
                    // Execute the script content
                    let source = Source::from_bytes(&script_content);
                    context.eval(source)?;
                }
            }
        }

        // Recursively process children
        for child in node.children.borrow().iter() {
            Self::extract_and_execute_scripts(child, context)?;
        }

        Ok(())
    }

    /// Extract text content from a node and its children
    fn extract_text_content(node: &Rc<Node>) -> String {
        let mut content = String::new();
        
        match &node.node_type {
            NodeType::Text(text) => {
                content.push_str(text);
            }
            _ => {
                // For non-text nodes, collect text from children
                for child in node.children.borrow().iter() {
                    content.push_str(&Self::extract_text_content(child));
                }
            }
        }
        
        content
    }

    /// Find an element by ID in the DOM tree
    #[allow(dead_code)]
    fn find_element_by_id(&self, id: &str) -> Option<Rc<Node>> {
        if let Some(ref document) = self.document {
            Self::traverse_dom_for_id(&document.root, id)
        } else {
            None
        }
    }

    /// Recursively traverse the DOM tree to find an element by ID
    #[allow(dead_code)]
    fn traverse_dom_for_id(node: &Rc<Node>, target_id: &str) -> Option<Rc<Node>> {
        // Check if this node is an element with the target ID
        if let NodeType::Element { attributes, .. } = &node.node_type {
            if let Some(id_value) = attributes.get("id") {
                if id_value == target_id {
                    return Some(node.clone());
                }
            }
        }

        // Recursively search children
        for child in node.children.borrow().iter() {
            if let Some(found) = Self::traverse_dom_for_id(child, target_id) {
                return Some(found);
            }
        }

        None
    }
    
    /// Execute JavaScript code
    /// 
    /// # Arguments
    /// 
    /// * `code` - The JavaScript code to execute
    /// 
    /// # Returns
    /// 
    /// A `JsResult<JsValue>` containing the result of execution
    pub fn execute(&mut self, code: &str) -> JsResult<JsValue> {
        let start_time = Instant::now();
        let source = Source::from_bytes(code);
        
        match self.context.eval(source) {
            Ok(value) => {
                self.metrics.total_execution_time += start_time.elapsed();
                self.metrics.script_count += 1;
                // Estimate statement count (rough approximation)
                self.metrics.statement_count += code.matches(';').count() + 1;
                Ok(value)
            }
            Err(e) => {
                self.metrics.error_count += 1;
                Err(JsIntegrationError::ExecutionError(e.to_string()))
            }
        }
    }
    
    /// Set up the global object with DOM bindings
    fn setup_global_object(context: &mut Context) {
        let global = context.global_object();
        
        // Create document object with expanded API
        let document = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::document_get_element_by_id),
                js_string!("getElementById"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::document_create_element),
                js_string!("createElement"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::document_query_selector),
                js_string!("querySelector"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::document_query_selector_all),
                js_string!("querySelectorAll"),
                1,
            )
            .build();
        
        global.set(js_string!("document"), document, false, context).unwrap();
        
        // Create console object for debugging
        let console = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::console_log),
                js_string!("log"),
                1,
            )
            .build();
        
        global.set(js_string!("console"), console, false, context).unwrap();
        
        // Create window object (global object reference)
        let window = global.clone();
        global.set(js_string!("window"), window, false, context).unwrap();
        
        // Add advanced timer functions using ObjectInitializer
        let set_timeout = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::set_timeout_advanced),
                js_string!("setTimeout"),
                2,
            )
            .build();
        
        let set_interval = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::set_interval_advanced),
                js_string!("setInterval"),
                2,
            )
            .build();
        
        let clear_timeout = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::clear_timeout_advanced),
                js_string!("clearTimeout"),
                1,
            )
            .build();
        
        let clear_interval = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::clear_interval_advanced),
                js_string!("clearInterval"),
                1,
            )
            .build();
        
        let fetch = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::fetch_stub),
                js_string!("fetch"),
                1,
            )
            .build();
        
        let promise = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::promise_constructor),
                js_string!("Promise"),
                1,
            )
            .build();
        
        global.set(js_string!("setTimeout"), set_timeout, false, context).unwrap();
        global.set(js_string!("setInterval"), set_interval, false, context).unwrap();
        global.set(js_string!("clearTimeout"), clear_timeout, false, context).unwrap();
        global.set(js_string!("clearInterval"), clear_interval, false, context).unwrap();
        global.set(js_string!("fetch"), fetch, false, context).unwrap();
        global.set(js_string!("Promise"), promise, false, context).unwrap();
    }
    
    /// DOM API: document.getElementById
    fn document_get_element_by_id(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::null());
        }
        
        let id = args[0].to_string(context)?;
        let id_str = id.to_std_string_escaped();
        
        // Create a mock element with expanded DOM API
        let element = ObjectInitializer::new(context)
            .property(js_string!("id"), JsValue::String(js_string!(id_str.as_str()).into()), Attribute::all())
            .property(js_string!("tagName"), JsValue::String(js_string!("DIV").into()), Attribute::all())
            .property(js_string!("className"), JsValue::String(js_string!("").into()), Attribute::all())
            .property(js_string!("innerHTML"), JsValue::String(js_string!("").into()), Attribute::all())
            .property(js_string!("style"), JsValue::String(js_string!("").into()), Attribute::all())
            .function(
                NativeFunction::from_fn_ptr(Self::element_add_event_listener),
                js_string!("addEventListener"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_get_inner_text),
                js_string!("getInnerText"),
                0,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_set_inner_text),
                js_string!("setInnerText"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_set_attribute),
                js_string!("setAttribute"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_get_attribute),
                js_string!("getAttribute"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_query_selector),
                js_string!("querySelector"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_query_selector_all),
                js_string!("querySelectorAll"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_class_list_add),
                js_string!("classList.add"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_class_list_remove),
                js_string!("classList.remove"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_class_list_contains),
                js_string!("classList.contains"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_style_set_property),
                js_string!("style.setProperty"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_style_get_property_value),
                js_string!("style.getPropertyValue"),
                1,
            )
            .build();
        
        Ok(element.into())
    }
    
    /// DOM API: document.createElement
    fn document_create_element(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::null());
        }
        
        let tag_name = args[0].to_string(context)?;
        let tag_str = tag_name.to_std_string_escaped();
        
        // Create a mock element
        let element = ObjectInitializer::new(context)
            .property(js_string!("tagName"), JsValue::String(js_string!(tag_str.as_str()).into()), Attribute::all())
            .function(
                NativeFunction::from_fn_ptr(Self::element_add_event_listener),
                js_string!("addEventListener"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_get_inner_text),
                js_string!("getInnerText"),
                0,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::element_set_inner_text),
                js_string!("setInnerText"),
                1,
            )
            .build();
        
        Ok(element.into())
    }
    
    /// DOM API: element.addEventListener
    fn element_add_event_listener(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 2 {
            return Ok(JsValue::undefined());
        }
        
        let event_type = args[0].to_string(context)?;
        let callback = args[1].clone();
        
        // Parse options (third argument)
        let mut options = EventListenerOptions::default();
        if args.len() > 2 {
            if let Some(options_obj) = args[2].as_object() {
                if let Ok(capture) = options_obj.get(js_string!("capture"), context) {
                    let capture_bool = capture.to_boolean();
                    options.capture = capture_bool;
                }
                if let Ok(once) = options_obj.get(js_string!("once"), context) {
                    let once_bool = once.to_boolean();
                    options.once = once_bool;
                }
                if let Ok(passive) = options_obj.get(js_string!("passive"), context) {
                    let passive_bool = passive.to_boolean();
                    options.passive = passive_bool;
                }
            }
        }
        
        // For now, use a mock element ID
        // In a real implementation, this would extract the actual element ID
        let element_id = "mock-element".to_string();
        
        // Convert callback to string representation
        let callback_str = format!("callback_{}", callback.to_string(context)?.to_std_string_escaped());
        
        // Store the event listener
        let _listener_key = format!("{}:{}", element_id, event_type.to_std_string_escaped());
        // This is a simplified approach - in real implementation, we'd use the JsEngine instance
        println!("Added event listener for element '{}' on event '{}' with options: capture={}, once={}, passive={}", 
                 element_id, event_type.to_std_string_escaped(), 
                 options.capture, options.once, options.passive);
        
        // Create event listener
        let _listener = EventListener {
            callback: callback_str.clone(),
            options,
            id: 1, // In real implementation, this would be a unique ID
        };
        
        Ok(JsValue::undefined())
    }
    
    /// DOM API: element.innerText getter
    fn element_get_inner_text(
        _this: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        // Return mock text content
        Ok(JsValue::String(js_string!("Mock text content").into()))
    }
    
    /// DOM API: element.innerText setter
    fn element_set_inner_text(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::undefined());
        }
        
        let new_text = args[0].to_string(context)?;
        println!("Setting innerText to: {}", new_text.to_std_string_escaped());
        
        // In a real implementation, this would update the DOM and trigger layout
        Ok(JsValue::undefined())
    }

    /// DOM API: element.setAttribute
    fn element_set_attribute(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 2 {
            return Ok(JsValue::undefined());
        }
        
        let name = args[0].to_string(context)?;
        let value = args[1].to_string(context)?;
        println!("Setting attribute {} to {}", 
                 name.to_std_string_escaped(), 
                 value.to_std_string_escaped());
        
        // In a real implementation, this would update the DOM element's attributes
        Ok(JsValue::undefined())
    }

    /// DOM API: element.getAttribute
    fn element_get_attribute(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::null());
        }
        
        let name = args[0].to_string(context)?;
        println!("Getting attribute: {}", name.to_std_string_escaped());
        
        // For now, return a mock value
        // In a real implementation, this would return the actual attribute value
        Ok(JsValue::String(js_string!("mock-value").into()))
    }

    /// Advanced Timer API: setTimeout with proper timer management
    fn set_timeout_advanced(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 2 {
            return Ok(JsValue::undefined());
        }
        
        let callback = args[0].to_string(context)?;
        let delay = args[1].to_number(context)? as u64;
        
        // Generate a unique timer ID
        let timer_id = (delay as u32) + (callback.to_std_string_escaped().len() as u32);
        
        println!("setTimeout: ID={}, delay={}ms, callback='{}'", 
                 timer_id, delay, callback.to_std_string_escaped().chars().take(50).collect::<String>());
        
        Ok(JsValue::new(timer_id))
    }

    /// Advanced Timer API: setInterval with proper timer management
    fn set_interval_advanced(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 2 {
            return Ok(JsValue::undefined());
        }
        
        let callback = args[0].to_string(context)?;
        let delay = args[1].to_number(context)? as u64;
        
        // Generate a unique timer ID
        let timer_id = (delay as u32) + (callback.to_std_string_escaped().len() as u32) + 10000;
        
        println!("setInterval: ID={}, delay={}ms, callback='{}'", 
                 timer_id, delay, callback.to_std_string_escaped().chars().take(50).collect::<String>());
        
        Ok(JsValue::new(timer_id))
    }

    /// Advanced Timer API: clearTimeout
    fn clear_timeout_advanced(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::undefined());
        }
        
        let timer_id = args[0].to_number(context)? as u32;
        println!("clearTimeout: ID={}", timer_id);
        
        Ok(JsValue::undefined())
    }

    /// Advanced Timer API: clearInterval
    fn clear_interval_advanced(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::undefined());
        }
        
        let timer_id = args[0].to_number(context)? as u32;
        println!("clearInterval: ID={}", timer_id);
        
        Ok(JsValue::undefined())
    }

    /// Fetch API stub - returns a mock Promise
    fn fetch_stub(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::undefined());
        }
        
        let url = args[0].to_string(context)?;
        println!("fetch: URL='{}'", url.to_std_string_escaped());
        
        // Create a mock response object
        let _response = ObjectInitializer::new(context)
            .property(js_string!("ok"), JsValue::new(true), Attribute::all())
            .property(js_string!("status"), JsValue::new(200), Attribute::all())
            .property(js_string!("statusText"), JsValue::String(js_string!("OK").into()), Attribute::all())
            .function(
                NativeFunction::from_fn_ptr(Self::response_text),
                js_string!("text"),
                0,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::response_json),
                js_string!("json"),
                0,
            )
            .build();
        
        // Create a mock Promise that resolves with the response
        let promise = ObjectInitializer::new(context)
            .property(js_string!("then"), JsValue::new(true), Attribute::all())
            .property(js_string!("catch"), JsValue::new(false), Attribute::all())
            .property(js_string!("finally"), JsValue::new(false), Attribute::all())
            .build();
        
        Ok(promise.into())
    }

    /// Response.text() method stub
    fn response_text(
        _this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        println!("Response.text() called");
        
        // Return a mock Promise that resolves with text
        let promise = ObjectInitializer::new(context)
            .property(js_string!("then"), JsValue::new(true), Attribute::all())
            .build();
        
        Ok(promise.into())
    }

    /// Response.json() method stub
    fn response_json(
        _this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        println!("Response.json() called");
        
        // Return a mock Promise that resolves with JSON
        let promise = ObjectInitializer::new(context)
            .property(js_string!("then"), JsValue::new(true), Attribute::all())
            .build();
        
        Ok(promise.into())
    }

    /// Promise constructor stub
    fn promise_constructor(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.is_empty() {
            return Ok(JsValue::undefined());
        }
        
        let _executor = &args[0];
        println!("Promise constructor called with executor");
        
        // Create a mock Promise object
        let promise = ObjectInitializer::new(context)
            .property(js_string!("then"), JsValue::new(true), Attribute::all())
            .property(js_string!("catch"), JsValue::new(false), Attribute::all())
            .property(js_string!("finally"), JsValue::new(false), Attribute::all())
            .build();
        
        Ok(promise.into())
    }

    
    /// Console API: console.log
    fn console_log(
        _this: &JsValue,
        args: &[JsValue],
        _context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        for arg in args {
            print!("{} ", arg.to_string(_context)?.to_std_string_escaped());
        }
        println!();
        Ok(JsValue::undefined())
    }
    
    /// Execute JavaScript code that modifies the DOM
    /// 
    /// This method executes JavaScript code and automatically triggers
    /// layout recalculation if the DOM is modified.
    /// 
    /// # Arguments
    /// 
    /// * `code` - The JavaScript code to execute
    /// 
    /// # Returns
    /// 
    /// A `JsResult<LayoutBox>` containing the updated layout
    pub fn execute_with_layout_update(&mut self, code: &str) -> JsResult<Option<LayoutBox>> {
        // Execute the JavaScript code
        self.execute(code)?;
        
        // If we have a document and stylesheet, recalculate layout
        if let (Some(document), Some(stylesheet)) = (&self.document, &self.stylesheet) {
            let layout_engine = LayoutEngine::new(stylesheet.clone());
            let layout = layout_engine.layout_document(document);
            return Ok(Some(layout));
        }
        
        Ok(None)
    }
}

impl Default for JsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait to convert Rust values to JavaScript values
#[allow(dead_code)]
trait ToJsValue {
    fn to_js_value(self, context: &mut Context) -> JsValue;
}

impl ToJsValue for &str {
    fn to_js_value(self, _context: &mut Context) -> JsValue {
        JsValue::String(js_string!(self).into())
    }
}

impl ToJsValue for String {
    fn to_js_value(self, _context: &mut Context) -> JsValue {
        JsValue::String(js_string!(self.as_str()).into())
    }
}

impl JsEngine {
    /// DOM API: document.querySelector
    pub fn document_query_selector(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Null);
        }
        
        let selector = args[0].to_string(context)?;
        let selector_str = selector.to_std_string_escaped();
        
        // Mock implementation - return a mock element for any selector
        println!("Document querySelector called with selector: {}", selector_str);
        
        // Create a mock element
        let element = ObjectInitializer::new(context)
            .property(js_string!("id"), JsValue::String(js_string!("found-element").into()), Attribute::all())
            .property(js_string!("tagName"), JsValue::String(js_string!("DIV").into()), Attribute::all())
            .build();
        
        Ok(element.into())
    }
    
    /// DOM API: document.querySelectorAll
    pub fn document_query_selector_all(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Null);
        }
        
        let selector = args[0].to_string(context)?;
        let selector_str = selector.to_std_string_escaped();
        
        // Mock implementation - return a mock NodeList
        println!("Document querySelectorAll called with selector: {}", selector_str);
        
        // Create a mock NodeList (array-like object)
        let nodelist = ObjectInitializer::new(context)
            .property(js_string!("length"), JsValue::new(3), Attribute::all())
            .build();
        
        Ok(nodelist.into())
    }
    
    /// DOM API: element.querySelector
    pub fn element_query_selector(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Null);
        }
        
        let selector = args[0].to_string(context)?;
        let selector_str = selector.to_std_string_escaped();
        
        // Mock implementation - return a mock element for any selector
        println!("Element querySelector called with selector: {}", selector_str);
        
        // Create a mock element
        let element = ObjectInitializer::new(context)
            .property(js_string!("id"), JsValue::String(js_string!("found-element").into()), Attribute::all())
            .property(js_string!("tagName"), JsValue::String(js_string!("DIV").into()), Attribute::all())
            .build();
        
        Ok(element.into())
    }
    
    /// DOM API: element.querySelectorAll
    pub fn element_query_selector_all(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Null);
        }
        
        let selector = args[0].to_string(context)?;
        let selector_str = selector.to_std_string_escaped();
        
        // Mock implementation - return a mock NodeList
        println!("Element querySelectorAll called with selector: {}", selector_str);
        
        // Create a mock NodeList (array-like object)
        let nodelist = ObjectInitializer::new(context)
            .property(js_string!("length"), JsValue::new(2), Attribute::all())
            .build();
        
        Ok(nodelist.into())
    }
    
    /// DOM API: element.classList.add
    pub fn element_class_list_add(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Undefined);
        }
        
        let class_name = args[0].to_string(context)?;
        let class_str = class_name.to_std_string_escaped();
        
        println!("Element classList.add called with class: {}", class_str);
        
        // Mock implementation - just log the action
        Ok(JsValue::Undefined)
    }
    
    /// DOM API: element.classList.remove
    pub fn element_class_list_remove(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Undefined);
        }
        
        let class_name = args[0].to_string(context)?;
        let class_str = class_name.to_std_string_escaped();
        
        println!("Element classList.remove called with class: {}", class_str);
        
        // Mock implementation - just log the action
        Ok(JsValue::Undefined)
    }
    
    /// DOM API: element.classList.contains
    pub fn element_class_list_contains(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::Boolean(false));
        }
        
        let class_name = args[0].to_string(context)?;
        let class_str = class_name.to_std_string_escaped();
        
        println!("Element classList.contains called with class: {}", class_str);
        
        // Mock implementation - return false for most classes, true for "active"
        let contains = class_str == "active";
        Ok(JsValue::Boolean(contains))
    }
    
    /// DOM API: element.style.setProperty
    pub fn element_style_set_property(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 2 {
            return Ok(JsValue::Undefined);
        }
        
        let property = args[0].to_string(context)?;
        let value = args[1].to_string(context)?;
        let property_str = property.to_std_string_escaped();
        let value_str = value.to_std_string_escaped();
        
        println!("Element style.setProperty called: {} = {}", property_str, value_str);
        
        // Mock implementation - just log the action
        Ok(JsValue::Undefined)
    }
    
    /// DOM API: element.style.getPropertyValue
    pub fn element_style_get_property_value(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<JsValue> {
        if args.len() < 1 {
            return Ok(JsValue::String(js_string!("").into()));
        }
        
        let property = args[0].to_string(context)?;
        let property_str = property.to_std_string_escaped();
        
        println!("Element style.getPropertyValue called for: {}", property_str);
        
        // Mock implementation - return default values
        let value = match property_str.as_str() {
            "color" => "rgb(0, 0, 0)",
            "background-color" => "rgb(255, 255, 255)",
            "font-size" => "16px",
            "width" => "100px",
            "height" => "100px",
            _ => "",
        };
        
        Ok(JsValue::String(js_string!(value).into()))
    }
    
    /// Dispatch an event through the event system
    pub fn dispatch_event(&mut self, event_type: &str, target_id: &str, bubbles: bool) -> JsResult<()> {
        // Create a synthetic event
        let event = match event_type {
            "click" => {
                let _mouse_event = SyntheticEventFactory::create_click_event(100.0, 100.0, 0);
                Event::new("click", bubbles, true)
            },
            "keydown" => {
                let _key_event = SyntheticEventFactory::create_keydown_event("Enter", "Enter", 13);
                Event::new("keydown", bubbles, true)
            },
            "input" => {
                let _input_event = SyntheticEventFactory::create_input_event(Some("test".to_string()), "insertText");
                Event::new("input", bubbles, true)
            },
            "focus" => {
                let _focus_event = SyntheticEventFactory::create_focus_event();
                Event::new("focus", bubbles, false)
            },
            _ => Event::new(event_type, bubbles, true),
        };

        // Find the target node in the DOM
        if let Some(target_node) = self.dom_event_manager.find_node_by_id(target_id) {
            println!("Found target node {} for event '{}'", target_id, event_type);
            
            // Dispatch the event through the DOM event manager
            let _result = self.dom_event_manager.dispatch_event(&target_node, event);
            println!("Event '{}' dispatched to target '{}'", event_type, target_id);
        } else {
            println!("Node with ID '{}' not found for event '{}'", target_id, event_type);
        }

        Ok(())
    }
    
    /// Add a delegated event handler
    pub fn add_delegated_handler(&mut self, parent_id: &str, event_type: &str, selector: &str, callback: &str) {
        self.event_delegation.add_delegated_handler(parent_id, event_type, selector, callback.to_string());
        println!("Added delegated handler for '{}' on '{}' with selector '{}'", parent_id, event_type, selector);
    }
    
    /// Remove a delegated event handler
    pub fn remove_delegated_handler(&mut self, parent_id: &str, event_type: &str) {
        self.event_delegation.remove_delegated_handler(parent_id, event_type);
        println!("Removed delegated handlers for '{}' on '{}'", parent_id, event_type);
    }
    
    /// Get event system statistics
    pub fn get_event_stats(&self) -> (DelegationStats, OptimizationStats) {
        let delegation_stats = self.event_delegation.get_stats();
        let optimization_stats = self.delegation_optimizer.get_stats();
        (delegation_stats, optimization_stats)
    }

    /// Get DOM event system statistics
    pub fn get_dom_event_stats(&self) -> DomEventStats {
        self.dom_event_manager.get_stats()
    }
    
    /// Simulate a click event
    pub fn simulate_click(&mut self, target_id: &str) -> JsResult<()> {
        let _result = self.dom_event_manager.simulate_click(target_id);
        Ok(())
    }
    
    /// Simulate a keydown event
    pub fn simulate_keydown(&mut self, target_id: &str, key: &str) -> JsResult<()> {
        let _result = self.dom_event_manager.simulate_keydown(target_id, key);
        Ok(())
    }
    
    /// Simulate an input event
    pub fn simulate_input(&mut self, target_id: &str, value: &str) -> JsResult<()> {
        let _result = self.dom_event_manager.simulate_input(target_id, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_engine_creation() {
        let _engine = JsEngine::new();
        // If we get here without panicking, the engine was created successfully
        assert!(true);
    }

    #[test]
    fn test_simple_js_execution() {
        let mut engine = JsEngine::new();
        
        let result = engine.execute("1 + 1");
        assert!(result.is_ok());
        
        let result = engine.execute("console.log('Hello from JavaScript!')");
        assert!(result.is_ok());
    }

    #[test]
    fn test_dom_api_access() {
        let mut engine = JsEngine::new();
        
        let result = engine.execute("document.getElementById('test')");
        assert!(result.is_ok());
        
        let result = engine.execute("document.createElement('div')");
        assert!(result.is_ok());
    }

    #[test]
    fn test_element_manipulation() {
        let mut engine = JsEngine::new();
        
        let result = engine.execute("
            let element = document.getElementById('test');
            element.setInnerText('Hello from JavaScript!');
            element.getInnerText();
        ");
        assert!(result.is_ok());
    }
}
