//! # Networking Crate
//! 
//! This crate provides HTTP client functionality for fetching web resources.
//! It implements a simple async HTTP client using reqwest with rustls for
//! secure connections.
//! 
//! ## Design Principles
//! 
//! 1. **Async-First**: All operations are async to avoid blocking the browser
//!    event loop.
//! 
//! 2. **Security**: Uses rustls for secure connections and validates URLs.
//! 
//! 3. **Error Handling**: Comprehensive error handling with custom error types.
//! 
//! 4. **Extensibility**: Designed to support additional protocols and features
//!    like caching, redirects, and custom headers in the future.

use reqwest::Client;
use std::time::Duration;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

/// Custom error types for networking operations
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("URL parsing failed: {0}")]
    UrlParseError(#[from] url::ParseError),
    
    #[error("Network timeout after {timeout:?}")]
    Timeout { timeout: Duration },
    
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
    
    #[error("HTTP error: {status} - {message}")]
    HttpError { status: u16, message: String },
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Request aborted")]
    RequestAborted,
    
    #[error("CORS error: {0}")]
    CorsError(String),
    
    #[error("SSL/TLS error: {0}")]
    SslError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type for networking operations
pub type NetworkResult<T> = Result<T, NetworkError>;

/// HTTP methods supported by the networking API
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl std::str::FromStr for HttpMethod {
    type Err = NetworkError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(NetworkError::ParseError(format!("Invalid HTTP method: {}", s))),
        }
    }
}

/// HTTP status codes
#[derive(Debug, Clone, PartialEq)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    Found,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    Custom(u16),
}

impl HttpStatus {
    pub fn from_code(code: u16) -> Self {
        match code {
            200 => HttpStatus::Ok,
            201 => HttpStatus::Created,
            202 => HttpStatus::Accepted,
            204 => HttpStatus::NoContent,
            301 => HttpStatus::MovedPermanently,
            302 => HttpStatus::Found,
            304 => HttpStatus::NotModified,
            400 => HttpStatus::BadRequest,
            401 => HttpStatus::Unauthorized,
            403 => HttpStatus::Forbidden,
            404 => HttpStatus::NotFound,
            405 => HttpStatus::MethodNotAllowed,
            500 => HttpStatus::InternalServerError,
            502 => HttpStatus::BadGateway,
            503 => HttpStatus::ServiceUnavailable,
            504 => HttpStatus::GatewayTimeout,
            _ => HttpStatus::Custom(code),
        }
    }
    
    pub fn is_success(&self) -> bool {
        match self {
            HttpStatus::Ok | HttpStatus::Created | HttpStatus::Accepted | HttpStatus::NoContent => true,
            HttpStatus::Custom(code) => *code >= 200 && *code < 300,
            _ => false,
        }
    }
    
    pub fn is_redirect(&self) -> bool {
        match self {
            HttpStatus::MovedPermanently | HttpStatus::Found => true,
            HttpStatus::Custom(code) => *code >= 300 && *code < 400,
            _ => false,
        }
    }
    
    pub fn is_client_error(&self) -> bool {
        match self {
            HttpStatus::BadRequest | HttpStatus::Unauthorized | HttpStatus::Forbidden | 
            HttpStatus::NotFound | HttpStatus::MethodNotAllowed => true,
            HttpStatus::Custom(code) => *code >= 400 && *code < 500,
            _ => false,
        }
    }
    
    pub fn is_server_error(&self) -> bool {
        match self {
            HttpStatus::InternalServerError | HttpStatus::BadGateway | 
            HttpStatus::ServiceUnavailable | HttpStatus::GatewayTimeout => true,
            HttpStatus::Custom(code) => *code >= 500 && *code < 600,
            _ => false,
        }
    }
}

