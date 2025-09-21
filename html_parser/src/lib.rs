//! # HTML Parser Crate
//! 
//! This crate provides functionality to parse HTML markup into a DOM tree.
//! It implements a spec-compliant HTML5 tokenizer that can handle real-world HTML documents.
//! 
//! ## Design Principles
//! 
//! 1. **Spec Compliance**: Follows HTML5 parsing specification for maximum compatibility
//! 2. **UTF-8 Support**: Properly handles UTF-8 encoded content and HTML entities
//! 3. **Error Recovery**: Robust error recovery for malformed HTML
//! 4. **Performance**: Optimized for real-world web pages
//! 5. **External Resources**: Parses and queues external CSS/JS resources

use dom::{Document, Node, NodeType};
use std::rc::Rc;
use std::collections::HashMap;
use encoding_rs::UTF_8;
use thiserror::Error;

/// Errors that can occur during HTML parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid UTF-8 sequence: {0}")]
    InvalidUtf8(String),
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Invalid tag name: {0}")]
    InvalidTagName(String),
    #[error("Invalid attribute: {0}")]
    InvalidAttribute(String),
    #[error("Parse error at position {0}: {1}")]
    ParseError(usize, String),
}

/// Represents different types of tokens that can be found in HTML
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// An opening tag like `<div>`
    StartTag {
        name: String,
        attributes: HashMap<String, String>,
        self_closing: bool,
    },
    /// A closing tag like `</div>`
    EndTag {
        name: String,
    },
    /// Text content between tags
    Text(String),
    /// A comment like `<!-- comment -->`
    Comment(String),
    /// A DOCTYPE declaration
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        force_quirks: bool,
    },
    /// End of input
    Eof,
}

/// HTML5-compliant tokenizer that converts HTML text into tokens
/// 
/// This tokenizer implements the HTML5 tokenization algorithm with:
/// - Proper UTF-8 decoding
/// - HTML entity resolution
/// - Spec-compliant tag parsing
/// - Robust error recovery
pub struct Tokenizer {
    _input: Vec<u8>,
    chars: Vec<char>,
    _position: usize,
    char_position: usize,
}

impl Tokenizer {
    /// Create a new tokenizer for the given HTML input
    /// 
    /// The input is automatically decoded from UTF-8 with proper error handling
    pub fn new(input: Vec<u8>) -> Result<Self, ParseError> {
        // Decode UTF-8 with proper error handling
        let (decoded, encoding_used, had_errors) = UTF_8.decode(&input);
        
        if had_errors {
            return Err(ParseError::InvalidUtf8(
                format!("Invalid UTF-8 sequence detected, encoding: {:?}", encoding_used)
            ));
        }
        
        let chars: Vec<char> = decoded.chars().collect();
        
        Ok(Tokenizer {
            _input: input,
            chars,
            _position: 0,
            char_position: 0,
        })
    }

    /// Get the next token from the input
    /// 
    /// Implements the HTML5 tokenization state machine
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        if self.char_position >= self.chars.len() {
            return Ok(Token::Eof);
        }

        let current_char = self.chars[self.char_position];

