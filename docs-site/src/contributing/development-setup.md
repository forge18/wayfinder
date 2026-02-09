# Development Setup

This guide will help you set up a development environment for contributing to Wayfinder.

## Prerequisites

Before you can develop Wayfinder, you'll need:

1. **Rust Toolchain**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install)
2. **Lua Development Libraries**: Wayfinder requires Lua development headers to build
3. **Git**: For version control
4. **IDE/Editor**: With Rust support (recommended: VSCode with rust-analyzer)

### Installing Lua Development Libraries

Follow the same instructions as in the [Installation Guide](../getting-started/installation.md#installing-lua-development-libraries).

## Cloning the Repository

Clone the Wayfinder repository:

```bash
git clone https://github.com/forge18/wayfinder.git
cd wayfinder
```

## Project Structure

Wayfinder follows a modular architecture:

```
wayfinder/
├── crates/
│   ├── wayfinder-core/     # Core DAP implementation
│   ├── wayfinder-cli/      # Command-line interface
│   └── wayfinder-tl/       # TypedLua integration (future)
├── docs/                   # Technical documentation
├── docs-site/              # User-facing documentation
├── scripts/                # Development scripts
└── Cargo.toml              # Workspace manifest
```

## Building the Project

### Development Build

To build Wayfinder in development mode:

```bash
cargo build
```

This creates debug binaries in `target/debug/`.

### Release Build

To build optimized binaries:

```bash
cargo build --release
```

This creates optimized binaries in `target/release/`.

### Building Specific Crates

To build only the core library:

```bash
cargo build -p wayfinder-core
```

To build only the CLI:

```bash
cargo build -p wayfinder-cli
```

## Running Tests

Wayfinder includes unit tests and integration tests. To run all tests:

```bash
cargo test
```

To run tests for a specific crate:

```bash
cargo test -p wayfinder-core
```

To run tests with output:

```bash
cargo test -- --nocapture
```

## Code Quality Tools

### Formatting

Wayfinder uses `rustfmt` for code formatting. To format the code:

```bash
cargo fmt
```

To check if code is properly formatted:

```bash
cargo fmt --check
```

### Linting

Wayfinder uses `clippy` for additional linting. To run clippy:

```bash
cargo clippy
```

To run clippy with error fixing suggestions:

```bash
cargo clippy --fix
```

## Development Workflow

### 1. Create a Branch

Always create a feature branch for your work:

```bash
git checkout -b feature/my-new-feature
```

### 2. Make Changes

Implement your changes, following the existing code style and patterns.

### 3. Run Tests

Ensure all tests pass:

```bash
cargo test
```

### 4. Format Code

Format your code with rustfmt:

```bash
cargo fmt
```

### 5. Lint Code

Check for linting issues:

```bash
cargo clippy
```

### 6. Commit Changes

Commit your changes with a descriptive message:

```bash
git add .
git commit -m "Add feature: description of what you did"
```

### 7. Push and Create Pull Request

Push your branch and create a pull request:

```bash
git push origin feature/my-new-feature
```

## Debugging Tips

### Logging

Wayfinder uses the `log` crate for logging. To enable logging during development:

```bash
RUST_LOG=debug cargo run -- launch script.lua
```

Or for trace-level logging:

```bash
RUST_LOG=trace cargo run -- launch script.lua
```

### Debugging with GDB/LLDB

To debug with GDB or LLDB, build in debug mode and use your preferred debugger:

```bash
cargo build
gdb target/debug/wayfinder
```

## Testing with IDEs

To test Wayfinder with various IDEs during development:

### Visual Studio Code

1. Open the Wayfinder workspace in VSCode
2. Install the CodeLLDB extension for debugging
3. Use the provided launch configurations in `.vscode/launch.json`

### Neovim

With nvim-dap configured, you can debug applications using Wayfinder through the DAP protocol. A dedicated Wayfinder Neovim extension is planned but not yet implemented.

## Documentation

When adding new features, please update the documentation:

1. Update user-facing docs in `docs-site/src/`
2. Update technical docs in `docs/` if applicable
3. Add examples where appropriate
4. Update the CHANGELOG.md file

## Submitting Changes

1. Ensure all tests pass
2. Ensure code is properly formatted
3. Ensure no clippy warnings
4. Write clear commit messages
5. Create a pull request with a descriptive title and detailed description
6. Link to any relevant issues

## Code Style Guidelines

- Follow Rust naming conventions
- Use rustfmt for consistent formatting
- Write documentation for public APIs
- Include tests for new functionality
- Keep functions focused and small
- Use meaningful variable names
- Prefer explicit error handling over panics

## Getting Help

If you need help with development:

1. Check the [Architecture Documentation](../../docs/plan.md)
2. Look at existing code for patterns and examples
3. Open an issue on GitHub for questions or discussion
4. Join the community chat if available