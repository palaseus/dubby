use css_parser::{CSSParser, CSSTokenizer, CSSToken};

fn main() {
    let css = "div { color: red; background-color: blue; }";
    println!("CSS input: {}", css);
    
    let mut parser = CSSParser::new(css.to_string());
    
    // Parse selectors manually
    let selectors = parser.parse_selectors().unwrap();
    println!("Selectors parsed: {:?}", selectors);
    
    // Check what token we're at now
    let next_token = parser.tokenizer.next_token();
    println!("Next token after selectors: {:?}", next_token);
    
    // Try to parse declarations
    match parser.parse_declarations() {
        Ok(declarations) => {
            println!("Declarations parsed: {:?}", declarations);
        }
        Err(e) => {
            println!("Error parsing declarations: {:?}", e);
        }
    }
}