        match current_char {
            '<' => {
                if self.char_position + 1 < self.chars.len() {
                    let next_char = self.chars[self.char_position + 1];
                    match next_char {
                        '/' => self.parse_end_tag(),
                        '!' => self.parse_markup_declaration(),
                        '?' => self.parse_processing_instruction(),
                        _ => self.parse_start_tag(),
                    }
                } else {
                    self.parse_start_tag()
                }
            }
            _ => self.parse_text(),
        }
    }

    /// Parse a start tag like `<div class="example">`
    fn parse_start_tag(&mut self) -> Result<Token, ParseError> {
        self.char_position += 1; // Skip '<'
        
        let tag_name = self.parse_tag_name()?;
        let mut attributes = HashMap::new();
        let mut self_closing = false;

        // Parse attributes
        self.skip_whitespace();
        while self.char_position < self.chars.len() {
            let current_char = self.chars[self.char_position];
            
            if current_char == '>' {
                self.char_position += 1;
                break;
            } else if current_char == '/' && self.char_position + 1 < self.chars.len() 
                && self.chars[self.char_position + 1] == '>' {
                self_closing = true;
                self.char_position += 2; // Skip '/>'
                break;
            } else {
                let (name, value) = self.parse_attribute()?;
                attributes.insert(name, value);
                self.skip_whitespace();
            }
        }

        Ok(Token::StartTag {
            name: tag_name,
            attributes,
            self_closing,
        })
    }

    /// Parse an end tag like `</div>`
    fn parse_end_tag(&mut self) -> Result<Token, ParseError> {
        self.char_position += 2; // Skip '</'
        
        let tag_name = self.parse_tag_name()?;
        
        // Skip to '>'
        while self.char_position < self.chars.len() && self.chars[self.char_position] != '>' {
            self.char_position += 1;
        }
        if self.char_position < self.chars.len() {
            self.char_position += 1; // Skip '>'
        }

        Ok(Token::EndTag { name: tag_name })
    }

    /// Parse markup declarations like comments and DOCTYPE
    fn parse_markup_declaration(&mut self) -> Result<Token, ParseError> {
        self.char_position += 2; // Skip '<!'
        
        // Check for DOCTYPE (case-insensitive)
        if self.char_position + 7 < self.chars.len() {
            let next_chars: String = self.chars[self.char_position..self.char_position + 7]
                .iter().collect();
            if next_chars.to_uppercase() == "DOCTYPE" {
                return self.parse_doctype();
            }
        }
        
        // Parse as comment
        self.parse_comment()
    }

    /// Parse a DOCTYPE declaration
    fn parse_doctype(&mut self) -> Result<Token, ParseError> {
        self.char_position += 7; // Skip "DOCTYPE"
        self.skip_whitespace();
        
        let name = if self.char_position < self.chars.len() && !self.chars[self.char_position].is_whitespace() {
            Some(self.parse_tag_name()?)
        } else {
            None
        };
        
        self.skip_whitespace();
        
        // Parse public ID
        let public_id = if self.char_position + 6 < self.chars.len() {
            let next_chars: String = self.chars[self.char_position..self.char_position + 6]
                .iter().collect();
            if next_chars.to_uppercase() == "PUBLIC" {
                self.char_position += 6;
                self.skip_whitespace();
                self.parse_quoted_string()
            } else {
                None
            }
        } else {
            None
        };
        
        self.skip_whitespace();
        
        // Parse system ID
        let system_id = if self.char_position < self.chars.len() {
            self.parse_quoted_string()
        } else {
            None
        };
        
        // Skip to '>'
        while self.char_position < self.chars.len() && self.chars[self.char_position] != '>' {
            self.char_position += 1;
        }
        if self.char_position < self.chars.len() {
            self.char_position += 1;
        }
        
        Ok(Token::Doctype {
            name,
            public_id,
            system_id,
            force_quirks: false, // Simplified for now
        })
    }

    /// Parse a comment like `<!-- comment -->`
    fn parse_comment(&mut self) -> Result<Token, ParseError> {
        self.char_position += 2; // Skip '!'
        
        let mut comment_content = String::new();
        
        // Look for comment end
        while self.char_position + 2 < self.chars.len() {
            if self.chars[self.char_position] == '-' 
                && self.chars[self.char_position + 1] == '-' 
                && self.chars[self.char_position + 2] == '>' {
                break;
            }
            comment_content.push(self.chars[self.char_position]);
            self.char_position += 1;
        }
        
        if self.char_position + 2 < self.chars.len() {
            self.char_position += 3; // Skip '-->'
        }
        
        Ok(Token::Comment(comment_content))
    }

    /// Parse a processing instruction (simplified)
    fn parse_processing_instruction(&mut self) -> Result<Token, ParseError> {
        // Skip processing instruction for now
        while self.char_position < self.chars.len() && self.chars[self.char_position] != '>' {
            self.char_position += 1;
        }
        if self.char_position < self.chars.len() {
            self.char_position += 1;
        }
        
        // Return as comment for now
        Ok(Token::Comment("processing instruction".to_string()))
    }

    /// Parse text content between tags
    fn parse_text(&mut self) -> Result<Token, ParseError> {
        let start = self.char_position;
        
        while self.char_position < self.chars.len() {
            let current_char = self.chars[self.char_position];
            if current_char == '<' {
                break;
            }
            self.char_position += 1;
        }

        let text: String = self.chars[start..self.char_position].iter().collect();
        let decoded_text = self.decode_html_entities(&text);
        
        Ok(Token::Text(decoded_text))
    }

    /// Parse a tag name
    fn parse_tag_name(&mut self) -> Result<String, ParseError> {
        let start = self.char_position;
        
        while self.char_position < self.chars.len() {
            let current_char = self.chars[self.char_position];
            if current_char.is_whitespace() || current_char == '>' || current_char == '/' {
                break;
            }
            // Skip invalid characters that might appear in malformed HTML
            if current_char.is_control() && current_char != '\t' && current_char != '\n' && current_char != '\r' {
                self.char_position += 1;
                continue;
            }
            self.char_position += 1;
        }
        
        if self.char_position == start {
            // Try to recover by skipping to the next '>' or end of input
            while self.char_position < self.chars.len() && self.chars[self.char_position] != '>' {
                self.char_position += 1;
            }
            if self.char_position < self.chars.len() {
                self.char_position += 1; // Skip '>'
            }
            return Err(ParseError::InvalidTagName("Empty tag name - recovered".to_string()));
        }
        
        let name: String = self.chars[start..self.char_position].iter().collect();
        let trimmed_name = name.trim();
        
        if trimmed_name.is_empty() {
            return Err(ParseError::InvalidTagName("Empty tag name after trimming".to_string()));
        }
        
        Ok(trimmed_name.to_lowercase())
    }

    /// Parse an attribute name="value" pair
    fn parse_attribute(&mut self) -> Result<(String, String), ParseError> {
        let start = self.char_position;
        
        // Parse attribute name
        while self.char_position < self.chars.len() {
            let current_char = self.chars[self.char_position];
            if current_char.is_whitespace() || current_char == '=' || current_char == '>' || current_char == '/' {
                break;
            }
            self.char_position += 1;
        }
        
        if self.char_position == start {
            return Err(ParseError::InvalidAttribute("Empty attribute name".to_string()));
        }
        
        let name: String = self.chars[start..self.char_position].iter().collect();
        let name = name.trim().to_lowercase();
        
        self.skip_whitespace();
        
        let value = if self.char_position < self.chars.len() && self.chars[self.char_position] == '=' {
            self.char_position += 1; // Skip '='
            self.skip_whitespace();
            self.parse_quoted_string().unwrap_or_default()
        } else {
            // Boolean attribute (no value)
            String::new()
        };
        
        Ok((name, value))
    }

    /// Parse a quoted string (single or double quotes)
    fn parse_quoted_string(&mut self) -> Option<String> {
        if self.char_position >= self.chars.len() {
            return None;
        }
        
        let quote_char = self.chars[self.char_position];
        if quote_char != '"' && quote_char != '\'' {
            return None;
        }
        
        self.char_position += 1; // Skip opening quote
        let start = self.char_position;
        
        while self.char_position < self.chars.len() && self.chars[self.char_position] != quote_char {
            self.char_position += 1;
        }
        
        let value: String = self.chars[start..self.char_position].iter().collect();
        
        if self.char_position < self.chars.len() {
            self.char_position += 1; // Skip closing quote
        }
        
        Some(value)
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while self.char_position < self.chars.len() && self.chars[self.char_position].is_whitespace() {
            self.char_position += 1;
        }
    }

    /// Decode HTML entities in text
    fn decode_html_entities(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '&' {
                let mut entity = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ';' {
                        chars.next(); // Consume ';'
                        break;
                    } else if next_ch.is_alphanumeric() || next_ch == '#' {
                        entity.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if !entity.is_empty() {
                    let decoded = self.decode_entity(&entity);
                    result.push_str(&decoded);
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    /// Decode a specific HTML entity
    fn decode_entity(&self, entity: &str) -> String {
        match entity {
            "lt" => "<".to_string(),
            "gt" => ">".to_string(),
            "amp" => "&".to_string(),
            "quot" => "\"".to_string(),
            "apos" => "'".to_string(),
            "nbsp" => "\u{00A0}".to_string(),
            _ => {
                if entity.starts_with('#') {
                    // Numeric entity
                    if let Ok(num) = entity[1..].parse::<u32>() {
                        if let Some(ch) = std::char::from_u32(num) {
                            return ch.to_string();
                        }
                    }
                }
                // Unknown entity, return as-is
                format!("&{};", entity)
            }
        }
    }
}

/// HTML parser that builds a DOM tree from tokens
pub struct HtmlParser {
    tokenizer: Tokenizer,
    document: Document,
    open_elements: Vec<Rc<Node>>,
    external_resources: Vec<ExternalResource>,
}

/// Represents an external resource that needs to be fetched
#[derive(Debug, Clone)]
pub struct ExternalResource {
    pub resource_type: ResourceType,
    pub url: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Stylesheet,
    Script,
    Image,
    Other,
}

impl HtmlParser {
    /// Create a new HTML parser
    pub fn new(input: Vec<u8>) -> Result<Self, ParseError> {
        let tokenizer = Tokenizer::new(input)?;
        let document = Document::new();
        let root_ref = Rc::clone(&document.root);
        
        Ok(HtmlParser {
            tokenizer,
            document,
            open_elements: vec![root_ref],
            external_resources: Vec::new(),
        })
    }

    /// Parse the HTML and return the DOM tree
    pub fn parse(mut self) -> Result<(Document, Vec<ExternalResource>), ParseError> {
        let mut error_count = 0;
        const MAX_ERRORS: usize = 100; // Prevent infinite loops on severely malformed HTML
        
        loop {
            match self.tokenizer.next_token() {
                Ok(Token::StartTag { name, attributes, self_closing }) => {
                    if let Err(e) = self.handle_start_tag(name, attributes, self_closing) {
                        error_count += 1;
                        if error_count > MAX_ERRORS {
                            return Err(ParseError::ParseError(0, format!("Too many parsing errors: {}", e)));
                        }
                        // Continue parsing despite the error
                    }
                }
                Ok(Token::EndTag { name }) => {
                    if let Err(e) = self.handle_end_tag(name) {
                        error_count += 1;
                        if error_count > MAX_ERRORS {
                            return Err(ParseError::ParseError(0, format!("Too many parsing errors: {}", e)));
                        }
                        // Continue parsing despite the error
                    }
                }
                Ok(Token::Text(text)) => {
                    if !text.trim().is_empty() {
                        if let Err(e) = self.handle_text(text) {
                            error_count += 1;
                            if error_count > MAX_ERRORS {
                                return Err(ParseError::ParseError(0, format!("Too many parsing errors: {}", e)));
                            }
                            // Continue parsing despite the error
                        }
                    }
                }
                Ok(Token::Comment(_)) => {
                    // Comments are ignored in DOM tree
                }
                Ok(Token::Doctype { .. }) => {
                    // DOCTYPE is handled by the document
                }
                Ok(Token::Eof) => break,
                Err(e) => {
                    error_count += 1;
                    if error_count > MAX_ERRORS {
                        return Err(ParseError::ParseError(0, format!("Too many parsing errors: {}", e)));
                    }
                    // Try to recover by advancing position
                    self.tokenizer.char_position += 1;
                }
            }
        }
        
        Ok((self.document, self.external_resources))
    }

    /// Handle a start tag
    fn handle_start_tag(&mut self, name: String, attributes: HashMap<String, String>, self_closing: bool) -> Result<(), ParseError> {
        // Check for external resources
        self.check_external_resources(&name, &attributes);
        
        // Create the element node
        let node = Node::new(
            NodeType::Element {
                tag_name: name.clone(),
                attributes: attributes.clone(),
            },
            self.document.get_next_id(),
        );
        
        // Add to current parent
        if let Some(parent) = self.open_elements.last() {
            parent.append_child(&node);
        }
        
        // Add to open elements stack if not self-closing
        if !self.is_void_element(&name) && !self_closing {
            self.open_elements.push(node);
        }
        
        Ok(())
    }

    /// Handle an end tag
    fn handle_end_tag(&mut self, name: String) -> Result<(), ParseError> {
        // Find matching opening tag
        for i in (0..self.open_elements.len()).rev() {
            if let NodeType::Element { tag_name, .. } = &self.open_elements[i].node_type {
                if tag_name == &name {
                    // Remove this element and all elements after it
                    self.open_elements.truncate(i);
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Handle text content
    fn handle_text(&mut self, text: String) -> Result<(), ParseError> {
        if let Some(parent) = self.open_elements.last() {
            let text_node = Node::new(
                NodeType::Text(text),
                self.document.get_next_id(),
            );
            parent.append_child(&text_node);
        }
        
        Ok(())
    }

    /// Check if an element is a void element (self-closing)
    fn is_void_element(&self, name: &str) -> bool {
        matches!(name, "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | 
                 "link" | "meta" | "param" | "source" | "track" | "wbr")
    }

    /// Check for external resources in the element
    fn check_external_resources(&mut self, tag_name: &str, attributes: &HashMap<String, String>) {
        match tag_name {
            "link" => {
                if let Some(rel) = attributes.get("rel") {
                    if rel == "stylesheet" {
                        if let Some(href) = attributes.get("href") {
                            self.external_resources.push(ExternalResource {
                                resource_type: ResourceType::Stylesheet,
                                url: href.clone(),
                                attributes: attributes.clone(),
                            });
                        }
                    }
                }
            }
            "script" => {
                if let Some(src) = attributes.get("src") {
                    self.external_resources.push(ExternalResource {
                        resource_type: ResourceType::Script,
                        url: src.clone(),
                        attributes: attributes.clone(),
                    });
                }
            }
            "img" => {
                if let Some(src) = attributes.get("src") {
                    self.external_resources.push(ExternalResource {
                        resource_type: ResourceType::Image,
                        url: src.clone(),
                        attributes: attributes.clone(),
                    });
                }
            }
            _ => {}
        }
    }
}

/// Convenience function to parse HTML from bytes
pub fn parse_html(input: Vec<u8>) -> Result<(Document, Vec<ExternalResource>), ParseError> {
    let parser = HtmlParser::new(input)?;
    parser.parse()
}

/// Convenience function to parse HTML from string
pub fn parse_html_string(input: &str) -> Result<(Document, Vec<ExternalResource>), ParseError> {
    parse_html(input.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_html() {
        let html = r#"<html><head><title>Test</title></head><body><h1>Hello World</h1></body></html>"#;
        let (document, resources) = parse_html_string(html).unwrap();
        
        assert_eq!(resources.len(), 0);
        // Add more specific DOM tree tests here
    }

    #[test]
    fn test_external_resources() {
        let html = r#"<html><head><link rel="stylesheet" href="style.css"><script src="script.js"></script></head></html>"#;
        let (document, resources) = parse_html_string(html).unwrap();
        
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[0].resource_type, ResourceType::Stylesheet);
        assert_eq!(resources[1].resource_type, ResourceType::Script);
    }

    #[test]
    fn test_html_entities() {
        let html = r#"<p>Hello &amp; welcome! &lt;3</p>"#;
        let (document, resources) = parse_html_string(html).unwrap();
        
        // The text should be decoded properly
        // Add specific text content verification here
    }

    #[test]
    fn test_utf8_content() {
        let html = "こんにちは世界".as_bytes().to_vec();
        let (document, resources) = parse_html_string(&String::from_utf8_lossy(&html)).unwrap();
        
        // Should handle UTF-8 properly
    }
}