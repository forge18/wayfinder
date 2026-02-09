# Wayfinder - Lua Debugger for VSCode

A powerful Debug Adapter Protocol (DAP) implementation for debugging Lua and LuaNext scripts in VSCode. Wayfinder supports multiple Lua versions (5.1, 5.2, 5.3, 5.4) and LuaNext with full source map support.

## Features

- 🐛 **Multi-Version Lua Support**: Debug Lua 5.1, 5.2, 5.3, 5.4, and LuaNext
- 🎯 **Auto-Detection**: Automatically detects the appropriate Lua runtime
- 📍 **Breakpoints**: Set conditional and unconditional breakpoints
- 🔍 **Variable Inspection**: Inspect variables and evaluate expressions
- 🔄 **Step Control**: Step over, into, and out of functions
- 📦 **Source Maps**: Full support for bundled LuaNext files with source maps
- 🎨 **Call Stack**: Navigate through call stacks during debugging
- ⚡ **Hot Reload**: Integration with Wayfinder's hot reload functionality

## Installation

### Prerequisites

- VSCode 1.75.0 or later
- Wayfinder CLI installed and in PATH (or configured path)
- Appropriate Lua runtime(s) installed

### Install from VSCode

1. Open VSCode
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Wayfinder"
4. Click Install

### Manual Installation

1. Clone or download the extension
2. Run `npm install` in the extension directory
3. Run `npm run esbuild` to build
4. Package with `npm run package`

## Quick Start

### 1. Debug a Lua File

#### Option A: Use Command Palette
1. Open a `.lua` or `.luax` file
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
3. Type "Debug File"
4. The debugger will start automatically

#### Option B: Use Right-Click Context Menu
1. Right-click on a `.lua` or `.luax` file in the editor
2. Select "Debug File"

