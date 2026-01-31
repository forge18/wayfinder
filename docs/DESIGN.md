# Wayfinder Design Document

**Repository:** luanext/wayfinder

A Debug Adapter Protocol (DAP) implementation for Lua and TypedLua.

## Overview

Wayfinder provides debugging capabilities for the Lua ecosystem. It implements the Debug Adapter Protocol for IDE integration and supports both plain Lua and TypedLua with source map translation.

**Key characteristics:**

- Rust-based DAP server
- Works with PUC Lua (5.1, 5.2, 5.3, 5.4) and LuaNext (5.1, 5.2, 5.3, 5.4)
- 8 runtime targets total
- Source map support for TypedLua debugging
- Breakpoints, stepping, stack inspection, variable watches
- IDE integration via standard DAP (VSCode, Neovim, JetBrains, etc.)

**Explicit non-goals:**

- Hot code reloading — separate concern
- Profiling — may be added later as separate tool
- Language features (syntax highlighting, LSP) — that's typedlua-lsp

## Architecture

Wayfinder is a monorepo containing the core debugger, TypedLua wrapper, and IDE extensions:

```
wayfinder/
├── crates/
│   ├── wayfinder-core/     # DAP implementation, debug logic
│   ├── wayfinder-cli/      # Binary
│   ├── wayfinder-lua/      # Debug helper scripts injected into Lua
│   └── wayfinder-tl/       # TypedLua source map translation
├── editors/
│   ├── vscode/             # VSCode extension
│   └── neovim/             # Neovim plugin
└── docs/
```

### Component Overview

Four crates plus IDE extensions:

```
┌─────────────────────────────────────────────────────────────────┐
│                      lua-sourcemap                              │
│                 (luanext-lib/lua-sourcemap)                     │
│                                                                 │
│  • Source map format definition                                 │
│  • Read/write bincode-serialized maps                          │
│  • Forward lookup: generated → original                        │
│  • Reverse lookup: original → generated                        │
│  • IndexedSourceMap for fast bidirectional queries             │
└──────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                        wayfinder                                │
│                   (lua-dap, core debugger)                      │
│                                                                 │
│  • DAP server implementation                                    │
│  • Breakpoints, stepping, stack frames                         │
│  • Variable inspection and watch expressions                   │
│  • Works with PUC Lua debug library                            │
│  • Works with LuaNext (native hooks or debug lib)              │
│  • No source map dependency — debugs .lua directly             │
└──────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                      wayfinder-tl                               │
│                   (typedlua-dap, wrapper)                       │
│                                                                 │
│  • Wraps wayfinder for TypedLua debugging                     │
│  • Intercepts DAP requests, translates positions               │
│  • Consumes source maps from lua-sourcemap                     │
│  • User sets breakpoint in .tl → translated to .lua            │
│  • Stack frames show .tl positions, not generated .lua         │
└──────────────────────────────────────────────────────────────────┘
```

**Dependency graph:**

```
luanext-lib/lua-sourcemap (standalone)
      │
      ├──────────────────┐
      │                  │
      ▼                  ▼
wayfinder           wayfinder-tl
(no sourcemap        (uses both)
 dependency)              │
      ▲                   │
      └───────────────────┘
         delegates to
```

---

## lua-sourcemap

### Format

Bincode-serialized Rust structs. No compression.

```rust
#[derive(Serialize, Deserialize)]
pub struct SourceMap {
    /// Schema version for evolution
    pub version: u8,
    
    /// Generated file path (e.g., "foo.lua")
    pub generated_file: String,
    
    /// Original source files (index → path)
    pub sources: Vec<String>,
    
    /// Optional embedded source content
    pub sources_content: Option<Vec<String>>,
    
    /// Position mappings, sorted by (gen_line, gen_col)
    pub mappings: Vec<Mapping>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Mapping {
    pub gen_line: u32,
    pub gen_col: u16,
    pub src_idx: u16,
    pub orig_line: u32,
    pub orig_col: u16,
}
```

### File Convention

```
foo.tl           → TypedLua source
foo.lua          → Generated Lua
foo.lua.map      → Source map (bincode)
```

### Lookups

**Forward lookup** (generated → original):

Used when debugger hits a breakpoint or steps to show user the original position.

