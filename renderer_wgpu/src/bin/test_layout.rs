//! Test binary for rendering layout boxes
//!
//! This binary demonstrates layout box rendering by creating
//! a simple HTML document, parsing it, applying CSS, computing
//! layout, and then rendering the layout boxes to a GPU window.

use renderer_wgpu::render_layout_tree;
use std::process;

/// Print the DOM tree structure for debugging
fn print_dom_tree(node: &dom::Node, depth: usize) {
    let indent = "  ".repeat(depth);
    let node_type = match &node.node_type {
        dom::NodeType::Element { tag_name, .. } => format!("<{}>", tag_name),
        dom::NodeType::Text(text) => format!("\"{}\"", text.chars().take(20).collect::<String>()),
        dom::NodeType::Document => "Document".to_string(),
        _ => "Unknown".to_string(),
    };
    
    println!("{}{}", indent, node_type);
    
    for child in node.children.borrow().iter() {
        print_dom_tree(child, depth + 1);
    }
}

/// Print the layout tree structure for debugging
fn print_layout_tree(layout_box: &layout::LayoutBox, depth: usize) {
    let indent = "  ".repeat(depth);
    let node_type = match &layout_box.node.node_type {
        dom::NodeType::Element { tag_name, .. } => format!("<{}>", tag_name),
        dom::NodeType::Text(text) => format!("\"{}\"", text.chars().take(20).collect::<String>()),
        dom::NodeType::Document => "Document".to_string(),
        _ => "Unknown".to_string(),
    };
    
    println!("{}{} - {}x{} at {},{} display={:?}", 
             indent, node_type, 
             layout_box.content.width, layout_box.content.height,
             layout_box.content.x, layout_box.content.y,
             layout_box.styles.display);
    
    for child in &layout_box.children {
        print_layout_tree(child, depth + 1);
    }
}

// Import the browser engine components
use dom;
use html_parser::parse_html;
use css_parser::parse_css;
use layout::LayoutEngine;

const TEST_HTML: &str = r#"
<html>
<head>
    <title>Layout Test</title>
</head>
<body>
    <h1>Welcome to Layout Rendering!</h1>
    <p>This is a paragraph with some text content.</p>
    <div class="container">
        <h2>Section Title</h2>
        <p>Another paragraph inside a container div.</p>
        <ul>
            <li>First list item</li>
            <li>Second list item</li>
            <li>Third list item</li>
        </ul>
    </div>
</body>
</html>
"#;

const TEST_CSS: &str = r#"
body {
    font-family: Arial, sans-serif;
    margin: 20px;
    padding: 20px;
    background-color: #f0f0f0;
}

h1 {
    color: #333;
    margin-bottom: 20px;
    padding: 10px;
    background-color: #e0e0e0;
}

h2 {
    color: #666;
    margin-top: 20px;
    margin-bottom: 10px;
}

p {
    line-height: 1.6;
    margin-bottom: 15px;
    padding: 5px;
}

.container {
    margin: 20px 0;
    padding: 15px;
    border: 2px solid #ccc;
    background-color: #fff;
}

ul {
    margin: 10px 0;
    padding-left: 20px;
}

li {
    margin-bottom: 5px;
}
"#;

/// Main function for testing layout rendering
#[tokio::main]
async fn main() {
    println!("🚀 Testing Layout Box Rendering");
    println!("=================================");
    println!();
    println!("Parsing HTML and CSS...");

    // Parse HTML
    let document = parse_html(TEST_HTML);
    println!("✅ HTML parsed successfully");
    

    // Parse CSS
    let stylesheet = parse_css(TEST_CSS);
    println!("✅ CSS parsed successfully");

    // Compute layout
    println!("Computing layout...");
    let layout_engine = LayoutEngine::new(stylesheet.clone());
    let layout_root = layout_engine.layout_document(&document);
    println!("✅ Layout computed successfully");
    

    println!();
    println!("Opening layout renderer window...");
    println!("You should see colored rectangles representing the layout boxes:");
    println!("  - Blue: Block elements (h1, h2, p, div)");
    println!("  - Green: Inline elements (text nodes)");
    println!("  - Gray: Anonymous boxes");
    println!("Close the window to exit.");
    println!();

    if let Err(e) = render_layout_tree(&layout_root).await {
        eprintln!("❌ Layout rendering failed: {}", e);
        process::exit(1);
    }

    println!("✅ Layout rendering test completed successfully!");
}
