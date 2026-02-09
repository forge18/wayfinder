# Wayfinder Neovim Plugin - Installation Guide

This guide covers installing the Wayfinder debug plugin for Neovim.

## Prerequisites

### Required

1. **Neovim 0.7 or later**
   ```bash
   nvim --version
   ```

2. **nvim-dap plugin**
   ```lua
   -- Packer
   use "mfussenegger/nvim-dap"

   -- Lazy
   { "mfussenegger/nvim-dap" }
   ```

3. **Wayfinder CLI**
   ```bash
   # From wayfinder repository
   cd /path/to/wayfinder
   cargo build --release
   # Binary at ./target/release/wayfinder
   ```

### Optional

- **Telescope** - For enhanced UI (recommended)
  ```lua
  use { "nvim-telescope/telescope.nvim", requires = { "nvim-lua/plenary.nvim" } }
  ```

## Installation Methods

### 1. Using Packer.nvim

Add to your `~/.config/nvim/init.lua`:

```lua
use {
  "forge18/wayfinder",
  branch = "main",
  rtp = "editors/neovim",
  requires = {
    "mfussenegger/nvim-dap",
    { "nvim-telescope/telescope.nvim", requires = { "nvim-lua/plenary.nvim" } },
  },
  config = function()
    require("wayfinder").setup({
      -- Your configuration here
    })
  end,
}
```

Then run `:PackerSync` in Neovim.

### 2. Using Lazy.nvim

Add to your `~/.config/nvim/init.lua`:

```lua
{
  "forge18/wayfinder",
  branch = "main",
  dir = "editors/neovim",
  dependencies = {
    "mfussenegger/nvim-dap",
    {
      "nvim-telescope/telescope.nvim",
      dependencies = { "nvim-lua/plenary.nvim" },
    },
  },
  config = function()
    require("wayfinder").setup()
  end,
}
```

Lazy will automatically install on startup.

### 3. Using vim-plug

Add to your `~/.config/nvim/init.vim`:

```vim
call plug#begin('~/.nvim/plugged')

Plug 'mfussenegger/nvim-dap'
Plug 'nvim-telescope/telescope.nvim' | Plug 'nvim-lua/plenary.nvim'
Plug 'forge18/wayfinder', { 'rtp': 'editors/neovim', 'branch': 'main' }

call plug#end()

" Setup in Lua section
lua << EOF
  require("wayfinder").setup({
    -- Your configuration here
  })
EOF
```

Then run `:PlugInstall`.

### 4. Manual Installation

```bash
# Create plugin directory
mkdir -p ~/.config/nvim/pack/wayfinder/start

# Clone wayfinder repo
cd ~/.config/nvim/pack/wayfinder/start
git clone https://github.com/forge18/wayfinder.git

# Copy neovim plugin files
cp -r wayfinder/editors/neovim/* ~/.config/nvim/pack/wayfinder/start/wayfinder/
```

### 5. From Wayfinder Repository

If you're working on the Wayfinder repository:

```bash
cd /path/to/wayfinder
# Symlink to nvim plugins directory
ln -s "$(pwd)/editors/neovim" ~/.config/nvim/pack/wayfinder/start/wayfinder
```

## Configuration

### Minimal Setup

```lua
require("wayfinder").setup()
```

This uses default configuration and auto-detects runtimes.

### Full Configuration

```lua
require("wayfinder").setup({
  -- Path to Wayfinder binary (auto-detected if in PATH)
  wayfinder_path = "wayfinder",

  -- Default DAP port (auto-increments for multiple sessions)
  default_port = 5858,

  -- Auto-detect Lua version from file extension
  auto_detect_runtime = true,

  -- Source map behavior for bundled files
  source_map_behavior = "ask", -- "ask", "lenient", "strict"

  -- Runtime binary paths
  runtime_paths = {
    lua51 = "lua5.1",
    lua52 = "lua5.2",
    lua53 = "lua5.3",
    lua54 = "lua5.4",
    luanext = "luanext",
  },
})

-- Optional: disable features
vim.g.wayfinder_use_keymaps = true          -- Enable key mappings
vim.g.wayfinder_disable_auto_setup = false  -- Auto-setup on load
vim.g.wayfinder_command_abbrev = true       -- Enable abbreviations
vim.g.wayfinder_quiet = false               -- Show messages
```

## Verifying Installation

### 1. Check Plugin Loaded

```vim
:checkhealth wayfinder
```

Or check if module loads:

```vim
:lua require("wayfinder")
```

Should not show any errors.

