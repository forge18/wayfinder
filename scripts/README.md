# Wayfinder Testing Scripts

This directory contains scripts for testing Wayfinder's multi-version Lua support.

## Scripts

### `install_lua_versions.sh`

Downloads and builds Lua versions 5.1, 5.2, 5.3, and 5.4 with shared library support **locally in the project directory**. No system installation required!

**Requirements:**
- C compiler (gcc/clang)
- make
- curl

**Usage:**
```bash
./scripts/install_lua_versions.sh
```

**Build locations:**

- Source: `./lua-builds/` (temporary build artifacts)
- Libraries: `./lua-libs/liblua5.1.dylib`, `./lua-libs/liblua5.2.dylib`, etc.

**What it does:**

1. Downloads Lua source tarballs from lua.org
2. Extracts and builds each version
3. Creates shared libraries (.dylib on macOS, .so on Linux)
4. Copies libraries to `./lua-libs/` directory

**Note:** The script builds everything locally - no sudo required, no system files modified!

### `test_lua_versions.sh`

Validates that all Lua versions are installed correctly and tests basic functionality.

**Requirements:**
- Lua versions installed (run `install_lua_versions.sh` first)
- Wayfinder built with dynamic loading: `cargo build --features dynamic-lua --no-default-features`

**Usage:**
```bash
./scripts/test_lua_versions.sh
```

**What it tests:**
- Library presence and symbol loading for each version
- Basic Lua functionality (variables, functions, control flow)
- Version-specific features (e.g., integer division in 5.3+)

### `run_benchmarks.sh`

Runs comprehensive performance benchmarks for Lua loading and execution across all versions.

**Requirements:**

- Lua versions installed (run `install_lua_versions.sh` first)
- Rust nightly (for better benchmark support)

**Usage:**
```bash
./scripts/run_benchmarks.sh
```

**What it benchmarks:**

- Library loading time (dynamic mode)
- Lua state creation performance
- Basic operations (push/pop numbers, strings, tables)
- Script execution (factorial, fibonacci, table manipulation)
- Compatibility shim overhead
- Static vs dynamic mode comparison

**Output:**

- Console summary of all benchmarks
- Detailed results in `benchmark-results/`
- HTML reports in `target/criterion/report/index.html`

**See also:** [Benchmarks Documentation](../docs/BENCHMARKS.md)

## Testing Dynamic Loading

### Build for Dynamic Loading

```bash
# Build with dynamic loading support (no build-time Lua dependency)
cargo build --features dynamic-lua --no-default-features
```

### Run Integration Tests

```bash
# Run the dynamic loading integration tests
cargo test --features dynamic-lua --no-default-features dynamic_loading

# Run all tests
cargo test --features dynamic-lua --no-default-features
```

### Manual Testing

Test Wayfinder with different Lua versions:

```bash
# Create a test script
cat > test.lua << 'EOF'
print("Running on: " .. _VERSION)
print("2 + 2 = " .. (2 + 2))
EOF

# Test with different versions
./target/debug/wayfinder launch --runtime lua5.1 test.lua
./target/debug/wayfinder launch --runtime lua5.2 test.lua
./target/debug/wayfinder launch --runtime lua5.3 test.lua
./target/debug/wayfinder launch --runtime lua5.4 test.lua
```

## LuaNext Testing

LuaNext is a transpiler that compiles to Lua. The `LuaNextRuntime` in Wayfinder debugs the compiled Lua code, so it works with all Lua versions that PUC Lua supports.

To test LuaNext:

1. Create a `.luax` file with type annotations
2. Compile it to Lua using the LuaNext compiler
3. Debug the compiled `.lua` file with Wayfinder

Example:
```bash
# Create LuaNext source
cat > test.luax << 'EOF'
type Point = { x: number, y: number }

function distance(p1: Point, p2: Point): number {
    local dx = p2.x - p1.x
    local dy = p2.y - p1.y
    return math.sqrt(dx * dx + dy * dy)
}

local p1: Point = {x = 0, y = 0}
local p2: Point = {x = 3, y = 4}
print("Distance: " .. distance(p1, p2))
EOF

# Compile to Lua 5.4
luanext compile test.luax --target=lua54 -o test.lua

# Debug with Wayfinder
./target/debug/wayfinder launch --runtime lua5.4 test.lua
```

## Troubleshooting

### Library Not Found

**Error:** `Failed to load Lua library: Could not find Lua X.X library`

**Solution:**
1. Run `./scripts/install_lua_versions.sh` to install all versions
2. Check that libraries exist:
   ```bash
   ls -l /usr/local/lib/liblua*.{dylib,so}
   ```
3. On macOS, check Homebrew locations too:
   ```bash
   ls -l /opt/homebrew/lib/liblua*.dylib
   ```

### Symbol Not Found

**Error:** `Failed to find symbol: ...`

**Solution:**
- Verify the library version matches expectations:
  ```bash
  nm -D /usr/local/lib/liblua5.4.dylib | grep lua_getglobal
  ```
- Ensure you installed the correct version (not a custom build)

### Permission Denied

**Error:** `Permission denied` during installation

**Solution:**
- Run the install script with sudo:
  ```bash
  sudo ./scripts/install_lua_versions.sh
  ```

### Build Failures

**Error:** Compilation errors during Lua build

**Solution:**
- Make sure you have development tools installed:
  - **macOS:** `xcode-select --install`
  - **Ubuntu/Debian:** `sudo apt-get install build-essential`
  - **Fedora:** `sudo dnf groupinstall "Development Tools"`

## Architecture Notes

### Static vs Dynamic Mode

- **Static mode** (default): Links against Lua 5.4 at compile time
  - Requires Lua 5.4 development libraries during build
  - Binary only works with Lua 5.4
  - Slightly faster (no function pointer indirection)

- **Dynamic mode** (experimental): Loads Lua libraries at runtime
  - No build-time Lua dependency
  - Single binary works with Lua 5.1, 5.2, 5.3, or 5.4
  - Version selected at runtime via CLI or config

### Version Compatibility

The dynamic loader implements compatibility shims for version-specific APIs:

- **`lua_pushglobaltable`**: Native in 5.2+, emulated for 5.1 using `lua_rawgeti`
- **`lua_pcall`**: Native in 5.1, uses `lua_pcallk` in 5.2+
- **`luaL_loadbufferx`**: Native in 5.2+, falls back to `luaL_loadbuffer` in 5.1

These shims ensure that Wayfinder's code can work transparently across all Lua versions.

## CI/CD Integration

To test in CI:

```yaml
- name: Install Lua versions
  run: ./scripts/install_lua_versions.sh

- name: Build with dynamic loading
  run: cargo build --features dynamic-lua --no-default-features

- name: Test dynamic loading
  run: cargo test --features dynamic-lua --no-default-features

- name: Test with each Lua version
  run: ./scripts/test_lua_versions.sh
```

## Future Enhancements

- [ ] Automatic version detection from script shebang
- [ ] Concurrent support for multiple Lua versions in same process
- [ ] Hot-swap Lua versions without restart
- [ ] Precompiled binaries with dynamic loading for all platforms
- [ ] Performance benchmarks comparing static vs dynamic mode
