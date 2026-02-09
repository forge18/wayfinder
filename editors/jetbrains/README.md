# Wayfinder - Lua Debugger for JetBrains IDEs

A comprehensive Debug Adapter Protocol (DAP) debugger plugin for JetBrains IDEs. Debug Lua and LuaNext scripts directly from IntelliJ IDEA, WebStorm, PyCharm, RubyMine, GoLand, Rider, CLion, and PhpStorm.

## Features

- 🐛 **Multi-Version Lua Support**: Debug Lua 5.1, 5.2, 5.3, 5.4, and LuaNext
- 🎯 **Auto-Detection**: Automatically detects the appropriate Lua runtime from file extension
- 📍 **Breakpoints**: Set line and conditional breakpoints directly in the editor
- 🔍 **Variable Inspection**: Inspect variables, locals, and evaluate expressions
- 🔄 **Step Control**: Step over, into, and out of functions with full stack support
- 📦 **Source Maps**: Full support for LuaNext files with automatic source map translation
- 🎨 **Run Configurations**: Full IDE integration with run configurations and context menus
- ⚡ **Project Configuration**: Load settings from wayfinder.yaml or wayfinder.toml

## Supported IDEs

| IDE | Minimum Version | Status |
|-----|-----------------|--------|
| IntelliJ IDEA | 2023.2 | ✅ |
| WebStorm | 2023.2 | ✅ |
| PyCharm | 2023.2 | ✅ |
| RubyMine | 2023.2 | ✅ |
| GoLand | 2023.2 | ✅ |
| CLion | 2023.2 | ✅ |
| Rider | 2023.2 | ✅ |
| PhpStorm | 2023.2 | ✅ |

## Installation

### From Plugin Marketplace

1. Open your JetBrains IDE
2. Go to **File → Settings → Plugins** (or **Preferences → Plugins** on macOS)
3. Search for "Wayfinder Lua Debugger"
4. Click **Install**
5. Restart your IDE

### Manual Installation