### 2. Check nvim-dap Available

```vim
:lua require("dap")
```

Should not show errors. If it does, ensure nvim-dap is installed.

### 3. Check Wayfinder Binary

```vim
:!which wayfinder
```

Should show path to Wayfinder binary.

### 4. Check Available Runtimes

```vim
:WayfinderRuntimes
```

Should list available Lua versions with ✓ or ✗ status.

## Troubleshooting Installation

### Error: "Wayfinder requires nvim-dap plugin"

**Cause**: nvim-dap is not installed.

**Fix**: Install nvim-dap first using your plugin manager.

```lua
-- Packer
use "mfussenegger/nvim-dap"
:PackerSync

-- Lazy (auto-installs)
{ "mfussenegger/nvim-dap" }
```

### Error: "nvim-dap not found" in terminal

**Cause**: Neovim can't find nvim-dap module.

**Fix**: Ensure nvim-dap is in the correct plugin directory:

```bash
ls ~/.config/nvim/pack/*/start/nvim-dap
# Should list nvim-dap files
```

### Error: "wayfinder: command not found"

**Cause**: Wayfinder binary not in PATH.

**Fix**: Configure custom path in setup:

```lua
require("wayfinder").setup({
  wayfinder_path = "/path/to/wayfinder"
})
```

Or add to PATH:

```bash
export PATH="/path/to/wayfinder/target/release:$PATH"
# Add to ~/.bashrc or ~/.zshrc
```

### Commands Not Available

**Cause**: Plugin didn't load properly.

**Fix**:
1. Check for errors: `:checkhealth`
2. Restart Neovim: `:q!` then reopen
3. Check plugin directory permissions:
   ```bash
   ls -la ~/.config/nvim/pack/wayfinder/start/
   ```

### Telescope Not Working

**Cause**: Telescope not installed or loaded before Wayfinder.

**Fix**: Ensure load order in configuration:

```lua
require("telescope").setup()  -- Load first
require("wayfinder").setup()  -- Then load wayfinder
```

Or disable Telescope fallback:

```lua
-- Use fallback UI instead
:WayfinderSelectRuntime  -- Will use vim.ui.select
```

## Post-Installation Setup

### 1. Configure Lua Paths (Optional)

If your Lua binaries are in non-standard locations:

```lua
require("wayfinder").setup({
  runtime_paths = {
    lua51 = "/usr/local/bin/lua5.1",
    lua54 = "/opt/lua/lua5.4",
    luanext = "/path/to/luanext",
  }
})
```

### 2. Set Default Port (Optional)

If port 5858 conflicts with other services:

```lua
require("wayfinder").setup({
  default_port = 6000
})
```

### 3. Configure nvim-dap UI (Optional)

For better debugging experience, install nvim-dap-ui:

```lua
-- Packer
use "rcarriga/nvim-dap-ui"

-- Lazy
{ "rcarriga/nvim-dap-ui", dependencies = { "mfussenegger/nvim-dap" } }
```

Then configure:

```lua
require("dapui").setup()

local dap = require("dap")
dap.listeners.after.event_initialized["dapui_config"] = function()
  require("dapui").open()
end
dap.listeners.before.event_terminated["dapui_config"] = function()
  require("dapui").close()
end
dap.listeners.before.event_exited["dapui_config"] = function()
  require("dapui").close()
end
```

### 4. Create wayfinder.yaml (Optional)

In your project directory:

```yaml
runtime: lua54
port: 5858
```

This sets the default runtime for your project.

## Updating

### Packer.nvim

```vim
:PackerSync
```

### Lazy.nvim

```vim
:Lazy sync
```

### vim-plug

```vim
:PlugUpdate
```

### Manual

```bash
cd ~/.config/nvim/pack/wayfinder/start/wayfinder
git pull origin main
```

## Uninstallation

### Packer.nvim

Remove from `~/.config/nvim/init.lua` and run:

```vim
:PackerSync
```

### Lazy.nvim

Remove from `~/.config/nvim/init.lua`. Lazy will clean up on next startup.

### vim-plug

Remove from `~/.config/nvim/init.vim` and run:

```vim
:PlugClean
```

### Manual

```bash
rm -rf ~/.config/nvim/pack/wayfinder/start/wayfinder
```

## Getting Help

- See [README.md](./README.md) for usage
- See [USAGE.md](./USAGE.md) for examples
- Check `:help wayfinder` for built-in help
- Run `:WayfinderRuntimes` to verify setup

---

**Ready to debug?** See [USAGE.md](./USAGE.md) for your first debug session!
