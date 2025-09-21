use css_parser::{CSSParser, CSSTokenizer, CSSToken};

fn main() {
    let css = "div { color: red; }";
    println!("CSS input: {}", css);
    
    let mut parser = CSSParser::new(css.to_string());
    
    // Test tokenizer position
    let token1 = parser.tokenizer.next_token();
    println!("First token: {:?}", token1);
    let token2 = parser.tokenizer.next_token();
    println!("Second token: {:?}", token2);
    let token3 = parser.tokenizer.next_token();
    println!("Third token: {:?}", token3);
    
    // Reset parser
    let mut parser2 = CSSParser::new(css.to_string());
    
    // Try to parse a rule
    match parser2.parse_rule() {
        Ok(rule) => {
            println!("Successfully parsed rule: {:?}", rule);
        }
        Err(e) => {
            println!("Error parsing rule: {:?}", e);
        }
    }
}
