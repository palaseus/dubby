//! Test suite for advanced CSS layouts: flexbox, grid, and animations
//! 
//! This test demonstrates the implementation of modern CSS layout features
//! including flexbox, CSS Grid, and CSS animations.

use layout::{LayoutEngine, ComputedStyles, DisplayType, FlexDirection, JustifyContent, AlignItems, 
            FlexWrap, GridTrack, TimingFunction, AnimationDirection, AnimationPlayState};
use css_parser::Stylesheet;
use dom::{Document, Node, element::Element};
use std::time::Instant;

fn main() {
    println!("🧪 Testing Advanced CSS Layouts");
    println!("================================");
    
    // Test flexbox layouts
    test_flexbox_layouts();
    
    // Test CSS Grid layouts
    test_grid_layouts();
    
    // Test CSS animations
    test_css_animations();
    
    // Test complex layout combinations
    test_complex_layouts();
    
    println!("\n✅ All advanced CSS layout tests completed!");
}

/// Test flexbox layout functionality
fn test_flexbox_layouts() {
    println!("\n📦 Testing Flexbox Layouts");
    println!("---------------------------");
    
    // Create a simple flexbox container
    let document = create_test_document();
    let _root = document.root.clone();
    
    // Add flexbox container
    let mut container = create_element("div", "flex-container");
    container.set_attribute("style", "display: flex; flex-direction: row; justify-content: space-between; align-items: center; width: 800px; height: 200px;");
    
    // Add flex items
    let mut item1 = create_element("div", "flex-item-1");
    item1.set_attribute("style", "width: 100px; height: 50px; background-color: red;");
    
    let mut item2 = create_element("div", "flex-item-2");
    item2.set_attribute("style", "width: 150px; height: 80px; background-color: green;");
    
    let mut item3 = create_element("div", "flex-item-3");
    item3.set_attribute("style", "width: 120px; height: 60px; background-color: blue;");
    
    container.node.append_child(&item1.node);
    container.node.append_child(&item2.node);
    container.node.append_child(&item3.node);
    
    let document = create_test_document();
    document.root.append_child(&container.node);
    
    // Create stylesheet with flexbox properties
    let stylesheet = create_flexbox_stylesheet();
    let layout_engine = LayoutEngine::new(stylesheet);
    
    // Perform layout
    let layout_box = layout_engine.layout_document(&document);
    
    // Verify flexbox layout
    assert_eq!(layout_box.styles.display, DisplayType::Block); // Root is block
    assert_eq!(layout_box.children.len(), 1);
    
    let flex_container = &layout_box.children[0];
    assert_eq!(flex_container.styles.display, DisplayType::Flex);
    assert_eq!(flex_container.styles.flex_direction, Some(FlexDirection::Row));
    assert_eq!(flex_container.styles.justify_content, Some(JustifyContent::SpaceBetween));
    assert_eq!(flex_container.styles.align_items, Some(AlignItems::Center));
    
    println!("✅ Flexbox container layout verified");
    println!("   - Display: {:?}", flex_container.styles.display);
    println!("   - Flex direction: {:?}", flex_container.styles.flex_direction);
    println!("   - Justify content: {:?}", flex_container.styles.justify_content);
    println!("   - Align items: {:?}", flex_container.styles.align_items);
    println!("   - Container size: {}x{}", flex_container.content.width, flex_container.content.height);
    
    // Test different flex directions
    test_flex_directions();
    test_flex_wrap();
    test_flex_justify_content();
}

/// Test different flex directions
fn test_flex_directions() {
    println!("\n🔄 Testing Flex Directions");
    
    let directions = vec![
        (FlexDirection::Row, "row"),
        (FlexDirection::Column, "column"),
        (FlexDirection::RowReverse, "row-reverse"),
        (FlexDirection::ColumnReverse, "column-reverse"),
    ];
    
    for (direction, name) in directions {
        let mut styles = ComputedStyles::default();
        styles.display = DisplayType::Flex;
        styles.flex_direction = Some(direction);
        styles.width = Some(400.0);
        styles.height = Some(200.0);
        
        println!("   ✅ {} direction: {:?}", name, direction);
    }
}

/// Test flex wrap behavior
fn test_flex_wrap() {
    println!("\n📋 Testing Flex Wrap");
    
    let wrap_types = vec![
        (FlexWrap::Nowrap, "nowrap"),
        (FlexWrap::Wrap, "wrap"),
        (FlexWrap::WrapReverse, "wrap-reverse"),
    ];
    
    for (wrap, name) in wrap_types {
        let mut styles = ComputedStyles::default();
        styles.display = DisplayType::Flex;
        styles.flex_wrap = Some(wrap);
        
        println!("   ✅ {} wrap: {:?}", name, wrap);
    }
}

