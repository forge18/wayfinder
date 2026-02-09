# Installation

This guide will help you install Wayfinder on your system. Wayfinder is distributed as a Rust crate and can be installed using Cargo.

## Prerequisites

Before installing Wayfinder, you'll need:

1. **Rust Toolchain**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install)
2. **Lua Development Libraries**: Wayfinder requires Lua development headers to build

### Installing Lua Development Libraries

#### macOS

```bash
# Using Homebrew
brew install lua@5.4

# Or using MacPorts
sudo port install lua54
```

#### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install liblua5.4-dev
```

#### Fedora

```bash
sudo dnf install lua-devel
```

#### CentOS/RHEL

```bash
sudo yum install lua-devel
```

#### From Source

If you prefer to build Lua from source:

```bash
wget https://www.lua.org/ftp/lua-5.4.7.tar.gz
tar -xzf lua-5.4.7.tar.gz
cd lua-5.4.7
make linux  # or 'make macosx' on macOS
sudo make install
```

## Installation Methods

### Method 1: Using Cargo (Recommended)

The easiest way to install Wayfinder is using Cargo:

```bash
cargo install wayfinder
```

This will download, compile, and install the latest version of Wayfinder.

### Method 2: Building from Source

If you want to build from source or contribute to Wayfinder:

```bash
# Clone the repository
git clone https://github.com/forge18/wayfinder.git
cd wayfinder

# Build the project
cargo build --release

# The binary will be available at target/release/wayfinder
```

### Method 3: Using Pre-built Binaries

Pre-built binaries are available on the [GitHub Releases page](https://github.com/forge18/wayfinder/releases).

## Dynamic Lua Loading (Experimental)

By default, Wayfinder uses static linking with Lua 5.4. This requires Lua 5.4 development libraries at build time.

An experimental dynamic Lua loading feature is available that allows the binary to work with any Lua version (5.1-5.4) at runtime without build-time dependencies. To enable:

```bash
cargo build --features dynamic-lua --no-default-features
```

⚠️ The dynamic-lua feature is experimental and requires additional runtime integration work. Use static-lua (default) for production.

## Verification

After installation, verify that Wayfinder is properly installed:

```bash
wayfinder version
```

This should display the version information for Wayfinder.

## Next Steps

Once installed, proceed to the [Quick Start Guide](quick-start.md) to learn how to use Wayfinder for debugging your Lua applications.