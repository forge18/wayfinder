# CLI Commands

Wayfinder provides several command-line interface commands for different debugging scenarios.

## Main Commands

### `wayfinder dap`

Starts Wayfinder in DAP server mode for IDE integration.

```bash
wayfinder dap [OPTIONS]
```

**Options:**

- `--port <PORT>`: Run in TCP mode on the specified port (default: stdio mode)
- `--help`: Display help information

**Examples:**

```bash
# Stdio mode (default - for IDE integration)
wayfinder dap

# TCP mode on specific port
wayfinder dap --port 5678
```

### `wayfinder launch`

Launches a Lua script with or without debugging.

```bash
wayfinder launch [OPTIONS] [SCRIPT]
```

**Options:**

- `--debug`: Enable debugging
- `--runtime <RUNTIME>`: Specify Lua runtime (lua51, lua52, lua53, lua54, luanext51, etc.)
- `--cwd <DIRECTORY>`: Set working directory
- `--help`: Display help information

**Arguments:**

- `SCRIPT`: Path to the Lua script to execute

**Examples:**

```bash
# Launch a script with debugging enabled
wayfinder launch --debug --runtime lua54 script.lua

# Launch without debugging (simple execution)
wayfinder launch script.lua

# Specify working directory
wayfinder launch --cwd /path/to/project --runtime lua54 script.lua
```

### `wayfinder attach`

Attaches to a running Lua process.

```bash
wayfinder attach [OPTIONS]
```

**Options:**

- `--port <PORT>`: Attach via TCP port
- `--pid <PID>`: Attach to process by PID (Unix systems only)
- `--help`: Display help information

**Examples:**

```bash
# Attach via TCP port
wayfinder attach --port 5678

# Attach to process by PID (Unix systems)
wayfinder attach --pid 12345
```

### `wayfinder hot-reload`

Reloads a module in a running debug session.

```bash
wayfinder hot-reload [OPTIONS] --module <MODULE>
```

**Options:**

- `--module <MODULE>`: Module name to reload (required)
- `--port <PORT>`: DAP server port (default: 5678)
- `--host <HOST>`: DAP server host (default: 127.0.0.1)
- `--help`: Display help information

**Examples:**

```bash
# Connect to DAP server and reload a module
wayfinder hot-reload --module mymodule --port 5678

# Default host is 127.0.0.1, can be changed
wayfinder hot-reload --module mymodule --port 5678 --host 192.168.1.100
```

### `wayfinder version`

Displays version information.

```bash
wayfinder version
```

## Global Options

These options can be used with any command:

- `-h, --help`: Print help information
- `-V, --version`: Print version information
- `-v, --verbose`: Enable verbose output

## Environment Variables

Wayfinder respects several environment variables:

- `WAYFINDER_CONFIG`: Path to configuration file
- `WAYFINDER_LOG_LEVEL`: Logging level (trace, debug, info, warn, error)
- `LUA_PATH`: Lua module search path
- `LUA_CPATH`: Lua C module search path

## Configuration File

Wayfinder can be configured using YAML configuration files loaded from:

1. Project directory: `./wayfinder.yaml`
2. Home directory: `~/.wayfinder.yaml`

CLI arguments take precedence over configuration file settings.

Example configuration:

```yaml
runtime: lua54
cwd: /path/to/project
env:
  LUA_PATH: "./?.lua;/usr/local/share/lua/5.4/?.lua"
  DEBUG_MODE: "true"
sourceMapBehavior: lenient
evaluate:
  mutate: true
```
