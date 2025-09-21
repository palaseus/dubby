use css_parser::{CSSParser, CSSTokenizer, CSSToken};

fn main() {
    let css = "div { color: red; background-color: blue; }";
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
    
    println!("\nParsing with CSSParser:");
    let mut parser = CSSParser::new(css.to_string());
    match parser.parse_stylesheet() {
        Ok(stylesheet) => {
            println!("Success! Rules: {}", stylesheet.rules.len());
            for (i, rule) in stylesheet.rules.iter().enumerate() {
                println!("  Rule {}: {:?}", i, rule);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