```rust
impl SourceMap {
    pub fn lookup(&self, gen_line: u32, gen_col: u16) -> Option<OriginalLocation> {
        // Binary search - mappings sorted by generated position
        let idx = self.mappings
            .binary_search_by_key(&(gen_line, gen_col), |m| (m.gen_line, m.gen_col))
            .unwrap_or_else(|i| i.saturating_sub(1));
        
        self.mappings.get(idx).map(|m| OriginalLocation {
            file: &self.sources[m.src_idx as usize],
            line: m.orig_line,
            col: m.orig_col,
        })
    }
}
```

**Reverse lookup** (original → generated):

Used when user sets breakpoint in .tl file to find corresponding .lua position.

```rust
pub struct IndexedSourceMap {
    map: SourceMap,
    by_original: Vec<usize>,  // indices sorted by (src_idx, orig_line, orig_col)
}

impl IndexedSourceMap {
    pub fn new(map: SourceMap) -> Self {
        let mut by_original: Vec<usize> = (0..map.mappings.len()).collect();
        by_original.sort_by_key(|&i| {
            let m = &map.mappings[i];
            (m.src_idx, m.orig_line, m.orig_col)
        });
        Self { map, by_original }
    }
    
    pub fn reverse_lookup(&self, src_idx: u16, orig_line: u32) -> Option<GeneratedLocation> {
        // Binary search on secondary index
        let idx = self.by_original
            .binary_search_by_key(&(src_idx, orig_line, 0), |&i| {
                let m = &self.map.mappings[i];
                (m.src_idx, m.orig_line, m.orig_col)
            })
            .unwrap_or_else(|i| i);
        
        self.by_original.get(idx).map(|&i| {
            let m = &self.map.mappings[i];
            GeneratedLocation {
                line: m.gen_line,
                col: m.gen_col,
            }
        })
    }
}
```

### Crate Structure

```
lua-sourcemap/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── types.rs      # SourceMap, Mapping, locations
    ├── read.rs       # from_file, from_reader
    ├── write.rs      # to_file, to_writer
    └── lookup.rs     # SourceMap::lookup, IndexedSourceMap
```

### Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
bincode = "1"
```

---

## wayfinder (lua-dap)

Core debugger implementing DAP for plain Lua files.

### DAP Support

**Requests:**

| Request | Status | Notes |
|---------|--------|-------|
| initialize | ✓ | Capabilities negotiation |
| launch | ✓ | Start Lua process |
| attach | ✓ | Attach to running process |
| disconnect | ✓ | End debug session |
| setBreakpoints | ✓ | Line breakpoints |
| setFunctionBreakpoints | ✓ | Break on function entry |
| setExceptionBreakpoints | ✓ | Break on Lua errors |
| continue | ✓ | Resume execution |
| next | ✓ | Step over |
| stepIn | ✓ | Step into function |
| stepOut | ✓ | Step out of function |
| pause | ✓ | Pause execution |
| stackTrace | ✓ | Get call stack |
| scopes | ✓ | Get variable scopes |
| variables | ✓ | Get variables in scope |
| evaluate | ✓ | Evaluate expression |
| source | ✓ | Get source code |

**Events:**

| Event | Description |
|-------|-------------|
| initialized | Debug adapter ready |
| stopped | Execution paused (breakpoint, step, etc.) |
| continued | Execution resumed |
| exited | Process exited |
| terminated | Debug session ended |
| output | stdout/stderr from debuggee |

### Runtime Support

Wayfinder supports 8 runtime targets:

| Runtime | Lua 5.1 | Lua 5.2 | Lua 5.3 | Lua 5.4 |
|---------|---------|---------|---------|--------|
| PUC Lua | ✓ | ✓ | ✓ | ✓ |
| LuaNext | ✓ | ✓ | ✓ | ✓ |

### Runtime Integration

**PUC Lua (5.1-5.4):**

Uses the `debug` library:

```lua
debug.sethook(fn, mask, count)  -- Install hook for line/call/return
debug.getinfo(level, what)       -- Stack frame info
debug.getlocal(level, index)     -- Local variables
debug.setlocal(level, index, v)  -- Modify locals
debug.getupvalue(fn, index)      -- Closure upvalues
debug.setupvalue(fn, index, v)   -- Modify upvalues
```

Hook function handles:

- Line events: check breakpoints, handle stepping
- Call events: function breakpoints, step into
- Return events: step out

**LuaNext (5.1-5.4):**

Two options:

1. Use the same debug library approach (compatibility)
2. Native debugging hooks in the JIT (better performance, future)

Initial implementation uses debug library for both runtimes.

**Version-specific considerations:**

| Feature | 5.1 | 5.2 | 5.3 | 5.4 |
|---------|-----|-----|-----|-----|
| debug.sethook | ✓ | ✓ | ✓ | ✓ |
| debug.getlocal | ✓ | ✓ | ✓ | ✓ |
| debug.getupvalue | ✓ | ✓ | ✓ | ✓ |
| debug.upvalueid | — | ✓ | ✓ | ✓ |
| debug.setcstacklimit | — | — | — | ✓ |

Wayfinder adapts to available debug library features per version.

### Communication

DAP uses JSON-RPC over stdio:

```
IDE ←─ JSON-RPC over stdio ─→ wayfinder ←─ debug lib ─→ Lua process
```

Wayfinder spawns or attaches to the Lua process and injects a debug helper script that communicates back.

### CLI

```bash
# Launch mode (PUC Lua)
wayfinder launch --runtime lua54 script.lua

