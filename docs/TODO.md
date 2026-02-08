# Wayfinder TODO

**Plan:** [docs/PLAN.md](./PLAN.md)
**Status:** Phase 1 Complete - Phase 2 Complete - Phase 3 Complete

---


## Phase 1: Foundation (Complete ✓)

### CLI Scaffolding

- [x] Create `wayfinder-core/Cargo.toml` with serde, serde_json, tokio, thiserror, async-trait
- [x] Create `wayfinder-core/src/lib.rs` with placeholder module
- [x] Create `wayfinder-cli/Cargo.toml` with clap, serde_yaml, wayfinder-core, home
- [x] Create `wayfinder-cli/src/lib.rs` with clap argument parsing
- [x] Implement `--version` command
- [x] Implement `dap` subcommand (stdio mode, optional --port)
- [x] Implement `launch` subcommand (--runtime, --cwd, script)
- [x] Implement `attach` subcommand (--port, --pid)
- [x] Create config loading with `wayfinder.yaml` parsing

**Structure:**

```text
crates/
├── wayfinder-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── dap/
│       │   ├── mod.rs              (Message, Response, Event, ProtocolMessage)
│       │   └── transport.rs        (JSON-RPC stdio transport)
│       ├── runtime/
│       │   ├── mod.rs              (DebugRuntime trait, types)
│       │   ├── mock.rs             (MockRuntime for testing)
│       │   └── puc_lua.rs         (PUCLuaRuntime stub)
│       └── session/
│           └── mod.rs              (DebugSession, DapServer)
└── wayfinder-cli/
    └── src/
        ├── lib.rs                  (CLI with clap, config loading)
        └── bin.rs                  (main function)

src/
└── main.rs                          (binary entry point)
```

---


## Phase 2: Core DAP (Complete ✓)

### DAP Protocol

- [x] Implement `dap/mod.rs` with Message, Request, Response, Event types
- [x] Implement `dap/transport.rs` for JSON-RPC over stdio
- [x] Implement capabilities negotiation (initialize request)
- [x] Implement all standard DAP requests (launch, attach, disconnect, etc.)
- [x] Implement DAP event sending (stopped, output, etc.)

### Runtime Abstraction

- [x] Implement `runtime/mod.rs` with DebugRuntime trait
- [x] Define RuntimeVersion enum (5.1-5.4, PUC vs LuaNext)
- [x] Define StepMode enum (over, in, out)
- [x] Define Frame, Variable, Value types
- [x] Define Breakpoint, Scope types

### Mock Runtime

- [x] Implement `runtime/mock.rs` with MockRuntime
- [x] Mock breakpoint, step, stack_trace, variables, evaluate operations

### PUC Lua Integration

- [x] Implement `runtime/puc_lua.rs` (stub with FFI structure)
- [x] Implement `runtime/lua_ffi.rs` with full Lua C API bindings
- [x] Implement `runtime/lua_state.rs` with safe Rust wrapper
- [x] Implement hook installation with `debug.sethook`
- [x] Implement `step` operations (over, in, out) - basic stepping working
- [x] Implement `set_breakpoint` using hook callbacks - infrastructure ready
- [x] Implement `stack_trace` with `debug.getinfo`
- [x] Implement `variables` (locals, upvalues, globals)
- [x] Implement `evaluate` (expression evaluation in frame)
- [x] Handle version-specific features (upvalueid, setcstacklimit)

### LuaNext Integration

- [x] Implement `runtime/luanext.rs` (initially same as PUC Lua)
- [x] Use `debug.sethook` compatibility layer

### Breakpoints

- [x] Implement `breakpoints.rs` with LineBreakpoint type
- [x] Implement `setBreakpoints` request handler
- [x] Implement `setFunctionBreakpoints` request handler
- [x] Implement `setExceptionBreakpoints` request handler
- [x] Hook into debug library for breakpoint triggering

### Stepping

- [x] Implement `next` (step over)
- [x] Implement `stepIn` (step into)
- [x] Implement `stepOut` (step out)
- [x] Implement `pause` request

