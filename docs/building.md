# Dynamic Lua Loading

## Overview

Wayfinder supports two modes for Lua integration:

1. **Static Linking (default)**: Links against Lua 5.4 at compile time
2. **Dynamic Loading (experimental)**: Loads Lua libraries at runtime, supporting versions 5.1-5.4

## Current Status

### ✅ Implemented

- **Feature flags**: `static-lua` (default) and `dynamic-lua`
- **Build system**: No Lua dependency required when using `dynamic-lua`
- **Dynamic loader** ([lua_loader.rs](../crates/wayfinder-core/src/runtime/lua_loader.rs)):
  - Runtime library discovery for Lua 5.1-5.4
  - Function pointer loading via `libloading`
  - Platform-specific library paths (macOS, Linux, Windows)
  - Thread-safe `Arc`-based shared access
- **Initialization system** ([lua_init.rs](../crates/wayfinder-core/src/runtime/lua_init.rs)):
  - One-time global initialization
  - Version selection at startup

### 🚧 Work in Progress

- **Runtime integration**: `lua_state.rs` needs to be updated to use `LuaLibrary` instead of static FFI
- **PUCLuaRuntime**: Needs to accept and use `LuaLibrary` instance
- **Version switching**: Infrastructure exists but needs runtime integration

### 📋 Remaining Work

1. **Refactor lua_state.rs** (~200 lines):
   - Add `#[cfg(feature = "dynamic-lua")]` for dynamic mode
   - Add `lib: LuaLibrary` field
   - Replace all `lua_*` calls with `self.lib.lua_*`
   - Keep `#[cfg(feature = "static-lua")]` for static mode

2. **Update PUCLuaRuntime** (~50 lines):
   - Accept `LuaLibrary` in constructor
   - Pass to `Lua::new_with_library()`
   - Thread through to all Lua state instances

3. **Update command initialization** (~30 lines):
   - Load library based on version config
   - Initialize before creating runtime
   - Handle loading errors gracefully

4. **Add comprehensive tests**:
   - Test with actual Lua 5.1, 5.2, 5.3, 5.4
   - Version switching
   - Feature flag combinations

## Building

### Static Linking (Default)

```bash
# Requires Lua 5.4 development libraries
cargo build
```

### Dynamic Loading (Experimental)

```bash
# No build-time Lua dependency
cargo build --features dynamic-lua --no-default-features
```

**Note**: The binary will compile but runtime integration is incomplete. The binary won't successfully debug Lua scripts yet in dynamic mode.

## Architecture

### Static Mode

```
┌─────────────────┐
│  Rust Binary    │
│                 │
│  ┌───────────┐  │
│  │ lua_state │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │  lua_ffi  │  │ ──── Compile-time link to liblua5.4
│  │  (static) │  │
│  └───────────┘  │
└─────────────────┘
```

### Dynamic Mode (Target)

```
┌─────────────────┐
│  Rust Binary    │
│                 │
│  ┌───────────┐  │
│  │ lua_state │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼──────────┐
│  │  lua_loader    │
│  │  (LuaLibrary)  │
│  └────────┬───────┘
│           │
└───────────┼────────┘
            │ Runtime dlopen
            ▼
    ┌───────────────┐
    │  liblua5.1    │
    │  liblua5.2    │  ◄─── Selected at runtime
    │  liblua5.3    │
    │  liblua5.4    │
    └───────────────┘
```

## Library Search Paths

The dynamic loader searches for Lua libraries in these locations:

### macOS
- `/opt/homebrew/lib/liblua{version}.dylib`
- `/usr/local/lib/liblua{version}.dylib`
- `/usr/lib/liblua{version}.dylib`

### Linux
- `/usr/lib/x86_64-linux-gnu/liblua{version}.so`
- `/usr/lib/liblua{version}.so`
- `/usr/local/lib/liblua{version}.so`

### Windows
- `lua{version}.dll` (in PATH)

## API Usage (When Complete)

### Static Mode

```rust
// Automatic - no changes needed
let runtime = PUCLuaRuntime::new()?;
```

### Dynamic Mode

```rust
use wayfinder_core::runtime::{lua_loader::LuaLibrary, LuaVersion};

// Load library for specific version
let lib = LuaLibrary::load(LuaVersion::V54)?;

// Create runtime with library
let runtime = PUCLuaRuntime::new_with_library(lib)?;
```

## Migration Guide

For code that currently uses static FFI:

**Before** (static-lua):
```rust
unsafe {
    lua_getglobal(state, name.as_ptr());
}
```

**After** (dynamic-lua):
```rust
unsafe {
    lib.lua_getglobal(state, name.as_ptr());
}
```

The `LuaLibrary` provides the same function signatures, just called as methods.

## Testing Dynamic Loading

Once integration is complete, test with different Lua versions:

```bash
# Install multiple Lua versions
brew install lua@5.1 lua@5.2 lua@5.3 lua@5.4  # macOS

# Build with dynamic loading
cargo build --features dynamic-lua --no-default-features

# Test with different versions
wayfinder launch --runtime lua51 test.lua
wayfinder launch --runtime lua52 test.lua
wayfinder launch --runtime lua53 test.lua
wayfinder launch --runtime lua54 test.lua
```

## Benefits of Dynamic Loading

1. **Single Binary**: One binary debugs all Lua versions (5.1-5.4)
2. **No Build Dependencies**: Users don't need Lua dev libraries installed
3. **Runtime Flexibility**: Switch between Lua versions without recompiling
4. **Distribution**: Easier to distribute - no library linking issues

## Performance Considerations

- **Startup**: Minimal overhead (~1-2ms to load library)
- **Runtime**: No overhead - function pointers are cached
- **Memory**: Negligible - shared `Arc<LuaLibrary>` instance

## Troubleshooting

**Error**: "Failed to load Lua library: ..."

**Solution**: Install the required Lua version's shared library:

```bash
# macOS
brew install lua@5.4

# Ubuntu/Debian
sudo apt-get install liblua5.4-0

# Fedora
sudo dnf install lua-libs
```

**Error**: "Failed to find symbol: ..."

**Solution**: The Lua library version may not match expectations. Verify with:

```bash
# Check what symbols are available
nm -D /path/to/liblua5.4.so | grep lua_getglobal
```

## Future Enhancements

- [ ] Complete runtime integration (lua_state.rs refactoring)
- [ ] Version auto-detection from script shebang or config
- [ ] Multiple concurrent Lua versions in same process
- [ ] Hot-swap Lua versions without restart
- [ ] Lazy loading - only load library when needed
- [ ] Precompiled binaries with dynamic loading for all platforms

## Contributing

To help complete the dynamic loading feature:

1. See remaining tasks in this document
2. Test with real Lua installations
3. Submit PRs for runtime integration
4. Add tests for version-specific behavior

## References

- [lua_loader.rs](../crates/wayfinder-core/src/runtime/lua_loader.rs) - Dynamic loader implementation
- [lua_init.rs](../crates/wayfinder-core/src/runtime/lua_init.rs) - Initialization system
- [build.rs](../crates/wayfinder-core/build.rs) - Build script with feature detection
