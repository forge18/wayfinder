# Wayfinder Neovim Plugin - Usage Guide

This guide covers how to use the Wayfinder debug plugin in Neovim.

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

### 2. Open in Neovim

```bash
nvim test.lua
```

### 3. Set a Breakpoint

Position cursor on line 6 (`return name`) and press `<F9>` to set a breakpoint.

You should see a red dot in the sign column.

### 4. Start Debugging

```vim
:WayfinderDebugFile
```

Or press `<C-F5>`.

### 5. Control Execution

- Press `<F5>` to continue
- Press `<F10>` to step over
- Press `<F11>` to step into
- Press `<S-F11>` to step out

### 6. Inspect Variables

Open nvim-dap-ui with `:DapUIToggle` to see variables, call stack, and watches.

**That's it! You're debugging Lua.** 🎉

## Commands

### Core Commands

#### `:WayfinderDebugFile [args...]`

Debug the current file with optional arguments.

**Examples:**

```vim
" Debug current file
:WayfinderDebugFile

" Debug with arguments
:WayfinderDebugFile arg1 arg2 arg3

" Shortcut
:WDF arg1 arg2
```

#### `:WayfinderSelectRuntime`

Choose which Lua version to use.

Opens Telescope picker if available, otherwise uses fallback UI.

**Example:**

```vim
:WayfinderSelectRuntime

" Select 'lua53' from the picker
```

#### `:WayfinderAttachProcess [port] [host]`

Attach to an already-running Lua debug session.

**Examples:**

```vim
" Attach to default port (5858)
:WayfinderAttachProcess

" Attach to custom port
:WayfinderAttachProcess 6000

" Attach to remote host
:WayfinderAttachProcess 5858 192.168.1.100

" Shortcut
:WAP 5858 localhost
```

#### `:WayfinderRuntimes`

List available Lua runtimes and their status.

**Example:**

```vim
:WayfinderRuntimes

" Output:
" Available Lua Runtimes:
" ========================================
"   lua51: ✗ not found (lua5.1)
"   lua52: ✓ available (lua5.2)
"   lua53: ✓ available (lua5.3)
"   lua54: ✓ available (lua5.4)
"   luanext: ✓ available (luanext)
" =========================================
```

### Key Mappings

Press these keys to control debugging:

| Key | Action |
|-----|--------|
| `<C-F5>` | Debug current file |
| `<C-S-R>` | Select runtime |
| `<C-S-A>` | Attach to process |
| `<F5>` | Continue/Resume |
| `<F10>` | Step over |
| `<F11>` | Step into |
| `<S-F11>` | Step out |
| `<F9>` | Toggle breakpoint |
| `<C-F9>` | Set conditional breakpoint |

## Debugging Workflow

### Basic Debugging

1. **Set breakpoint** - Press `<F9>` on the line
2. **Start debug** - Press `<C-F5>`
3. **Step through** - Use `<F10>` and `<F11>`
4. **Continue** - Press `<F5>` to run to next breakpoint
5. **Stop** - Press `<F5>` then choose "terminate"

### Inspecting Variables

#### Option 1: Hover (if hovering works)

```lua
local x = 42
local y = x + 10  -- Hover over 'x' to see its value
```

#### Option 2: Use nvim-dap-ui

```vim
:DapUIToggle
```

Opens a window showing:
- Variables (locals, globals, upvalues)
- Call stack
- Watches and breakpoints
- Floating preview on hover

#### Option 3: Evaluate Expression

```vim
:DapEval print(x)
" Shows result in floating window
```

### Setting Conditional Breakpoints

1. Press `<C-F9>` on the line
2. Enter condition: `x > 10`
3. Breakpoint triggers only when condition is true

**Example:**

```lua
for i = 1, 100 do
  print(i)  -- Press <C-F9>, enter: i == 50
end
```

Execution pauses only when `i == 50`.

### Debugging with Arguments

Pass arguments to your script:

```vim
:WayfinderDebugFile arg1 arg2 arg3
```

Script receives `arg` table: `arg[1] = "arg1"`, `arg[2] = "arg2"`, etc.

**Example script (`args_test.lua`):**

```lua
print("Arguments:")
for i, arg in ipairs(arg) do
  print(string.format("  arg[%d] = %s", i, arg))
end
```

Debug with:

```vim
:WayfinderDebugFile hello world lua
```

### Debugging Different Lua Versions

#### Auto-Detection

Wayfinder auto-detects based on file extension:

```bash
# .lua → Lua 5.4 (default)
nvim my_script.lua

# .luax → LuaNext
nvim my_script.luax
```

#### Manual Selection

```vim
:WayfinderSelectRuntime
" Choose desired version from picker
:WayfinderDebugFile
```

### Debugging LuaNext Files

LuaNext files (`.luax`) automatically use the LuaNext runtime:

```lua
-- main.luax (LuaNext syntax)
local name: string = "Lua"
local count: integer = 42

print(name, count)
```

Debug normally:

```vim
:WayfinderDebugFile
```

### Multi-File Debugging

Debug across multiple files by:

1. Setting breakpoints in any open file
2. Stepping into functions in other files
3. The debugger handles source lookup automatically

**Example project structure:**

```
project/
├── main.lua
├── utils.lua
└── config.lua
```

1. Open `main.lua` in Neovim
2. Set breakpoint in `utils.lua`
3. Debug `main.lua`
4. Execution pauses at breakpoint in `utils.lua`

## Advanced Usage

### Custom Debug Configurations

Create custom configurations in `~/.config/nvim/init.lua`:

```lua
local dap = require("dap")

-- Add custom configuration for test runner
table.insert(dap.configurations.lua, {
  type = "wayfinder",
  request = "launch",
  name = "Run Tests",
  program = "${workspaceFolder}/test/runner.lua",
  cwd = "${workspaceFolder}",
  stopOnEntry = false,
})

-- Then use with:
-- :DapContinue (or <F5>) and select "Run Tests"
```

### Debugging with Environment Variables

Pass environment variables to your script:

```bash
# Set environment variable
export DEBUG=1

# Then debug in Neovim
:WayfinderDebugFile
```

Or set in Lua:

```lua
vim.env.DEBUG = "1"
```

### Remote Debugging

If Wayfinder is running on another machine:

```vim
:WayfinderAttachProcess 5858 192.168.1.100
```

Ensures Wayfinder DAP server is accessible on the remote address.

### Debugging with External Console

Output to external terminal while debugging:

1. Start Wayfinder with output redirection
2. Attach to debug server from Neovim

**Example:**

```bash
# Terminal 1: Start script with debug server
wayfinder dap-server --port 5858 --runtime lua54 --script main.lua

# Terminal 2: Neovim with attach
nvim main.lua
:WayfinderAttachProcess 5858
```

## Telescope Integration

### Runtime Selection

Using Telescope for better UI:

```vim
:WayfinderSelectRuntime
```

Opens Telescope picker with:
- Runtime names (lua51, lua52, lua53, lua54, luanext)
- Availability status (✓ or ✗)
- Binary paths
- Fuzzy search support

### Configure Runtime

Using Telescope to edit configuration:

```lua
require("wayfinder.telescope").runtime_config()
```

Select a setting to edit:
- Runtime paths
- Wayfinder binary
- DAP port

### Debug File Picker

Open file picker for debugging:

```lua
require("wayfinder.telescope").debug_file_picker()
```

Searches current directory for `.lua` and `.luax` files.

## Troubleshooting

### "module 'wayfinder' not found"

**Cause**: Plugin not installed or loaded.

**Fix**:
1. Check plugin directory: `ls ~/.config/nvim/pack/*/start/`
2. Restart Neovim: `:q!` then reopen
3. Reinstall: Follow [INSTALLATION.md](./INSTALLATION.md)

### "nvim-dap not found"

**Cause**: nvim-dap not installed.

**Fix**: Install nvim-dap using your plugin manager.

```lua
-- Packer
use "mfussenegger/nvim-dap"
:PackerSync

-- Lazy (auto-installs)
{ "mfussenegger/nvim-dap" }
```

### Breakpoints not working

**Cause**: Runtime not found or misconfigured.

**Fix**:
1. Check: `:WayfinderRuntimes`
2. Ensure binary is in PATH: `:!which lua5.4`
3. Configure custom path:
   ```lua
   require("wayfinder").setup({
     runtime_paths = {
       lua54 = "/path/to/lua5.4"
     }
   })
   ```

### "Wayfinder exited with code 1"

**Cause**: Script execution error.

**Fix**:
1. Check script syntax: `:!lua main.lua`
2. Review Wayfinder output: Check Neovim log
3. Try attaching instead of launching

### Debugging hangs/freezes

**Cause**: Breakpoint in non-existent code or network timeout.

**Fix**:
1. Press `<C-c>` to interrupt
2. Check script paths are correct
3. Verify Wayfinder is running: `:!ps aux | grep wayfinder`

### Variables not showing

**Cause**: nvim-dap-ui not installed or DAP configuration issue.

**Fix**:
1. Install nvim-dap-ui:
   ```lua
   use "rcarriga/nvim-dap-ui"
   ```
2. Toggle UI: `:DapUIToggle`
3. Check scopes: `:DapUIScopes`

## Performance Tips

1. **Reduce breakpoints** - Fewer breakpoints = faster execution
2. **Use conditional breakpoints** - Better than stepping manually
3. **Limit variable inspection** - Inspecting large tables is slow
4. **Use watches carefully** - Don't watch huge data structures
5. **Keep Wayfinder updated** - New versions may be faster

## Tips & Tricks

### Favorite Configurations

Save commonly used debug commands:

```vim
" In ~/.config/nvim/init.lua
vim.keymap.set('n', '<Leader>dt', ':WayfinderDebugFile<CR>', { noremap = true })
vim.keymap.set('n', '<Leader>dr', ':WayfinderSelectRuntime<CR>', { noremap = true })
vim.keymap.set('n', '<Leader>da', ':WayfinderAttachProcess<CR>', { noremap = true })
```

### Persistent Breakpoints

Breakpoints persist during debug session but not between sessions. To save:

Create a breakpoint save/load script:

```lua
-- ~/.config/nvim/init.lua
local function save_breakpoints()
  local dap = require("dap")
  local breakpoints = dap.list_breakpoints()
  -- Save to file
end
```

### Keyboard Shortcuts Cheat Sheet

Print in a floating window:

```lua
vim.keymap.set('n', '<Leader>?', function()
  vim.notify("F5:Continue F10:Step F11:Into S-F11:Out F9:Toggle C-F9:Condition", vim.log.levels.INFO)
end)
```

## Next Steps

- Read [README.md](./README.md) for full feature list
- Check [INSTALLATION.md](./INSTALLATION.md) if you have setup issues
- Explore nvim-dap docs: https://github.com/mfussenegger/nvim-dap
- Try examples in your own projects

---

**Happy debugging!** 🐛✨

Need help? See the [README.md](./README.md#troubleshooting) troubleshooting section.