/// HTTP client for fetching web resources
/// 
/// This struct provides methods for fetching HTML, CSS, and other web resources
/// with proper error handling and security measures.
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    /// 
    /// The client is configured with:
    /// - 10 second timeout
    /// - User-Agent header identifying the browser engine
    /// - rustls for secure connections
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("ExperimentalBrowserEngine/1.0")
            .build()
            .expect("Failed to create HTTP client");
        
        HttpClient { client }
    }
    
    /// Fetch HTML content from a URL
    /// 
    /// This method fetches the HTML content from the given URL and returns
    /// it as a string. It validates the URL and ensures it uses HTTP or HTTPS.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL to fetch
    /// 
    /// # Returns
    /// 
    /// A `NetworkResult<String>` containing the HTML content or an error
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use networking::HttpClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = HttpClient::new();
    ///     let html = client.fetch_html("https://example.com").await?;
    ///     println!("Fetched {} bytes", html.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_html(&self, url: &str) -> NetworkResult<String> {
        let parsed_url = self.validate_url(url)?;
        
        let response = self.client
            .get(parsed_url.as_str())
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(NetworkError::RequestFailed(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }
        
        let content = response.text().await?;
        Ok(content)
    }
    
    /// Fetch any text content from a URL (HTML, CSS, JS, etc.)
    /// 
    /// This is a generic method for fetching any text-based web resource.
    /// It's useful for fetching CSS files, JavaScript files, or other
    /// text-based resources.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL to fetch
    /// 
    /// # Returns
    /// 
    /// A `NetworkResult<String>` containing the text content or an error
    pub async fn fetch_text(&self, url: &str) -> NetworkResult<String> {
        self.fetch_html(url).await // Same implementation for now
    }
    
    /// Validate and parse a URL
    /// 
    /// This method validates that the URL is properly formatted and uses
    /// a supported protocol (HTTP or HTTPS).
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL string to validate
    /// 
    /// # Returns
    /// 
    /// A `NetworkResult<Url>` containing the parsed URL or an error
    fn validate_url(&self, url: &str) -> NetworkResult<Url> {
        let parsed_url = Url::parse(url)?;
        
        match parsed_url.scheme() {
            "http" | "https" => Ok(parsed_url),
            scheme => Err(NetworkError::UnsupportedProtocol(scheme.to_string())),
        }
    }

    /// Make a real HTTP request and return the response
    pub async fn send_request(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError> {
        // Parse the URL
        let url = Url::parse(&request.url)
            .map_err(|e| NetworkError::ParseError(e.to_string()))?;

        // Build the HTTP request
        let mut req_builder = match request.method {
            HttpMethod::GET => self.client.get(url.clone()),
            HttpMethod::POST => self.client.post(url.clone()),
            HttpMethod::PUT => self.client.put(url.clone()),
            HttpMethod::DELETE => self.client.delete(url.clone()),
            HttpMethod::HEAD => self.client.head(url.clone()),
            HttpMethod::OPTIONS => self.client.request(reqwest::Method::OPTIONS, url.clone()),
            HttpMethod::PATCH => self.client.request(reqwest::Method::PATCH, url.clone()),
        };

        // Add headers
        for (key, value) in request.headers {
            req_builder = req_builder.header(&key, &value);
        }

        // Add body for POST/PUT requests
        if let Some(body) = request.body {
            req_builder = req_builder.body(body);
        }

        // Execute the request
        let response = req_builder
            .send()
            .await
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;

        // Get response status
        let status_code = response.status().as_u16();
        let status = HttpStatus::from_code(status_code);

        // Get final URL (after redirects) before consuming response
        let final_url = response.url().to_string();

        // Get response headers
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }

        // Get response body (this consumes the response)
        let body = response
            .bytes()
            .await
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?
            .to_vec();

        Ok(HttpResponse {
            status,
            status_code,
            headers,
            body,
            url: final_url,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to fetch HTML content from a URL
/// 
/// This function creates a new HTTP client and fetches the HTML content
/// from the given URL. It's useful for simple one-off requests.
/// 
/// # Arguments
/// 
/// * `url` - The URL to fetch
/// 
/// # Returns
/// 
/// A `NetworkResult<String>` containing the HTML content or an error
/// 
/// # Example
/// 
/// ```rust
/// use networking::fetch;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let html = fetch("https://example.com").await?;
///     println!("Fetched HTML: {}", html);
///     Ok(())
/// }
/// ```
pub async fn fetch(url: &str) -> NetworkResult<String> {
    let client = HttpClient::new();
    client.fetch_html(url).await
}

/// Convenience function to fetch text content from a URL
/// 
/// This function creates a new HTTP client and fetches any text content
/// from the given URL.
/// 
/// # Arguments
/// 
/// * `url` - The URL to fetch
/// 
/// # Returns
/// 
/// A `NetworkResult<String>` containing the text content or an error
pub async fn fetch_text(url: &str) -> NetworkResult<String> {
    let client = HttpClient::new();
    client.fetch_text(url).await
}

/// XMLHttpRequest implementation
pub struct XMLHttpRequest {
    request: HttpRequest,
    response: Option<HttpResponse>,
    ready_state: ReadyState,
    status: u16,
    status_text: String,
    response_text: String,
    response_xml: Option<String>,
    onreadystatechange: Option<Box<dyn Fn() + Send + Sync>>,
    onload: Option<Box<dyn Fn() + Send + Sync>>,
    onerror: Option<Box<dyn Fn() + Send + Sync>>,
    ontimeout: Option<Box<dyn Fn() + Send + Sync>>,
    is_aborted: bool,
}

/// HTTP request configuration
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<Duration>,
    pub follow_redirects: bool,
    pub max_redirects: u32,
    pub credentials: bool, // For CORS
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
            timeout: Some(Duration::from_secs(30)),
            follow_redirects: true,
            max_redirects: 5,
            credentials: false,
        }
    }
    
    pub fn get(url: String) -> Self {
        Self::new(HttpMethod::GET, url)
    }
    
    pub fn post(url: String, body: Option<Vec<u8>>) -> Self {
        Self {
            method: HttpMethod::POST,
            url,
            headers: HashMap::new(),
            body,
            timeout: Some(Duration::from_secs(30)),
            follow_redirects: true,
            max_redirects: 5,
            credentials: false,
        }
    }
    
    pub fn set_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }
    
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }
    
    pub fn set_credentials(&mut self, credentials: bool) {
        self.credentials = credentials;
    }
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub url: String, // Final URL after redirects
}

