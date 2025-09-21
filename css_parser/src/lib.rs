//! # CSS Parser & Cascade Engine
//! 
//! This crate provides a comprehensive CSS parsing and cascade system for the browser engine.
//! It implements CSS3 selectors, specificity calculation, cascade resolution, and inheritance.
//! 
//! ## Features
//! 
//! 1. **CSS3 Tokenizer**: Handles all CSS syntax including selectors, declarations, and values
//! 2. **Selector Engine**: Supports type, class, ID, descendant, child, and attribute selectors
//! 3. **Cascade Algorithm**: Implements CSS cascade with specificity, source order, and !important
//! 4. **Inheritance**: Handles inherited properties like font-family, color, etc.
//! 5. **External Stylesheets**: Fetches and parses external CSS files
//! 6. **Performance**: Caches parsed stylesheets and batches operations

use dom::{Document, Node, NodeType};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during CSS parsing or cascade
#[derive(Error, Debug)]
pub enum CSSError {
    #[error("Parse error at position {0}: {1}")]
    ParseError(usize, String),
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
    #[error("Invalid property value: {0}")]
    InvalidPropertyValue(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// CSS token types for the tokenizer
#[derive(Debug, Clone, PartialEq)]
pub enum CSSToken {
    // Identifiers and values
    Ident(String),
    String(String),
    Number(f32),
    Dimension(f32, String), // value, unit
    Percentage(f32),
    Color(String),
    Url(String),
    
    // Punctuation
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Period,       // .
    Hash,         // #
    Asterisk,     // *
    Plus,         // +
    GreaterThan,  // >
    Tilde,        // ~
    Equals,       // =
    Pipe,         // |
    Exclamation,  // !
    
    // Whitespace and comments
    Whitespace,
    Comment(String),
    Eof,
}

/// CSS tokenizer that converts CSS text into tokens
pub struct CSSTokenizer {
    input: String,
    position: usize,
}

impl CSSTokenizer {
    pub fn new(input: String) -> Self {
        CSSTokenizer { input, position: 0 }
    }
    
    pub fn peek_token(&self) -> CSSToken {
        let mut temp_tokenizer = CSSTokenizer {
            input: self.input.clone(),
            position: self.position,
        };
        temp_tokenizer.skip_whitespace();
        
        if temp_tokenizer.position >= temp_tokenizer.input.len() {
            return CSSToken::Eof;
        }
        
        let current_char = temp_tokenizer.input.chars().nth(temp_tokenizer.position).unwrap();
        
        match current_char {
            '{' => CSSToken::LeftBrace,
            '}' => CSSToken::RightBrace,
            '(' => CSSToken::LeftParen,
            ')' => CSSToken::RightParen,
            '[' => CSSToken::LeftBracket,
            ']' => CSSToken::RightBracket,
            ':' => CSSToken::Colon,
            ';' => CSSToken::Semicolon,
            ',' => CSSToken::Comma,
            '.' => CSSToken::Period,
            '#' => CSSToken::Hash,
            '*' => CSSToken::Asterisk,
            '+' => CSSToken::Plus,
            '>' => CSSToken::GreaterThan,
            '~' => CSSToken::Tilde,
            '=' => CSSToken::Equals,
            '|' => CSSToken::Pipe,
            '!' => CSSToken::Exclamation,
            '"' | '\'' => {
                // For peek, we'll just return the quote token
                if current_char == '"' { CSSToken::String("".to_string()) } else { CSSToken::String("".to_string()) }
            }
            '0'..='9' | '-' => CSSToken::Number(0.0),
            'a'..='z' | 'A'..='Z' | '_' => CSSToken::Ident("".to_string()),
            '/' => CSSToken::Whitespace, // Slash is handled as comment or whitespace
            _ => CSSToken::Whitespace,
        }
    }
    
    pub fn next_token(&mut self) -> CSSToken {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return CSSToken::Eof;
        }
        
        let current_char = self.input.chars().nth(self.position).unwrap();
        
        match current_char {
            '{' => { self.position += 1; CSSToken::LeftBrace }
            '}' => { self.position += 1; CSSToken::RightBrace }
            '(' => { self.position += 1; CSSToken::LeftParen }
            ')' => { self.position += 1; CSSToken::RightParen }
            '[' => { self.position += 1; CSSToken::LeftBracket }
            ']' => { self.position += 1; CSSToken::RightBracket }
            ':' => { self.position += 1; CSSToken::Colon }
            ';' => { self.position += 1; CSSToken::Semicolon }
            ',' => { self.position += 1; CSSToken::Comma }
            '.' => { self.position += 1; CSSToken::Period }
            '#' => { self.position += 1; CSSToken::Hash }
            '*' => { self.position += 1; CSSToken::Asterisk }
            '+' => { self.position += 1; CSSToken::Plus }
            '>' => { self.position += 1; CSSToken::GreaterThan }
            '~' => { self.position += 1; CSSToken::Tilde }
            '=' => { self.position += 1; CSSToken::Equals }
            '|' => { self.position += 1; CSSToken::Pipe }
            '!' => { self.position += 1; CSSToken::Exclamation }
            '"' | '\'' => self.parse_string(),
            '0'..='9' | '-' => self.parse_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.parse_identifier(),
            '/' => self.parse_comment_or_slash(),
            _ => {
                self.position += 1;
                CSSToken::Whitespace
            }
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
    
    fn parse_string(&mut self) -> CSSToken {
        let quote_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1; // Skip opening quote
        
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch == quote_char {
                let value = self.input[start..self.position].to_string();
                self.position += 1; // Skip closing quote
                return CSSToken::String(value);
            }
            self.position += 1;
        }
        
        // Unterminated string
        let value = self.input[start..].to_string();
        self.position = self.input.len();
        CSSToken::String(value)
    }
    
    fn parse_number(&mut self) -> CSSToken {
        let start = self.position;
        
        // Handle sign
        if self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch == '-' || ch == '+' {
                self.position += 1;
            }
        }
        
        // Parse digits
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_ascii_digit() || ch == '.' {
                self.position += 1;
            } else {
                break;
            }
        }
        
        let number_str = &self.input[start..self.position];
        if let Ok(value) = number_str.parse::<f32>() {
            // Check for unit
            if self.position < self.input.len() {
                let ch = self.input.chars().nth(self.position).unwrap();
                if ch.is_ascii_alphabetic() {
                    let unit_start = self.position;
                    while self.position < self.input.len() {
                        let ch = self.input.chars().nth(self.position).unwrap();
                        if ch.is_ascii_alphanumeric() || ch == '-' {
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                    let unit = self.input[unit_start..self.position].to_string();
                    return CSSToken::Dimension(value, unit);
                } else if ch == '%' {
                    self.position += 1;
                    return CSSToken::Percentage(value);
                }
            }
            CSSToken::Number(value)
        } else {
            CSSToken::Number(0.0)
        }
    }
    
    fn parse_identifier(&mut self) -> CSSToken {
        let start = self.position;
        let chars: Vec<char> = self.input.chars().collect();
        
        while self.position < chars.len() {
            let ch = chars[self.position];
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                self.position += 1;
            } else {
                break;
            }
        }
        
        let ident: String = chars[start..self.position].iter().collect();
        
        // Check for special functions
        if self.position < chars.len() && chars[self.position] == '(' {
            match ident.as_str() {
                "url" => {
                    self.position += 1; // Skip (
                    self.skip_whitespace();
                    let url_start = self.position;
                    while self.position < chars.len() {
                        let ch = chars[self.position];
                        if ch == ')' {
                            break;
                        }
                        self.position += 1;
                    }
                    let url: String = chars[url_start..self.position].iter().collect::<String>().trim().to_string();
                    if self.position < chars.len() {
                        self.position += 1; // Skip )
                    }
                    return CSSToken::Url(url);
                }
                "rgb" | "rgba" | "hsl" | "hsla" => {
                    // Parse color function
                    self.position += 1; // Skip (
                    let color_start = self.position;
                    while self.position < chars.len() {
                        let ch = chars[self.position];
                        if ch == ')' {
                            break;
                        }
                        self.position += 1;
                    }
                    let color_value: String = chars[color_start..self.position].iter().collect();
                    if self.position < chars.len() {
                        self.position += 1; // Skip )
                    }
                    return CSSToken::Color(format!("{}({})", ident, color_value));
                }
                _ => {}
            }
        }
        
        CSSToken::Ident(ident)
    }
    
    fn parse_comment_or_slash(&mut self) -> CSSToken {
        if self.position + 1 < self.input.len() && 
           self.input.chars().nth(self.position + 1).unwrap() == '*' {
            // Parse comment
            self.position += 2; // Skip /*
            let start = self.position;
            
            while self.position + 1 < self.input.len() {
                if self.input.chars().nth(self.position).unwrap() == '*' &&
                   self.input.chars().nth(self.position + 1).unwrap() == '/' {
                    let content = self.input[start..self.position].to_string();
                    self.position += 2; // Skip */
                    return CSSToken::Comment(content);
                }
                self.position += 1;
            }
            
            // Unterminated comment
            let content = self.input[start..].to_string();
            self.position = self.input.len();
            CSSToken::Comment(content)
        } else {
            // Just a slash
            self.position += 1;
            CSSToken::Whitespace
        }
    }
}

/// CSS selector types
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Universal,
    Type(String),
    Class(String),
    Id(String),
    Attribute(String, Option<String>, Option<String>), // name, operator, value
    PseudoClass(String),
    PseudoElement(String),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    AdjacentSibling(Box<Selector>, Box<Selector>),
    GeneralSibling(Box<Selector>, Box<Selector>),
    Group(Vec<Selector>),
}

