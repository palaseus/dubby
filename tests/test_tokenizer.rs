use css_parser::{CSSTokenizer, CSSToken};

fn main() {
    let css = "div { color: red; }";
    println!("CSS input: {}", css);
    
    let mut tokenizer = CSSTokenizer::new(css.to_string());
    println!("Tokens:");
    loop {
        let token = tokenizer.next_token();
        println!("  {:?}", token);
        if matches!(token, CSSToken::Eof) {
            break;
        }
    }
    
    // Test again with a new tokenizer
    println!("\nTesting with new tokenizer:");
    let mut tokenizer2 = CSSTokenizer::new(css.to_string());
    let token1 = tokenizer2.next_token();
    println!("First token: {:?}", token1);
    let token2 = tokenizer2.next_token();
    println!("Second token: {:?}", token2);
}
