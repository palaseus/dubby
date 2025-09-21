//! # Layout Engine Crate
//! 
//! This crate provides the layout engine for the browser, implementing the CSS
//! box model and flow layout algorithms. It takes styled DOM elements and
//! calculates their positions and sizes.
//! 
//! ## Design Principles
//! 
//! 1. **Box Model**: Implements the standard CSS box model with margin, border,
//!    padding, and content areas.
//! 
//! 2. **Flow Layout**: Implements block and inline flow layout algorithms
//!    following CSS specifications.
//! 
//! 3. **Tree Traversal**: Layout is calculated by traversing the DOM tree
//!    and computing box properties for each element.
//! 
//! 4. **Extensibility**: Designed to support additional layout modes like
//!    flexbox and grid in the future.

use dom::{Document, Node, NodeType};
use css_parser::{Stylesheet, Selector, CSSValue};
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Represents the computed styles for an element
/// 
/// This struct contains all the CSS properties that have been computed
/// for a DOM element, including values from the cascade and inheritance.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyles {
    /// Display property (block, inline, none, etc.)
    pub display: DisplayType,
    /// Width of the content area
    pub width: Option<f32>,
    /// Height of the content area
    pub height: Option<f32>,
    /// Margin values (top, right, bottom, left)
    pub margin: BoxSides,
    /// Border values (top, right, bottom, left)
    pub border: BoxSides,
    /// Padding values (top, right, bottom, left)
    pub padding: BoxSides,
    /// Background color
    pub background_color: Option<String>,
    /// Text color
    pub color: Option<String>,
    /// Font size
    pub font_size: Option<f32>,
    /// Font family
    pub font_family: Option<String>,
    /// Font weight
    pub font_weight: Option<String>,
    /// Text alignment
    pub text_align: Option<String>,
    /// Flexbox properties
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<f32>,
    pub align_self: Option<AlignSelf>,
    /// Grid properties
    pub grid_template_columns: Option<Vec<GridTrack>>,
    pub grid_template_rows: Option<Vec<GridTrack>>,
    pub grid_gap: Option<f32>,
    pub grid_column_gap: Option<f32>,
    pub grid_row_gap: Option<f32>,
    pub grid_column_start: Option<i32>,
    pub grid_column_end: Option<i32>,
    pub grid_row_start: Option<i32>,
    pub grid_row_end: Option<i32>,
    /// Animation properties
    pub transition_duration: Option<f32>,
    pub transition_delay: Option<f32>,
    pub transition_timing_function: Option<TimingFunction>,
    pub animation_name: Option<String>,
    pub animation_duration: Option<f32>,
    pub animation_delay: Option<f32>,
    pub animation_iteration_count: Option<f32>,
    pub animation_direction: Option<AnimationDirection>,
    pub animation_fill_mode: Option<AnimationFillMode>,
    pub animation_play_state: Option<AnimationPlayState>,
}

/// Represents the display type of an element
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayType {
    /// Block-level element
    Block,
    /// Inline element
    Inline,
    /// Inline-block element
    InlineBlock,
    /// Flex container
    Flex,
    /// Grid container
    Grid,
    /// Hidden element
    None,
}

/// Flexbox direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Flexbox wrap behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

/// Flexbox justify content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Flexbox align items
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

/// Flexbox align content
#[derive(Debug, Clone, PartialEq)]
pub enum AlignContent {
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
}

/// Flexbox align self
#[derive(Debug, Clone, PartialEq)]
pub enum AlignSelf {
    Auto,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

/// Grid track size
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrack {
    /// Fixed size (e.g., 100px)
    Fixed(f32),
    /// Fractional unit (e.g., 1fr)
    Fractional(f32),
    /// Auto size
    Auto,
    /// Min-content
    MinContent,
    /// Max-content
    MaxContent,
}

/// Animation timing function
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimingFunction {
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(i32),
}

/// Animation direction
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Animation fill mode
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

/// Animation play state
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

/// Represents the four sides of a box (top, right, bottom, left)
#[derive(Debug, Clone, PartialEq)]
pub struct BoxSides {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BoxSides {
    /// Create a new BoxSides with all sides set to the same value
    pub fn new(all: f32) -> Self {
        BoxSides {
            top: all,
            right: all,
            bottom: all,
            left: all,
        }
    }
    
