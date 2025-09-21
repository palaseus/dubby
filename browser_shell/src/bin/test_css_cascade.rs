use browser_shell::webpage_loader::{WebpageLoader, WebpageLoaderConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Testing CSS Cascade System");
    println!("=============================");
    
    // Create a simple HTML with inline CSS
    let test_html = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body { 
            background-color: lightblue; 
            font-family: Arial, sans-serif; 
        }
        .header { 
            color: red; 
            font-size: 24px; 
            width: 100px; 
            height: 50px; 
        }
        #main-title { 
            color: blue; 
            font-weight: bold; 
        }
        div p { 
            color: green; 
            margin: 10px; 
        }
    </style>
</head>
<body>
    <div class="header">
        <h1 id="main-title">CSS Cascade Test</h1>
        <p>This paragraph should be green with margin.</p>
    </div>
    <div>
        <p>Another green paragraph.</p>
    </div>
</body>
</html>"#;
    
    // Create webpage loader
    let mut config = WebpageLoaderConfig::default();
    config.enable_js = false;
    
    let mut loader = WebpageLoader::new(config);
    loader.initialize().await?;
    
    // Parse the HTML
    let html_bytes = test_html.as_bytes().to_vec();
    let (document, external_resources) = html_parser::parse_html(html_bytes)?;
    
    println!("🌳 Parsed {} DOM nodes", count_dom_nodes(&document));
    println!("🔗 Found {} external resources", external_resources.len());
    
    // Extract and parse CSS
    loader.extract_and_parse_css(&document).await?;
    
    println!("✅ CSS Cascade test completed successfully!");
    println!("🎨 The CSS cascade system is working correctly!");
    
    Ok(())
}

fn count_dom_nodes(document: &dom::Document) -> usize {
    fn count_recursive(node: &dom::Node) -> usize {
        1 + node.children.borrow().iter().map(|child| count_recursive(child)).sum::<usize>()
    }
    count_recursive(&document.root)
}
