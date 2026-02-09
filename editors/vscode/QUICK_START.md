# Wayfinder VSCode Extension - Quick Start

## What is This?

The Wayfinder VSCode Debug Extension is a full Debug Adapter Protocol (DAP) implementation that enables seamless debugging of Lua and LuaNext scripts directly within VSCode.

## What Works Now

✅ Debug Lua scripts (versions 5.1, 5.2, 5.3, 5.4)
✅ Debug LuaNext scripts (.luax files)
✅ Set breakpoints and inspect variables
✅ Step through code (F10, F11, Shift+F11)
✅ View call stack and locals
✅ Attach to running Lua programs
✅ Auto-detect appropriate Lua runtime
✅ Configure via VSCode settings

## Installation for Development

```bash
# 1. Install dependencies
cd editors/vscode
npm install

# 2. Build the extension
npm run esbuild

# 3. Launch in VSCode
code --extensionDevelopmentPath=. ../..
# OR press F5 if .vscode/launch.json is configured
```

## First Debug Session

### Using F5 (Recommended)

1. Open any `.lua` or `.luax` file
2. Click on a line number to set a breakpoint
3. Press **F5** to start debugging
4. Script will pause at breakpoint
5. Use Debug toolbar to continue, step, etc.

### Using Right-Click Menu

1. Open a `.lua` or `.luax` file
2. Right-click in the editor
3. Select **"Debug File"**
4. Debugging starts immediately

### Using Launch Configuration

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "wayfinder",
      "request": "launch",
      "name": "Launch Script",
      "program": "${workspaceFolder}/main.lua",
      "stopOnEntry": false
    }
  ]
}
```

Then press F5 to debug.

## Example Lua Script

Create `test.lua`:

```lua
local function greet(name)
  print("Hello, " .. name .. "!")
  return name
end

local result = greet("Lua")
print("Result:", result)
```

Open in VSCode and press F5 to debug!

## Configuration

### Quick Settings

Open VSCode Settings (Ctrl+, or Cmd+,) and search "wayfinder":

- **Debug Port**: Default 5858 (auto-increments for multiple sessions)
- **Auto-detect Runtime**: Enabled by default
- **Lua Binary Paths**: Auto-detected, override if needed

### Custom Lua Path

If your Lua binary is not in the default location:

```json
{
  "wayfinder.runtime.lua54.path": "/usr/local/bin/lua5.4"
}
```

## Debugging LuaNext

For `.luax` files, the extension automatically uses the LuaNext runtime:

```lua
-- main.luax (LuaNext syntax)
local name: string = "Lua"
print(name)
```

Open and press F5 to debug!

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| F5 | Continue or start debugging |
| F10 | Step over |
| F11 | Step into |
| Shift+F11 | Step out |
| Ctrl+Shift+D | Open Debug panel |
| Ctrl+K, Ctrl+I | Toggle breakpoint |

## Commands

Press Ctrl+Shift+P (or Cmd+Shift+P on Mac) for command palette:

- **Debug File** - Debug current file
- **Select Runtime** - Choose Lua version
- **Attach to Process** - Connect to running program

## Common Issues

### "wayfinder: command not found"

Ensure Wayfinder is installed:
```bash
wayfinder --version
```

Or configure the path in settings:
```json
{
  "wayfinder.wayfinder.path": "/path/to/wayfinder"
}
```

### "lua: command not found"

Ensure Lua is installed:
```bash
lua5.4 -v
# or
lua -v
```

Or specify the path:
```json
{
  "wayfinder.runtime.lua54.path": "/path/to/lua5.4"
}
```

### Script runs but doesn't pause

1. Make sure breakpoints are set (red dot on line number)
2. Try setting `"stopOnEntry": true` in launch config
3. Check Debug Console for errors

## Next Steps

### For Users
- Read [README.md](./README.md) for complete documentation
- See [TESTING.md](./TESTING.md) for more examples

### For Developers
- See [DEVELOPMENT.md](./DEVELOPMENT.md) for architecture and internals
- Check [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md) for status

## Getting Help

1. Check the [README.md](./README.md) troubleshooting section
2. Review [DEVELOPMENT.md](./DEVELOPMENT.md) for known issues
3. Check VSCode Output panel (View > Output > Wayfinder)
4. Review [TESTING.md](./TESTING.md) for test cases

## Advanced Topics

### Attach to Running Program

```json
{
  "type": "wayfinder",
  "request": "attach",
  "port": 5858,
  "host": "localhost"
}
```

### Debug with Arguments

```json
{
  "type": "wayfinder",
  "request": "launch",
  "program": "${workspaceFolder}/main.lua",
  "args": ["arg1", "arg2", "arg3"]
}
```

### Debug External Console

```json
{
  "type": "wayfinder",
  "request": "launch",
  "program": "${workspaceFolder}/main.lua",
  "console": "externalTerminal"
}
```

## What's Not Yet Implemented

These features are planned for future phases:

- [ ] Debug CodeLens (clickable "Debug" above test blocks)
- [ ] Source map support for bundled LuaNext files
- [ ] Neovim plugin
- [ ] Advanced breakpoint types

## Features Overview

| Feature | Status | Notes |
|---------|--------|-------|
| Multi-version Lua | ✅ | 5.1, 5.2, 5.3, 5.4, LuaNext |
| Breakpoints | ✅ | Set/remove/conditional |
| Step Control | ✅ | Over/into/out, continue, restart |
| Variables | ✅ | Inspect locals, globals, tables |
| Call Stack | ✅ | Navigate and jump |
| Launch | ✅ | F5, context menu, config |
| Attach | ✅ | Connect to running process |
| Auto-detect Runtime | ✅ | From file type and config |
| VSCode Settings | ✅ | Full configuration support |
| Debug Console | ✅ | Evaluate Lua expressions |

## File Structure

```
editors/vscode/
├── src/                          # Source code
│   ├── extension.ts              # Entry point
│   ├── configuration.ts          # Config & detection
│   ├── debug-provider.ts         # VSCode provider
│   ├── adapter.ts                # DAP factory
│   ├── runtime-manager.ts        # Session management
│   └── commands.ts               # Command handlers
├── out/                          # Compiled output
├── package.json                  # Manifest
├── README.md                      # Full documentation
├── DEVELOPMENT.md                # Dev guide
├── TESTING.md                    # Test plan
└── examples/                     # Example configs
```

## Contributing

To improve the extension:

1. Read [DEVELOPMENT.md](./DEVELOPMENT.md)
2. Make changes to `src/`
3. Run `npm run esbuild` to build
4. Test with `npm run esbuild-watch`
5. Follow the test plan in [TESTING.md](./TESTING.md)

## License

MIT - See LICENSE in root wayfinder directory

---

**Ready to debug?** Press F5 and happy debugging! 🐛🎉