    /// Create a new BoxSides with vertical and horizontal values
    pub fn new_vertical_horizontal(vertical: f32, horizontal: f32) -> Self {
        BoxSides {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
    
    /// Create a new BoxSides with individual values
    pub fn new_individual(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        BoxSides { top, right, bottom, left }
    }
}

/// Represents a layout box for an element
/// 
/// This struct contains the calculated dimensions and position of an element
/// after layout has been performed. It includes the box model properties
/// and positioning information.
#[derive(Debug, Clone)]
pub struct LayoutBox {
    /// The DOM node this box represents
    pub node: Rc<Node>,
    /// The computed styles for this element
    pub styles: ComputedStyles,
    /// The dimensions of the content area
    pub content: Dimensions,
    /// The dimensions of the padding area
    pub padding: Dimensions,
    /// The dimensions of the border area
    pub border: Dimensions,
    /// The dimensions of the margin area
    pub margin: Dimensions,
    /// Child layout boxes
    pub children: Vec<LayoutBox>,
    /// Animation state
    pub animation_state: AnimationState,
}

/// Animation state for tracking CSS animations
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// Whether animation is currently running
    pub is_running: bool,
    /// Animation start time
    pub start_time: Option<Instant>,
    /// Current animation progress (0.0 to 1.0)
    pub progress: f32,
    /// Animation iteration count
    pub iteration_count: f32,
    /// Whether animation is paused
    pub is_paused: bool,
    /// Pause time
    pub pause_time: Option<Instant>,
}

impl Default for ComputedStyles {
    fn default() -> Self {
        ComputedStyles {
            display: DisplayType::Block,
            width: None,
            height: None,
            margin: BoxSides::new(0.0),
            border: BoxSides::new(0.0),
            padding: BoxSides::new(0.0),
            background_color: None,
            color: Some("black".to_string()),
            font_size: Some(16.0),
            font_family: Some("serif".to_string()),
            font_weight: Some("normal".to_string()),
            text_align: Some("left".to_string()),
            // Flexbox properties
            flex_direction: None,
            flex_wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            align_self: None,
            // Grid properties
            grid_template_columns: None,
            grid_template_rows: None,
            grid_gap: None,
            grid_column_gap: None,
            grid_row_gap: None,
            grid_column_start: None,
            grid_column_end: None,
            grid_row_start: None,
            grid_row_end: None,
            // Animation properties
            transition_duration: None,
            transition_delay: None,
            transition_timing_function: None,
            animation_name: None,
            animation_duration: None,
            animation_delay: None,
            animation_iteration_count: None,
            animation_direction: None,
            animation_fill_mode: None,
            animation_play_state: None,
        }
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            is_running: false,
            start_time: None,
            progress: 0.0,
            iteration_count: 0.0,
            is_paused: false,
            pause_time: None,
        }
    }
}

/// Represents dimensions (width and height)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    /// Create new dimensions
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Dimensions { x, y, width, height }
    }
    
    /// Create new dimensions with zero position
    pub fn new_size(width: f32, height: f32) -> Self {
        Dimensions { x: 0.0, y: 0.0, width, height }
    }
    
    /// Get the right edge position
    pub fn right(&self) -> f32 {
        self.x + self.width
    }
    
    /// Get the bottom edge position
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

/// A style matcher that matches CSS selectors against DOM elements
/// 
/// This struct handles the process of matching CSS selectors to DOM elements
/// and computing the specificity of matches to determine which styles apply.
pub struct StyleMatcher {
    /// The stylesheet containing CSS rules
    stylesheet: Stylesheet,
}

impl StyleMatcher {
    /// Create a new style matcher with the given stylesheet
    pub fn new(stylesheet: Stylesheet) -> Self {
        StyleMatcher { stylesheet }
    }
    
    /// Compute styles for a DOM element
    /// 
    /// This method matches CSS selectors against the element and computes
    /// the final styles based on specificity and inheritance.
    pub fn compute_styles(&self, element: &Rc<Node>) -> ComputedStyles {
        let mut styles = self.get_default_styles(element);
        
        // Apply styles from matching rules
        for rule in &self.stylesheet.rules {
            for selector in &rule.selectors {
                if self.matches_selector(selector, element) {
                    self.apply_rule(&mut styles, rule);
                }
            }
        }
        
        // Apply inherited styles
        self.apply_inherited_styles(&mut styles, element);
        
        styles
    }
    
    /// Get default styles for elements
    fn get_default_styles(&self, element: &Rc<Node>) -> ComputedStyles {
        // Determine default display type based on element type
        let default_display = match &element.node_type {
            dom::NodeType::Element { tag_name, .. } => {
                match tag_name.as_str() {
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "div" | "ul" | "ol" | "li" | "body" | "html" => DisplayType::Block,
                    "span" | "a" | "em" | "strong" | "code" => DisplayType::Inline,
                    _ => DisplayType::Block, // Default to block for unknown elements
                }
            },
            dom::NodeType::Text(_) => DisplayType::Inline,
            _ => DisplayType::Block,
        };
        
        ComputedStyles {
            display: default_display,
            width: None,
            height: None,
            margin: BoxSides::new(0.0),
            border: BoxSides::new(0.0),
            padding: BoxSides::new(0.0),
            background_color: None,
            color: Some("black".to_string()),
            font_size: Some(16.0),
            font_family: Some("serif".to_string()),
            font_weight: Some("normal".to_string()),
            text_align: Some("left".to_string()),
            // Flexbox properties
            flex_direction: None,
            flex_wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            align_self: None,
            // Grid properties
            grid_template_columns: None,
            grid_template_rows: None,
            grid_gap: None,
            grid_column_gap: None,
            grid_row_gap: None,
            grid_column_start: None,
            grid_column_end: None,
            grid_row_start: None,
            grid_row_end: None,
            // Animation properties
            transition_duration: None,
            transition_delay: None,
            transition_timing_function: None,
            animation_name: None,
            animation_duration: None,
            animation_delay: None,
            animation_iteration_count: None,
            animation_direction: None,
            animation_fill_mode: None,
            animation_play_state: None,
        }
    }
    
    /// Check if a selector matches an element
    fn matches_selector(&self, selector: &Selector, element: &Rc<Node>) -> bool {
        match selector {
            Selector::Universal => true,
            Selector::Type(tag_name) => {
                if let NodeType::Element { tag_name: element_tag, .. } = &element.node_type {
                    element_tag == tag_name
                } else {
                    false
                }
            }
            Selector::Class(class_name) => {
                if let NodeType::Element { attributes, .. } = &element.node_type {
                    attributes.get("class").map_or(false, |class_attr| {
                        class_attr.split_whitespace().any(|c| c == class_name)
                    })
                } else {
                    false
                }
            }
            Selector::Id(id_name) => {
                if let NodeType::Element { attributes, .. } = &element.node_type {
                    attributes.get("id").map_or(false, |id_attr| id_attr == id_name)
                } else {
                    false
                }
            }
            Selector::Descendant(ancestor, descendant) => {
                self.matches_selector(descendant, element) && 
                self.has_ancestor_matching(element, ancestor)
            }
            Selector::Child(parent, child) => {
                self.matches_selector(child, element) && 
                self.has_parent_matching(element, parent)
            }
            _ => false, // Other combinators not implemented yet
        }
    }
    