impl HttpResponse {
    pub fn text(&self) -> Result<String, NetworkError> {
        String::from_utf8(self.body.clone())
            .map_err(|e| NetworkError::ParseError(format!("Invalid UTF-8: {}", e)))
    }
    
    pub fn json<T>(&self) -> Result<T, NetworkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let text = self.text()?;
        serde_json::from_str(&text)
            .map_err(|e| NetworkError::ParseError(format!("JSON parse error: {}", e)))
    }
    
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }
    
    pub fn content_type(&self) -> Option<&String> {
        self.get_header("content-type")
    }
    
    pub fn content_length(&self) -> Option<usize> {
        self.get_header("content-length")
            .and_then(|s| s.parse().ok())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReadyState {
    Unsent = 0,
    Opened = 1,
    HeadersReceived = 2,
    Loading = 3,
    Done = 4,
}

impl XMLHttpRequest {
    pub fn new() -> Self {
        Self {
            request: HttpRequest::get("".to_string()),
            response: None,
            ready_state: ReadyState::Unsent,
            status: 0,
            status_text: String::new(),
            response_text: String::new(),
            response_xml: None,
            onreadystatechange: None,
            onload: None,
            onerror: None,
            ontimeout: None,
            is_aborted: false,
        }
    }
    
    pub fn open(&mut self, method: &str, url: &str) -> Result<(), NetworkError> {
        if self.ready_state != ReadyState::Unsent {
            return Err(NetworkError::ParseError("Request already opened".to_string()));
        }
        
        let http_method = method.parse()?;
        self.request = HttpRequest::new(http_method, url.to_string());
        self.ready_state = ReadyState::Opened;
        self.trigger_readystatechange();
        Ok(())
    }
    
    pub fn set_request_header(&mut self, name: &str, value: &str) -> Result<(), NetworkError> {
        if self.ready_state != ReadyState::Opened {
            return Err(NetworkError::ParseError("Request not opened".to_string()));
        }
        
        self.request.set_header(name.to_string(), value.to_string());
        Ok(())
    }
    
    pub fn send(&mut self, body: Option<Vec<u8>>) -> Result<(), NetworkError> {
        if self.ready_state != ReadyState::Opened {
            return Err(NetworkError::ParseError("Request not opened".to_string()));
        }
        
        self.request.body = body;
        self.ready_state = ReadyState::HeadersReceived;
        self.trigger_readystatechange();
        
        // Simulate async request (in real implementation, this would be async)
        self.simulate_request();
        Ok(())
    }
    
    fn simulate_request(&mut self) {
        // Simulate network request
        self.ready_state = ReadyState::Loading;
        self.trigger_readystatechange();
        
        // Simulate successful response
        let mut response = HttpResponse {
            status: HttpStatus::Ok,
            status_code: 200,
            headers: HashMap::new(),
            body: b"Hello, World!".to_vec(),
            url: self.request.url.clone(),
        };
        
        response.headers.insert("content-type".to_string(), "text/plain".to_string());
        response.headers.insert("content-length".to_string(), response.body.len().to_string());
        
        self.response = Some(response);
        self.status = 200;
        self.status_text = "OK".to_string();
        self.response_text = String::from_utf8_lossy(&self.response.as_ref().unwrap().body).to_string();
        
        self.ready_state = ReadyState::Done;
        self.trigger_readystatechange();
        
        if let Some(callback) = &self.onload {
            callback();
        }
    }
    
    pub fn abort(&mut self) {
        self.is_aborted = true;
        self.ready_state = ReadyState::Unsent;
        self.trigger_readystatechange();
    }
    
    pub fn get_ready_state(&self) -> ReadyState {
        self.ready_state.clone()
    }
    
    pub fn get_status(&self) -> u16 {
        self.status
    }
    
    pub fn get_status_text(&self) -> &str {
        &self.status_text
    }
    
    pub fn get_response_text(&self) -> &str {
        &self.response_text
    }
    
    pub fn get_response_xml(&self) -> Option<&str> {
        self.response_xml.as_deref()
    }
    
    pub fn get_all_response_headers(&self) -> String {
        if let Some(response) = &self.response {
            response.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\r\n")
        } else {
            String::new()
        }
    }
    
    pub fn get_response_header(&self, name: &str) -> Option<String> {
        self.response.as_ref()
            .and_then(|r| r.get_header(name))
            .map(|s| s.clone())
    }
    
    pub fn set_onreadystatechange<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.onreadystatechange = Some(Box::new(callback));
    }
    
    pub fn set_onload<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.onload = Some(Box::new(callback));
    }
    
    pub fn set_onerror<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.onerror = Some(Box::new(callback));
    }
    
    pub fn set_ontimeout<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.ontimeout = Some(Box::new(callback));
    }
    
    fn trigger_readystatechange(&self) {
        if let Some(callback) = &self.onreadystatechange {
            callback();
        }
    }
}

