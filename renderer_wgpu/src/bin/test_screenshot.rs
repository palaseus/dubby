use renderer_wgpu::render_layout_tree_offscreen;
use layout::LayoutEngine;
use html_parser::parse_html;
use css_parser::parse_css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖼️  Testing Screenshot Capture");
    println!("===============================");

    // Test HTML and CSS
    const TEST_HTML: &str = r#"
    <html>
    <head>
        <title>Screenshot Test</title>
    </head>
    <body>
        <h1>Layout Screenshot Test</h1>
        <p>This is a test of the screenshot functionality.</p>
        <div class="container">
            <h2>Nested Content</h2>
            <p>This content should be visible in the screenshot.</p>
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
        margin-bottom: 15px;
    }

    p {
        color: #444;
        line-height: 1.5;
        margin-bottom: 10px;
    }

    .container {
        border: 2px solid #ccc;
        padding: 15px;
        margin: 20px 0;
        background-color: #fff;
    }
    "#;

    // Parse HTML and CSS
    println!("Parsing HTML and CSS...");
    let document = parse_html(TEST_HTML);
    println!("✅ HTML parsed successfully");

    let stylesheet = parse_css(TEST_CSS);
    println!("✅ CSS parsed successfully");

    // Compute layout
    println!("Computing layout...");
    let layout_engine = LayoutEngine::new(stylesheet);
    let layout_root = layout_engine.layout_document(&document);
    println!("✅ Layout computed successfully");

    // Render to offscreen surface and capture screenshot
    println!("Rendering to offscreen surface...");
    let width = 800;
    let height = 600;
    let filename = "layout_screenshot.ppm";
    
    render_layout_tree_offscreen(&layout_root, width, height, filename).await?;
    println!("✅ Screenshot captured successfully");
    
    println!("\n🎉 Screenshot test completed!");
    println!("Screenshot saved as: {}", filename);
    println!("You can view it with: convert {} layout_screenshot.png", filename);

    Ok(())
}
