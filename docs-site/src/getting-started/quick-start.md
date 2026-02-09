# Quick Start

This guide will help you get started with Wayfinder quickly. We'll cover the basic usage patterns and show you how to debug your Lua applications.

## Launch Mode

The simplest way to use Wayfinder is in launch mode, where it starts your Lua script with debugging enabled.

### Basic Launch

To launch a Lua script with debugging:

```bash
wayfinder launch --debug --runtime lua54 script.lua
```

This command will:
1. Start the Wayfinder DAP server
2. Launch your Lua script with debugging enabled
3. Wait for a debugger to connect (if using an IDE)

### Launch Without Debugging

You can also use Wayfinder to simply run Lua scripts:

```bash
wayfinder launch script.lua
```

This executes the script normally without debugging capabilities.

### Specifying Working Directory

To run your script in a specific directory:

```bash
wayfinder launch --cwd /path/to/project --runtime lua54 script.lua
```

## DAP Server Mode

For IDE integration, Wayfinder runs as a DAP server that your IDE can connect to.

### Stdio Mode (Default)

Most IDEs use stdio communication by default:

```bash
wayfinder dap
```

### TCP Mode

For manual connections or specific setups, you can run Wayfinder in TCP mode:

```bash
wayfinder dap --port 5678
```

## Attach Mode

Wayfinder can also attach to a running Lua process.

### Attaching via TCP Port

```bash
wayfinder attach --port 5678
```

### Attaching to Process by PID (Unix Systems)

```bash
wayfinder attach --pid 12345
```

## Hot Reload

Wayfinder supports hot code reloading for rapid development iterations.

### Reloading a Module

To reload a module in a running debug session:

```bash
wayfinder hot-reload --module mymodule --port 5678
```

You can also specify a different host if needed:

```bash
wayfinder hot-reload --module mymodule --port 5678 --host 192.168.1.100
```

## Configuration

Wayfinder can be configured using YAML configuration files. Create a `wayfinder.yaml` file in your project directory:

```yaml
# Runtime configuration
runtime: lua54

# Working directory for script execution
cwd: /path/to/project

# Environment variables
env:
  LUA_PATH: "./?.lua;/usr/local/share/lua/5.4/?.lua"
  DEBUG_MODE: "true"

# Source map behavior (for TypedLua debugging)
# Options: "ask", "lenient", "strict"
sourceMapBehavior: lenient

# Hot reload configuration
evaluate:
  mutate: true  # Allow variable mutation during evaluation
```

Configuration files are loaded from:
1. Project directory: `./wayfinder.yaml`
2. Home directory: `~/.wayfinder.yaml`

CLI arguments take precedence over configuration file settings.

## IDE Integration

Most modern IDEs support the Debug Adapter Protocol. Here's how to configure popular IDEs:

### Visual Studio Code

Create a `.vscode/launch.json` file in your project:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug Lua Script",
      "type": "lua",
      "request": "launch",
      "program": "${workspaceFolder}/script.lua",
      "runtime": "lua54"
    }
  ]
}
```

### Neovim (with nvim-dap)

Configure nvim-dap to use Wayfinder as the debug adapter. See the [IDE Integration Guide](../guides/ide-integration.md#neovim-nvim-dap) for detailed configuration instructions.

## Next Steps

- Learn about [breakpoints and debugging techniques](../language/breakpoints.md)
- Explore [configuration options](reference/configuration.md)
- Understand [IDE integration](../guides/ide-integration.md)