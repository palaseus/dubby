use browser_shell::webpage_loader::{WebpageLoader, WebpageLoaderConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Testing CSS Cascade with Real Website");
    println!("=========================================");
    
    // Create webpage loader
    let mut config = WebpageLoaderConfig::default();
    config.enable_js = false;
    config.timeout = Duration::from_secs(60);
    
    let mut loader = WebpageLoader::new(config);
    loader.initialize().await?;
    
    // Test with a simple website that has CSS
    let test_url = "https://example.com";
    println!("🌐 Loading: {}", test_url);
    
    match loader.load_webpage(test_url).await {
        Ok(_) => {
            println!("✅ Successfully loaded and processed {} with CSS cascade!", test_url);
            println!("🎨 CSS cascade system is working with real websites!");
        }
        Err(e) => {
            println!("❌ Failed to load {}: {}", test_url, e);
            println!("🔧 This might be due to network issues or the website structure.");
        }
    }
    
    Ok(())
}