    /// Check if an element has an ancestor matching the selector
    fn has_ancestor_matching(&self, element: &Rc<Node>, selector: &Selector) -> bool {
        if let Some(parent) = element.parent.borrow().upgrade() {
            if self.matches_selector(selector, &parent) {
                return true;
            }
            return self.has_ancestor_matching(&parent, selector);
        }
        false
    }
    
    /// Check if an element has a parent matching the selector
    fn has_parent_matching(&self, element: &Rc<Node>, selector: &Selector) -> bool {
        if let Some(parent) = element.parent.borrow().upgrade() {
            return self.matches_selector(selector, &parent);
        }
        false
    }
    
    /// Apply a CSS rule to the computed styles
    fn apply_rule(&self, styles: &mut ComputedStyles, rule: &css_parser::CSSRule) {
        for declaration in &rule.declarations {
            self.apply_declaration(styles, declaration);
        }
    }
    
    /// Apply a CSS declaration to the computed styles
    fn apply_declaration(&self, styles: &mut ComputedStyles, declaration: &css_parser::CSSDeclaration) {
        match declaration.property.as_str() {
            "display" => {
                if let CSSValue::Keyword(value) = &declaration.value {
                    styles.display = match value.as_str() {
                        "block" => DisplayType::Block,
                        "inline" => DisplayType::Inline,
                        "inline-block" => DisplayType::InlineBlock,
                        "none" => DisplayType::None,
                        _ => styles.display.clone(),
                    };
                }
            }
            "width" => {
                if let CSSValue::Dimension(value, unit) = &declaration.value {
                    styles.width = Some(self.convert_length(*value, unit));
                }
            }
            "height" => {
                if let CSSValue::Dimension(value, unit) = &declaration.value {
                    styles.height = Some(self.convert_length(*value, unit));
                }
            }
            "margin" => {
                styles.margin = self.parse_box_sides(&declaration.value);
            }
            "padding" => {
                styles.padding = self.parse_box_sides(&declaration.value);
            }
            "border" => {
                styles.border = self.parse_box_sides(&declaration.value);
            }
            "background-color" | "background" => {
                if let CSSValue::Color(color) = &declaration.value {
                    styles.background_color = Some(color.clone());
                }
            }
            "color" => {
                match &declaration.value {
                    CSSValue::Color(color) => {
                        styles.color = Some(color.clone());
                    }
                    CSSValue::Keyword(keyword) => {
                        styles.color = Some(keyword.clone());
                    }
                    _ => {}
                }
            }
            "font-size" => {
                if let CSSValue::Dimension(value, unit) = &declaration.value {
                    styles.font_size = Some(self.convert_length(*value, unit));
                }
            }
            "font-family" => {
                if let CSSValue::String(family) = &declaration.value {
                    styles.font_family = Some(family.clone());
                }
            }
            "font-weight" => {
                if let CSSValue::Keyword(weight) = &declaration.value {
                    styles.font_weight = Some(weight.clone());
                }
            }
            "text-align" => {
                if let CSSValue::Keyword(align) = &declaration.value {
                    styles.text_align = Some(align.clone());
                }
            }
            _ => {} // Ignore unknown properties
        }
    }
    
    /// Convert a CSS length value to pixels
    fn convert_length(&self, value: f32, unit: &str) -> f32 {
        match unit {
            "px" => value,
            "em" => value * 16.0, // Assuming 16px base font size
            "rem" => value * 16.0,
            "pt" => value * 1.33, // 1pt = 1.33px
            "pc" => value * 16.0, // 1pc = 16px
            "in" => value * 96.0, // 1in = 96px
            "cm" => value * 37.8, // 1cm = 37.8px
            "mm" => value * 3.78, // 1mm = 3.78px
            _ => value, // Default to pixels
        }
    }
    
    /// Parse box sides from a CSS value
    fn parse_box_sides(&self, value: &CSSValue) -> BoxSides {
        match value {
            CSSValue::Dimension(val, unit) => {
                BoxSides::new(self.convert_length(*val, unit))
            }
            _ => BoxSides::new(0.0),
        }
    }
    
    /// Apply inherited styles from parent elements
    fn apply_inherited_styles(&self, styles: &mut ComputedStyles, element: &Rc<Node>) {
        if let Some(parent) = element.parent.borrow().upgrade() {
            let parent_styles = self.compute_styles(&parent);
            
            // Inherit certain properties
            if styles.color.is_none() {
                styles.color = parent_styles.color;
            }
            if styles.font_size.is_none() {
                styles.font_size = parent_styles.font_size;
            }
            if styles.font_family.is_none() {
                styles.font_family = parent_styles.font_family;
            }
            if styles.font_weight.is_none() {
                styles.font_weight = parent_styles.font_weight;
            }
            if styles.text_align.is_none() {
                styles.text_align = parent_styles.text_align;
            }
        }
    }
}

/// A layout engine that calculates positions and sizes for DOM elements
/// 
/// This struct implements the CSS box model and flow layout algorithms.
/// It takes a styled DOM tree and produces a layout tree with calculated
/// dimensions and positions.
pub struct LayoutEngine {
    /// The style matcher for computing styles
    style_matcher: StyleMatcher,
}

impl LayoutEngine {
    /// Create a new layout engine with the given stylesheet
    pub fn new(stylesheet: Stylesheet) -> Self {
        LayoutEngine {
            style_matcher: StyleMatcher::new(stylesheet),
        }
    }
    