### Stack Inspection

- [x] Implement `stackTrace` request handler
- [x] Collect frames via `debug.getinfo(2, 'nSluf')`
- [x] Return source reference and line/column for each frame

### Variables

- [x] Implement `scopes` request handler
- [x] Implement `variables` request handler
- [x] Support globals via `_G` traversal
- [x] Support local variables via `debug.getlocal`
- [x] Support upvalues via `debug.getupvalue`
- [x] Implement table expansion with depth limits
- [x] Handle circular reference detection

### Expression Evaluation

- [x] Implement `evaluate` request handler
- [x] Safe Lua evaluation sandbox
- [x] Read-only evaluation by default

### Session Management

- [x] Debug session lifecycle (initialize → terminate)
- [x] Event loop for handling DAP messages
- [x] Thread/process management for debuggee

### Phase 2 Tests

- [x] Integration tests with each Lua version (5.1, 5.2, 5.3, 5.4)
- [x] DAP protocol compliance tests
- [x] Breakpoint accuracy tests
- [x] Variable inspection tests
- [x] Expression evaluation tests

### Phase 3 Tests

- [x] Watchpoint functionality tests
- [x] Evaluate mutation tests
- [x] Integration tests for all Phase 3 features

---

## Phase 3: Advanced Features (3 weeks) (Complete ✓)

### Conditional Breakpoints

- [x] Implement `conditions.rs` for expression evaluation
- [x] Add condition field to Breakpoint type
- [x] Evaluate condition at breakpoint hit
- [x] Only pause if condition is truthy
- [x] Support complex Lua expressions

### Logpoints

- [x] Add logMessage field to Breakpoint type
- [x] Implement `breakpoints/logpoint.rs`
- [x] Parse `{expression}` format strings
- [x] Output via DebugConsole event
- [x] Do not pause when logpoint triggered

### Hit Count Filtering

- [x] Implement `breakpoints/hit_count.rs`
- [x] Add hitCondition field to Breakpoint type
- [x] Track hits per breakpoint
- [x] Implement `>= N`, `== N`, `mod N` parsing
- [x] Reset count on breakpoint configuration change

### Exception Filters

- [x] Implement exception breakpoint filtering
- [x] Classify errors by type/message
- [x] User-configurable filters via setExceptionBreakpoints
- [x] Default: break on all errors

### Watchpoints

- [x] Implement `watchpoints.rs` data structures
- [x] Add dataBreakpoint type
- [x] Track initial variable value
- [x] Check on each hook invocation
- [x] Implement `debug.upvalueid` for closure variables
- [x] Table field watchpoints via metatable __newindex
- [x] Create `inject/watchpoint.lua` for runtime detection
- [x] Runtime integration for watchpoint detection

### Evaluate Mutation (Opt-in)

- [x] Add `evaluate.mutate` config option
- [x] When enabled: use `debug.setlocal`
- [x] When enabled: use `debug.setupvalue`
- [x] Track modifications for clarity
- [x] Add safety checks/sandboxing

### Phase 3 Tests

- [x] Conditional breakpoint accuracy tests
- [x] Logpoint output verification tests
- [x] Hit count behavior tests
- [x] Watchpoint triggering tests
- [x] Mutation safety tests

---

## Phase 4: LuaNext Integration (3-4 weeks)

### Source Map Loader

