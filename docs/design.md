# Wayfinder Implementation Plan

**Date:** February 7, 2026
**Status:** Planning Complete - Ready for Implementation

## Overview

Wayfinder is a Debug Adapter Protocol (DAP) implementation for Lua and LuaNext (formerly TypedLua) with source map translation for debugging high-level source code while running compiled output.

## Key Adaptations from DESIGN.md

| Aspect            | Original DESIGN.md    | Updated Plan                                          |
|-------------------|-----------------------|-------------------------------------------------------|
| Source map format | Bincode               | JSON Source Map v3 with VLQ (from typedlua-sourcemap) |
| Source map crate  | `lua-sourcemap` (new) | `typedlua-sourcemap` (reuse from TypedLua)            |
| File extension    | `.tl`                 | `.luax` (LuaNext migration)                           |
| Source map modes  | Separate files        | Separate files + inline (base64 data URI)             |
| Multi-file maps   | Unspecified           | Bundle mode support                                   |

## Feature Matrix

### Core Debugging (Phase 2)

| Feature               | Description                               |
|-----------------------|-------------------------------------------|
| Line breakpoints      | Set breakpoints by source line            |
| Function breakpoints  | Break on function name/definition         |
| Exception breakpoints | Break on Lua errors                       |
| Step over             | Execute to next line in current frame     |
| Step into             | Step into function calls                  |
| Step out              | Step until current function returns       |
| Stack traces          | Full call stack with frame information    |
| Variable inspection   | Expand locals, upvalues, globals, tables  |
| Expression evaluation | Evaluate Lua expressions in frame context |
| Multi-runtime         | PUC Lua 5.1-5.4 + LuaNext 5.1-5.4         |

### Advanced Features (Phase 3)

| Feature                 | Description                                |
|-------------------------|--------------------------------------------|
| Conditional breakpoints | Break only when expression is truthy       |
| Logpoints               | Print message without pausing              |
| Hit count filtering     | Break on Nth hit (`>= N`, `== N`, `mod N`) |
| Exception filters       | Break on specific error types/messages     |
| Watchpoints             | Break when variable value changes          |
| Evaluate mutation       | Allow setlocal/setupvalue (opt-in)         |

### LuaNext Integration (Phase 4)

| Feature               | Description                               |
|-----------------------|-------------------------------------------|
| Source map loading    | JSON v3 + VLQ decoding                    |
| Bundle mode support   | Multi-file source maps in single output   |
| Position translation  | Forward (gen→orig) and reverse (orig→gen) |
| Coroutine debugging   | List, switch, and name coroutines         |
| Source map preference | User choice: lenient/strict/ask           |

### Bonus Features (Phase 5+)

| Feature          | Description                                |
|------------------|--------------------------------------------|
| Hot code reload  | Full module reload with state preservation |
| CLI enhancements | Launch, attach, DAP server modes           |

## Decisions Summary

1. **Source Map Format**: Use `typedlua-sourcemap` crate (JSON Source Map v3 with VLQ encoding) - no new crate needed
2. **Bundle Mode**: Support multi-file source maps in single concatenated output
3. **Coroutine Naming**: Use `debug.setname` (Lua 5.2+) for coroutine identification in UI
4. **State Preservation**: Warn users when hot reload cannot preserve all state
5. **Evaluate Mutation**: Read-only by default, opt-in via `wayfinder.evaluate.mutate: true`
6. **Remote Debugging**: Deferred (not in current plan)
7. **Hot Code Reload**: Full module reload (not function-level)

---

## Phase Breakdown

### Phase 1: Foundation (2 weeks)

**Goal:** CLI scaffolding and DAP transport

**Deliverables:**

```
crates/
├── wayfinder-core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Placeholder
└── wayfinder-cli/
    ├── Cargo.toml
    └── src/
        └── main.rs             # CLI entry point
```

**CLI Commands:**

```bash
wayfinder dap           # DAP server mode (stdio)
wayfinder launch        # Launch and debug
wayfinder attach        # Attach to running process
wayfinder version       # Show version
```

---

### Phase 2: Core DAP (4-5 weeks)

**Goal:** Full DAP server implementation

**Deliverables:**

```
wayfinder-core/src/
├── dap/
│   ├── mod.rs              # Protocol types
│   ├── messages.rs         # Specific messages
│   └── transport.rs        # JSON-RPC over stdio
├── debug/
│   ├── mod.rs              # Session orchestrator
│   ├── breakpoints.rs      # Breakpoint management
│   ├── stepping.rs         # Step operations
│   ├── stack.rs            # Stack frame extraction
│   ├── variables.rs        # Variable inspection
│   └── evaluate.rs         # Expression evaluation
├── runtime/
│   ├── mod.rs              # Runtime trait
│   ├── puc_lua.rs          # PUC Lua 5.1-5.4
│   └── luanext.rs          # LuaNext 5.1-5.4
└── session.rs              # Session lifecycle
```

**Runtime Abstraction:**

