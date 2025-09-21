use css_parser::CSSParser;

fn main() {
    let css = "div { color: red; }";
    println!("CSS input: {}", css);
    
    let mut parser = CSSParser::new(css.to_string());
    match parser.parse_stylesheet() {
        Ok(stylesheet) => {
            println!("Success! Rules: {}", stylesheet.rules.len());
            for (i, rule) in stylesheet.rules.iter().enumerate() {
                println!("  Rule {}: selectors={}, declarations={}", 
                    i, rule.selectors.len(), rule.declarations.len());
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