- [ ] Create `wayfinder-tl/Cargo.toml`
- [ ] Add typedlua-sourcemap dependency
- [ ] Implement `SourceMapSource` enum (file, inline, data URI)
- [ ] Implement source map file loading
- [ ] Implement inline source map extraction (--# sourceMappingURL)
- [ ] Implement data URI parsing
- [ ] Implement source map caching

### Bundle Mode Support

- [ ] Handle multi-file source maps
- [ ] Parse `sources` array
- [ ] Use source index in reverse lookup
- [ ] Support all files in bundle

### Position Translation

- [ ] Implement `translator.rs`
- [ ] Implement `forward_lookup` (generated → original)
- [ ] Implement `reverse_lookup` (original → generated)
- [ ] Handle bundle mode (multiple source files)
- [ ] Handle missing mappings gracefully

### DAP Message Translation

- [ ] Implement `dap_wrapper.rs`
- [ ] Intercept `setBreakpoints`: translate source paths/lines
- [ ] Intercept `source`: return .luax content
- [ ] Intercept `stopped`: reverse translate positions
- [ ] Intercept `stackTrace`: translate all frames
- [ ] Handle both .lua and .luax files

### Coroutine Debugging

- [ ] Implement `coroutine.rs`
- [ ] Enumerate active coroutines
- [ ] Show coroutine status (suspended/running/dead)
- [ ] Implement coroutine switching
- [ ] Use `debug.setname` for naming (Lua 5.2+)
- [ ] Display coroutine name in stack frames
- [ ] Add `breakOnAll` coroutine option

### Source Map Preference

- [ ] Implement `sourceMapBehavior` config option
- [ ] Implement "ask" behavior (prompt user)
- [ ] Implement "lenient" behavior (debug .lua only)
- [ ] Implement "strict" behavior (error if missing)
- [ ] Persist user preference

### Phase 4 Tests

- [ ] Source map accuracy tests
- [ ] Round-trip translation tests (forward + reverse)
- [ ] Bundle mode tests
- [ ] Coroutine switching tests
- [ ] Mixed .lua/.luax debugging tests

---

## Phase 5: Hot Code Reload (2 weeks)

### State Capture

- [ ] Implement `state_capture.rs`
- [ ] Capture global table entries
- [ ] Capture upvalues for existing functions
- [ ] Record table structure and contents
- [ ] Detect circular references

### Module Reload

- [ ] Implement `hot_reload.rs`
- [ ] Compile new module source via LuaNext
- [ ] Execute to get new module table
- [ ] Call new module chunk

### State Restoration

- [ ] Restore global variables (if they existed)
- [ ] Preserve table contents where possible
- [ ] Handle new/deleted fields
- [ ] Generate warnings for unpreserved state
- [ ] Output warnings to console/DAP output

### Update Propagation

- [ ] Find existing closures referencing old module
- [ ] Update module table reference
- [ ] Preserve function identity where possible
- [ ] Handle closures with captured state

### User Interface

- [ ] Add `:HotReload` CLI command
- [ ] Add DAP protocol extension (custom request)
- [ ] Status output with warnings
- [ ] Documentation for hot reload limitations

### Phase 5 Tests

- [ ] State preservation tests
- [ ] Module reload functionality tests
- [ ] Edge case tests (circular refs, closures)
- [ ] Warning output tests

---

## Phase 6: CLI and Polish (2 weeks)

### Launch Mode

- [ ] Implement `commands/launch.rs`
- [ ] Parse `--runtime` argument
- [ ] Spawn Lua process with debug arguments
- [ ] Inject debug helper scripts
- [ ] Forward stdio communication

### Attach Mode

- [ ] Implement `commands/attach.rs`
- [ ] Implement `--port` argument
- [ ] Connect to running Lua process
- [ ] Handle connection errors gracefully

### DAP Server Mode

- [ ] Implement `commands/dap.rs`
- [ ] Support `--port` for TCP server
- [ ] Handle TCP connections
- [ ] Multi-client support

### Configuration

- [ ] Implement `config.rs`
- [ ] Load `wayfinder.yaml` from project directory
- [ ] Load `wayfinder.yaml` from home directory
- [ ] CLI argument precedence over config file
- [ ] Default value handling

### Documentation

- [ ] Update README with new features
- [ ] Add configuration documentation
- [ ] Add hot reload documentation
- [ ] Add troubleshooting section

### Phase 6 Tests

- [ ] CLI argument parsing tests
- [ ] Configuration file tests
- [ ] Integration tests with real Lua files

---

## Deferred (Future Phases)

### Remote Debugging

- [ ] TCP attach mode (future phase)
- [ ] Network security considerations
- [ ] Multi-client debugging

### IDE Extensions

- [ ] VSCode extension scaffolding
- [ ] Debug configuration provider
- [ ] Runtime auto-detection
- [ ] "Debug File" command
- [ ] "Debug Test" CodeLens (Canary integration)
- [ ] Neovim plugin scaffolding
- [ ] nvim-dap auto-configuration
- [ ] Telescope runtime picker

### Additional Features

- [ ] Function-level hot reload (future enhancement)
- [ ] Remote process debugging
- [ ] Profiling integration
- [ ] Memory inspection

---

## Crate Structure Summary

```text
crates/
├── wayfinder-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── dap/
│       │   ├── mod.rs              (Message, Response, Event, ProtocolMessage)
│       │   └── transport.rs        (JSON-RPC stdio transport)
│       ├── runtime/
│       │   ├── mod.rs              (DebugRuntime trait, types)
│       │   ├── lua_ffi.rs          (Lua C API FFI bindings)
│       │   ├── lua_state.rs        (Safe Rust wrapper for Lua state)
│       │   ├── mock.rs             (MockRuntime for testing)
│       │   └── puc_lua.rs         (PUCLuaRuntime with FFI)
│       └── session/
│           └── mod.rs              (DebugSession, DapServer)
├── wayfinder-luax/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                  (Source map translator for LuaNext)
└── wayfinder-cli/
    └── src/
        └── lib.rs                  (CLI with clap, config loading)

src/
└── main.rs                          (binary entry point)
```

---


## Current Dependencies

```toml
# wayfinder-core/Cargo.toml
[dependencies]
serde = "1"
serde_json = "1"
tokio = { version = "1", features = ["full"] }
thiserror = "1"
async-trait = "0.1"
libc = "0.2"
luanext-sourcemap = { path = "../luanext/crates/luanext-sourcemap" }

# wayfinder-cli/Cargo.toml
[dependencies]
wayfinder-core = { path = "../wayfinder-core" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
serde = "1"
serde_yaml = "0.9"
home = "0.5"

[[bin]]
name = "wayfinder"
path = "src/main.rs"
```

**Submodules:**
- `crates/luanext/` - Git submodule: https://github.com/forge18/luanext.git
  - Contains: luanext-core, luanext-parser, luanext-typechecker, luanext-sourcemap

---


## Current Status

**Phase 1: Complete ✓**
- CLI scaffolding done
- Working: `wayfinder --help`, `wayfinder dap`, `wayfinder launch`, `wayfinder attach`
- Config file loading working (`wayfinder.yaml`)

**Phase 2: Complete ✓**
- DAP protocol types implemented (Message, Response, Event)
- Runtime abstraction trait with async methods
- Mock runtime for testing
- Debug session and DAP server scaffolding
- Transport layer for JSON-RPC over stdio
- Lua FFI bindings (`lua_ffi.rs`) with full Lua 5.4 C API
- Lua state wrapper (`lua_state.rs`) with safe Rust bindings
- PUCLuaRuntime with full FFI integration
  - Hook infrastructure with `debug.sethook`
  - Line stepping with `lua_hook_callback`
  - Step modes (In, Over, Out) with depth tracking
  - Stack trace via `debug.getinfo`
  - Variables inspection
  - Expression evaluation
  - Breakpoint infrastructure ready
- Comprehensive test suite for all core functionality

**Phase 3: Complete ✓**
- Conditional breakpoints with expression evaluation
- Logpoints with variable substitution
- Hit count filtering with complex conditions
- Exception filtering with detailed exception info
- Data breakpoints (watchpoints) with full runtime support
- Evaluate mutation with full runtime support including debug.setlocal/debug.setupvalue

**Next: Begin Phase 4 - LuaNext Integration**

```bash
# Test current CLI
cargo run -- --help
cargo run -- launch script.lua
```

**LuaNext Integration**
- `luanext` git submodule added
- `luanext-sourcemap` available via submodule
- Source map translator infrastructure ready
