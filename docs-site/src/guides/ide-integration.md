# IDE Integration

Wayfinder integrates with popular IDEs through the Debug Adapter Protocol (DAP), providing a consistent debugging experience across different development environments.

## Supported IDEs

Wayfinder works with any IDE that supports the Debug Adapter Protocol, including:

- **Visual Studio Code**
- **Neovim** (with nvim-dap)
- **JetBrains IDEs** (IntelliJ IDEA, WebStorm, etc.)
- **Sublime Text** (with appropriate plugins)
- **Vim** (with Vimspector)
- **Emacs** (with dap-mode)

## Visual Studio Code

### Installation

1. Install the Wayfinder binary as described in the [Installation Guide](../getting-started/installation.md)
2. No specific extension is required for VS Code since it has built-in DAP support

### Configuration

Create a `.vscode/launch.json` file in your project root:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug Lua Script",
      "type": "lua",
      "request": "launch",
      "program": "${workspaceFolder}/main.lua",
      "runtime": "lua54",
      "cwd": "${workspaceFolder}",
      "stopOnEntry": false,
      "args": [],
      "env": {},
      "console": "internalConsole"
    },
    {
      "name": "Attach to Running Process",
      "type": "lua",
      "request": "attach",
      "port": 5678,
      "host": "127.0.0.1"
    }
  ]
}
```

### Debugging Features

VS Code provides rich debugging features with Wayfinder:

- **Breakpoints**: Click in the margin to set breakpoints
- **Variable inspection**: View locals, upvalues, and globals in the Variables panel
- **Call stack**: Navigate between stack frames
- **Debug console**: Evaluate expressions interactively
- **Watches**: Monitor specific expressions
- **Step controls**: Step over, step into, step out

## Neovim (nvim-dap)

Wayfinder works with Neovim through the nvim-dap plugin. A dedicated Wayfinder Neovim extension is planned but not yet implemented.

### Installation

Install nvim-dap using your preferred plugin manager:

```lua
-- Using packer.nvim
use { "mfussenegger/nvim-dap" }

-- Using vim-plug
Plug 'mfussenegger/nvim-dap'
```

### Configuration

Add Wayfinder configuration to your Neovim config:

```lua
local dap = require('dap')

-- Wayfinder adapter configuration
dap.adapters.wayfinder = {
  type = 'executable',
  command = 'wayfinder',
  args = { 'dap' }
}

