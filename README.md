# Wayfinder

A Debug Adapter Protocol (DAP) implementation for Lua and TypedLua.

## Overview

Wayfinder provides debugging capabilities for the Lua ecosystem. It implements the Debug Adapter Protocol for IDE integration and supports both plain Lua and TypedLua with source map translation.

## Features

### Core Features

- Rust-based DAP server with full protocol support
- Works with PUC Lua (5.1, 5.2, 5.3, 5.4) and LuaNext (5.1, 5.2, 5.3, 5.4)
- Source map support for TypedLua debugging
- IDE integration via standard DAP (VSCode, Neovim, JetBrains, etc.)

### Debugging Capabilities

- **Breakpoints**: Line breakpoints, function breakpoints, and exception breakpoints
- **Conditional Breakpoints**: Break only when expressions evaluate to true
- **Logpoints**: Output debug messages without pausing execution
- **Hit Count Filtering**: Break after N hits or on specific hit patterns
- **Stepping**: Step over, step in, step out with depth tracking
- **Stack Inspection**: Full call stack with frame inspection
- **Variable Watches**: Locals, upvalues, globals, and table expansion
- **Data Breakpoints (Watchpoints)**: Break when variable values change
- **Expression Evaluation**: Evaluate Lua expressions in any frame
- **Coroutine Debugging**: Switch between coroutines and debug concurrent code

### Advanced Features

- **Hot Code Reload**: Reload modules without restarting your application
- **Source Map Translation**: Debug TypedLua (.luax) files with automatic position mapping
- **Configurable Behavior**: YAML-based configuration with CLI overrides

## Installation

```bash
cargo install wayfinder
```

**Note**: By default, Wayfinder uses static linking with Lua 5.4. This requires Lua 5.4 development libraries at build time.

**Experimental**: Dynamic Lua loading is available as an opt-in feature that allows the binary to work with any Lua version (5.1-5.4) at runtime without build-time dependencies. To enable:

```bash
cargo build --features dynamic-lua --no-default-features
```

⚠️ The dynamic-lua feature is experimental and requires additional runtime integration work. Use static-lua (default) for production.

## Usage

### Launch Mode

Launch a Lua script with or without debugging:

```bash
# Launch a script with debugging enabled
wayfinder launch --debug --runtime lua54 script.lua

# Launch without debugging (simple execution)
wayfinder launch script.lua

# Specify working directory
wayfinder launch --cwd /path/to/project --runtime lua54 script.lua
```

### DAP Server Mode

Run as a DAP server for IDE integration:

```bash
# Stdio mode (default - for IDE integration)
wayfinder dap

# TCP mode on specific port
wayfinder dap --port 5678
```

### Attach Mode

Attach to a running Lua process:

```bash
# Attach via TCP port
wayfinder attach --port 5678

# Attach to process by PID (Unix systems)
wayfinder attach --pid 12345
```

### Hot Reload

Reload a module in a running debug session:

```bash
# Connect to DAP server and reload a module
wayfinder hot-reload --module mymodule --port 5678

# Default host is 127.0.0.1, can be changed
wayfinder hot-reload --module mymodule --port 5678 --host 192.168.1.100
```

## Configuration

Wayfinder can be configured using YAML configuration files. Configuration files are loaded from:

1. Project directory: `./wayfinder.yaml`
2. Home directory: `~/.wayfinder.yaml`

CLI arguments take precedence over configuration file settings.

### Configuration File Example

Create a `wayfinder.yaml` file in your project directory:

```yaml
# Runtime configuration
runtime: lua54

# Working directory for script execution
cwd: /path/to/project

# Environment variables
env:
  LUA_PATH: "./?.lua;/usr/local/share/lua/5.4/?.lua"
  DEBUG_MODE: "true"

# Source map behavior (for TypedLua debugging)
# Options: "ask", "lenient", "strict"
sourceMapBehavior: lenient

# Hot reload configuration
evaluate:
  mutate: true  # Allow variable mutation during evaluation
```

### Configuration Options

- **runtime**: Lua runtime to use (e.g., `lua54`, `lua53`, `lua52`, `lua51`)
- **cwd**: Working directory for script execution
- **env**: Environment variables as key-value pairs
- **sourceMapBehavior**: How to handle missing source maps
  - `ask`: Prompt user when source map is missing
  - `lenient`: Debug .lua files only if source map is missing
  - `strict`: Error if source map is missing for .luax files
- **evaluate.mutate**: Enable variable mutation during expression evaluation (opt-in for safety)

## Hot Code Reload

Hot code reload allows you to update modules in a running application without restarting. This is useful for rapid iteration during development.

### How It Works

1. Start your application with debugging enabled:

   ```bash
   wayfinder launch --debug --runtime lua54 myapp.lua
   ```

2. In another terminal, send a hot reload request:

   ```bash
   wayfinder hot-reload --module mymodule --port 5678
   ```

3. Wayfinder will:
   - Compile the new module source
   - Capture existing state (globals, upvalues)
   - Execute the new module code
   - Attempt to preserve state where possible

### Limitations

Hot reload has some limitations due to Lua's runtime behavior:

- **Function Identity**: Existing function references are not updated
- **Closures**: Functions with captured upvalues may not be updated
- **Metatables**: Table metatables are not automatically preserved
- **State Migration**: Some state may not be preservable

See `docs/hot_reload/limitations.md` for detailed information about limitations and workarounds.

## Troubleshooting

### Build Issues

**Problem**: `cargo build` fails with "cannot find -llua5.4"

**Solution**: Install Lua 5.4 development libraries:

```bash
# macOS
brew install lua@5.4

# Ubuntu/Debian
sudo apt-get install liblua5.4-dev

# Fedora
sudo dnf install lua-devel

# From source
wget https://www.lua.org/ftp/lua-5.4.7.tar.gz
tar -xzf lua-5.4.7.tar.gz
cd lua-5.4.7
make linux  # or 'make macosx' on macOS
sudo make install
```

**Problem**: Library builds but binary fails to link

**Solution**: The library (`cargo build -p wayfinder-cli --lib`) doesn't require Lua libraries, but the binary does. Make sure Lua 5.4 development libraries are installed system-wide. The wayfinder-core crate uses FFI bindings that require linking against liblua5.4 at build time.

### Runtime Issues

**Problem**: "Connection timeout - is the DAP server running?"

**Solution**: Make sure the DAP server is running in TCP mode before trying to connect:

```bash
# Terminal 1: Start DAP server
wayfinder dap --port 5678

# Terminal 2: Connect with hot reload
wayfinder hot-reload --module mymodule --port 5678
```

**Problem**: Hot reload fails with "Invalid response from DAP server"

**Solution**: Ensure the DAP server supports the `hotReload` custom request. This requires wayfinder-core with hot reload support (Phase 5+).

**Problem**: Breakpoints not triggering

**Solution**:

- Verify the script path matches the source file path
- Check that the Lua version matches the runtime configuration
- For TypedLua, ensure source maps are correctly configured

### Debug Output

Enable verbose logging by checking the DAP output channel in your IDE, or run in debug mode:

```bash
# Launch with debug output
wayfinder launch --debug --runtime lua54 script.lua
```

## Documentation

See [docs/DESIGN.md](docs/DESIGN.md) for detailed architecture and implementation details.

## License

MIT
