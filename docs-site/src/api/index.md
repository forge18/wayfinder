# API Reference

The Wayfinder API documentation is automatically generated from the source code using rustdoc. It provides detailed information about the public interfaces, structs, enums, and functions available in the Wayfinder crates.

## Crates Documentation

- **wayfinder-core**: Core debugging functionality and DAP implementation
- **wayfinder-cli**: Command-line interface and launch functionality
- **wayfinder-tl**: LuaNext/TypedLua integration (planned)

## Accessing API Documentation

### Online Documentation

The latest API documentation is available at: [https://forge18.github.io/wayfinder/api/](https://forge18.github.io/wayfinder/api/)

### Local Documentation

To generate and view API documentation locally:

```bash
# Generate documentation
cargo doc --open

# Or for a specific crate
cargo doc -p wayfinder-core --open
```

### Documentation Coverage

The API documentation includes:

- **Struct and Enum Definitions**: Detailed field and variant descriptions
- **Function Signatures**: Parameter and return value documentation
- **Trait Implementations**: Implemented traits and their methods
- **Module Organization**: Logical grouping of related functionality
- **Examples**: Code examples for common usage patterns

## Key API Areas

### Core Debugging Interfaces

- **DAP Transport**: Message handling and protocol implementation
- **Debug Session**: Session lifecycle and state management
- **Breakpoint Management**: Breakpoint storage and validation
- **Variable Inspection**: Variable retrieval and formatting

### Runtime Integration

- **Lua Runtime Abstraction**: Generic interface for different Lua versions
- **PUC Lua Implementations**: Specific implementations for standard Lua
- **LuaNext Integration**: TypedLua and source map support

### CLI Commands

- **Command Parsing**: Argument handling and validation
- **Execution Modes**: Launch, attach, and DAP server functionality
- **Configuration Management**: YAML parsing and merging

## Contributing to Documentation

To improve the API documentation:

1. **Add Documentation Comments**: Use `///` for public APIs in Rust code
2. **Include Examples**: Provide code examples for complex functionality
3. **Document Errors**: Explain when functions can fail and why
4. **Update Regularly**: Keep documentation in sync with code changes

## Next Steps

- Explore the [development setup guide](../contributing/development-setup.md)
- Learn about the [architecture](../../docs/plan.md)
- Understand [IDE integration](../guides/ide-integration.md)