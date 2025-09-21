//! Simple test for GPU rendering
//! 
//! This test creates a simple HTML document, parses it, applies CSS,
//! computes layout, and renders it with GPU.

use html_parser::parse_html;
use css_parser::parse_css;
use layout::LayoutEngine;
use renderer_wgpu::render_layout_tree;

const SIMPLE_HTML: &str = r#"
<html>
<body>
    <h1>Hello World</h1>
    <p>This is a test paragraph.</p>
</body>
</html>
"#;

const SIMPLE_CSS: &str = r#"
body { margin: 20px; }
h1 { color: blue; font-size: 24px; }
p { color: green; font-size: 16px; }
"#;

#[tokio::main]
async fn main() {
    println!("🚀 Simple GPU Rendering Test");
    println!("=============================");
    
    // Parse HTML
    let document = parse_html(SIMPLE_HTML);
    println!("✅ HTML parsed");
    
    // Parse CSS
    let stylesheet = parse_css(SIMPLE_CSS);
    println!("✅ CSS parsed");
    
    // Compute layout
    let layout_engine = LayoutEngine::new(stylesheet);
    let layout_root = layout_engine.layout_document(&document);
    println!("✅ Layout computed");
    
    // Render with GPU
    println!("Opening GPU renderer...");
    if let Err(e) = render_layout_tree(&layout_root).await {
        eprintln!("❌ GPU rendering failed: {}", e);
    } else {
        println!("✅ GPU rendering completed");
    }
}