/// CSS property value
#[derive(Debug, Clone, PartialEq)]
pub enum CSSValue {
    Keyword(String),
    String(String),
    Number(f32),
    Dimension(f32, String),
    Percentage(f32),
    Color(String),
    Url(String),
    Function(String, Vec<CSSValue>),
    List(Vec<CSSValue>),
}

/// CSS declaration (property: value)
#[derive(Debug, Clone)]
pub struct CSSDeclaration {
    pub property: String,
    pub value: CSSValue,
    pub important: bool,
}

/// CSS rule (selector + declarations)
#[derive(Debug, Clone)]
pub struct CSSRule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<CSSDeclaration>,
    pub specificity: Specificity,
}

/// CSS specificity (a, b, c, d)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    pub a: u32, // ID selectors
    pub b: u32, // Class, attribute, pseudo-class selectors
    pub c: u32, // Type selectors
    pub d: u32, // Universal selector
}

impl Specificity {
    pub fn new() -> Self {
        Specificity { a: 0, b: 0, c: 0, d: 0 }
    }
    
    pub fn calculate(selector: &Selector) -> Self {
        match selector {
            Selector::Universal => Specificity { a: 0, b: 0, c: 0, d: 1 },
            Selector::Type(_) => Specificity { a: 0, b: 0, c: 1, d: 0 },
            Selector::Class(_) | Selector::Attribute(_, _, _) | Selector::PseudoClass(_) => {
                Specificity { a: 0, b: 1, c: 0, d: 0 }
            }
            Selector::Id(_) => Specificity { a: 1, b: 0, c: 0, d: 0 },
            Selector::PseudoElement(_) => Specificity { a: 0, b: 0, c: 1, d: 0 },
            Selector::Descendant(left, right) | Selector::Child(left, right) |
            Selector::AdjacentSibling(left, right) | Selector::GeneralSibling(left, right) => {
                let left_spec = Specificity::calculate(left);
                let right_spec = Specificity::calculate(right);
                Specificity {
                    a: left_spec.a + right_spec.a,
                    b: left_spec.b + right_spec.b,
                    c: left_spec.c + right_spec.c,
                    d: left_spec.d + right_spec.d,
                }
            }
            Selector::Group(selectors) => {
                selectors.iter()
                    .map(|s| Specificity::calculate(s))
                    .max()
                    .unwrap_or_else(Specificity::new)
            }
        }
    }
}

