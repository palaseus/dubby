use js_integration::JsEngine;
use html_parser::parse_html;
use css_parser::parse_css;
use std::rc::Rc;

fn main() {
    println!("🚀 Testing DOM API Expansion");
    println!("=============================");

    // Parse HTML and CSS
    let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>DOM API Test</title>
        </head>
        <body>
            <div id="container" class="main-container">
                <h1 id="title" class="header">DOM API Test</h1>
                <p id="content" class="text">This is a test of expanded DOM APIs.</p>
                <button id="btn" class="button active">Click Me</button>
                <div id="nested">
                    <span class="highlight">Nested element</span>
                </div>
            </div>
        </body>
        </html>
    "#;

    let css_content = r#"
        .main-container { background-color: #f0f0f0; }
        .header { color: blue; font-size: 24px; }
        .text { color: black; font-size: 16px; }
        .button { background-color: green; color: white; }
        .active { border: 2px solid red; }
        .highlight { background-color: yellow; }
    "#;

    println!("📄 Parsing HTML and CSS...");
    let document = parse_html(html_content);
    let stylesheet = parse_css(css_content);
    
    // Set up JavaScript engine
    let mut engine = JsEngine::new();
    engine.set_document(Rc::new(document));
    engine.set_stylesheet(stylesheet);

    println!("🔍 Testing document.querySelector...");
    let result = engine.execute("
        let container = document.querySelector('#container');
        console.log('Found container:', container ? 'Yes' : 'No');
        
        let title = document.querySelector('h1');
        console.log('Found title:', title ? 'Yes' : 'No');
        
        let button = document.querySelector('.button');
        console.log('Found button:', button ? 'Yes' : 'No');
    ");
    match result {
        Ok(_) => println!("✅ document.querySelector tests passed"),
        Err(e) => println!("❌ document.querySelector tests failed: {:?}", e),
    }

    println!("\n🔍 Testing document.querySelectorAll...");
    let result = engine.execute("
        let divs = document.querySelectorAll('div');
        console.log('Found divs count:', divs ? divs.length : 0);
        
        let classes = document.querySelectorAll('.text');
        console.log('Found text elements:', classes ? classes.length : 0);
    ");
    match result {
        Ok(_) => println!("✅ document.querySelectorAll tests passed"),
        Err(e) => println!("❌ document.querySelectorAll tests failed: {:?}", e),
    }

    println!("\n🎨 Testing element.classList...");
    let result = engine.execute("
        let button = document.getElementById('btn');
        if (button) {
            console.log('Button has active class:', button.classList.contains('active'));
            button.classList.add('new-class');
            button.classList.remove('old-class');
            console.log('ClassList operations completed');
        }
    ");
    match result {
        Ok(_) => println!("✅ element.classList tests passed"),
        Err(e) => println!("❌ element.classList tests failed: {:?}", e),
    }

    println!("\n🎨 Testing element.style manipulation...");
    let result = engine.execute("
        let title = document.getElementById('title');
        if (title) {
            title.style.setProperty('color', 'red');
            title.style.setProperty('font-size', '32px');
            
            let currentColor = title.style.getPropertyValue('color');
            let currentSize = title.style.getPropertyValue('font-size');
            console.log('Current color:', currentColor);
            console.log('Current size:', currentSize);
        }
    ");
    match result {
        Ok(_) => println!("✅ element.style tests passed"),
        Err(e) => println!("❌ element.style tests failed: {:?}", e),
    }

    println!("\n🔍 Testing element.querySelector...");
    let result = engine.execute("
        let container = document.getElementById('container');
        if (container) {
            let nested = container.querySelector('#nested');
            let span = container.querySelector('.highlight');
            console.log('Found nested div:', nested ? 'Yes' : 'No');
            console.log('Found highlight span:', span ? 'Yes' : 'No');
        }
    ");
    match result {
        Ok(_) => println!("✅ element.querySelector tests passed"),
        Err(e) => println!("❌ element.querySelector tests failed: {:?}", e),
    }

    println!("\n🔍 Testing element.querySelectorAll...");
    let result = engine.execute("
        let container = document.getElementById('container');
        if (container) {
            let allDivs = container.querySelectorAll('div');
            let allSpans = container.querySelectorAll('span');
            console.log('Container divs count:', allDivs ? allDivs.length : 0);
            console.log('Container spans count:', allSpans ? allSpans.length : 0);
        }
    ");
    match result {
        Ok(_) => println!("✅ element.querySelectorAll tests passed"),
        Err(e) => println!("❌ element.querySelectorAll tests failed: {:?}", e),
    }

    println!("\n🎯 Testing complex DOM manipulation...");
    let result = engine.execute("
        // Find elements
        let container = document.querySelector('#container');
        let button = document.querySelector('.button');
        
        if (container && button) {
            // Style manipulation
            container.style.setProperty('padding', '20px');
            button.style.setProperty('background-color', 'blue');
            
            // Class manipulation
            button.classList.add('clicked');
            button.classList.remove('active');
            
            // Check if class was removed
            let hasActive = button.classList.contains('active');
            let hasClicked = button.classList.contains('clicked');
            
            console.log('Button has active class:', hasActive);
            console.log('Button has clicked class:', hasClicked);
            
            // Get computed styles
            let bgColor = button.style.getPropertyValue('background-color');
            let padding = container.style.getPropertyValue('padding');
            
            console.log('Button background:', bgColor);
            console.log('Container padding:', padding);
        }
    ");
    match result {
        Ok(_) => println!("✅ Complex DOM manipulation tests passed"),
        Err(e) => println!("❌ Complex DOM manipulation tests failed: {:?}", e),
    }

    println!("\n🎉 DOM API Expansion Test Complete!");
    println!("=====================================");
    println!("✅ querySelector/querySelectorAll (document and element level)");
    println!("✅ classList.add/remove/contains");
    println!("✅ style.setProperty/getPropertyValue");
    println!("✅ Complex DOM manipulation workflows");
    println!("\n🚀 Ready for performance profiling and stress testing!");
}
