use js_integration::JsEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing JavaScript Engine Integration");
    println!("=======================================");

    // Create a new JavaScript engine
    let mut js_engine = JsEngine::new();
    println!("✅ JavaScript engine created successfully");

    // Test basic JavaScript execution
    println!("\n📝 Testing basic JavaScript execution...");
    let basic_js = r#"
        console.log("Hello from JavaScript!");
        console.log("2 + 2 =", 2 + 2);
        console.log("Math.PI =", Math.PI);
    "#;

    js_engine.execute_with_layout_update(basic_js)?;
    println!("✅ Basic JavaScript execution successful");

    // Test DOM API
    println!("\n🌐 Testing DOM API...");
    let dom_js = r#"
        console.log("Testing DOM API...");
        
        // Test document.getElementById
        let element = document.getElementById("test-element");
        console.log("Found element:", element);
        
        // Test document.createElement
        let newElement = document.createElement("div");
        console.log("Created element:", newElement);
        
        // Test element properties
        console.log("Element tagName:", newElement.tagName);
        console.log("Element innerText:", newElement.getInnerText());
        
        // Test element methods
        newElement.setInnerText("Hello from JavaScript!");
        console.log("Set innerText, new value:", newElement.getInnerText());
        
        // Test event listener
        newElement.addEventListener("click", function() {
            console.log("Button clicked!");
        });
        
        console.log("DOM API test completed!");
    "#;

    js_engine.execute_with_layout_update(dom_js)?;
    println!("✅ DOM API test successful");

    // Test error handling
    println!("\n⚠️  Testing error handling...");
    let error_js = r#"
        console.log("Testing error handling...");
        // This should cause a syntax error
        let invalid = ;
    "#;

    match js_engine.execute_with_layout_update(error_js) {
        Ok(_) => println!("❌ Expected error but got success"),
        Err(e) => println!("✅ Caught expected error: {}", e),
    }

    println!("\n🎉 JavaScript engine integration test completed!");
    println!("The engine is ready for DOM integration and script execution.");

    Ok(())
}