1. Download the latest plugin JAR from [releases](https://github.com/forge18/wayfinder/releases)
2. Open **File → Settings → Plugins**
3. Click ⚙️ → **Install Plugin from Disk**
4. Select the downloaded JAR file
5. Restart your IDE

### From Source

```bash
cd /path/to/wayfinder/editors/jetbrains
gradle build
# Plugin JAR is in build/distributions/
```

## Quick Start

### 1. Create a Test Lua File

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

- Click on the line number margin (left side) next to a line
- A red dot appears, indicating the breakpoint is set

### 3. Start Debugging

**Option A: Right-click the file**
```
Right-click → Debug 'test.lua'
```

**Option B: Use the menu**
```
Run → Debug → Debug File
```

**Option C: Keyboard shortcut**
```
Ctrl+F5 (Windows/Linux) or Cmd+F5 (macOS)
```

### 4. Control Execution

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Continue | F5 | Cmd+F5 |
| Step Over | F10 | F10 |
| Step Into | F11 | F11 |
| Step Out | Shift+F11 | Shift+F11 |

## Configuration

### IDE Settings

Go to **File → Settings → Languages & Frameworks → Lua (Wayfinder)**:

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Wayfinder Path | String | `wayfinder` | Path to Wayfinder binary |
| Default Port | Number | `5858` | DAP server port (auto-increments) |
| Default Runtime | Dropdown | `lua54` | Default Lua version |
| Auto-Detect Runtime | Checkbox | ✓ | Auto-detect from file extension |
| Source Map Behavior | Dropdown | `ask` | How to handle source maps |

### Project Configuration

Create a `wayfinder.yaml` file in your project root:

```yaml
# Lua runtime to use
runtime: lua54

# DAP server port
port: 5858

# How to handle source maps for bundled files
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

Override settings via environment variables:

```bash
export WAYFINDER_PATH="/path/to/wayfinder"
export WAYFINDER_PORT="6000"
```

## Run Configurations

### Creating a Run Configuration

1. Click **Edit Configurations** in the toolbar
2. Click **+** to add a new configuration
3. Select **Lua (Wayfinder)**
4. Configure:
   - **Script**: Path to Lua script to debug
   - **Runtime**: Lua version to use
   - **Working Directory**: Directory for script execution
   - **Arguments**: Command-line arguments for the script

### Example Configurations

**Debug with Arguments:**
```
Script: src/main.lua
Arguments: --config config.lua --verbose
Runtime: lua54
```

**Debug LuaNext:**
```
Script: src/main.luax
Runtime: luanext
```

**Remote Debugging:**
```
Script: script.lua
Runtime: lua54
(set Port in IDE settings to match remote)
```

## Commands & Actions

### Menu Items

| Action | Menu | Shortcut |
|--------|------|----------|
| Debug File | Run → Debug File | Ctrl+F5 |
| Select Runtime | Run → Select Lua Runtime | — |
| List Runtimes | Run → List Available Runtimes | — |

### Context Menu

Right-click a `.lua` or `.luax` file:

- **Debug 'filename.lua'** - Start debug session
- **Run 'filename.lua'** - Run without debugging (if enabled)

## Debugging Workflow

### Setting Breakpoints

1. Click the line number margin to set a breakpoint
2. A red circle appears on the line
3. For conditional breakpoints, right-click the red circle and select **Edit Breakpoint**

### Variable Inspection

During a debug session:

- **Hover over variables** - See their values in a popup
- **Use the Variables panel** - View all local and global variables
- **Use the Watch window** - Add custom expressions to watch
- **Use the Console** - Evaluate expressions in the current frame

### Step Debugging

- **Step Over (F10)** - Execute next line, don't descend into functions
- **Step Into (F11)** - Descend into the next function call
- **Step Out (Shift+F11)** - Execute until the current function returns

### Call Stack

The Call Stack panel shows:
- All active function calls
- Current frame (highlighted)
- Ability to click frames to inspect that scope

## Troubleshooting

### "Wayfinder binary not found"

**Solution:** Configure the path in **Settings → Languages & Frameworks → Lua**

```
Wayfinder Path: /path/to/wayfinder
```

Or set environment variable:

```bash
export WAYFINDER_PATH="/path/to/wayfinder"
```

### "Lua 5.4 not found"

**Solution:** Install the required Lua version:

```bash
# macOS
brew install lua@5.4

# Ubuntu/Debian
sudo apt-get install liblua5.4-dev

# Windows (Chocolatey)
choco install lua54
```

Or configure custom path:

```yaml
# wayfinder.yaml
runtime: lua54
# Set in environment or IDE settings
```

### Breakpoints not triggering

**Common Causes:**
- Script path doesn't match source file path
- Wrong Lua version selected
- Runtime not installed

**Solutions:**
1. Run **Run → List Available Runtimes** to verify installations
2. Check the script path in Run Configuration
3. Ensure the Lua version matches your script

### "Debug session failed"

**Causes:**
- Wayfinder crashed (check script syntax)
- Port already in use
- Insufficient permissions

**Solutions:**
1. Try running the script directly: `lua test.lua`
2. Check error messages in IDE event log
3. Verify Wayfinder binary works: `wayfinder --version`
4. Try a different port in settings

## Advanced Features

### Hot Code Reload

Not yet supported in this IDE version. Coming in a future release.

### Remote Debugging

For remote Lua processes:

1. Start Wayfinder on remote machine:
   ```bash
   wayfinder dap --port 5858
   ```

2. In IDE, go to **Run → Attach to Process**
3. Enter remote host and port

### Custom Expressions

In the Debug Console, evaluate custom expressions:

```lua
print(myvar)
return {a=1, b=2}
```

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Debug File | Ctrl+F5 |
| Debug Last Configuration | Ctrl+F5 (if configured) |
| Run to Cursor | Ctrl+G |
| Evaluate Expression | Alt+F9 |
| Step Into | F11 |
| Step Over | F10 |
| Step Out | Shift+F11 |
| Resume | F9 |
| Pause | Ctrl+Alt+P |
| Stop | Ctrl+F2 |

## Performance Tips

1. **Minimize breakpoints** - Each breakpoint has a small performance cost
2. **Use conditional breakpoints** - Better than stepping manually
3. **Avoid inspecting large tables** - Can slow down execution
4. **Keep Wayfinder updated** - New versions may be faster
5. **Use watches carefully** - Don't watch huge data structures

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Follow Kotlin style guide (see .idea/codeStyles/)
4. Test with multiple IDE versions
5. Submit a pull request

## Known Limitations

- **REPL**: Limited expression evaluation support
- **Remote Debugging**: Requires manual host/port configuration
- **Source Maps**: Full support for LuaNext planned in Phase 4
- **Hot Reload**: Not yet supported (Phase 5)

## Support

- **Bugs**: [GitHub Issues](https://github.com/forge18/wayfinder/issues)
- **Discussions**: [GitHub Discussions](https://github.com/forge18/wayfinder/discussions)
- **Documentation**: [Wayfinder Docs](https://forge18.github.io/wayfinder/)

## License

MIT - See [LICENSE](../../LICENSE) in the root directory

---

**Happy debugging!** 🐛✨