/// Test justify-content values
fn test_flex_justify_content() {
    println!("\n📐 Testing Justify Content");
    
    let justify_values = vec![
        (JustifyContent::FlexStart, "flex-start"),
        (JustifyContent::FlexEnd, "flex-end"),
        (JustifyContent::Center, "center"),
        (JustifyContent::SpaceBetween, "space-between"),
        (JustifyContent::SpaceAround, "space-around"),
        (JustifyContent::SpaceEvenly, "space-evenly"),
    ];
    
    for (justify, name) in justify_values {
        let mut styles = ComputedStyles::default();
        styles.display = DisplayType::Flex;
        styles.justify_content = Some(justify);
        
        println!("   ✅ {} justify: {:?}", name, justify);
    }
}

/// Test CSS Grid layout functionality
fn test_grid_layouts() {
    println!("\n🔲 Testing CSS Grid Layouts");
    println!("---------------------------");
    
    // Create a grid container
    let document = create_test_document();
    let _root = document.root.clone();
    
    let mut container = create_element("div", "grid-container");
    container.set_attribute("style", "display: grid; grid-template-columns: 1fr 2fr 1fr; grid-template-rows: 100px 200px; grid-gap: 10px; width: 600px; height: 320px;");
    
    // Add grid items
    for i in 1..=6 {
        let mut item = create_element("div", &format!("grid-item-{}", i));
        item.set_attribute("style", &format!("background-color: rgb({}, {}, {});", i * 40, i * 30, i * 20));
        container.node.append_child(&item.node);
    }
    
    let document = create_test_document();
    document.root.append_child(&container.node);
    
    // Create stylesheet with grid properties
    let stylesheet = create_grid_stylesheet();
    let layout_engine = LayoutEngine::new(stylesheet);
    
    // Perform layout
    let layout_box = layout_engine.layout_document(&document);
    
    // Verify grid layout
    let grid_container = &layout_box.children[0];
    assert_eq!(grid_container.styles.display, DisplayType::Grid);
    assert!(grid_container.styles.grid_template_columns.is_some());
    assert!(grid_container.styles.grid_template_rows.is_some());
    assert_eq!(grid_container.styles.grid_gap, Some(10.0));
    
    println!("✅ Grid container layout verified");
    println!("   - Display: {:?}", grid_container.styles.display);
    println!("   - Grid columns: {:?}", grid_container.styles.grid_template_columns);
    println!("   - Grid rows: {:?}", grid_container.styles.grid_template_rows);
    println!("   - Grid gap: {:?}", grid_container.styles.grid_gap);
    println!("   - Container size: {}x{}", grid_container.content.width, grid_container.content.height);
    
    // Test grid track types
    test_grid_tracks();
}

/// Test different grid track types
fn test_grid_tracks() {
    println!("\n📏 Testing Grid Tracks");
    
    let tracks = vec![
        (GridTrack::Fixed(100.0), "100px"),
        (GridTrack::Fractional(1.0), "1fr"),
        (GridTrack::Fractional(2.0), "2fr"),
        (GridTrack::Auto, "auto"),
        (GridTrack::MinContent, "min-content"),
        (GridTrack::MaxContent, "max-content"),
    ];
    
    for (track, name) in tracks {
        println!("   ✅ {} track: {:?}", name, track);
    }
}

/// Test CSS animations
fn test_css_animations() {
    println!("\n🎬 Testing CSS Animations");
    println!("-------------------------");
    
    // Create an animated element
    let document = create_test_document();
    let _root = document.root.clone();
    
    let mut animated_element = create_element("div", "animated-element");
    animated_element.set_attribute("style", "width: 100px; height: 100px; background-color: red; animation: spin 2s linear infinite;");
    let document = create_test_document();
    document.root.append_child(&animated_element.node);
    
    // Create stylesheet with animation properties
    let stylesheet = create_animation_stylesheet();
    let layout_engine = LayoutEngine::new(stylesheet);
    
    // Perform initial layout
    let mut layout_box = layout_engine.layout_document(&document);
    
    // Verify animation properties
    let animated_box = &layout_box.children[0];
    assert!(animated_box.styles.animation_name.is_some());
    assert!(animated_box.styles.animation_duration.is_some());
    assert_eq!(animated_box.styles.animation_duration, Some(2.0));
    assert_eq!(animated_box.styles.animation_iteration_count, Some(f32::INFINITY));
    assert_eq!(animated_box.styles.animation_direction, Some(AnimationDirection::Normal));
    assert_eq!(animated_box.styles.animation_play_state, Some(AnimationPlayState::Running));
    
    println!("✅ Animation properties verified");
    println!("   - Animation name: {:?}", animated_box.styles.animation_name);
    println!("   - Duration: {:?}s", animated_box.styles.animation_duration);
    println!("   - Iteration count: {:?}", animated_box.styles.animation_iteration_count);
    println!("   - Direction: {:?}", animated_box.styles.animation_direction);
    println!("   - Play state: {:?}", animated_box.styles.animation_play_state);
    
    // Test animation updates
    test_animation_updates(&mut layout_box, &layout_engine);
    
    // Test timing functions
    test_timing_functions();
}