-- Wayfinder configuration
dap.configurations.lua = {
  {
    type = 'wayfinder',
    request = 'launch',
    name = 'Launch Lua Script',
    program = '${file}',
    runtime = 'lua54'
  },
  {
    type = 'wayfinder',
    request = 'attach',
    name = 'Attach to Process',
    port = 5858
  }
}
```

### Usage

- Use `:DapToggleBreakpoint` to set breakpoints
- Use `:DapContinue` to start debugging
- Use `:DapStepOver`, `:DapStepInto`, `:DapStepOut` for navigation
- Use `:DapuiToggle` to open the debugging UI

Note: A dedicated Wayfinder Neovim extension with enhanced features is planned but not yet implemented. The current integration uses the standard DAP protocol through nvim-dap.

## JetBrains IDEs

### Configuration

1. Go to **Run/Debug Configurations**
2. Click **Add New Configuration** (+)
3. Select **Lua** or **Generic Debugger**
4. Configure the settings:

```
Name: Debug Lua Script
Host: localhost
Port: 5678
Program: path/to/your/script.lua
Working Directory: $ProjectFileDir$
Environment Variables: (optional)
```

### Debugging Workflow

1. Set breakpoints by clicking in the gutter
2. Click the Debug button or press Shift+F9
3. Use the Debug tool window to inspect variables and control execution

## Sublime Text

### Installation

Install the appropriate debugging package for Sublime Text, such as **Debugger**.

### Configuration

Create a `.sublime-project` file with debug configurations:

```json
{
  "folders": [
    {
      "path": "."
    }
  ],
  "settings": {
    "debugger_configurations": [
      {
        "name": "Debug Lua",
        "type": "lua",
        "request": "launch",
        "program": "${file}",
        "runtime": "lua54"
      }
    ]
  }
}
```

## Vim/Vimscript

### Vimspector Configuration

Create a `.vimspector.json` file in your project:

```json
{
  "configurations": {
    "Lua - Launch": {
      "adapter": "wayfinder",
      "configuration": {
        "request": "launch",
        "program": "${file}",
        "runtime": "lua54"
      }
    },
    "Lua - Attach": {
      "adapter": "wayfinder",
      "configuration": {
        "request": "attach",
        "port": 5678
      }
    }
  }
}
```

Adapter configuration in your Vimspector config:

```json
{
  "adapters": {
    "wayfinder": {
      "command": ["wayfinder", "dap"],
      "name": "wayfinder"
    }
  }
}
```

## Configuration Options

All IDEs support these common configuration options:

### Launch Configuration

```json
{
  "name": "Launch Program",
  "type": "lua",
  "request": "launch",
  "program": "main.lua",
  "runtime": "lua54",
  "cwd": "${workspaceFolder}",
  "args": ["arg1", "arg2"],
  "env": {
    "DEBUG": "true"
  },
  "stopOnEntry": false,
  "sourceMapBehavior": "lenient"
}
```

### Attach Configuration

```json
{
  "name": "Attach to Process",
  "type": "lua",
  "request": "attach",
  "port": 5678,
  "host": "127.0.0.1"
}
```

## Debugging Features Across IDEs

### Universal Features

All DAP-compliant IDEs provide:

- **Breakpoint management**: Set, enable/disable, conditional breakpoints
- **Variable inspection**: View and modify program state
- **Call stack navigation**: Move between function calls
- **Expression evaluation**: Execute code in debug context
- **Step controls**: Fine-grained execution control
- **Exception handling**: Break on errors

### IDE-Specific Features

Different IDEs may offer additional features:

- **VS Code**: Integrated terminal, inline variable values, debug visualizers
- **Neovim**: Modal editing integration, custom keybindings
- **JetBrains**: Advanced code insight, integrated version control
- **Sublime Text**: Minimal interface, fast performance

## Troubleshooting IDE Integration

### Connection Issues

If the IDE can't connect to Wayfinder:

1. **Verify Wayfinder installation**: Ensure `wayfinder` is in your PATH
2. **Check port availability**: Make sure the specified port isn't in use
3. **Firewall settings**: Verify that firewall isn't blocking connections
4. **IDE configuration**: Double-check configuration file syntax

### Breakpoint Problems

If breakpoints aren't working:

1. **File paths**: Ensure file paths match between IDE and debugger
2. **Executable lines**: Set breakpoints on actual executable code lines
3. **Source maps**: For TypedLua, verify source map configuration
4. **Program flow**: Confirm that the code with breakpoints is actually executed

### Variable Inspection Issues

If variables aren't displaying correctly:

1. **Scope selection**: Ensure you're viewing the correct stack frame
2. **Optimization**: Compiler optimizations might eliminate variables
3. **Timing**: Variables might not be initialized yet
4. **Complex types**: Some data structures might not display fully

## Best Practices

### Configuration Management

1. **Version control**: Commit debug configurations to share with team
2. **Environment variables**: Use environment-specific settings
3. **Relative paths**: Use relative paths for better portability
4. **Multiple configurations**: Create configs for different scenarios

### Debugging Workflow

1. **Start simple**: Begin with basic launch configurations
2. **Incremental complexity**: Add features as needed
3. **Consistent naming**: Use descriptive configuration names
4. **Documentation**: Comment complex configurations

## Next Steps

- Learn about [LuaNext debugging](luanext-debugging.md)
- Explore [hot code reload](hot-code-reload.md)
- Understand [multi-version support](multi-version-support.md)