//! Test binary for the GPU renderer
//!
//! This binary demonstrates basic GPU rendering capabilities
//! by opening a window and rendering a simple colored rectangle.

use renderer_wgpu::run_gpu_test;
use std::process;

/// Main function for testing GPU rendering
#[tokio::main]
async fn main() {
    println!("🚀 Testing GPU Renderer");
    println!("=======================");
    println!();
    println!("Opening GPU renderer window...");
    println!("You should see a window with a red rectangle on a blue background.");
    println!("Close the window to exit.");
    println!();

    if let Err(e) = run_gpu_test().await {
        eprintln!("❌ GPU renderer test failed: {}", e);
        process::exit(1);
    }

    println!("✅ GPU renderer test completed successfully!");
}
