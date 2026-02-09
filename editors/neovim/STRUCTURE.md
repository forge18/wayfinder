# Wayfinder Neovim Plugin - File Structure

## Directory Layout

```
editors/neovim/
├── lua/wayfinder/
│   ├── init.lua              # Main plugin entry point
│   ├── config.lua            # Configuration management
│   ├── dap.lua               # DAP adapter configuration
│   ├── commands.lua          # User commands and keymaps
│   └── telescope.lua         # Telescope integration (optional)
├── plugin/
│   └── wayfinder.vim         # Vim plugin loader
├── README.md                 # Plugin documentation
├── INSTALLATION.md           # Installation guide
├── USAGE.md                  # Usage examples and guide
├── STRUCTURE.md              # This file
└── .gitignore                # Git exclusions
```

## File Descriptions

### `lua/wayfinder/init.lua` (Main Entry Point)

**Purpose**: Core plugin module that provides public API

**Responsibilities**:
- Plugin configuration storage
- Session management (tracking active debug sessions)
- Port allocation
- Module initialization
- Public API functions

**Key Functions**:
- `setup(user_config)` - Initialize plugin with user config
- `get_next_port()` - Allocate next available DAP port
- `get_runtime()` / `set_runtime()` - Manage current runtime
- `register_session()` / `unregister_session()` - Track sessions
- `has_dap()` / `has_telescope()` - Check dependencies

**Usage**:
```lua
local wayfinder = require("wayfinder")
wayfinder.setup({ ... })
```

### `lua/wayfinder/config.lua` (Configuration)

**Purpose**: Configuration loading and management

**Responsibilities**:
- Load config from multiple sources (user, env, yaml)
- Runtime detection logic
- Path resolution and verification
- Variable substitution

**Key Functions**:
- `setup(user_config)` - Initialize configuration
- `get(key, default)` - Get config value with dot notation
- `set(key, value)` - Set config value
- `detect_runtime(filepath)` - Auto-detect Lua version
- `verify_runtime(runtime)` - Check if runtime is available
- `get_wayfinder_path()` - Find Wayfinder binary
- `substitute_variables(str)` - Replace ${var} in strings

**Configuration Sources** (in order):
1. User config parameter to `setup()`
2. Environment variables (WAYFINDER_PATH, WAYFINDER_PORT)
3. vim.g.wayfinder_config
4. Default values

### `lua/wayfinder/dap.lua` (DAP Integration)

**Purpose**: nvim-dap adapter configuration and integration

**Responsibilities**:
- Register Wayfinder as nvim-dap adapter
- Create and run debug configurations
- Handle process spawning
- Manage debug server lifecycle

**Key Functions**:
- `setup(plugin_config)` - Register DAP adapter
- `debug_file(runtime, args)` - Debug current file
- `attach(port, host)` - Attach to running process

**DAP Adapter Registration**:
- Adapter type: `wayfinder`
- Supported requests: `launch`, `attach`
- Communication: stdio with DAP protocol

**Debug Configurations**:
- `Launch Lua Script` - Debug current file
- `Launch with Arguments` - Debug with args
- `Launch LuaNext` - Debug LuaNext file
- `Attach to Process` - Attach to running process

### `lua/wayfinder/commands.lua` (User Commands)

**Purpose**: User-facing commands and keymaps

**Responsibilities**:
- Register Neovim user commands
- Implement command handlers
- Setup keymaps
- Fallback UI for runtime selection

**Registered Commands**:
- `:WayfinderDebugFile` - Debug current file
- `:WayfinderSelectRuntime` - Choose Lua version
- `:WayfinderAttachProcess` - Attach to process
- `:WayfinderRuntimes` - List available runtimes

**Default Keymaps** (can be disabled):
- `<C-F5>` - Debug File
- `<C-S-R>` - Select Runtime
- `<C-S-A>` - Attach Process
- `<F5>` - Continue (nvim-dap)
- `<F10>` - Step Over (nvim-dap)
- `<F11>` - Step Into (nvim-dap)
- `<S-F11>` - Step Out (nvim-dap)
- `<F9>` - Toggle Breakpoint (nvim-dap)
- `<C-F9>` - Conditional Breakpoint (nvim-dap)

### `lua/wayfinder/telescope.lua` (Telescope Integration)

**Purpose**: Enhanced UI using Telescope (optional)

**Responsibilities**:
- Telescope-based runtime selection
- Configuration editor with Telescope
- File picker for debugging
- UI improvements over fallback

**Key Functions**:
- `select_runtime()` - Telescope runtime picker
- `runtime_config()` - Telescope config editor
- `debug_file_picker()` - Telescope file picker
- `edit_config_value(key)` - Edit single config value

