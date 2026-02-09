# Wayfinder - Lua Debugger for ZeroBrane Studio

Integrate Wayfinder DAP debugger directly into ZeroBrane Studio for seamless Lua debugging. Supports Lua 5.1, 5.2, 5.3, 5.4, and LuaNext with automatic runtime detection.

## Features

- 🐛 **Multi-Version Lua Support**: Debug Lua 5.1, 5.2, 5.3, 5.4, and LuaNext
- 🎯 **Auto-Detection**: Automatically detects Lua runtime from file extension
- 📍 **Breakpoints**: Line breakpoints with conditional support
- 🔍 **Variable Inspection**: Inspect locals, globals, and upvalues
- 🔄 **Step Control**: Step over, into, and out of functions
- 📦 **Source Maps**: Support for LuaNext bundled files
- 🌍 **Project Config**: Load settings from `wayfinder.yaml` or `wayfinder.toml`
- ⚡ **Quick Start**: 5-minute setup with auto-detection

## Installation

### ZeroBrane Studio Version

**Minimum Version**: ZeroBrane Studio 1.73+

### Installation Steps

1. **Download the plugin:**
   ```bash
   cd ~/.zbstudio/packages
   git clone https://github.com/forge18/wayfinder
   ```

   Or for Windows:
   ```bash
   cd "C:\Users\YourUsername\.zbstudio\packages"
   git clone https://github.com/forge18/wayfinder
   ```

2. **Copy plugin files:**
   ```bash
   cp wayfinder/editors/zerobrane/* ~/.zbstudio/packages/wayfinder/
   ```

3. **Restart ZeroBrane Studio**

### From Source

```bash
cd /path/to/wayfinder/editors/zerobrane
# Copy to ZeroBrane plugins directory
cp -r . ~/.zbstudio/packages/wayfinder/
```

## Quick Start (5 minutes)

### 1. Create a Test Script

Create `test.lua`:

```lua
local function greet(name)
  print("Hello, " .. name .. "!")
  return name
end

local result = greet("Lua")
print("Result:", result)
```

### 2. Set a Breakpoint

Click on the line number to set a breakpoint:
- A red dot indicates the breakpoint

### 3. Start Debugging

**Option A: Menu**
```
Tools → Debug File
```

**Option B: Keyboard**
```
Ctrl+F5
```

### 4. Control Execution

| Action | Key |
|--------|-----|
| Continue | F5 |
| Step Over | F10 |
| Step Into | F11 |
| Step Out | Shift+F11 |
| Pause | Ctrl+Alt+P |
| Stop | Ctrl+F2 |

## Commands

### Available Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| Debug File | Ctrl+F5 | Start debugging current file |
| Debug with Arguments | — | Debug with command-line arguments |
| Select Runtime | — | Choose Lua version to use |
| List Runtimes | — | Show installed Lua versions |
| Load Project Config | — | Reload wayfinder.yaml |

### Right-Click Context Menu

Right-click a `.lua` or `.luax` file:

- **Debug 'filename.lua'** - Start debug session

## Configuration

### IDE Settings

Go to **Edit → Preferences → Wayfinder**:

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Wayfinder Path | String | `wayfinder` | Path to Wayfinder binary |
| Default Port | Number | `5858` | DAP server port |
| Default Runtime | Choice | `lua54` | Default Lua version |
| Auto-Detect Runtime | Boolean | `true` | Auto-detect from file extension |
| Source Map Behavior | Choice | `ask` | How to handle source maps |

### Project Configuration

Create `wayfinder.yaml` in project root:

```yaml
# Lua runtime to use
runtime: lua54

# DAP server port
port: 5858

# How to handle source maps
sourceMapBehavior: ask

# Environment variables
env:
  LUA_PATH: "./?.lua"
  DEBUG: "true"
```

Or use `wayfinder.toml`:

```toml
runtime = "lua54"
port = 5858
source_map_behavior = "ask"

[env]
LUA_PATH = "./?.lua"
```

### Environment Variables

Override settings via environment:

```bash
export WAYFINDER_PATH="/path/to/wayfinder"
export WAYFINDER_PORT="6000"
```

## Debugging Workflow

### Setting Breakpoints

1. Click on line number in editor
2. Red dot appears on the line
3. Right-click for conditional breakpoints

### Variable Inspection

During debug session:

