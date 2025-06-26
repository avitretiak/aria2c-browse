# aria2c-browse

A minimal Windows URL scheme handler for aria2c that opens downloaded files and folders in Windows Explorer.
Intended to be registered as a URL handler for `aria2://` URL schemes.

## Features

- **Zero dependencies**: Uses only Rust standard library
- **Minimal binary**: Optimized for size and performance
- **Robust**: Handles URL-encoded paths gracefully with comprehensive error handling
- **Windows-native**: Intended to be registered as `aria2://` URL scheme in Windows registry via the provided batch file
- **Explorer integration**: Opens directories directly and if possible focuses the targeted file

## Quick Start

### Build
```bash
cargo build --release
```

### Install to System Path (Example)
```bash
# Copy the compiled executable to C:\bin (create directory if it doesn't exist)
mkdir -p /c/bin
cp target/release/aria2c-browse.exe /c/bin/

# Add C:\bin to PATH if not already present (optional)
# You can add this to your ~/.bashrc or ~/.profile
export PATH="/c/bin:$PATH"
```

### Register URL Scheme
```bash
register_handler.bat
```

The script will auto-detect the executable location and handle existing registrations.

## Usage

The application is designed to be used as a protocol handler, not directly from command line, if opened without an argument, it will fall back to `%USERPROFILE/Downloads`.

### URL Scheme
The handler processes URLs with format:
```
aria2://browse/path=<url-encoded-path>
```

Examples:
```
aria2://browse/path=C%3A%5CUsers%5Cusername%5CDownloads
aria2://browse/path=C%3A%5CUsers%5Cusername%5CDocuments%5Cfile.txt
aria2://browse/path=D%3A%5CProgram%20Files%5CApp%5Cconfig.ini
```

## Development

### Testing
```bash
# Run all tests
cargo test

# Run specific test categories
cargo test test_url_decode
cargo test test_integration
```