```rust
trait DebugRuntime {
    fn version(&self) -> RuntimeVersion;
    fn set_breakpoint(&self, file: &str, line: u32) -> Result<()>;
    fn step(&self, mode: StepMode) -> Result<()>;
    fn stack_trace(&self, thread_id: u64) -> Result<Vec<Frame>>;
    fn variables(&self, frame_id: u64, scope: VariableScope) -> Result<Vec<Variable>>;
    fn evaluate(&self, frame_id: u64, expression: &str) -> Result<Value>;
}
```

---

### Phase 3: Advanced Features (3 weeks)

**Goal:** Conditionals, watchpoints, mutation

**Deliverables:**

```
wayfinder-core/src/debug/
├── conditions.rs           # Expression evaluation for conditions
├── breakpoints/
│   ├── conditional.rs      # Condition evaluation
│   ├── logpoint.rs         # Log message without stopping
│   └── hit_count.rs        # Break after N hits
└── watchpoints.rs          # Data breakpoints
```

**Features:**

- **Conditional Breakpoints**: Lua expression must be truthy
- **Logpoints**: `{expression}` interpolation, no pause
- **Hit Count**: `>= N`, `== N`, `mod N`
- **Exception Filters**: Break on specific error types
- **Watchpoints**: Track variable value changes
- **Evaluate Mutation**: Opt-in via config

---

### Phase 4: LuaNext Integration (3-4 weeks)

**Goal:** Source maps and coroutine debugging

**Deliverables:**

```
crates/wayfinder-tl/
├── Cargo.toml
└── src/
    ├── lib.rs              # TypedLua wrapper entry
    ├── translator.rs       # Position translation
    ├── dap_wrapper.rs      # Intercept and translate DAP
    └── coroutine.rs        # Coroutine debugging
```

**Dependencies:**

```toml
[dependencies]
wayfinder-core = { path = "../wayfinder-core" }
typedlua-sourcemap = { path = "../typedlua/crates/typedlua-sourcemap" }
```

**Features:**

- JSON v3 source map loading (file, inline, data URI)
- Bundle mode support (multi-file source maps)
- Forward/reverse position translation
- Coroutine enumeration, switching, naming
- Source map preference: lenient/strict/ask

---

### Phase 5: Hot Code Reload (2 weeks)

**Goal:** Live code updates without restarting

**Deliverables:**

```
wayfinder-core/src/debug/
├── hot_reload.rs           # Module reload logic
└── state_capture.rs        # Capture/restore module state
```

**Workflow:**

1. Capture module state (globals, upvalues, tables)
2. Reload module source
3. Restore captured state
4. Warn on unpreserved state

**Example Warning:**

```
[Warning] Hot reload completed, but could not preserve:
  - Global 'foo': type changed from table to number
```

---

### Phase 6: CLI and Polish (2 weeks)

**Goal:** Production-ready CLI

**Commands:**

```bash
wayfinder dap           # stdio (default)
wayfinder dap --port 12345  # TCP server
wayfinder launch --runtime lua54 script.lua
wayfinder attach --port 9229
```

**Configuration (wayfinder.yaml):**

```yaml
runtime: lua54
stopOnEntry: false
sourceMapBehavior: ask
evaluate:
  mutate: false
coroutines:
  breakOnAll: true
```

---

## Dependencies

```toml
# wayfinder-core/Cargo.toml
[dependencies]
serde = "1"
serde_json = "1"
tokio = { version = "1", features = ["full"] }
thiserror = "1"

# wayfinder-cli/Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
serde = "1"
serde_yaml = "1"
wayfinder-core = { path = "../wayfinder-core" }

# wayfinder-tl/Cargo.toml
[dependencies]
wayfinder-core = { path = "../wayfinder-core" }
typedlua-sourcemap = { path = "../typedlua/crates/typedlua-sourcemap" }
```

---

## Timeline

| Phase                  | Duration        | Key Output                          |
|------------------------|-----------------|-------------------------------------|
| 1: Foundation          | 2 weeks         | CLI scaffolding                     |
| 2: Core DAP            | 4-5 weeks       | Full debug server                   |
| 3: Advanced Features   | 3 weeks         | Conditionals, watchpoints, mutation |
| 4: LuaNext Integration | 3-4 weeks       | Source maps + coroutines            |
| 5: Hot Code Reload     | 2 weeks         | Live code updates                   |
| 6: CLI and Polish      | 2 weeks         | Production CLI                      |
| **Total**              | **16-19 weeks** |                                     |

---

## Deferred

| Feature                   | Reason                 |
|---------------------------|------------------------|
| Remote debugging          | Lower priority         |
| Function-level hot reload | Full module sufficient |
| VSCode/Neovim extensions  | Future phase           |

---

## References

- Original design: [docs/DESIGN.md](./DESIGN.md)
- TypedLua repository: `/Users/forge18/Repos/typedlua`
- Source map implementation: `typedlua-sourcemap` crate
- Debug Adapter Protocol: [microsoft/debug-adapter-protocol](https://microsoft.github.io/debug-adapter-protocol)
