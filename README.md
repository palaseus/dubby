# Dubby Browser Engine

A web browser engine implementation in Rust, designed to be modular, extensible, and educational. This project implements a complete browser engine from scratch without relying on existing engines like Chromium, WebKit, or Gecko.

## Features

- **HTML Parser**: Complete HTML5 parsing with proper error recovery
- **CSS Engine**: CSS parsing, cascade, and layout calculation
- **Layout Engine**: Block and inline layout with flexbox support
- **JavaScript Runtime**: Boa engine integration with DOM bindings
- **Event System**: Complete event handling and propagation
- **GPU Rendering**: Hardware-accelerated rendering with WGPU
- **Networking**: HTTP client with async support
- **Modern JavaScript**: Promises, microtasks, and fetch API

## Architecture

The engine is built as a collection of modular crates:

```
dubby/
├── dom/                 # Document Object Model
├── html_parser/         # HTML5 parsing
├── css_parser/          # CSS parsing and cascade
├── layout/              # Layout calculation
├── renderer/            # Software rendering
├── renderer_wgpu/       # GPU rendering
├── js_integration/      # JavaScript engine integration
├── networking/          # HTTP client
├── browser_shell/       # Command-line interface
└── event_loop/          # Event processing
```

## Building

### Prerequisites

- Rust 1.70 or later
- Git

### Compilation

```bash
git clone <repository-url>
cd dubby
cargo build --release
```

### Testing

```bash
cargo test
```

## Usage

### Command Line Interface

The browser engine provides a command-line interface for loading and rendering web pages:

```bash
# Load a local HTML file
cargo run --bin browser_shell -- --load-file example.html

# Load a URL
cargo run --bin browser_shell -- --load-url https://example.com

# Enable JavaScript tracing
cargo run --bin browser_shell -- --trace-js --load-file example.html

# Set fetch timeout
cargo run --bin browser_shell -- --fetch-timeout 5000 --load-url https://example.com
```

### Demo Modes

```bash
# Run Promise and microtask demo
cargo run --bin browser_shell -- --demo promise

# Run fetch API demo
cargo run --bin browser_shell -- --demo fetch

# Run comprehensive demo
cargo run --bin browser_shell -- --demo comprehensive
```

## JavaScript Support

The engine includes a complete JavaScript runtime with:

- **ES6+ Features**: Arrow functions, classes, modules
- **DOM API**: Element manipulation, event handling
- **Promises**: Native Promise implementation with microtask queue
- **Fetch API**: HTTP requests with async/await support
- **AbortController**: Request cancellation
- **Timers**: setTimeout and setInterval

### Example JavaScript

```javascript
// Promise and async/await
async function loadData() {
    try {
        const response = await fetch('/api/data');
    const data = await response.json();
        document.getElementById('content').textContent = data.message;
  } catch (error) {
        console.error('Failed to load data:', error);
    }
}

// Event handling
document.addEventListener('click', (event) => {
    console.log('Clicked:', event.target);
});

// DOM manipulation
const element = document.createElement('div');
element.textContent = 'Hello, World!';
document.body.appendChild(element);
```

## Performance

The engine includes comprehensive performance monitoring:

- **Layout Metrics**: Box calculation times, tree depth
- **Rendering Stats**: GPU draw calls, frame times
- **JavaScript Performance**: Execution times, memory usage
- **Network Metrics**: Request latency, throughput

## Development

### Project Structure

Each crate is designed to be independently testable and reusable:

- **dom/**: Core DOM tree implementation
- **html_parser/**: HTML5 tokenization and parsing
- **css_parser/**: CSS parsing with specificity calculation
- **layout/**: Box model and layout algorithms
- **renderer_wgpu/**: GPU-accelerated rendering
- **js_integration/**: JavaScript engine integration
- **networking/**: HTTP client with async support

### Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p dom
cargo test -p css_parser
cargo test -p js_integration

# Run with output
cargo test -- --nocapture
```

### Code Quality

The project maintains high code quality standards:

- All compiler warnings resolved
- Comprehensive test coverage
- Modular architecture
- Clear documentation

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome. Please ensure:

1. All tests pass
2. No compiler warnings
3. Code follows Rust conventions
4. Documentation is updated

## Status

The browser engine is functional and includes all major components:

- HTML parsing and DOM construction
- CSS parsing and style calculation
- Layout engine with flexbox support
- GPU-accelerated rendering
- JavaScript runtime with modern features
- Event system and user interaction
- Network requests and resource loading

The engine can successfully load and render web pages, execute JavaScript, and handle user interactions.