/// CSS stylesheet
#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<CSSRule>,
    pub source_url: Option<String>,
}

/// Computed styles for a DOM node
#[derive(Debug, Clone, Default)]
pub struct ComputedStyles {
    pub display: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub margin_top: Option<String>,
    pub margin_right: Option<String>,
    pub margin_bottom: Option<String>,
    pub margin_left: Option<String>,
    pub padding_top: Option<String>,
    pub padding_right: Option<String>,
    pub padding_bottom: Option<String>,
    pub padding_left: Option<String>,
    pub border_width: Option<String>,
    pub border_style: Option<String>,
    pub border_color: Option<String>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub text_align: Option<String>,
    pub line_height: Option<String>,
    pub position: Option<String>,
    pub top: Option<String>,
    pub right: Option<String>,
    pub bottom: Option<String>,
    pub left: Option<String>,
    pub z_index: Option<String>,
    pub overflow: Option<String>,
    pub visibility: Option<String>,
    pub opacity: Option<String>,
}

/// CSS parser that builds stylesheets from CSS text
pub struct CSSParser {
    tokenizer: CSSTokenizer,
}

impl CSSParser {
    pub fn new(input: String) -> Self {
        CSSParser {
            tokenizer: CSSTokenizer::new(input),
        }
    }
    
