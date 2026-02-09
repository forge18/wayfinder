# Introduction

Wayfinder is a Debug Adapter Protocol (DAP) implementation for Lua and TypedLua. It provides powerful debugging capabilities for the Lua ecosystem, enabling developers to debug their applications with full IDE integration.

## What is Wayfinder?

Wayfinder is a Rust-based DAP server that implements the Debug Adapter Protocol, allowing integration with popular IDEs like Visual Studio Code, Neovim (through nvim-dap), and JetBrains products. It supports both plain Lua and TypedLua with source map translation, making it possible to debug high-level source code while running compiled output.

## Key Features

### Core Debugging Capabilities

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
- **Multi-Version Support**: Works with Lua 5.1, 5.2, 5.3, 5.4 and LuaNext variants
- **Configurable Behavior**: YAML-based configuration with CLI overrides

## Why Wayfinder?

Wayfinder was designed to address the lack of robust debugging tools in the Lua ecosystem. While Lua is a powerful and flexible language, debugging complex applications can be challenging without proper tooling. Wayfinder provides:

1. **Professional Debugging Experience**: Full IDE integration with breakpoints, variable inspection, and call stack navigation
2. **TypedLua Support**: First-class support for TypedLua with source map translation
3. **Performance**: Built with Rust for fast, reliable debugging
4. **Flexibility**: Works with multiple Lua versions and IDEs
5. **Extensibility**: Modular architecture allows for easy extension and customization

## Getting Started

To get started with Wayfinder, check out the [Installation Guide](getting-started/installation.md) and [Quick Start](getting-started/quick-start.md) documentation.

## Architecture Overview

Wayfinder follows a modular architecture with three main components:

1. **wayfinder-core**: The core DAP implementation and debugging logic
2. **wayfinder-cli**: Command-line interface for launching and controlling the debugger
3. **wayfinder-tl**: LuaNext/TypedLua integration with source map support

See the [Architecture Documentation](../docs/plan.md) for detailed information about the implementation plan and design decisions.