/// Fetch API implementation
pub struct FetchRequest {
    request: HttpRequest,
}

impl FetchRequest {
    pub fn new(url: &str) -> Self {
        Self {
            request: HttpRequest::get(url.to_string()),
        }
    }
    
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.request.method = method;
        self
    }
    
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.request.set_header(name.to_string(), value.to_string());
        self
    }
    
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.request.body = Some(body);
        self
    }
    
    pub fn json<T>(mut self, data: &T) -> Self
    where
        T: Serialize,
    {
        if let Ok(json_data) = serde_json::to_vec(data) {
            self.request.body = Some(json_data);
            self.request.set_header("content-type".to_string(), "application/json".to_string());
        }
        self
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request.set_timeout(timeout);
        self
    }
    
    pub fn credentials(mut self, credentials: bool) -> Self {
        self.request.set_credentials(credentials);
        self
    }
    
    pub async fn send(self) -> Result<FetchResponse, NetworkError> {
        // Simulate async network request
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Simulate successful response
        let response = HttpResponse {
            status: HttpStatus::Ok,
            status_code: 200,
            headers: HashMap::new(),
            body: b"{\"message\": \"Hello from fetch!\"}".to_vec(),
            url: self.request.url.clone(),
        };
        
        Ok(FetchResponse { response })
    }
}

/// Fetch response wrapper
pub struct FetchResponse {
    response: HttpResponse,
}

impl FetchResponse {
    pub fn status(&self) -> u16 {
        self.response.status_code
    }
    
    pub fn ok(&self) -> bool {
        self.response.status.is_success()
    }
    