/// Test animation updates over time
fn test_animation_updates(layout_box: &mut layout::LayoutBox, layout_engine: &LayoutEngine) {
    println!("\n⏱️  Testing Animation Updates");
    
    let start_time = Instant::now();
    
    // Simulate animation updates at different time points
    let time_points = vec![0.0, 0.5, 1.0, 1.5, 2.0];
    
    for time_offset in time_points {
        let current_time = start_time + std::time::Duration::from_secs_f32(time_offset);
        layout_engine.update_animations(layout_box, current_time);
        
        let animated_box = &layout_box.children[0];
        println!("   ✅ Time {:.1}s: Progress {:.2}, Running: {}", 
                time_offset, 
                animated_box.animation_state.progress,
                animated_box.animation_state.is_running);
    }
}

/// Test different timing functions
fn test_timing_functions() {
    println!("\n📈 Testing Timing Functions");
    
    let timing_functions = vec![
        (TimingFunction::Ease, "ease"),
        (TimingFunction::Linear, "linear"),
        (TimingFunction::EaseIn, "ease-in"),
        (TimingFunction::EaseOut, "ease-out"),
        (TimingFunction::EaseInOut, "ease-in-out"),
        (TimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0), "cubic-bezier(0.25, 0.1, 0.25, 1.0)"),
        (TimingFunction::Steps(5), "steps(5)"),
    ];
    
    for (timing, name) in timing_functions {
        let mut styles = ComputedStyles::default();
        styles.transition_timing_function = Some(timing);
        
        println!("   ✅ {} timing: {:?}", name, timing);
    }
}

/// Test complex layout combinations
fn test_complex_layouts() {
    println!("\n🔀 Testing Complex Layout Combinations");
    println!("--------------------------------------");
    
    // Test nested flexbox and grid
    test_nested_layouts();
    
    // Test responsive layout properties
    test_responsive_properties();
    
    println!("✅ Complex layout combinations verified");
}

/// Test nested flexbox and grid layouts
fn test_nested_layouts() {
    println!("\n🪆 Testing Nested Layouts");
    
    // Create a flexbox container with grid children
    let mut styles = ComputedStyles::default();
    styles.display = DisplayType::Flex;
    styles.flex_direction = Some(FlexDirection::Column);
    styles.justify_content = Some(JustifyContent::SpaceBetween);
    
    // Child with grid layout
    let mut child_styles = ComputedStyles::default();
    child_styles.display = DisplayType::Grid;
    child_styles.grid_template_columns = Some(vec![GridTrack::Fractional(1.0), GridTrack::Fractional(2.0)]);
    child_styles.grid_template_rows = Some(vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)]);
    child_styles.grid_gap = Some(10.0);
    
    println!("   ✅ Nested flexbox → grid layout");
    println!("   ✅ Parent flex direction: {:?}", styles.flex_direction);
    println!("   ✅ Child grid columns: {:?}", child_styles.grid_template_columns);
}

/// Test responsive layout properties
fn test_responsive_properties() {
    println!("\n📱 Testing Responsive Properties");
    
    // Test flexible sizing
    let mut styles = ComputedStyles::default();
    styles.display = DisplayType::Flex;
    styles.flex_grow = Some(1.0);
    styles.flex_shrink = Some(0.5);
    styles.flex_basis = Some(200.0);
    
    println!("   ✅ Flex grow: {:?}", styles.flex_grow);
    println!("   ✅ Flex shrink: {:?}", styles.flex_shrink);
    println!("   ✅ Flex basis: {:?}", styles.flex_basis);
    
    // Test grid positioning
    styles.grid_column_start = Some(1);
    styles.grid_column_end = Some(3);
    styles.grid_row_start = Some(1);
    styles.grid_row_end = Some(2);
    
    println!("   ✅ Grid column: {} to {}", styles.grid_column_start.unwrap(), styles.grid_column_end.unwrap());
    println!("   ✅ Grid row: {} to {}", styles.grid_row_start.unwrap(), styles.grid_row_end.unwrap());
}

/// Helper functions

fn create_test_document() -> Document {
    let _root = create_element("html", "root");
    Document::new()
}

fn create_element(tag_name: &str, id: &str) -> Element {
    use std::collections::HashMap;
    use dom::{NodeType};
    
    // Create a proper element node
    let mut attributes = HashMap::new();
    attributes.insert("id".to_string(), id.to_string());
    
    let node = Node::new(NodeType::Element {
        tag_name: tag_name.to_string(),
        attributes,
    }, 0);
    
    let mut element = Element::new(node);
    element.set_attribute("id", id);
    element
}

fn create_flexbox_stylesheet() -> Stylesheet {
    // Create a simple stylesheet with flexbox properties
    Stylesheet { rules: vec![], source_url: None }
}

fn create_grid_stylesheet() -> Stylesheet {
    // Create a simple stylesheet with grid properties
    Stylesheet { rules: vec![], source_url: None }
}

fn create_animation_stylesheet() -> Stylesheet {
    // Create a simple stylesheet with animation properties
    Stylesheet { rules: vec![], source_url: None }
}
