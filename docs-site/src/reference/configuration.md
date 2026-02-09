# Configuration

Wayfinder can be configured using YAML configuration files. This allows you to set default behaviors and options without having to specify them on the command line every time.

## Configuration File Locations

Wayfinder looks for configuration files in the following locations (in order of precedence):

1. **Project directory**: `./wayfinder.yaml`
2. **Home directory**: `~/.wayfinder.yaml`

Command-line arguments take precedence over configuration file settings.

## Configuration Options

### Runtime Configuration

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `runtime` | String | Lua runtime to use | `lua54` |
| `cwd` | String | Working directory for script execution | Current directory |
| `env` | Object | Environment variables as key-value pairs | `{}` |

### Debugging Configuration

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `sourceMapBehavior` | String | How to handle missing source maps (`ask`, `lenient`, `strict`) | `ask` |
| `stopOnEntry` | Boolean | Automatically break at the first line of the program | `false` |

### Expression Evaluation

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `evaluate.mutate` | Boolean | Enable variable mutation during expression evaluation | `false` |

### Coroutine Debugging

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `coroutines.breakOnAll` | Boolean | Break when any coroutine is created | `true` |

## Example Configuration File

```yaml
# Runtime configuration
runtime: lua54
cwd: /path/to/project

# Environment variables
env:
  LUA_PATH: "./?.lua;/usr/local/share/lua/5.4/?.lua"
  DEBUG_MODE: "true"
  MY_APP_CONFIG: "production"

# Source map behavior (for TypedLua debugging)
# Options: "ask", "lenient", "strict"
sourceMapBehavior: lenient

# Stop at first line of program
stopOnEntry: false

# Hot reload configuration
evaluate:
  mutate: true  # Allow variable mutation during evaluation

# Coroutine debugging
coroutines:
  breakOnAll: true
```

## Runtime Options

The `runtime` option specifies which Lua interpreter to use. Supported values include:

- `lua51`: Lua 5.1
- `lua52`: Lua 5.2
- `lua53`: Lua 5.3
- `lua54`: Lua 5.4 (default)
- `luanext51`: LuaNext 5.1
- `luanext52`: LuaNext 5.2
- `luanext53`: LuaNext 5.3
- `luanext54`: LuaNext 5.4

## Source Map Behavior

When debugging TypedLua files (.luax), Wayfinder uses source maps to translate between the compiled code and the original source. The `sourceMapBehavior` option controls how missing source maps are handled:

- `ask`: Prompt user when source map is missing
- `lenient`: Debug .lua files only if source map is missing
- `strict`: Error if source map is missing for .luax files

## Environment Variables

Environment variables can be set in the configuration file and will be available to the debugged program:

```yaml
env:
  LUA_PATH: "./?.lua;/usr/local/share/lua/5.4/?.lua"
  DATABASE_URL: "postgresql://localhost/myapp"
  LOG_LEVEL: "debug"
```

## Conditional Configuration

While the YAML configuration doesn't support complex conditionals, you can achieve different configurations by using different files or by overriding settings with command-line arguments.

For example, you might have a development configuration:

```yaml
# wayfinder.dev.yaml
runtime: lua54
evaluate:
  mutate: true
sourceMapBehavior: lenient
```

And a production configuration:

```yaml
# wayfinder.prod.yaml
runtime: lua54
evaluate:
  mutate: false
sourceMapBehavior: strict
```

Then specify which configuration to use:

```bash
wayfinder launch --config wayfinder.prod.yaml script.lua
```
