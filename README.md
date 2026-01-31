# Wayfinder

A Debug Adapter Protocol (DAP) implementation for Lua and TypedLua.

## Overview

Wayfinder provides debugging capabilities for the Lua ecosystem. It implements the Debug Adapter Protocol for IDE integration and supports both plain Lua and TypedLua with source map translation.

## Features

- Rust-based DAP server
- Works with PUC Lua (5.1, 5.2, 5.3, 5.4) and LuaNext (5.1, 5.2, 5.3, 5.4)
- Source map support for TypedLua debugging
- Breakpoints, stepping, stack inspection, variable watches
- IDE integration via standard DAP (VSCode, Neovim, JetBrains, etc.)

## Installation

```bash
cargo install wayfinder
```

## Usage

```bash
# Launch a Lua script with debugging
wayfinder launch --runtime lua54 script.lua

# DAP server mode for IDE integration
wayfinder dap
```

## Documentation

See [docs/DESIGN.md](docs/DESIGN.md) for detailed architecture and implementation details.

## License

MIT