    /// Create a new layout engine without a stylesheet (for use with computed styles)
    pub fn new_empty() -> Self {
        LayoutEngine {
            style_matcher: StyleMatcher::new(Stylesheet { rules: vec![], source_url: None }),
        }
    }
    
    /// Compute layout using pre-computed styles from CSS cascade
    pub fn compute_layout_with_styles(&mut self, document: &Document, computed_styles: &HashMap<u64, css_parser::ComputedStyles>) -> LayoutBox {
        // Convert CSS parser styles to layout styles and create a simple layout
        self.layout_element_with_computed_styles(&document.root, computed_styles, Dimensions::new(0.0, 0.0, 800.0, 600.0))
    }
    
    /// Layout element using pre-computed styles
    fn layout_element_with_computed_styles(&self, element: &Rc<Node>, computed_styles: &HashMap<u64, css_parser::ComputedStyles>, containing_block: Dimensions) -> LayoutBox {
        // Get computed styles for this element
        let css_styles = computed_styles.get(&element.id);
        
        // Convert to layout ComputedStyles
        let styles = if let Some(css_styles) = css_styles {
            ComputedStyles {
                display: match css_styles.display.as_deref() {
                    Some("block") => DisplayType::Block,
                    Some("inline") => DisplayType::Inline,
                    Some("inline-block") => DisplayType::InlineBlock,
                    Some("flex") => DisplayType::Flex,
                    Some("grid") => DisplayType::Grid,
                    Some("none") => DisplayType::None,
                    _ => DisplayType::Block,
                },
                width: css_styles.width.as_ref().and_then(|w| w.replace("px", "").parse::<f32>().ok()),
                height: css_styles.height.as_ref().and_then(|h| h.replace("px", "").parse::<f32>().ok()),
                margin: BoxSides::new(0.0), // Simplified for now
                border: BoxSides::new(0.0),
                padding: BoxSides::new(0.0),
                background_color: css_styles.background_color.clone(),
                color: css_styles.color.clone(),
                font_size: css_styles.font_size.as_ref().and_then(|f| f.replace("px", "").parse::<f32>().ok()),
                font_family: css_styles.font_family.clone(),
                font_weight: css_styles.font_weight.clone(),
                text_align: css_styles.text_align.clone(),
                flex_direction: None,
                flex_wrap: None,
                justify_content: None,
                align_items: None,
                align_content: None,
                flex_grow: None,
                flex_shrink: None,
                flex_basis: None,
                align_self: None,
                grid_template_columns: None,
                grid_template_rows: None,
                grid_gap: None,
                grid_column_gap: None,
                grid_row_gap: None,
                grid_column_start: None,
                grid_column_end: None,
                grid_row_start: None,
                grid_row_end: None,
                transition_duration: None,
                transition_delay: None,
                transition_timing_function: None,
                animation_name: None,
                animation_duration: None,
                animation_delay: None,
                animation_iteration_count: None,
                animation_direction: None,
                animation_fill_mode: None,
                animation_play_state: None,
            }
        } else {
            // Use default styles
            self.style_matcher.get_default_styles(element)
        };
        
        // Skip elements with display: none
        if styles.display == DisplayType::None {
            return LayoutBox {
                node: Rc::clone(element),
                styles,
                content: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                padding: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                border: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                margin: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                children: Vec::new(),
                animation_state: AnimationState::default(),
            };
        }
        
        // Calculate content dimensions
        let content_width = styles.width.unwrap_or(containing_block.width);
        let content_height = styles.height.unwrap_or(20.0); // Default height
        
        let content = Dimensions::new(0.0, 0.0, content_width, content_height);
        
        // Layout children
        let mut children = Vec::new();
        let mut child_y = 0.0;
        
        for child in element.children.borrow().iter() {
            let child_layout = self.layout_element_with_computed_styles(child, computed_styles, content);
            child_y += child_layout.content.height;
            children.push(child_layout);
        }
        
        // Update content height based on children
        let final_height = if children.is_empty() {
            content_height
        } else {
            child_y.max(content_height)
        };
        
        let final_content = Dimensions::new(0.0, 0.0, content_width, final_height);
        
        LayoutBox {
            node: Rc::clone(element),
            styles,
            content: final_content,
            padding: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            border: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            margin: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            children,
            animation_state: AnimationState::default(),
        }
    }
    
    /// Layout a document and return the root layout box
    /// 
    /// This method performs the complete layout process:
    /// 1. Compute styles for all elements
    /// 2. Build a layout tree
    /// 3. Calculate dimensions and positions
    pub fn layout_document(&self, document: &Document) -> LayoutBox {
        // Try using the document root directly instead of document_element()
        let root_element = &document.root;
        
        
        self.layout_element(root_element, Dimensions::new(0.0, 0.0, 800.0, 600.0))
    }
    