# Launch mode (LuaNext)
wayfinder launch --runtime luanext54 script.lua

# Attach mode
wayfinder attach --port 9229

# DAP server mode (for IDE integration)
wayfinder dap
```

**Runtime options:**

| Option | Runtime |
|--------|--------|
| `lua51` | PUC Lua 5.1 |
| `lua52` | PUC Lua 5.2 |
| `lua53` | PUC Lua 5.3 |
| `lua54` | PUC Lua 5.4 |
| `luanext51` | LuaNext 5.1 |
| `luanext52` | LuaNext 5.2 |
| `luanext53` | LuaNext 5.3 |
| `luanext54` | LuaNext 5.4 |

### Configuration

```yaml
# wayfinder.yaml (optional)
runtime: lua54          # lua51, lua52, lua53, lua54, luanext51, luanext52, luanext53, luanext54
stopOnEntry: false
cwd: .
env:
  LUA_PATH: "./?.lua"
```

### Crate Structure

See monorepo structure above. Core crates:

```
crates/
├── wayfinder-core/     # DAP implementation, debug logic
├── wayfinder-cli/      # Binary
└── wayfinder-lua/      # Debug helper scripts injected into Lua
```

---

## wayfinder-tl (typedlua-dap)

Thin wrapper that adds source map translation for TypedLua.

### How It Works

```
User sets breakpoint in foo.tl:42
         │
         ▼
wayfinder-tl receives setBreakpoints request
         │
         ▼
Load foo.lua.map, reverse lookup: tl:42 → lua:37
         │
         ▼
Forward to wayfinder: setBreakpoints for foo.lua:37
         │
         ▼
Runtime hits breakpoint at foo.lua:37
         │
         ▼
wayfinder sends stopped event with foo.lua:37
         │
         ▼
wayfinder-tl intercepts, forward lookup: lua:37 → tl:42
         │
         ▼
Send to IDE: stopped at foo.tl:42
```

### Translated Requests/Events

**Requests (original → generated):**

- `setBreakpoints`: Translate source paths and line numbers
- `source`: Return .tl content, not .lua

**Events (generated → original):**

- `stopped`: Translate frame locations
- `stackTrace` response: Translate all frame locations

### Configuration

```yaml
# wayfinder.yaml
typedlua:
  enabled: true
  sourceMapDir: ./build    # Where .lua.map files live
```

### Crate Structure

Part of the wayfinder monorepo:

```
crates/
└── wayfinder-tl/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── translator.rs    # Position translation logic
        └── dap_wrapper.rs   # Intercept and translate DAP messages
