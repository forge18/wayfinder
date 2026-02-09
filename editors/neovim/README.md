# Wayfinder - Lua Debugger for Neovim

A powerful Debug Adapter Protocol (DAP) plugin for debugging Lua and LuaNext scripts in Neovim. Wayfinder supports multiple Lua versions (5.1, 5.2, 5.3, 5.4) and LuaNext with full source map support.

## Features

- 🐛 **Multi-Version Lua Support**: Debug Lua 5.1, 5.2, 5.3, 5.4, and LuaNext
- 🎯 **Auto-Detection**: Automatically detects the appropriate Lua runtime
- 📍 **Breakpoints**: Set conditional and unconditional breakpoints
- 🔍 **Variable Inspection**: Inspect variables and evaluate expressions
- 🔄 **Step Control**: Step over, into, and out of functions
- 📦 **Source Maps**: Full support for bundled LuaNext files with source maps
- 🎨 **Call Stack**: Navigate through call stacks during debugging
- 🔭 **Telescope Integration**: Use Telescope for runtime selection (optional)
- ⚡ **Flexible Configuration**: Fully configurable via Lua or environment variables

## Requirements

- Neovim 0.7+
- [nvim-dap](https://github.com/mfussenegger/nvim-dap) - Required
- [Telescope](https://github.com/nvim-telescope/telescope.nvim) - Optional, for enhanced UI
- Wayfinder CLI - [Installation Guide](../../README.md)

## Installation

### Using a Plugin Manager

**Packer.nvim:**
```lua
use {
  "forge18/wayfinder",
  branch = "main",
  rtp = "editors/neovim",
  requires = {
    "mfussenegger/nvim-dap",
    -- optional:
    { "nvim-telescope/telescope.nvim", requires = { "nvim-lua/plenary.nvim" } },
  },
  config = function()
    require("wayfinder").setup({
      -- your config here
    })
  end,
}
```

**vim-plug:**
```vim
Plug "mfussenegger/nvim-dap"
Plug "nvim-telescope/telescope.nvim"
Plug "forge18/wayfinder", { "rtp": "editors/neovim" }

" Then in your init.vim:
lua require("wayfinder").setup()
```

**Lazy.nvim:**
```lua
{
  "forge18/wayfinder",
  branch = "main",
  dir = "editors/neovim",
  dependencies = {
    "mfussenegger/nvim-dap",
    { "nvim-telescope/telescope.nvim", dependencies = { "nvim-lua/plenary.nvim" } },
  },
  config = function()
    require("wayfinder").setup()
  end,
}
```

### Manual Installation

```bash
mkdir -p ~/.config/nvim/pack/wayfinder/start
cd ~/.config/nvim/pack/wayfinder/start
git clone https://github.com/forge18/wayfinder.git
# Copy the editors/neovim directory to the plugin directory
cp -r wayfinder/editors/neovim/* wayfinder/
```

## Quick Start

### 1. Debug a Lua File

```vim
:WayfinderDebugFile
```

Or with key mapping (Ctrl+F5):
```
<C-F5>
```

This will:
1. Auto-detect the appropriate Lua runtime
2. Set breakpoints in the current file
3. Start the debug session

### 2. Select Runtime Manually

```vim
:WayfinderSelectRuntime
```

Or with key mapping (Ctrl+Shift+R):
```
<C-S-R>
```

This opens a Telescope picker or fallback UI to select the Lua version.

### 3. Attach to Running Process

```vim
:WayfinderAttachProcess 5858 localhost
```

Or with key mapping (Ctrl+Shift+A):
```
<C-S-A>
```

### 4. List Available Runtimes

```vim
:WayfinderRuntimes
```

Shows which Lua versions are installed and available.

## Configuration

### Default Setup

```lua
require("wayfinder").setup()
```

### Custom Configuration

```lua
require("wayfinder").setup({
  wayfinder_path = "wayfinder",  -- Path to Wayfinder binary
  default_port = 5858,            -- DAP port
  auto_detect_runtime = true,     -- Auto-detect Lua version
  source_map_behavior = "ask",    -- "ask", "lenient", "strict"
  runtime_paths = {
    lua51 = "lua5.1",
    lua52 = "lua5.2",
    lua53 = "lua5.3",
    lua54 = "lua5.4",
    luanext = "luanext",
  },
})
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `wayfinder_path` | string | "wayfinder" | Path to Wayfinder CLI binary |
| `default_port` | number | 5858 | Default DAP port (auto-increments) |
| `auto_detect_runtime` | boolean | true | Auto-detect Lua version |
| `source_map_behavior` | string | "ask" | How to handle source maps |
| `runtime_paths.lua51` | string | "lua5.1" | Path to Lua 5.1 binary |
| `runtime_paths.lua52` | string | "lua5.2" | Path to Lua 5.2 binary |
| `runtime_paths.lua53` | string | "lua5.3" | Path to Lua 5.3 binary |
| `runtime_paths.lua54` | string | "lua5.4" | Path to Lua 5.4 binary |
| `runtime_paths.luanext` | string | "luanext" | Path to LuaNext binary |

### Environment Variables

Override configuration via environment variables:

```bash
export WAYFINDER_PATH="/path/to/wayfinder"
export WAYFINDER_PORT="6000"
```

### Via vim.g

```lua
vim.g.wayfinder_config = {
  default_port = 6000,
  runtime_paths = {
    lua54 = "/usr/local/bin/lua5.4",
  },
}
require("wayfinder").setup()
```

### Disable Features

```lua
-- Disable auto key mappings
vim.g.wayfinder_use_keymaps = false

-- Disable auto setup
vim.g.wayfinder_disable_auto_setup = true

-- Suppress startup message
vim.g.wayfinder_quiet = true

-- Disable command abbreviations
vim.g.wayfinder_command_abbrev = false
```

## Commands

### Main Commands

| Command | Arguments | Description |
|---------|-----------|-------------|
| `:WayfinderDebugFile` | `[args...]` | Debug current file with optional args |
| `:WayfinderSelectRuntime` | none | Select Lua runtime (Telescope or UI) |
| `:WayfinderAttachProcess` | `[port] [host]` | Attach to running process |
| `:WayfinderRuntimes` | none | List available Lua runtimes |

### Abbreviations

```vim
" These are enabled by default (disable with g:wayfinder_command_abbrev = 0)
:WDF     " WayfinderDebugFile
:WSR     " WayfinderSelectRuntime
:WAP     " WayfinderAttachProcess
:WRT     " WayfinderRuntimes
```

## Key Mappings

Default key mappings (disable with `g:wayfinder_use_keymaps = 0`):

| Mapping | Action |
|---------|--------|
| `<C-F5>` | Debug current file |
| `<C-S-R>` | Select runtime |
| `<C-S-A>` | Attach to process |
| `<F5>` | Continue/Resume |
| `<F10>` | Step over |
| `<F11>` | Step into |
| `<S-F11>` | Step out |
| `<F9>` | Toggle breakpoint |
| `<C-F9>` | Set conditional breakpoint |

The last 5 mappings are standard nvim-dap mappings.

## Supported Runtimes

| Runtime | Version | File Type | Status |
|---------|---------|-----------|--------|
| Lua | 5.1 | .lua | ✅ |
| Lua | 5.2 | .lua | ✅ |
| Lua | 5.3 | .lua | ✅ |
| Lua | 5.4 | .lua | ✅ (default) |
| LuaNext | Latest | .luax | ✅ |

## Debugging Workflow

### 1. Setting Breakpoints
- Press `<F9>` to toggle a breakpoint
- Press `<C-F9>` to set a conditional breakpoint
- Breakpoints are shown in the sign column (red markers)

### 2. Starting Debug Session
- `:WayfinderDebugFile` to debug current file
- Or `:DapContinue` to run previously configured session

### 3. Stepping Through Code
- `<F10>` - Step over
- `<F11>` - Step into function
- `<S-F11>` - Step out of function

### 4. Inspecting Variables
- Hover over variables to see values
- Use `:DapUIScopes` to open variable inspector
- Use `:DapEval` to evaluate expressions

### 5. Call Stack Navigation
- Use `:DapUIScopes` to see current call stack
- Jump to stack frames to inspect local scope

## Telescope Integration

If Telescope is installed, enhanced UI is available:

```lua
require("wayfinder.telescope").select_runtime()
require("wayfinder.telescope").runtime_config()
require("wayfinder.telescope").debug_file_picker()
```

### Telescope Commands

The telescope integration provides:
- Runtime selection picker
- Configuration editor picker
- File picker for debugging

## Runtime Detection

Wayfinder automatically detects the appropriate runtime:

1. **File extension check**
   - `.luax` → LuaNext
   - `.lua` → continues to step 2

2. **Workspace configuration**
   - Reads `wayfinder.yaml` for `runtime:` field

3. **Default**
   - Lua 5.4 for `.lua` files
   - LuaNext for `.luax` files

### Example wayfinder.yaml

```yaml
runtime: lua53
port: 5858
```

## Troubleshooting

### "Wayfinder requires nvim-dap plugin"

Install nvim-dap first:

```lua
use { "mfussenegger/nvim-dap" }
```

Then reload or restart Neovim.

### "wayfinder: command not found"

Ensure Wayfinder is installed:

```bash
wayfinder --version
```

Or configure custom path:

```lua
require("wayfinder").setup({
  wayfinder_path = "/path/to/wayfinder"
})
```

### Lua runtime not found

Check which version you have:

```bash
lua -v
lua5.4 -v
```

Configure the correct path:

```lua
require("wayfinder").setup({
  runtime_paths = {
    lua54 = "/usr/local/bin/lua5.4"
  }
})
```

### Breakpoints not working

1. Ensure nvim-dap is properly configured
2. Check that Wayfinder binary is in PATH
3. Verify script is executable
4. Try `:WayfinderRuntimes` to verify runtime availability

### Debug session won't start

1. Check `:WayfinderRuntimes` output
2. Verify Wayfinder binary is installed
3. Check Neovim log: `:Telescope notify`
4. Try manual attach instead: `:WayfinderAttachProcess`

## Advanced Usage

### Custom Debug Configuration

Create custom debug configurations in `init.lua`:

```lua
local dap = require("dap")
local wayfinder_dap = require("wayfinder.dap")

-- Add custom configuration
table.insert(dap.configurations.lua, {
  type = "wayfinder",
  request = "launch",
  name = "Debug with Custom Args",
  program = "${file}",
  args = { "arg1", "arg2" },
  stopOnEntry = true,
})
```

### Debugging with Multiple Files

Use workspace debugging with nvim-dap compound configurations:

```lua
dap.configurations.lua = {
  -- ... existing configs ...
  {
    type = "wayfinder",
    request = "launch",
    name = "Debug All Tests",
    program = "${workspaceFolder}/test/runner.lua",
  },
}
```

### Environment Variables

Pass environment variables to debugged script:

```vim
:WayfinderDebugFile DEBUG=1 PROFILE=dev
```

## Performance Considerations

- **First start**: May take 1-2 seconds as Wayfinder initializes
- **Breakpoint hit**: <100ms typically
- **Variable inspection**: <50ms per request
- **Memory**: ~20MB for Wayfinder process during debug

## Known Limitations

1. **Source maps** - Full support coming in Phase 4
2. **Remote debugging** - Currently local only
3. **REPL** - Limited Lua evaluation support
4. **Conditional breakpoints** - Basic support

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Follow existing code style (Lua conventions)
4. Test with multiple Lua versions
5. Submit a pull request

## License

MIT - See LICENSE in the root wayfinder directory

## Project Links

- [Wayfinder GitHub](https://github.com/forge18/wayfinder)
- [Wayfinder Main Documentation](../../README.md)
- [nvim-dap Documentation](https://github.com/mfussenegger/nvim-dap)
- [Telescope Documentation](https://github.com/nvim-telescope/telescope.nvim)

## Getting Help

- **Installation issues** → See [INSTALLATION.md](./INSTALLATION.md)
- **Usage questions** → See [USAGE.md](./USAGE.md)
- **Bug reports** → GitHub issues

---

**Happy debugging!** 🐛✨
