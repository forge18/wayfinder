# Wayfinder IDE Extensions Implementation Plan

## Overview

This document outlines the implementation plan for Wayfinder IDE extensions, focusing on VSCode as the primary target with Neovim as a secondary target. The extensions will provide seamless debugging integration for both standard Lua (.lua) and LuaNext (.luax) files.

## Current State Analysis

### ✅ Fully Functional Backend

- Wayfinder CLI with complete DAP protocol implementation
- Multi-runtime support (PUC Lua 5.1-5.4, LuaNext)
- Launch/attach/DAP server modes
- Hot reload functionality
- Source map translation for bundled LuaNext files

### ✅ Existing Infrastructure

- LuaNext VSCode extension at `crates/luanext/editors/vscode/` (language support only)

### ❌ Missing Components

- Wayfinder debugging extension for VSCode
- Neovim DAP plugin

## Implementation Approach

Based on user requirements:

- **Priority:** VSCode Extension First
- **File Support:** Both .lua and .luax files
- **Runtime Detection:** Auto-detect from workspace
- **Location:** `editors/vscode/`

## Phase 1: VSCode Extension Foundation (1 week)

**Location:** `editors/vscode/`

### Create

1. **`package.json`** - Extension manifest
   - Contributes debug configuration type "wayfinder"
   - Registers "Debug Lua File" command
   - Language support for both `.lua` and `.luax`
   - Configuration schema for runtimes, ports, etc.

2. **`src/extension.ts`** - Main extension entry
   - Register debug configuration provider
   - Register command handlers
   - Auto-detect Wayfinder binary location

3. **`src/configuration.ts`** - Runtime detection
   - Scan workspace for `wayfinder.yaml`
   - Detect available Lua versions (lua5.1, lua5.2, etc.)
   - Auto-suggest runtime based on file type (.lua → PUC, .luax → LuaNext)

4. **`src/debug-provider.ts`** - DebugConfigurationProvider
   - Provide initial debug configurations
   - Resolve dynamic configurations
   - Handle compound configurations

**Test:** Simple F5 debugging with auto-detected runtime

## Phase 2: Debug Adapter Integration (3 days)

### Create

1. **`src/adapter.ts`** - WayfinderDebugAdapterDescriptorFactory
   - Spawn Wayfinder process for debugging
   - Handle stdio communication
   - Manage Wayfinder lifecycle

2. **`src/protocol.ts`** - DAP message helpers
   - Translate VSCode debug requests to Wayfinder
   - Handle DAP events from Wayfinder
   - Logging and error handling

3. **`src/runtime-manager.ts`** - Runtime lifecycle
   - Start/stop Wayfinder processes
   - Track active debug sessions
   - Cleanup on extension deactivate

**Test:** Launch debugging with breakpoints working

## Phase 3: Features & Polish (4 days)

### Commands

1. **"Debug File"** - Right-click context menu on .lua/.luax files
2. **Debug CodeLens** - "Debug Test" above `test("...", function() ...)` blocks
3. **"Select Runtime"** - Command palette to choose/verify Lua version
4. **"Attach to Process"** - List running Lua processes with ports/PIDs

### Debugging Features

- Variable inspection in sidebar
- Breakpoints, conditional breakpoints
- Step over/in/out
- Call stack navigation
- Debug console with Lua evaluation

### Configuration Options

- `wayfinder.debug.port` - Default DAP port (auto-increment)
- `wayfinder.runtime.lua51.path` - Custom Lua 5.1 binary path
- `wayfinder.runtime.lua52.path` - Custom Lua 5.2 binary path
- `wayfinder.runtime.lua53.path` - Custom Lua 5.3 binary path
- `wayfinder.runtime.lua54.path` - Custom Lua 5.4 binary path
- `wayfinder.runtime.luanext.path` - LuaNext binary path
- `wayfinder.debug.autoDetectRuntime` - Enable auto-detection (default: true)
- `wayfinder.debug.sourceMapBehavior` - "ask" | "lenient" | "strict"

## Phase 4: LuaNext/Bundle Support (3 days)

### Source Maps

- Auto-detect .luax files with inline source maps
- Handle multi-file bundles through DAP translator
- Reverse lookup for breakpoints in original sources

### Bundle Debugging

- "Debug Bundle" command for complex projects
- Source map preference UI (ask/lenient/strict)
- Seamless stepping between .luax → .lua source

### Integration

- Collaborate with existing LuaNext language extension
- Share configuration where appropriate
- Don't conflict with luanext-lsp features

## Phase 5: Documentation & Testing (2 days)

### Create

1. **`README.md`** - Installation, configuration, usage examples
2. **`TESTING.md`** - Test plan and manual test cases
3. **Example configurations** in `examples/` directory
4. **Debug configuration snippets** for common setups

### Test Coverage

- All Lua versions (5.1, 5.2, 5.3, 5.4, LuaNext)
- Basic debugging workflow
- Conditional breakpoints
- Hot reload integration
- Source maps (.luax files)
- Corona/LÖVE framework projects

## Phase 6: Neovim Plugin (2 weeks - deferred)

**Location:** `editors/neovim/`

### Structure

```
editors/neovim/
├── lua/wayfinder/
│   ├── init.lua              # Main plugin entry
│   ├── dap.lua               # DAP adapter configuration
│   ├── config.lua            # Configuration management
│   ├── commands.lua          # User commands (:WayfinderDebug, etc)
│   └── telescope.lua         # Telescope integration for runtime picker
├── plugin/wayfinder.vim      # Vim plugin entry
└── README.md
```

### Features

- Auto-configure nvim-dap
- `:WayfinderDebugFile` command
- `:WayfinderSelectRuntime` with Telescope
- Runtime auto-detection from workspace
- Support for both .lua and .luax files

## Dependencies & Assumptions

### VSCode API Dependencies

- `@vscode/debugadapter` - DAP protocol implementation
- `@vscode/debugprotocol` - TypeScript types for DAP
- Existing Wayfinder binary accessible in PATH or configurable

### Development Dependencies

- TypeScript 5.3+
- ESBuild or Webpack for bundling
- Mocha for testing

### File Structure

```
editors/vscode/
├── package.json
├── tsconfig.json
├── webpack.config.js
├── src/
│   ├── extension.ts
│   ├── configuration.ts
│   ├── debug-provider.ts
│   ├── adapter.ts
│   ├── protocol.ts
│   ├── runtime-manager.ts
│   └── commands.ts
├── test/
│   ├── configuration.test.ts
│   └── debug-provider.test.ts
└── README.md
```

## Timeline

- **Phase 1:** 1 week
- **Phase 2:** 3 days
- **Phase 3:** 4 days
- **Phase 4:** 3 days
- **Phase 5:** 2 days

**Total: ~3 weeks** for fully functional VSCode extension

## Risk Mitigation

1. **Wayfinder Binary Detection:**
   - Try `which wayfinder` → `~/.cargo/bin/wayfinder` → config path
   - Provide explicit config option if auto-detection fails

2. **Multiple Lua Versions:**
   - Use `runtime.Version` enum from Wayfinder core
   - Test each version with factorial.lua example

3. **Source Map Complexity:**
   - Phase 4 isolated to DAP translator layer
   - Fallback to "lenient" mode (debug generated code)
