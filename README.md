# aria2c-browse

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Windows](https://img.shields.io/badge/platform-Windows-blue.svg)](https://www.microsoft.com/windows)
[![Release](https://github.com/avitretiak/aria2c-browse/actions/workflows/release.yml/badge.svg)](https://github.com/avitretiak/aria2c-browse/actions/workflows/release.yml)

A minimal Windows URL scheme handler for aria2c that opens downloaded files and folders in Windows Explorer.
Intended to be registered as a URL handler for `aria2://` URL schemes.

## ✨ Features

- **Zero dependencies**: Uses only Rust standard library
- **Minimal binary**: Optimized for size and performance (~50KB executable)
- **Robust**: Handles URL-encoded paths gracefully with comprehensive error handling
- **Windows-native**: Intended to be registered as `aria2://` URL scheme in Windows registry
- **Explorer integration**: Opens directories directly and focuses targeted files
- **Fallback support**: Opens Downloads folder when no path is specified

## 🚀 Quick Start

### Prerequisites
- Windows 10/11
- Rust (for building from source)
- Administrator privileges (for registry registration)

### Download Pre-built Binary
1. Download the latest release from [GitHub Releases](https://github.com/avitretiak/aria2c-browse/releases)
2. Extract the ZIP file
3. Run `register_handler.bat` as Administrator
4. Test with: `aria2://browse/path=C%3A%5CUsers%5Ctest`

### Build from Source
```bash
# Clone the repository
git clone https://github.com/avitretiak/aria2c-browse.git
cd aria2c-browse

# Build release binary
cargo build --release

# Install to system path (optional)
mkdir -p C:\bin
copy target\release\aria2c-browse.exe C:\bin\
# Add C:\bin to your PATH environment variable
```

### Register URL Scheme
```bash
# Run as Administrator
register_handler.bat "C:\path\to\aria2c-browse.exe"
```

The script will auto-detect the executable location and handle existing registrations.

## 📖 Usage

The application is designed to be used as a protocol handler, not directly from command line. If opened without an argument, it will fall back to `%USERPROFILE%\Downloads`.

### URL Scheme Format
The handler processes URLs with format:
```
aria2://browse/path=<url-encoded-path>
```

### Examples
```bash
# Open Downloads folder
aria2://browse/path=C%3A%5CUsers%5Cusername%5CDownloads

# Open specific file (will select it in Explorer)
aria2://browse/path=C%3A%5CUsers%5Cusername%5CDocuments%5Cfile.txt

# Open directory
aria2://browse/path=D%3A%5CProgram%20Files%5CApp
```

### Integration with aria2c
Add this to your aria2c configuration file (`aria2.conf`):
```ini
# Open downloaded files/folders in Explorer
on-download-complete=aria2://browse/path=%d
```

## 🛠️ Development

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Windows 10/11

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check for issues
cargo check
cargo clippy
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test categories
cargo test test_url_decode
cargo test test_integration

# Run with verbose output
cargo test -- --nocapture
```

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE.md](LICENSE.md) file for details.