- **Hover variables** - See values in tooltip
- **Watch panel** - Add custom expressions
- **Output panel** - Evaluate expressions
- **Stack panel** - View call stack and locals

### Step Debugging

- **Step Over (F10)** - Execute line, skip function calls
- **Step Into (F11)** - Descend into function
- **Step Out (Shift+F11)** - Exit current function
- **Continue (F5)** - Run to next breakpoint

### Expression Evaluation

In the Debug Console:

```lua
print(myvar)
return {a=1, b=2}
table.insert(list, "item")
```

## Debugging Different Lua Versions

### Auto-Detection

File extension determines runtime:

```bash
# Lua 5.4 (default for .lua)
nvim script.lua

# LuaNext (for .luax)
nvim script.luax
```

### Manual Selection

```
Tools → Select Runtime
```

Then choose desired version.

### Project Default

Set in `wayfinder.yaml`:

```yaml
runtime: lua53
```

## Advanced Features

### Debugging with Arguments

```
Tools → Debug with Arguments
```

Then enter arguments:

```
arg1 arg2 arg3
```

### Remote Debugging

Not yet supported in ZeroBrane version. Coming soon.

### Hot Code Reload

Not yet supported. Planned for Phase 5.

## Troubleshooting

### "Wayfinder not found"

**Solution:** Configure path in preferences or set environment variable:

```bash
export WAYFINDER_PATH="/path/to/wayfinder"
```

### "Lua 5.4 not found"

**Solution:** Install required version:

```bash
# macOS
brew install lua@5.4

# Ubuntu/Debian
sudo apt-get install liblua5.4-dev

# Windows (Chocolatey)
choco install lua54
```

### Breakpoints not triggering

1. Run **Tools → List Runtimes** to verify installation
2. Check script path matches source file
3. Ensure correct Lua version selected

### Debug session fails

**Causes:**
- Script syntax error
- Port already in use
- Wayfinder crashed

**Solutions:**
1. Test script manually: `lua test.lua`
2. Check ZeroBrane output panel for errors
3. Try different port in preferences
4. Verify Wayfinder works: `wayfinder --version`

## File Structure

```
editors/zerobrane/
├── wayfinder.lua          # Core Wayfinder module
├── plugin.lua             # ZeroBrane plugin integration
├── README.md              # This file
├── .gitignore             # Git exclusions
└── examples/
    ├── test.lua           # Basic example
    ├── args_example.lua   # Arguments example
    ├── luanext_example.luax # LuaNext example
    └── wayfinder.yaml     # Example config
```

## Performance Tips

1. **Minimize breakpoints** - Each has small performance cost
2. **Use conditional breakpoints** - Better than manual stepping
3. **Avoid inspecting large tables** - Can slow execution
4. **Keep Wayfinder updated** - New versions may be faster
5. **Watch carefully** - Don't watch huge data structures

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Debug File | Ctrl+F5 |
| Continue | F5 |
| Step Over | F10 |
| Step Into | F11 |
| Step Out | Shift+F11 |
| Pause | Ctrl+Alt+P |
| Stop | Ctrl+F2 |

## Contributing

Contributions welcome! To contribute:

1. Fork the repository
2. Create a feature branch
3. Follow Lua style conventions
4. Test with multiple Lua versions
5. Submit a pull request

## Known Limitations

- **REPL**: Limited expression evaluation
- **Remote Debugging**: Not yet supported
- **Source Maps**: Full support coming (Phase 4)
- **Hot Reload**: Not yet supported (Phase 5)
- **Coroutines**: Limited support

## Support

- **Issues**: [GitHub Issues](https://github.com/forge18/wayfinder/issues)
- **Discussions**: [GitHub Discussions](https://github.com/forge18/wayfinder/discussions)
- **Documentation**: [Wayfinder Docs](https://forge18.github.io/wayfinder/)
- **ZeroBrane Docs**: [ZeroBrane Studio](https://studio.zerobrane.com/)

## License

MIT - See [LICENSE](../../LICENSE) in root directory

## Changelog

### 1.0.0

- Initial release
- Multi-version Lua support (5.1-5.4, LuaNext)
- Auto-detection of Lua runtime
- Project configuration support
- Basic breakpoint support

---

**Happy debugging!** 🐛✨

**Questions?** See [Troubleshooting](#troubleshooting) or open an issue on GitHub.