#### Option C: Use Launch Configuration
1. Create or open `.vscode/launch.json`
2. Add a debug configuration:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "wayfinder",
      "request": "launch",
      "name": "Launch Lua Script",
      "program": "${workspaceFolder}/main.lua",
      "cwd": "${workspaceFolder}",
      "runtime": "lua54",
      "stopOnEntry": false
    }
  ]
}
```

3. Press F5 to start debugging

### 2. Debug with Different Lua Versions

In your launch configuration, specify the runtime:

```json
{
  "type": "wayfinder",
  "request": "launch",
  "name": "Debug with Lua 5.3",
  "program": "${workspaceFolder}/main.lua",
  "runtime": "lua53"
}
```

Available runtimes:
- `lua51` - Lua 5.1
- `lua52` - Lua 5.2
- `lua53` - Lua 5.3
- `lua54` - Lua 5.4 (default)
- `luanext` - LuaNext

### 3. Debug LuaNext Files

For `.luax` files, the debugger automatically selects the LuaNext runtime:

```json
{
  "type": "wayfinder",
  "request": "launch",
  "name": "Debug LuaNext",
  "program": "${workspaceFolder}/main.luax",
  "runtime": "luanext"
}
```

### 4. Attach to Running Process

To attach to an already-running Lua process with DAP enabled:

1. Press `Ctrl+Shift+P` and type "Attach to Process"
2. Enter the port number
3. Enter the host (default: localhost)

Or use a launch configuration:

```json
{
  "type": "wayfinder",
  "request": "attach",
  "name": "Attach to Process",
  "port": 5858,
  "host": "localhost"
}
```

## Configuration

Configure Wayfinder via VSCode settings. Open settings (Ctrl+, or Cmd+,) and search for "wayfinder":

### Debug Settings

- **`wayfinder.debug.port`** (number, default: 5858)
  - Default DAP port (auto-incremented if multiple sessions)

- **`wayfinder.debug.autoDetectRuntime`** (boolean, default: true)
  - Automatically detect Lua runtime from workspace configuration

- **`wayfinder.debug.sourceMapBehavior`** (string, options: "ask", "lenient", "strict", default: "ask")
  - How to handle source maps for bundled LuaNext files

### Runtime Paths

- **`wayfinder.runtime.lua51.path`** (string, default: "lua5.1")
  - Path to Lua 5.1 binary

- **`wayfinder.runtime.lua52.path`** (string, default: "lua5.2")
  - Path to Lua 5.2 binary

- **`wayfinder.runtime.lua53.path`** (string, default: "lua5.3")
  - Path to Lua 5.3 binary

- **`wayfinder.runtime.lua54.path`** (string, default: "lua5.4")
  - Path to Lua 5.4 binary

- **`wayfinder.runtime.luanext.path`** (string, default: "luanext")
  - Path to LuaNext binary

### Wayfinder Path

- **`wayfinder.wayfinder.path`** (string, default: "wayfinder")
  - Path to Wayfinder CLI binary (auto-detected if not set)

### Example Configuration

`.vscode/settings.json`:

```json
{
  "wayfinder.debug.port": 5858,
  "wayfinder.debug.autoDetectRuntime": true,
  "wayfinder.debug.sourceMapBehavior": "lenient",
  "wayfinder.runtime.lua54.path": "/usr/local/bin/lua5.4",
  "wayfinder.runtime.luanext.path": "~/.cargo/bin/luanext",
  "wayfinder.wayfinder.path": "~/.cargo/bin/wayfinder"
}
```

## Launch Configuration Reference

### Launch Request

Starts a new Lua process and debugs it.

```json
{
  "type": "wayfinder",
  "request": "launch",
  "name": "Launch Script",
  "program": "${workspaceFolder}/main.lua",
  "cwd": "${workspaceFolder}",
  "args": ["arg1", "arg2"],
  "runtime": "lua54",
  "port": 5858,
  "stopOnEntry": false,
  "console": "integratedTerminal"
}
```

**Properties:**
- `program` (required): Path to the Lua script
- `cwd`: Working directory (default: ${workspaceFolder})
- `args`: Command-line arguments to pass to the script
- `runtime`: Lua version to use (default: lua54)
- `port`: DAP port (default: 5858, auto-incremented)
- `stopOnEntry`: Stop at the first line (default: false)
- `console`: Where to display output ("integratedTerminal", "externalTerminal", "internalConsole")

### Attach Request

Attaches to an already-running Lua process.

```json
{
  "type": "wayfinder",
  "request": "attach",
  "name": "Attach",
  "port": 5858,
  "host": "localhost"
}
```

**Properties:**
- `port` (required): DAP port of the running process
- `host`: Host address (default: "localhost")

## Commands

| Command | Binding | Description |
|---------|---------|-------------|
| `wayfinder.debugFile` | Right-click context menu | Debug the current file |
| `wayfinder.selectRuntime` | Command palette | Select or verify Lua runtime |
| `wayfinder.attachProcess` | Command palette | Attach to a running process |

## Debugging Workflow

### 1. Setting Breakpoints
- Click on the line number to set a breakpoint
- Right-click on a breakpoint to add conditions

### 2. Starting Debugging
- Press F5 or use "Run > Start Debugging"
- Select "Wayfinder" configuration if prompted

### 3. Using the Debug Interface
- **Debug Toolbar**: Control execution (Continue, Pause, Step Over, Step Into, Step Out, Restart, Stop)
- **Variables Panel**: View and inspect local/global variables
- **Call Stack**: Navigate through the call stack
- **Debug Console**: Evaluate Lua expressions

### 4. Debugging LuaNext with Source Maps
- Source maps are automatically detected
- Step through bundled code with original source visibility
- Breakpoints set in original sources are translated to bundle positions

## Troubleshooting

### Wayfinder Binary Not Found

If you see "Failed to start Wayfinder":

1. Ensure Wayfinder is installed:
   ```bash
   wayfinder --version
   ```

2. If not in PATH, set the custom path in settings:
   ```json
   {
     "wayfinder.wayfinder.path": "/path/to/wayfinder"
   }
   ```

### Lua Runtime Not Found

If the debugger can't find your Lua installation:

1. Check which version you have:
   ```bash
   lua -v
   lua5.4 -v
   ```

2. Configure the correct path:
   ```json
   {
     "wayfinder.runtime.lua54.path": "/path/to/lua5.4"
   }
   ```

### Breakpoints Not Working

1. Ensure you're debugging a launch configuration (not attach)
2. Check that the script file is accessible
3. Verify the runtime version matches your script requirements

### Debug Session Won't Start

1. Check the Debug Console for error messages
2. Verify your launch configuration
3. Ensure the program path is correct
4. Try running the script directly: `lua your_script.lua`

## Development

### Building

```bash
npm install
npm run esbuild
```

### Development Mode

For active development with hot reload:

```bash
npm run esbuild-watch
```

### Testing

```bash
npm run pretest
npm test
```

### Linting

```bash
npm run lint
```

### Packaging

To create a `.vsix` file for distribution:

```bash
npm run package
```

## Architecture

The extension consists of:

- **`extension.ts`**: Main entry point, registers providers and commands
- **`configuration.ts`**: Runtime detection and configuration management
- **`debug-provider.ts`**: VSCode debug configuration provider
- **`adapter.ts`**: Debug adapter descriptor factory
- **`runtime-manager.ts`**: Manages debug session lifecycle
- **`commands.ts`**: Command handlers for user actions

## Project Links

- [Wayfinder GitHub](https://github.com/forge18/wayfinder)
- [VSCode Debug Protocol](https://microsoft.github.io/debug-adapter-protocol/)
- [LuaNext](https://github.com/forge18/luanext)

## License

MIT

## Contributing

Contributions are welcome! Please submit issues and pull requests to the [Wayfinder repository](https://github.com/forge18/wayfinder).