```

### Dependencies

```toml
[dependencies]
wayfinder-core = { path = "../wayfinder-core" }
lua-sourcemap = "1"  # External crate
```

---

## IDE Extensions

Wayfinder includes optional IDE extensions that enhance the debugging experience. These are not required—DAP works without them—but they smooth out common workflows.

### editors/vscode

VSCode extension for improved Lua debugging UX.

**Features:**

| Feature | Description |
|---------|-------------|
| Debug configuration snippets | Type "wayfinder" in launch.json → get templates |
| Runtime auto-detection | Scans PATH for lua51/lua54/luanext, populates dropdown |
| "Debug File" command | One click to debug current file, no config needed |
| "Debug Test" CodeLens | Click above `describe()` or `it()` to debug that test |
| Runtime picker | Status bar shows current runtime, click to switch |
| Attach helper | List running Lua processes, click to attach |

**What it doesn't do:**

- Implement DAP (that's wayfinder-core)
- Syntax highlighting (that's a language extension)
- LSP features (that's typedlua-lsp)

**Structure:**

```
editors/vscode/
├── package.json           # Extension manifest, contributes debuggers
├── src/
│   ├── extension.ts       # Activation, commands
│   ├── runtimeDetector.ts # Find installed Lua runtimes
│   ├── debugProvider.ts   # Dynamic debug configurations
│   └── codeLens.ts        # "Debug Test" links for Canary
├── schemas/
│   └── launch.schema.json # JSON schema for launch.json validation
└── package-lock.json
```

**package.json contributes:**

```json
{
  "contributes": {
    "debuggers": [
      {
        "type": "wayfinder",
        "label": "Wayfinder: Lua",
        "program": "wayfinder",
        "args": ["dap"],
        "languages": ["lua"]
      },
      {
        "type": "wayfinder-tl",
        "label": "Wayfinder: TypedLua",
        "program": "wayfinder-tl",
        "args": ["dap"],
        "languages": ["typedlua"]
      }
    ],
    "commands": [
      { "command": "wayfinder.debugFile", "title": "Debug Current File" },
      { "command": "wayfinder.selectRuntime", "title": "Select Lua Runtime" }
    ]
  }
}
```

### editors/neovim

Neovim plugin that configures nvim-dap for Wayfinder.

**Features:**

| Feature | Description |
|---------|-------------|
| Auto-configure nvim-dap | `:WayfinderSetup` registers adapters and configs |
| Runtime detection | Finds available Lua runtimes |
| Telescope picker | Select runtime interactively |
| Canary integration | Debug test under cursor |

**Structure:**

```
editors/neovim/
├── lua/
│   └── wayfinder/
│       ├── init.lua       # Setup, user config
│       ├── dap.lua        # nvim-dap configuration
│       ├── runtime.lua    # Detect installed runtimes
│       └── telescope.lua  # Runtime picker extension
├── plugin/
│   └── wayfinder.lua      # Auto-load commands
└── README.md
```

**Usage:**

```lua
-- In init.lua or lazy.nvim config
require('wayfinder').setup({
  default_runtime = 'lua54',  -- or 'luanext54'
  auto_detect = true,         -- Scan PATH for runtimes
})

-- Commands
vim.keymap.set('n', '<leader>dd', ':WayfinderDebugFile<CR>')
vim.keymap.set('n', '<leader>dt', ':WayfinderDebugTest<CR>')
vim.keymap.set('n', '<leader>dr', ':WayfinderSelectRuntime<CR>')
```

---

## IDE Integration (Manual)

For IDEs without a Wayfinder extension, DAP works out of the box with manual configuration.

### VSCode (manual)

DAP works out of the box with VSCode's generic debug adapter support. A simple `launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "wayfinder",
      "request": "launch",
      "name": "Debug Lua (PUC 5.4)",
      "program": "${file}",
      "runtime": "lua54"
    },
    {
      "type": "wayfinder",
      "request": "launch",
      "name": "Debug Lua (LuaNext 5.4)",
      "program": "${file}",
      "runtime": "luanext54"
    },
    {
      "type": "wayfinder-tl",
      "request": "launch",
      "name": "Debug TypedLua",
      "program": "${file}",
      "runtime": "luanext54"
    }
  ]
}
```

### Neovim (manual)

Without the wayfinder.nvim plugin, configure nvim-dap directly:

```lua
local dap = require('dap')

dap.adapters.wayfinder = {
  type = 'executable',
  command = 'wayfinder',
  args = { 'dap' },
}