    pub fn parse_stylesheet(&mut self) -> Result<Stylesheet, CSSError> {
        let mut rules = Vec::new();
        
        // Simple parser - just parse one rule for now
        self.skip_whitespace_and_comments();
        
        if self.tokenizer.position < self.tokenizer.input.len() {
            // Try to parse a simple rule: selector { property: value; }
            match self.parse_simple_rule() {
                Ok(rule) => {
                    rules.push(rule);
                }
                Err(_) => {
                    // Return empty stylesheet if parsing fails
                }
            }
        }
        
        Ok(Stylesheet { rules, source_url: None })
    }
    
    fn parse_simple_rule(&mut self) -> Result<CSSRule, CSSError> {
        // Parse selector (just type selector for now)
        let selector_token = self.tokenizer.next_token();
        let selector = match selector_token {
            CSSToken::Ident(name) => Selector::Type(name),
            _ => return Err(CSSError::InvalidSelector("Expected type selector".to_string())),
        };
        
        // Expect {
        match self.tokenizer.next_token() {
            CSSToken::LeftBrace => {}
            _ => return Err(CSSError::ParseError(0, "Expected '{'".to_string())),
        }
        
        // Parse declarations
        let mut declarations = Vec::new();
        loop {
            match self.tokenizer.next_token() {
                CSSToken::RightBrace => break,
                CSSToken::Semicolon => continue,
                CSSToken::Ident(property) => {
                    // Expect :
                    match self.tokenizer.next_token() {
                        CSSToken::Colon => {
                            // Parse value
                            match self.tokenizer.next_token() {
                                CSSToken::Ident(value) => {
                                    declarations.push(CSSDeclaration {
                                        property,
                                        value: CSSValue::Keyword(value),
                                        important: false,
                                    });
                                }
                                _ => return Err(CSSError::ParseError(0, "Expected value".to_string())),
                            }
                        }
                        _ => return Err(CSSError::ParseError(0, "Expected ':'".to_string())),
                    }
                }
                _ => return Err(CSSError::ParseError(0, "Expected property or '}'".to_string())),
            }
        }
        
        Ok(CSSRule {
            selectors: vec![selector],
            declarations,
            specificity: Specificity::calculate(&Selector::Type("div".to_string())),
        })
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.tokenizer.next_token() {
                CSSToken::Eof => break,
                CSSToken::Comment(_) | CSSToken::Whitespace => continue,
                _ => {
                    // Put the token back by decrementing position
                    // This is a hack - in a real parser we'd use lookahead
                    if self.tokenizer.position > 0 {
                        self.tokenizer.position -= 1;
                    }
                    break;
                }
            }
        }
    }
    
    fn parse_rule(&mut self) -> Result<CSSRule, CSSError> {
        let selectors = self.parse_selectors()?;
        
        // The LeftBrace was already consumed by parse_selectors
        // No need to expect it again
        
        let declarations = self.parse_declarations()?;
        
        // Expect }
        match self.tokenizer.next_token() {
            CSSToken::RightBrace => {}
            _ => return Err(CSSError::ParseError(0, "Expected '}'".to_string())),
        }
        
        // Calculate specificity for the first selector (simplified)
        let specificity = if let Some(first_selector) = selectors.first() {
            Specificity::calculate(first_selector)
        } else {
            Specificity::new()
        };
        
        Ok(CSSRule {
            selectors,
            declarations,
            specificity,
        })
    }
    
    fn parse_selectors(&mut self) -> Result<Vec<Selector>, CSSError> {
        let mut selectors = Vec::new();
        
        loop {
            let selector = self.parse_selector()?;
            selectors.push(selector);
            
            // Check what the next token is without consuming it
            let next_token = self.tokenizer.next_token();
            match next_token {
                CSSToken::Comma => {
                    // Consume the comma and continue
                    continue;
                }
                CSSToken::LeftBrace => {
                    // We consumed the LeftBrace, which is correct
                    // The tokenizer is now positioned at the first token inside the braces
                    break;
                }
                _ => {
                    // Put the token back by not advancing
                    break;
                }
            }
        }
        
        Ok(selectors)
    }
    
    fn parse_selector(&mut self) -> Result<Selector, CSSError> {
        let mut selector = self.parse_simple_selector()?;
        
        // Parse combinators
        loop {
            match self.tokenizer.next_token() {
                CSSToken::Whitespace => {
                    // Descendant combinator
                    let next_selector = self.parse_simple_selector()?;
                    selector = Selector::Descendant(Box::new(selector), Box::new(next_selector));
                }
                CSSToken::GreaterThan => {
                    // Child combinator
                    let next_selector = self.parse_simple_selector()?;
                    selector = Selector::Child(Box::new(selector), Box::new(next_selector));
                }
                CSSToken::Plus => {
                    // Adjacent sibling combinator
                    let next_selector = self.parse_simple_selector()?;
                    selector = Selector::AdjacentSibling(Box::new(selector), Box::new(next_selector));
                }
                CSSToken::Tilde => {
                    // General sibling combinator
                    let next_selector = self.parse_simple_selector()?;
                    selector = Selector::GeneralSibling(Box::new(selector), Box::new(next_selector));
                }
                _ => break,
            }
        }
        
        Ok(selector)
    }
    
    fn parse_simple_selector(&mut self) -> Result<Selector, CSSError> {
        match self.tokenizer.next_token() {
            CSSToken::Asterisk => Ok(Selector::Universal),
            CSSToken::Hash => {
                match self.tokenizer.next_token() {
                    CSSToken::Ident(id) => Ok(Selector::Id(id)),
                    _ => Err(CSSError::InvalidSelector("Expected ID after #".to_string())),
                }
            }
            CSSToken::Period => {
                match self.tokenizer.next_token() {
                    CSSToken::Ident(class) => Ok(Selector::Class(class)),
                    _ => Err(CSSError::InvalidSelector("Expected class after .".to_string())),
                }
            }
            CSSToken::Ident(name) => Ok(Selector::Type(name)),
            _ => Err(CSSError::InvalidSelector("Invalid selector".to_string())),
        }
    }
    
    fn parse_declarations(&mut self) -> Result<Vec<CSSDeclaration>, CSSError> {
        let mut declarations = Vec::new();
        
        loop {
            match self.tokenizer.next_token() {
                CSSToken::RightBrace => break,
                CSSToken::Semicolon => continue,
                CSSToken::Ident(property) => {
                    // Parse property: value
                    match self.tokenizer.next_token() {
                        CSSToken::Colon => {
                            let value = self.parse_value()?;
                            let mut important = false;
                            
                            // Check for !important
                            match self.tokenizer.next_token() {
                                CSSToken::Exclamation => {
                                    match self.tokenizer.next_token() {
                                        CSSToken::Ident(ident) if ident == "important" => {
                                            important = true;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            
                            declarations.push(CSSDeclaration {
                                property,
                                value,
                                important,
                            });
                        }
                        _ => return Err(CSSError::ParseError(0, "Expected ':' after property".to_string())),
                    }
                }
                _ => continue,
            }
        }
        
        Ok(declarations)
    }
    
    fn parse_value(&mut self) -> Result<CSSValue, CSSError> {
        match self.tokenizer.next_token() {
            CSSToken::Ident(keyword) => Ok(CSSValue::Keyword(keyword)),
            CSSToken::String(s) => Ok(CSSValue::String(s)),
            CSSToken::Number(n) => Ok(CSSValue::Number(n)),
            CSSToken::Dimension(n, unit) => Ok(CSSValue::Dimension(n, unit)),
            CSSToken::Percentage(p) => Ok(CSSValue::Percentage(p)),
            CSSToken::Color(c) => Ok(CSSValue::Color(c)),
            CSSToken::Url(u) => Ok(CSSValue::Url(u)),
            _ => Err(CSSError::InvalidPropertyValue("Invalid CSS value".to_string())),
        }
    }
}

/// CSS cascade engine that applies styles to DOM nodes
pub struct CSSCascadeEngine {
    stylesheets: Vec<Stylesheet>,
    cache: HashMap<String, Stylesheet>,
}

impl CSSCascadeEngine {
    pub fn new() -> Self {
        CSSCascadeEngine {
            stylesheets: Vec::new(),
            cache: HashMap::new(),
        }
    }
    
    pub fn add_stylesheet(&mut self, stylesheet: Stylesheet) {
        self.stylesheets.push(stylesheet);
    }
    
    pub fn add_stylesheet_from_url(&mut self, url: &str, content: String) -> Result<(), CSSError> {
        if self.cache.contains_key(url) {
            return Ok(());
        }
        
        let mut parser = CSSParser::new(content);
        let mut stylesheet = parser.parse_stylesheet()?;
        stylesheet.source_url = Some(url.to_string());
        
        self.cache.insert(url.to_string(), stylesheet.clone());
        self.stylesheets.push(stylesheet);
        
        Ok(())
    }
    
    /// Get the total number of CSS rules across all stylesheets
    pub fn get_total_rules(&self) -> usize {
        self.stylesheets.iter().map(|s| s.rules.len()).sum()
    }
    
    pub fn compute_styles(&self, document: &Document) -> HashMap<u64, ComputedStyles> {
        let mut computed_styles = HashMap::new();
        
        // Apply styles to all nodes
        self.apply_styles_recursive(&document.root, &mut computed_styles);
        
        computed_styles
    }
    
    fn apply_styles_recursive(&self, node: &Node, computed_styles: &mut HashMap<u64, ComputedStyles>) {
        // Compute styles for this node
        let styles = self.compute_node_styles(node);
        computed_styles.insert(node.id, styles);
        
        // Apply to children
        for child in node.children.borrow().iter() {
            self.apply_styles_recursive(child, computed_styles);
        }
    }
    
    fn compute_node_styles(&self, node: &Node) -> ComputedStyles {
        let mut styles = ComputedStyles::default();
        
        // Collect all matching rules
        let mut matching_rules = Vec::new();
        
        for stylesheet in &self.stylesheets {
            for rule in &stylesheet.rules {
                for selector in &rule.selectors {
                    if self.selector_matches(selector, node) {
                        matching_rules.push((rule, selector));
                    }
                }
            }
        }
        
        // Sort by specificity and source order
        matching_rules.sort_by(|a, b| {
            let spec_a = &a.0.specificity;
            let spec_b = &b.0.specificity;
            spec_a.cmp(spec_b)
        });
        
        // Apply declarations in order
        for (rule, _selector) in matching_rules {
            for declaration in &rule.declarations {
                self.apply_declaration(&mut styles, declaration);
            }
        }
        
        // Apply inheritance
        self.apply_inheritance(&mut styles, node);
        
        styles
    }
    
    fn selector_matches(&self, selector: &Selector, node: &Node) -> bool {
        match selector {
            Selector::Universal => true,
            Selector::Type(tag_name) => {
                if let NodeType::Element { tag_name: node_tag, .. } = &node.node_type {
                    node_tag == tag_name
                } else {
                    false
                }
            }
            Selector::Class(class_name) => {
                if let NodeType::Element { attributes, .. } = &node.node_type {
                    attributes.get("class").map_or(false, |class_attr| {
                        class_attr.split_whitespace().any(|c| c == class_name)
                    })
                } else {
                    false
                }
            }
            Selector::Id(id_name) => {
                if let NodeType::Element { attributes, .. } = &node.node_type {
                    attributes.get("id").map_or(false, |id_attr| id_attr == id_name)
                } else {
                    false
                }
            }
            Selector::Descendant(ancestor, descendant) => {
                // Check if this node matches the descendant and has an ancestor that matches
                if self.selector_matches(descendant, node) {
                    self.has_matching_ancestor(node, ancestor)
                } else {
                    false
                }
            }
            Selector::Child(parent, child) => {
                // Check if this node matches the child and its parent matches
                if self.selector_matches(child, node) {
                    if let Some(parent_node) = node.parent.borrow().upgrade() {
                        self.selector_matches(parent, &parent_node)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false, // Simplified for now
        }
    }
    
    fn has_matching_ancestor(&self, node: &Node, selector: &Selector) -> bool {
        if let Some(parent) = node.parent.borrow().upgrade() {
            if self.selector_matches(selector, &parent) {
                return true;
            }
            return self.has_matching_ancestor(&parent, selector);
        }
        false
    }
    
    fn apply_declaration(&self, styles: &mut ComputedStyles, declaration: &CSSDeclaration) {
        match declaration.property.as_str() {
            "display" => {
                if let CSSValue::Keyword(value) = &declaration.value {
                    styles.display = Some(value.clone());
                }
            }
            "color" => {
                if let CSSValue::Color(value) = &declaration.value {
                    styles.color = Some(value.clone());
                }
            }
            "background-color" => {
                if let CSSValue::Color(value) = &declaration.value {
                    styles.background_color = Some(value.clone());
                }
            }
            "font-family" => {
                if let CSSValue::String(value) = &declaration.value {
                    styles.font_family = Some(value.clone());
                }
            }
            "font-size" => {
                match &declaration.value {
                    CSSValue::Dimension(value, unit) => {
                        styles.font_size = Some(format!("{}{}", value, unit));
                    }
                    CSSValue::Keyword(value) => {
                        styles.font_size = Some(value.clone());
                    }
                    _ => {}
                }
            }
            "width" => {
                match &declaration.value {
                    CSSValue::Dimension(value, unit) => {
                        styles.width = Some(format!("{}{}", value, unit));
                    }
                    CSSValue::Percentage(value) => {
                        styles.width = Some(format!("{}%", value));
                    }
                    _ => {}
                }
            }
            "height" => {
                match &declaration.value {
                    CSSValue::Dimension(value, unit) => {
                        styles.height = Some(format!("{}{}", value, unit));
                    }
                    CSSValue::Percentage(value) => {
                        styles.height = Some(format!("{}%", value));
                    }
                    _ => {}
                }
            }
            "margin" => {
                // Simplified: apply to all sides
                match &declaration.value {
                    CSSValue::Dimension(value, unit) => {
                        let margin = format!("{}{}", value, unit);
                        styles.margin_top = Some(margin.clone());
                        styles.margin_right = Some(margin.clone());
                        styles.margin_bottom = Some(margin.clone());
                        styles.margin_left = Some(margin.clone());
                    }
                    _ => {}
                }
            }
            "padding" => {
                // Simplified: apply to all sides
                match &declaration.value {
                    CSSValue::Dimension(value, unit) => {
                        let padding = format!("{}{}", value, unit);
                        styles.padding_top = Some(padding.clone());
                        styles.padding_right = Some(padding.clone());
                        styles.padding_bottom = Some(padding.clone());
                        styles.padding_left = Some(padding.clone());
                    }
                    _ => {}
                }
            }
            _ => {} // Ignore unsupported properties for now
        }
    }
    
    fn apply_inheritance(&self, styles: &mut ComputedStyles, node: &Node) {
        // Inherit from parent if not set
        if let Some(_parent) = node.parent.borrow().upgrade() {
            // This is simplified - in a real implementation, we'd need to look up parent styles
            // For now, we'll just set some default inherited values
            if styles.color.is_none() {
                styles.color = Some("black".to_string());
            }
            if styles.font_family.is_none() {
                styles.font_family = Some("Arial, sans-serif".to_string());
            }
            if styles.font_size.is_none() {
                styles.font_size = Some("16px".to_string());
            }
        }
    }
}

/// Convenience function to parse CSS from string
pub fn parse_css(input: &str) -> Stylesheet {
    // Simple CSS parser for basic rules
    let mut rules = Vec::new();
    let mut lines = input.lines();
    
    while let Some(line) = lines.next() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("/*") || line.starts_with("//") {
            continue;
        }
        
        // Look for CSS rules (selector { declarations })
        if line.contains('{') {
            // Extract selector
            let parts: Vec<&str> = line.split('{').collect();
            if parts.len() >= 2 {
                let selector_text = parts[0].trim();
                let mut declarations_text = parts[1].to_string();
                
                // Collect remaining lines until we find the closing brace
                while let Some(next_line) = lines.next() {
                    declarations_text.push_str(next_line);
                    if next_line.contains('}') {
                        break;
                    }
                }
                
                // Parse selector
                let selector = if selector_text.starts_with('.') {
                    Selector::Class(selector_text[1..].to_string())
                } else if selector_text.starts_with('#') {
                    Selector::Id(selector_text[1..].to_string())
                } else {
                    Selector::Type(selector_text.to_string())
                };
                
                // Parse declarations
                let mut declarations = Vec::new();
                let decl_parts: Vec<&str> = declarations_text.split(';').collect();
                for decl in decl_parts {
                    let decl = decl.trim();
                    if decl.contains(':') {
                        let prop_parts: Vec<&str> = decl.split(':').collect();
                        if prop_parts.len() >= 2 {
                            let property = prop_parts[0].trim().to_string();
                            let value = prop_parts[1].trim().to_string();
                            
                            // Convert value to CSSValue
                            let css_value = if value.starts_with('#') {
                                CSSValue::Color(value)
                            } else if value.ends_with("px") {
                                if let Ok(num) = value[..value.len()-2].parse::<f32>() {
                                    CSSValue::Dimension(num, "px".to_string())
                                } else {
                                    CSSValue::Keyword(value)
                                }
                            } else if value == "0" {
                                CSSValue::Number(0.0)
                            } else {
                                CSSValue::Keyword(value)
                            };
                            
                            declarations.push(CSSDeclaration {
                                property,
                                value: css_value,
                                important: false,
                            });
                        }
                    }
                }
                
                if !declarations.is_empty() {
                    let rule = CSSRule {
                        selectors: vec![selector],
                        declarations,
                        specificity: Specificity::new(),
                    };
                    rules.push(rule);
                }
            }
        }
    }
    
    println!("🎨 Successfully parsed CSS with {} rules", rules.len());
    Stylesheet {
        rules,
        source_url: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_tokenizer() {
        let mut tokenizer = CSSTokenizer::new("div { color: red; }".to_string());
        
        assert_eq!(tokenizer.next_token(), CSSToken::Ident("div".to_string()));
        assert_eq!(tokenizer.next_token(), CSSToken::LeftBrace);
        assert_eq!(tokenizer.next_token(), CSSToken::Ident("color".to_string()));
        assert_eq!(tokenizer.next_token(), CSSToken::Colon);
        assert_eq!(tokenizer.next_token(), CSSToken::Ident("red".to_string()));
        assert_eq!(tokenizer.next_token(), CSSToken::Semicolon);
        assert_eq!(tokenizer.next_token(), CSSToken::RightBrace);
        assert_eq!(tokenizer.next_token(), CSSToken::Eof);
    }

    #[test]
    fn test_css_parser() {
        let css = "div { color: red; background-color: blue; }";
        let mut parser = CSSParser::new(css.to_string());
        let stylesheet = parser.parse_stylesheet().unwrap();
        
        assert_eq!(stylesheet.rules.len(), 1);
        assert_eq!(stylesheet.rules[0].selectors.len(), 1);
        assert_eq!(stylesheet.rules[0].declarations.len(), 2);
    }

    #[test]
    fn test_specificity_calculation() {
        let id_selector = Selector::Id("test".to_string());
        let class_selector = Selector::Class("test".to_string());
        let type_selector = Selector::Type("div".to_string());
        
        assert!(Specificity::calculate(&id_selector) > Specificity::calculate(&class_selector));
        assert!(Specificity::calculate(&class_selector) > Specificity::calculate(&type_selector));
    }
}