    /// Layout a single element and its children
    fn layout_element(&self, element: &Rc<Node>, containing_block: Dimensions) -> LayoutBox {
        let styles = self.style_matcher.compute_styles(element);
        
        // Skip elements with display: none
        if styles.display == DisplayType::None {
            return LayoutBox {
                node: Rc::clone(element),
                styles,
                content: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                padding: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                border: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                margin: Dimensions::new(0.0, 0.0, 0.0, 0.0),
                children: Vec::new(),
                animation_state: AnimationState::default(),
            };
        }
        
        // Calculate content dimensions
        let content_width = styles.width.unwrap_or(containing_block.width);
        // For height, we'll calculate it based on content after laying out children
        let content_height = styles.height.unwrap_or(0.0);
        
        let mut layout_box = LayoutBox {
            node: Rc::clone(element),
            styles: styles.clone(),
            content: Dimensions::new(0.0, 0.0, content_width, content_height),
            padding: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            border: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            margin: Dimensions::new(0.0, 0.0, 0.0, 0.0),
            children: Vec::new(),
            animation_state: AnimationState::default(),
        };
        
        // Layout children
        self.layout_children(&mut layout_box, containing_block);
        
        // Calculate total dimensions including padding, border, and margin
        self.calculate_box_dimensions(&mut layout_box);
        
        layout_box
    }
    
    /// Layout the children of an element
    fn layout_children(&self, parent: &mut LayoutBox, containing_block: Dimensions) {
        match parent.styles.display {
            DisplayType::Flex => {
                self.layout_flex_children(parent, containing_block);
            }
            DisplayType::Grid => {
                self.layout_grid_children(parent, containing_block);
            }
            _ => {
                self.layout_block_children(parent, containing_block);
            }
        }
    }
    
    /// Layout children using block layout (default)
    fn layout_block_children(&self, parent: &mut LayoutBox, containing_block: Dimensions) {
        let mut current_y = 0.0;
        
        for child_node in parent.node.children.borrow().iter() {
            let child_layout = self.layout_element(child_node, containing_block);
            
            // Position the child
            let mut positioned_child = child_layout;
            positioned_child.content.y = current_y;
            
            // Update current position for next child
            current_y += positioned_child.margin.height + 
                        positioned_child.border.height + 
                        positioned_child.padding.height + 
                        positioned_child.content.height;
            
            parent.children.push(positioned_child);
        }
        
        // Update parent height to contain all children
        if !parent.children.is_empty() {
            let total_height = current_y;
            parent.content.height = total_height.max(parent.content.height);
        } else {
            // If no children, set a minimum height based on font size for text content
            let font_size = parent.styles.font_size.unwrap_or(16.0);
            let min_height = font_size * 1.2; // Line height of 1.2
            parent.content.height = parent.content.height.max(min_height);
        }
    }
    
    /// Layout children using flexbox
    fn layout_flex_children(&self, parent: &mut LayoutBox, containing_block: Dimensions) {
        let flex_direction = parent.styles.flex_direction.clone().unwrap_or(FlexDirection::Row);
        let justify_content = parent.styles.justify_content.clone().unwrap_or(JustifyContent::FlexStart);
        let align_items = parent.styles.align_items.clone().unwrap_or(AlignItems::Stretch);
        let flex_wrap = parent.styles.flex_wrap.clone().unwrap_or(FlexWrap::Nowrap);
        
        // First pass: layout all children
        let mut children = Vec::new();
        for child_node in parent.node.children.borrow().iter() {
            let child_layout = self.layout_element(child_node, containing_block);
            children.push(child_layout);
        }
        
        // Calculate available space
        let available_width = parent.content.width;
        let available_height = parent.content.height;
        
        // Apply flexbox layout
        match flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.layout_flex_row(&mut children, available_width, available_height, 
                                   justify_content, align_items, flex_wrap);
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.layout_flex_column(&mut children, available_width, available_height, 
                                      justify_content, align_items, flex_wrap);
            }
        }
        
        // Update parent dimensions
        if !children.is_empty() {
            let max_x = children.iter().map(|c| c.content.x + c.content.width).fold(0.0, f32::max);
            let max_y = children.iter().map(|c| c.content.y + c.content.height).fold(0.0, f32::max);
            parent.content.width = max_x.max(parent.content.width);
            parent.content.height = max_y.max(parent.content.height);
        }
        