dap.configurations.lua = {
  {
    type = 'wayfinder',
    request = 'launch',
    name = 'Debug Lua (PUC 5.4)',
    program = '${file}',
    runtime = 'lua54',  -- lua51, lua52, lua53, lua54
  },
  {
    type = 'wayfinder',
    request = 'launch',
    name = 'Debug Lua (LuaNext 5.4)',
    program = '${file}',
    runtime = 'luanext54',  -- luanext51, luanext52, luanext53, luanext54
  },
}
```

### JetBrains

JetBrains IDEs support DAP through plugins. Configure a "Debug Adapter" run configuration pointing to `wayfinder dap`.

### Integration with typedlua-lsp

The LSP and DAP servers remain separate processes. However, they can share:

- Source map loading (both need it)
- File watching
- Configuration

Future consideration: Single binary with `--lsp` and `--dap` modes sharing state.

---

## Integration with Ecosystem

### Canary

Debug failing tests:

```bash
canary test --debug tests/foo_test.lua
# Launches wayfinder, sets breakpoint at test, runs
```

### Error Messages

LuaNext and TypedLua can use lua-sourcemap to translate error positions:

```
-- Runtime error in generated code
foo.lua:37: attempt to index nil value

-- With source map translation
foo.tl:42: attempt to index nil value
```

### Coverage

Canary's coverage tool can use source maps to report coverage against .tl files:

```
Coverage Report (foo.tl):
  Line 42: covered
  Line 43: covered
  Line 44: not covered
```

---

## Dependencies Summary

**Rust crates:**

| Crate | Dependencies |
|-------|--------------|
| lua-sourcemap | serde, bincode |
| wayfinder-core | serde, serde_json (DAP), tokio (async IO) |
| wayfinder-cli | wayfinder-core, clap |
| wayfinder-tl | wayfinder-core, lua-sourcemap |

**IDE extensions:**

| Extension | Dependencies |
|-----------|--------------|
| editors/vscode | vscode (types), typescript |
| editors/neovim | nvim-dap (peer) |

---

## Implementation Phases

### Phase 1: lua-sourcemap

- [ ] Define types (SourceMap, Mapping)
- [ ] Implement read/write with bincode
- [ ] Implement forward lookup
- [ ] Implement IndexedSourceMap with reverse lookup
- [ ] Unit tests
- [ ] Publish to crates.io

### Phase 2: wayfinder-core

- [ ] DAP protocol types
- [ ] JSON-RPC transport
- [ ] PUC Lua debug adapter (debug library integration)
- [ ] Breakpoints (line, function, exception)
- [ ] Stepping (next, stepIn, stepOut)
- [ ] Stack inspection
- [ ] Variable inspection
- [ ] Expression evaluation

### Phase 3: wayfinder-cli

- [ ] Launch mode
- [ ] Attach mode
- [ ] DAP server mode
- [ ] Configuration file support

### Phase 4: wayfinder-tl

- [ ] Source map loading
- [ ] Request translation (setBreakpoints, source)
- [ ] Event translation (stopped, stackTrace)
- [ ] Integration tests with TypedLua

### Phase 5: IDE Extensions

- [ ] VSCode extension scaffolding
- [ ] Debug configuration provider
- [ ] Runtime auto-detection
- [ ] "Debug File" command
- [ ] "Debug Test" CodeLens (Canary integration)
- [ ] Publish to VSCode marketplace
- [ ] Neovim plugin scaffolding
- [ ] nvim-dap auto-configuration
- [ ] Telescope runtime picker
- [ ] Publish to GitHub / luarocks

### Phase 6: Documentation & Polish

- [ ] Getting started guide
- [ ] Manual IDE configuration docs (JetBrains, Emacs)
- [ ] Troubleshooting guide
- [ ] Video walkthrough

---

## Open Questions

1. **LuaNext native debugging** — Should LuaNext expose debugging hooks beyond the standard debug library? Would enable better performance and JIT-aware debugging.

2. **Conditional breakpoints** — DAP supports conditions. Implement by evaluating Lua expression at breakpoint?

3. **Remote debugging** — Support debugging Lua processes on remote machines over TCP?

4. **Multi-file source maps** — TypedLua projects compile many .tl files. Single project-wide source map or one per file?
