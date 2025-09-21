use html_parser::{parse_html_string, Tokenizer, Token};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <html_file>", args[0]);
        return Ok(());
    }
    
    let html_content = fs::read_to_string(&args[1])?;
    println!("🔍 Debugging HTML Parser");
    println!("=========================");
    println!("Input: {}", html_content);
    println!();
    
    // Test tokenizer directly
    println!("🔧 Testing Tokenizer:");
    let mut tokenizer = Tokenizer::new(html_content.as_bytes().to_vec())?;
    let mut token_count = 0;
    
    loop {
        match tokenizer.next_token() {
            Ok(Token::Eof) => break,
            Ok(token) => {
                token_count += 1;
                println!("  Token {}: {:?}", token_count, token);
                if token_count > 20 {
                    println!("  ... (stopping after 20 tokens)");
                    break;
                }
            }
            Err(e) => {
                println!("  ❌ Error at token {}: {}", token_count + 1, e);
                break;
            }
        }
    }
    
    println!();
    println!("🌳 Testing Full Parser:");
    match parse_html_string(&html_content) {
        Ok((document, resources)) => {
            println!("✅ Successfully parsed!");
            println!("  DOM nodes: {}", count_dom_nodes(&document));
            println!("  External resources: {}", resources.len());
        }
        Err(e) => {
            println!("❌ Parse error: {}", e);
        }
    }
    
    Ok(())
}

fn count_dom_nodes(document: &dom::Document) -> usize {
    fn count_recursive(node: &dom::Node) -> usize {
        1 + node.children.borrow().iter().map(|child| count_recursive(child)).sum::<usize>()
    }
    count_recursive(&document.root)
}