        parent.children = children;
    }
    
    /// Layout flex items in a row
    fn layout_flex_row(&self, children: &mut Vec<LayoutBox>, available_width: f32, available_height: f32,
                      justify_content: JustifyContent, align_items: AlignItems, flex_wrap: FlexWrap) {
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut line_height = 0.0;
        
        for child in children.iter_mut() {
            // Check if we need to wrap
            if flex_wrap == FlexWrap::Wrap && current_x + child.content.width > available_width && current_x > 0.0 {
                current_x = 0.0;
                current_y += line_height;
                line_height = 0.0;
            }
            
            // Position the child
            child.content.x = current_x;
            child.content.y = current_y;
            
            // Align items vertically
            match align_items {
                AlignItems::FlexStart => {
                    // Already positioned at top
                }
                AlignItems::FlexEnd => {
                    child.content.y = current_y + available_height - child.content.height;
                }
                AlignItems::Center => {
                    child.content.y = current_y + (available_height - child.content.height) / 2.0;
                }
                AlignItems::Stretch => {
                    child.content.height = available_height;
                }
                AlignItems::Baseline => {
                    // For now, treat as flex-start
                }
            }
            
            // Update position for next child
            current_x += child.content.width;
            line_height = line_height.max(child.content.height);
        }
        
        // Apply justify-content
        if !children.is_empty() {
            let total_width = children.iter().map(|c| c.content.width).sum::<f32>();
            let extra_space = available_width - total_width;
            
            match justify_content {
                JustifyContent::FlexStart => {
                    // Already positioned correctly
                }
                JustifyContent::FlexEnd => {
                    let offset = extra_space;
                    for child in children.iter_mut() {
                        child.content.x += offset;
                    }
                }
                JustifyContent::Center => {
                    let offset = extra_space / 2.0;
                    for child in children.iter_mut() {
                        child.content.x += offset;
                    }
                }
                JustifyContent::SpaceBetween => {
                    if children.len() > 1 {
                        let gap = extra_space / (children.len() - 1) as f32;
                        for (i, child) in children.iter_mut().enumerate() {
                            if i > 0 {
                                child.content.x += gap * i as f32;
                            }
                        }
                    }
                }
                JustifyContent::SpaceAround => {
                    let gap = extra_space / children.len() as f32;
                    for (i, child) in children.iter_mut().enumerate() {
                        child.content.x += gap * (i as f32 + 0.5);
                    }
                }
                JustifyContent::SpaceEvenly => {
                    let gap = extra_space / (children.len() + 1) as f32;
                    for (i, child) in children.iter_mut().enumerate() {
                        child.content.x += gap * (i as f32 + 1.0);
                    }
                }
            }
        }
    }
    
    /// Layout flex items in a column
    fn layout_flex_column(&self, children: &mut Vec<LayoutBox>, available_width: f32, available_height: f32,
                         justify_content: JustifyContent, align_items: AlignItems, flex_wrap: FlexWrap) {
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut line_width = 0.0;
        
        for child in children.iter_mut() {
            // Check if we need to wrap
            if flex_wrap == FlexWrap::Wrap && current_y + child.content.height > available_height && current_y > 0.0 {
                current_y = 0.0;
                current_x += line_width;
                line_width = 0.0;
            }
            
            // Position the child
            child.content.x = current_x;
            child.content.y = current_y;
            
            // Align items horizontally
            match align_items {
                AlignItems::FlexStart => {
                    // Already positioned at left
                }
                AlignItems::FlexEnd => {
                    child.content.x = current_x + available_width - child.content.width;
                }
                AlignItems::Center => {
                    child.content.x = current_x + (available_width - child.content.width) / 2.0;
                }
                AlignItems::Stretch => {
                    child.content.width = available_width;
                }
                AlignItems::Baseline => {
                    // For now, treat as flex-start
                }
            }
            
            // Update position for next child
            current_y += child.content.height;
            line_width = line_width.max(child.content.width);
        }
        
        // Apply justify-content
        if !children.is_empty() {
            let total_height = children.iter().map(|c| c.content.height).sum::<f32>();
            let extra_space = available_height - total_height;
            
            match justify_content {
                JustifyContent::FlexStart => {
                    // Already positioned correctly
                }
                JustifyContent::FlexEnd => {
                    let offset = extra_space;
                    for child in children.iter_mut() {
                        child.content.y += offset;
                    }
                }
                JustifyContent::Center => {
                    let offset = extra_space / 2.0;
                    for child in children.iter_mut() {
                        child.content.y += offset;
                    }
                }
                JustifyContent::SpaceBetween => {
                    if children.len() > 1 {
                        let gap = extra_space / (children.len() - 1) as f32;
                        for (i, child) in children.iter_mut().enumerate() {
                            if i > 0 {
                                child.content.y += gap * i as f32;
                            }
                        }
                    }
                }
                JustifyContent::SpaceAround => {
                    let gap = extra_space / children.len() as f32;
                    for (i, child) in children.iter_mut().enumerate() {
                        child.content.y += gap * (i as f32 + 0.5);
                    }
                }
                JustifyContent::SpaceEvenly => {
                    let gap = extra_space / (children.len() + 1) as f32;
                    for (i, child) in children.iter_mut().enumerate() {
                        child.content.y += gap * (i as f32 + 1.0);
                    }
                }
            }
        }
    }
    
    /// Layout children using CSS Grid
    fn layout_grid_children(&self, parent: &mut LayoutBox, containing_block: Dimensions) {
        let grid_template_columns = parent.styles.grid_template_columns.clone().unwrap_or_else(|| {
            vec![GridTrack::Fractional(1.0)]
        });
        let grid_template_rows = parent.styles.grid_template_rows.clone().unwrap_or_else(|| {
            vec![GridTrack::Fractional(1.0)]
        });
        let grid_gap = parent.styles.grid_gap.unwrap_or(0.0);
        
        // Calculate grid dimensions
        let (column_widths, row_heights) = self.calculate_grid_tracks(
            &grid_template_columns, &grid_template_rows, 
            parent.content.width, parent.content.height, grid_gap
        );
        
        // First pass: layout all children
        let mut children = Vec::new();
        for child_node in parent.node.children.borrow().iter() {
            let child_layout = self.layout_element(child_node, containing_block);
            children.push(child_layout);
        }
        
        // Position children in grid
        let mut child_index = 0;
        for (row_idx, &row_height) in row_heights.iter().enumerate() {
            for (col_idx, &col_width) in column_widths.iter().enumerate() {
                if child_index < children.len() {
                    let child = &mut children[child_index];
                    
                    // Calculate position
                    let x = column_widths[..col_idx].iter().sum::<f32>() + grid_gap * col_idx as f32;
                    let y = row_heights[..row_idx].iter().sum::<f32>() + grid_gap * row_idx as f32;
                    
                    child.content.x = x;
                    child.content.y = y;
                    child.content.width = col_width;
                    child.content.height = row_height;
                    
                    child_index += 1;
                }
            }
        }
        
        // Update parent dimensions
        if !children.is_empty() {
            let total_width = column_widths.iter().sum::<f32>() + grid_gap * (column_widths.len() - 1) as f32;
            let total_height = row_heights.iter().sum::<f32>() + grid_gap * (row_heights.len() - 1) as f32;
            parent.content.width = total_width.max(parent.content.width);
            parent.content.height = total_height.max(parent.content.height);
        }
        
        parent.children = children;
    }
    
    /// Calculate grid track sizes
    fn calculate_grid_tracks(&self, columns: &[GridTrack], rows: &[GridTrack], 
                           available_width: f32, available_height: f32, gap: f32) -> (Vec<f32>, Vec<f32>) {
        // Calculate column widths
        let mut column_widths = Vec::new();
        let mut fixed_width = 0.0;
        let mut fractional_columns = 0;
        
        for track in columns {
            match track {
                GridTrack::Fixed(size) => {
                    column_widths.push(*size);
                    fixed_width += *size;
                }
                GridTrack::Fractional(_fr) => {
                    column_widths.push(0.0); // Placeholder
                    fractional_columns += 1;
                }
                GridTrack::Auto => {
                    column_widths.push(100.0); // Default auto size
                    fixed_width += 100.0;
                }
                GridTrack::MinContent => {
                    column_widths.push(50.0); // Default min-content size
                    fixed_width += 50.0;
                }
                GridTrack::MaxContent => {
                    column_widths.push(200.0); // Default max-content size
                    fixed_width += 200.0;
                }
            }
        }
        
        // Distribute remaining space among fractional columns
        if fractional_columns > 0 {
            let remaining_width = available_width - fixed_width - gap * (columns.len() - 1) as f32;
            let fractional_unit = remaining_width / fractional_columns as f32;
            
            for (i, track) in columns.iter().enumerate() {
                if let GridTrack::Fractional(fr) = track {
                    column_widths[i] = fractional_unit * fr;
                }
            }
        }
        
        // Calculate row heights (similar logic)
        let mut row_heights = Vec::new();
        let mut fixed_height = 0.0;
        let mut fractional_rows = 0;
        
        for track in rows {
            match track {
                GridTrack::Fixed(size) => {
                    row_heights.push(*size);
                    fixed_height += *size;
                }
                GridTrack::Fractional(_fr) => {
                    row_heights.push(0.0); // Placeholder
                    fractional_rows += 1;
                }
                GridTrack::Auto => {
                    row_heights.push(100.0); // Default auto size
                    fixed_height += 100.0;
                }
                GridTrack::MinContent => {
                    row_heights.push(50.0); // Default min-content size
                    fixed_height += 50.0;
                }
                GridTrack::MaxContent => {
                    row_heights.push(200.0); // Default max-content size
                    fixed_height += 200.0;
                }
            }
        }
        
        // Distribute remaining space among fractional rows
        if fractional_rows > 0 {
            let remaining_height = available_height - fixed_height - gap * (rows.len() - 1) as f32;
            let fractional_unit = remaining_height / fractional_rows as f32;
            
            for (i, track) in rows.iter().enumerate() {
                if let GridTrack::Fractional(fr) = track {
                    row_heights[i] = fractional_unit * fr;
                }
            }
        }
        
        (column_widths, row_heights)
    }
    
    /// Update animations for a layout tree
    pub fn update_animations(&self, layout_box: &mut LayoutBox, current_time: Instant) {
        self.update_element_animation(layout_box, current_time);
        
        // Recursively update children
        for child in &mut layout_box.children {
            self.update_animations(child, current_time);
        }
    }
    
    /// Update animation for a single element
    fn update_element_animation(&self, layout_box: &mut LayoutBox, current_time: Instant) {
        let styles = &layout_box.styles;
        
        // Check if element has animations
        if styles.animation_name.is_some() && styles.animation_duration.is_some() {
            let duration = styles.animation_duration.unwrap();
            let delay = styles.animation_delay.unwrap_or(0.0);
            let iteration_count = styles.animation_iteration_count.unwrap_or(1.0);
            let play_state = styles.animation_play_state.clone().unwrap_or(AnimationPlayState::Running);
            
            // Initialize animation if not started
            if !layout_box.animation_state.is_running && play_state == AnimationPlayState::Running {
                layout_box.animation_state.is_running = true;
                layout_box.animation_state.start_time = Some(current_time + Duration::from_secs_f32(delay));
                layout_box.animation_state.progress = 0.0;
                layout_box.animation_state.iteration_count = 0.0;
            }
            
            // Update animation progress
            if let Some(start_time) = layout_box.animation_state.start_time {
                if play_state == AnimationPlayState::Running && !layout_box.animation_state.is_paused {
                    let elapsed = current_time.duration_since(start_time).as_secs_f32();
                    let total_duration = duration * iteration_count;
                    
                    if elapsed < total_duration {
                        layout_box.animation_state.progress = (elapsed / duration) % 1.0;
                        layout_box.animation_state.iteration_count = (elapsed / duration).floor();
                        
                        // Apply animation transformations
                        self.apply_animation_transforms(layout_box);
                    } else {
                        // Animation finished
                        layout_box.animation_state.is_running = false;
                        layout_box.animation_state.progress = 1.0;
                        layout_box.animation_state.iteration_count = iteration_count;
                    }
                }
            }
        }
    }
    
    /// Apply animation transformations to an element
    fn apply_animation_transforms(&self, layout_box: &mut LayoutBox) {
        let progress = layout_box.animation_state.progress;
        let timing_function = &layout_box.styles.transition_timing_function;
        
        // Apply easing function
        let eased_progress = match timing_function {
            Some(TimingFunction::Ease) => self.ease_in_out_cubic(progress),
            Some(TimingFunction::Linear) => progress,
            Some(TimingFunction::EaseIn) => self.ease_in_cubic(progress),
            Some(TimingFunction::EaseOut) => self.ease_out_cubic(progress),
            Some(TimingFunction::EaseInOut) => self.ease_in_out_cubic(progress),
            Some(TimingFunction::CubicBezier(c1, c2, c3, c4)) => self.cubic_bezier(progress, *c1, *c2, *c3, *c4),
            Some(TimingFunction::Steps(steps)) => self.step_function(progress, *steps),
            None => progress,
        };
        
        // Apply simple transform animations (scale, rotate, translate)
        // This is a simplified implementation - real CSS animations would be more complex
        let scale_factor = 1.0 + (eased_progress * 0.2); // Scale from 1.0 to 1.2
        let _rotation = eased_progress * 360.0; // Rotate 360 degrees
        
        // Apply transformations to content dimensions
        layout_box.content.width *= scale_factor;
        layout_box.content.height *= scale_factor;
        
        // Note: In a real implementation, you would apply these transformations
        // during rendering rather than modifying the layout dimensions
    }
    
    /// Easing functions for animations
    fn ease_in_cubic(&self, t: f32) -> f32 {
        t * t * t
    }
    
    fn ease_out_cubic(&self, t: f32) -> f32 {
        let f = t - 1.0;
        f * f * f + 1.0
    }
    
    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let f = 2.0 * t - 2.0;
            0.5 * f * f * f + 1.0
        }
    }
    
    fn cubic_bezier(&self, t: f32, c1: f32, _c2: f32, c3: f32, _c4: f32) -> f32 {
        // Simplified cubic bezier implementation
        // Real implementation would use more sophisticated math
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let _mt3 = mt2 * mt;
        
        3.0 * mt2 * t * c1 + 3.0 * mt * t2 * c3 + t3
    }
    
    fn step_function(&self, t: f32, steps: i32) -> f32 {
        (t * steps as f32).floor() / steps as f32
    }
    
    /// Calculate the total box dimensions including padding, border, and margin
    fn calculate_box_dimensions(&self, layout_box: &mut LayoutBox) {
        let styles = &layout_box.styles;
        
        // Calculate padding dimensions
        layout_box.padding = Dimensions::new(
            layout_box.content.x,
            layout_box.content.y,
            layout_box.content.width + styles.padding.left + styles.padding.right,
            layout_box.content.height + styles.padding.top + styles.padding.bottom,
        );
        
        // Calculate border dimensions
        layout_box.border = Dimensions::new(
            layout_box.padding.x,
            layout_box.padding.y,
            layout_box.padding.width + styles.border.left + styles.border.right,
            layout_box.padding.height + styles.border.top + styles.border.bottom,
        );
        
        // Calculate margin dimensions
        layout_box.margin = Dimensions::new(
            layout_box.border.x,
            layout_box.border.y,
            layout_box.border.width + styles.margin.left + styles.margin.right,
            layout_box.border.height + styles.margin.top + styles.margin.bottom,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_parser::parse_css;

    #[test]
    fn test_computed_styles() {
        let styles = ComputedStyles {
            display: DisplayType::Block,
            width: Some(100.0),
            height: Some(50.0),
            margin: BoxSides::new(10.0),
            border: BoxSides::new(1.0),
            padding: BoxSides::new(5.0),
            background_color: Some("red".to_string()),
            color: Some("white".to_string()),
            font_size: Some(16.0),
            font_family: Some("Arial".to_string()),
            font_weight: Some("bold".to_string()),
            text_align: Some("center".to_string()),
            // Flexbox properties
            flex_direction: None,
            flex_wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            align_self: None,
            // Grid properties
            grid_template_columns: None,
            grid_template_rows: None,
            grid_gap: None,
            grid_column_gap: None,
            grid_row_gap: None,
            grid_column_start: None,
            grid_column_end: None,
            grid_row_start: None,
            grid_row_end: None,
            // Animation properties
            transition_duration: None,
            transition_delay: None,
            transition_timing_function: None,
            animation_name: None,
            animation_duration: None,
            animation_delay: None,
            animation_iteration_count: None,
            animation_direction: None,
            animation_fill_mode: None,
            animation_play_state: None,
        };
        
        assert_eq!(styles.display, DisplayType::Block);
        assert_eq!(styles.width, Some(100.0));
        assert_eq!(styles.background_color, Some("red".to_string()));
    }

    #[test]
    fn test_box_sides() {
        let box_sides = BoxSides::new(10.0);
        assert_eq!(box_sides.top, 10.0);
        assert_eq!(box_sides.right, 10.0);
        assert_eq!(box_sides.bottom, 10.0);
        assert_eq!(box_sides.left, 10.0);
        
        let box_sides_vh = BoxSides::new_vertical_horizontal(5.0, 10.0);
        assert_eq!(box_sides_vh.top, 5.0);
        assert_eq!(box_sides_vh.right, 10.0);
        assert_eq!(box_sides_vh.bottom, 5.0);
        assert_eq!(box_sides_vh.left, 10.0);
    }

    #[test]
    fn test_dimensions() {
        let dims = Dimensions::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(dims.x, 10.0);
        assert_eq!(dims.y, 20.0);
        assert_eq!(dims.width, 100.0);
        assert_eq!(dims.height, 50.0);
        assert_eq!(dims.right(), 110.0);
        assert_eq!(dims.bottom(), 70.0);
    }

    #[test]
    fn test_style_matcher() {
        let css = "h1 { color: red; font-size: 24px; }";
        let stylesheet = parse_css(css);
        let matcher = StyleMatcher::new(stylesheet);
        
        // Create a test element
        let doc = Document::new();
        let h1 = doc.create_element("h1");
        
        let styles = matcher.compute_styles(&h1);
        assert_eq!(styles.color, Some("red".to_string()));
        assert_eq!(styles.font_size, Some(24.0));
    }
}