**Usage**:
```lua
require("wayfinder.telescope").select_runtime()
```

**Fallback Behavior**:
- If Telescope not installed, commands gracefully degrade
- Uses `vim.ui.select()` and `vim.ui.input()` instead
- Full functionality maintained without Telescope

### `plugin/wayfinder.vim` (Vim Plugin Loader)

**Purpose**: Traditional Vim plugin entry point

**Responsibilities**:
- Check Neovim version
- Check nvim-dap availability
- Auto-initialize plugin
- Setup standard keymaps
- Register abbreviations

**Key Features**:
- Guard clause prevents double-loading
- Initialization happens automatically
- Can be disabled with `g:wayfinder_disable_auto_setup`
- Default keymaps for nvim-dap
- Command abbreviations (WDF, WSR, WAP, WRT)

**Vim Variables** (customizable):
- `g:wayfinder_config` - Configuration table
- `g:wayfinder_use_keymaps` - Enable keymaps (default: 1)
- `g:wayfinder_disable_auto_setup` - Disable auto-setup (default: 0)
- `g:wayfinder_command_abbrev` - Enable abbreviations (default: 1)
- `g:wayfinder_quiet` - Suppress messages (default: 0)

## Data Flow

### Initialization

```
plugin/wayfinder.vim (auto-loads)
  ↓
lua/wayfinder/init.lua (setup() called)
  ├─ lua/wayfinder/config.lua (initialize)
  ├─ lua/wayfinder/dap.lua (register adapter)
  ├─ lua/wayfinder/commands.lua (register commands)
  └─ lua/wayfinder/telescope.lua (register if available)
```

### Debug Session Start

```
User: :WayfinderDebugFile
  ↓
commands.lua (debug_file handler)
  ├─ config.lua (detect_runtime)
  ├─ dap.lua (debug_file)
  └─ nvim-dap.run(config)
    ↓
dap.lua (wayfinder adapter callback)
  ├─ config.lua (get Wayfinder path, substitute variables)
  ├─ Spawn: wayfinder dap-server --port 5858 --runtime lua54 --script main.lua
  └─ Return: { type = "server", host = "localhost", port = 5858 }
    ↓
nvim-dap (connects to DAP server)
  ↓
User debugging (breakpoints, stepping, etc.)
```

### Configuration Loading

```
setup({ user_config })
  ↓
config.lua: setup()
  ├─ Merge user_config into defaults
  ├─ Load from environment variables
  ├─ Load from vim.g.wayfinder_config
  └─ Store in config table
```

## Module Dependencies

```
init.lua (public API)
  ├─ config.lua (required)
  ├─ dap.lua (required)
  ├─ commands.lua (required)
  └─ telescope.lua (optional, auto-loaded)

dap.lua
  ├─ init.lua (reference)
  ├─ config.lua (required)
  └─ nvim-dap (external)

commands.lua
  ├─ init.lua (reference)
  ├─ config.lua (required)
  ├─ dap.lua (required)
  └─ telescope.lua (optional)

telescope.lua
  ├─ init.lua (reference)
  ├─ config.lua (required)
  └─ telescope (external, optional)
```

## Configuration Hierarchy

```
Default values (in init.lua)
  ↓
Environment variables (WAYFINDER_PATH, WAYFINDER_PORT)
  ↓
vim.g.wayfinder_config
  ↓
setup() user_config parameter ← Highest priority
```

## Extension Points

### Adding New Commands

1. Add command handler in `commands.lua`
2. Call `vim.api.nvim_create_user_command()` in `setup()`
3. Register in `wayfinder.vim` if needed

### Adding New Configurations

1. Add to default config in `init.lua`
2. Add getter/setter in `config.lua` if special handling needed
3. Document in README.md and USAGE.md

### Customizing DAP Behavior

1. Modify `dap.lua` setup function
2. Or create custom config tables in user's init.lua
3. Pass to `dap.run()` with custom table

### Enhancing Telescope Integration

1. Modify or extend functions in `telescope.lua`
2. Add new pickers for additional features
3. Maintain fallback for non-Telescope users

## Testing Locations

Test scripts should be placed in:
- `test/` directory at plugin root
- Or in separate test repository

## Documentation Structure

```
README.md          - Main documentation and features
INSTALLATION.md    - Setup and troubleshooting
USAGE.md           - Examples and workflows
STRUCTURE.md       - This file (architecture)
```

---

**For plugin development:**
- Read module headers for intended use
- Follow Lua conventions and style
- Maintain backward compatibility
- Document new features in README.md