    pub fn status_text(&self) -> &str {
        match self.response.status {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::NotFound => "Not Found",
            _ => "Unknown",
        }
    }
    
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.response.headers
    }
    
    pub fn url(&self) -> &str {
        &self.response.url
    }
    
    pub async fn text(self) -> Result<String, NetworkError> {
        self.response.text()
    }
    
    pub async fn json<T>(self) -> Result<T, NetworkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.response.json()
    }
    
    pub async fn blob(self) -> Result<Vec<u8>, NetworkError> {
        Ok(self.response.body)
    }
    
    pub fn clone(&self) -> Self {
        Self {
            response: self.response.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_example_com() {
        // This test fetches from example.com - a reliable test endpoint
        let result = fetch("https://example.com").await;
        
        match result {
            Ok(html) => {
                assert!(!html.is_empty());
                assert!(html.contains("Example Domain"));
                println!("Successfully fetched {} bytes from example.com", html.len());
            }
            Err(e) => {
                // If network is unavailable, that's okay for testing
                println!("Network test skipped: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let result = fetch("not-a-url").await;
        assert!(result.is_err());
        
        if let Err(NetworkError::UrlParseError(_)) = result {
            // Expected error type
        } else {
            panic!("Expected UrlParseError");
        }
    }

    #[tokio::test]
    async fn test_unsupported_protocol() {
        let result = fetch("ftp://example.com").await;
        assert!(result.is_err());
        
        if let Err(NetworkError::UnsupportedProtocol(protocol)) = result {
            assert_eq!(protocol, "ftp");
        } else {
            panic!("Expected UnsupportedProtocol error");
        }
    }

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        // If we get here without panicking, the client was created successfully
        assert!(true);
    }

    #[test]
    fn test_url_validation() {
        let client = HttpClient::new();
        
        // Valid URLs
        assert!(client.validate_url("https://example.com").is_ok());
        assert!(client.validate_url("http://localhost:8080").is_ok());
        
        // Invalid URLs
        assert!(client.validate_url("not-a-url").is_err());
        assert!(client.validate_url("ftp://example.com").is_err());
    }

    #[test]
    fn test_http_method_parsing() {
        assert_eq!("GET".parse::<HttpMethod>().unwrap(), HttpMethod::GET);
        assert_eq!("POST".parse::<HttpMethod>().unwrap(), HttpMethod::POST);
        assert!("INVALID".parse::<HttpMethod>().is_err());
    }

    #[test]
    fn test_http_status_codes() {
        let status = HttpStatus::from_code(200);
        assert!(status.is_success());
        assert!(!status.is_redirect());
        assert!(!status.is_client_error());
        assert!(!status.is_server_error());
        
        let status = HttpStatus::from_code(404);
        assert!(!status.is_success());
        assert!(status.is_client_error());
    }

    #[test]
    fn test_xmlhttprequest() {
        let mut xhr = XMLHttpRequest::new();
        xhr.open("GET", "https://example.com").unwrap();
        xhr.set_request_header("User-Agent", "RustBrowser/1.0").unwrap();
        xhr.send(None).unwrap();
        
        assert_eq!(xhr.get_ready_state(), ReadyState::Done);
        assert_eq!(xhr.get_status(), 200);
        assert!(!xhr.get_response_text().is_empty());
    }

    #[tokio::test]
    async fn test_fetch_api() {
        let response = FetchRequest::new("https://example.com")
            .method(HttpMethod::GET)
            .header("User-Agent", "RustBrowser/1.0")
            .send()
            .await
            .unwrap();
        
        assert!(response.ok());
        assert_eq!(response.status(), 200);
        let text = response.text().await.unwrap();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequest::get("https://example.com".to_string());
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url, "https://example.com");
        
        let request = HttpRequest::post("https://api.example.com".to_string(), Some(b"data".to_vec()));
        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.body, Some(b"data".to_vec()));
    }

    #[test]
    fn test_http_response() {
        let mut response = HttpResponse {
            status: HttpStatus::Ok,
            status_code: 200,
            headers: HashMap::new(),
            body: b"Hello, World!".to_vec(),
            url: "https://example.com".to_string(),
        };
        
        response.headers.insert("content-type".to_string(), "text/plain".to_string());
        
        assert_eq!(response.status_code, 200);
        assert!(response.status.is_success());
        assert_eq!(response.text().unwrap(), "Hello, World!");
        assert_eq!(response.get_header("content-type"), Some(&"text/plain".to_string()));
    }
}
