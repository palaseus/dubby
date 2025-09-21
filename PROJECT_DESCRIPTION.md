# Dubby - A Modern Rust Browser Engine

**Dubby** is a high-performance, modular browser engine written entirely in Rust, designed to demonstrate modern web browser architecture and showcase the power of systems programming for web technologies.

## 🚀 Key Features

- **Complete Browser Pipeline**: Full end-to-end implementation from HTTP networking to GPU-accelerated rendering
- **Real Website Loading**: Successfully loads and processes real websites including Google, GitHub, and Wikipedia
- **Modular Architecture**: Clean separation of concerns across specialized crates
- **High Performance**: Sub-second loading times with detailed performance metrics
- **Cross-Platform**: Built with Rust's excellent cross-platform support

## 🏗️ Architecture

**Dubby** is organized into focused, reusable crates:

- **`html_parser`** - Robust HTML5 parsing with UTF-8 support
- **`css_parser`** - CSS3 parsing with cascade and specificity handling  
- **`dom`** - Document Object Model with event system
- **`layout`** - Advanced CSS layout engine with box model
- **`renderer_wgpu`** - GPU-accelerated rendering using WGPU
- **`js_integration`** - JavaScript engine integration with Boa
- **`networking`** - HTTP client with modern web standards
- **`browser_shell`** - Complete browser application

## 📊 Performance

- **Google.com**: 114 DOM nodes processed in 176ms
- **Wikipedia**: 3,119 DOM nodes processed in 388ms  
- **Example.com**: 19 DOM nodes processed in 275ms

## 🎯 Goals

This project serves as both a learning resource and a demonstration of:
- Modern browser engine architecture
- Rust's capabilities for systems programming
- Performance optimization techniques
- Clean, modular software design

## 🛠️ Technology Stack

- **Language**: Rust 2021 Edition
- **Graphics**: WGPU for cross-platform GPU rendering
- **JavaScript**: Boa JavaScript engine integration
- **Networking**: Tokio async runtime with HTTP/2 support
- **Parsing**: Custom parsers for HTML5 and CSS3

## 🚀 Getting Started

```bash
# Clone and build
git clone https://github.com/yourusername/dubby.git
cd dubby
cargo build

# Load a real website
cargo run --bin demo_real_websites

# GPU rendering with screenshot
cargo run --bin render_real_website https://example.com --screenshot
```

**Dubby** represents the future of browser engines - fast, safe, and built with modern systems programming principles.
