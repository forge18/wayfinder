# Hot Code Reload

Hot code reload allows you to update modules in a running application without restarting, enabling rapid iteration during development. Wayfinder provides robust support for hot reloading Lua modules.

## What is Hot Code Reload?

Hot code reload (also known as hot swapping or live reloading) is the ability to update code in a running application without stopping and restarting the entire program. This feature is particularly valuable during development for:

- **Rapid iteration**: See changes immediately without restart delays
- **State preservation**: Maintain application state while updating code
- **Productivity boost**: Eliminate the stop-edit-start cycle

## How It Works

### Basic Process

1. **Application running**: Your Lua program is executing with Wayfinder attached
2. **Code change**: You modify a module file
3. **Reload request**: Send a hot reload command to Wayfinder
4. **State capture**: Wayfinder captures current module state
5. **Code reload**: New module code is loaded and executed
6. **State restore**: Captured state is restored where possible
7. **Continued execution**: Program continues with updated code

### Example Workflow

```lua
-- math.lua (initial version)
local M = {}

function M.add(a, b)
    return a + b
end

function M.multiply(a, b)
    return a * b
end

return M
```

```lua
-- main.lua
local math = require("math")

while true do
    local result = math.add(5, 3)  -- ← This will use updated code after reload
    print("Result:", result)
    socket.sleep(1)  -- Wait 1 second
end
```

During development:
1. Start debugging: `wayfinder launch --debug main.lua`
2. Modify `math.lua`:
   ```lua
   function M.add(a, b)
       return a + b + 1  -- ← Changed implementation
   end
   ```
3. Trigger reload: `wayfinder hot-reload --module math --port 5678`

## State Preservation

### What Gets Preserved

Wayfinder attempts to preserve:

- **Global variables**: Values in `_G` with the same names
- **Table structures**: Existing tables maintain their identity
- **Function upvalues**: Closures retain references to upvalues
- **Metatables**: Table metatables are preserved
- **Registry entries**: Lua registry values associated with the module

### Example Preservation

```lua
-- Before reload
local counter = 0  -- ← This global will be preserved

local M = {}
M.constants = { PI = 3.14159 }  -- ← This table structure will be preserved

function M.increment()
    counter = counter + 1  -- ← Counter value preserved
    return counter
end

return M
```

After hot reload, `counter` retains its value and `M.constants` keeps its structure.

### What Cannot Be Preserved

Certain changes cannot be automatically preserved:

- **Function identity**: Existing function references become stale
- **Closure upvalues**: Functions with captured upvalues may not update correctly
- **Method definitions**: Changes to method implementations in existing tables
- **Prototype changes**: Metatable changes to existing objects
- **C function replacements**: C functions cannot be hot reloaded

## Usage Patterns

### Command Line Interface

Trigger hot reload from the command line:

```bash
# Basic reload
wayfinder hot-reload --module mymodule --port 5678

# Specify host
wayfinder hot-reload --module mymodule --port 5678 --host 192.168.1.100

# Reload multiple modules
wayfinder hot-reload --module module1 --module module2 --port 5678
```

### Programmatic Interface

Within your application, you can trigger reloads programmatically:

```lua
-- Trigger reload from within the debugged application
debug.reloadModule("mymodule")
```

### IDE Integration

Most IDEs with Wayfinder integration provide hot reload buttons or commands:
- **VS Code**: Command palette → "Wayfinder: Hot Reload Module"
- **Neovim**: `:WayfinderHotReload moduleName`
- **JetBrains**: Right-click → "Hot Reload Module"

## Advanced Features

### Selective Reloading

Reload specific parts of a module:

```lua
-- Reload only a specific function
wayfinder hot-reload --module mymodule --function updateConfig --port 5678

-- Reload only a specific table
wayfinder hot-reload --module mymodule --table constants --port 5678
```

### Conditional Reloading

Apply conditions to hot reloads:

```lua
-- Reload only if certain criteria are met
wayfinder hot-reload --module mymodule --condition "development" --port 5678
```

### Batch Reloading

Reload multiple modules in dependency order:

```bash
# Reload modules in order
wayfinder hot-reload --module utils --port 5678
wayfinder hot-reload --module database --port 5678
wayfinder hot-reload --module api --port 5678
```

## Configuration

### Enable Hot Reload

Hot reload is enabled by default but can be controlled via configuration:

```yaml
# wayfinder.yaml
hotReload:
  enabled: true
  preserveState: true
  notifyOnChange: true
```

### State Preservation Settings

Control what state is preserved:

```yaml
hotReload:
  preserve:
    globals: true
    tables: true
    upvalues: true
    metatables: true
  warnings:
    functionIdentity: true
    closureUpvalues: true
```

### Auto-Reload

Configure automatic reloading when files change:

```yaml
hotReload:
  auto: true
  watchPaths:
    - "src/**/*.lua"
    - "lib/**/*.lua"
  ignorePaths:
    - "node_modules/**"
    - "*.tmp"
```

## Limitations and Constraints

### Lua Language Limitations

Due to Lua's runtime behavior:

- **Function References**: Existing references to reloaded functions become invalid
- **Closure Behavior**: Closures with upvalues may not behave as expected
- **Garbage Collection**: Some objects may be collected during reload
- **Memory Layout**: Internal memory representation may change

### Module System Constraints

- **require() Behavior**: Standard Lua module caching affects reload behavior
- **Package.loaded**: Modules in `package.loaded` need special handling
- **Dependency Chains**: Complex dependencies can make reloading unpredictable

### Performance Considerations

- **Reload Overhead**: Each reload has processing overhead
- **Memory Fragmentation**: Frequent reloading can fragment memory
- **State Tracking**: State preservation requires additional memory

## Best Practices

### Module Design for Reload

Design modules to work well with hot reload:

```lua
-- Good: Stateless functions
local M = {}

function M.process(data)
    return data * 2
end

return M

-- Better: Explicit state management
local M = {}
M._state = {}

function M.initialize()
    M._state.counter = 0
end

function M.increment()
    M._state.counter = M._state.counter + 1
    return M._state.counter
end

return M
```

### State Management

Manage state explicitly for better reload behavior:

```lua
-- Use module tables for state
local M = {}
M.config = {}
M.cache = {}

-- Initialize state in a dedicated function
function M.setup(initialConfig)
    M.config = initialConfig or {}
    M.cache = {}
end

return M
```

### Dependency Handling

Handle dependencies carefully:

```lua
-- Cache dependencies at module level for easier reload
local M = {}
M.dependencies = {}

function M.setDependency(name, module)
    M.dependencies[name] = module
end

function M.getDependency(name)
    return M.dependencies[name]
end
```

## Troubleshooting

### Reload Failures

If hot reload fails:

1. **Check module name**: Ensure the module name matches `require()` calls
2. **Verify connection**: Confirm Wayfinder is still connected
3. **Syntax errors**: Check for syntax errors in the reloaded module
4. **File permissions**: Ensure the module file is readable

### State Issues

If state isn't preserved as expected:

1. **Global naming**: Ensure globals have consistent names
2. **Table identity**: Verify tables are the same objects
3. **Upvalue capture**: Check closure behavior
4. **Registry usage**: Examine registry-based state storage

### Performance Problems

For slow reloads:

1. **Module size**: Break large modules into smaller ones
2. **State complexity**: Simplify state structures
3. **Dependency chain**: Reduce complex dependencies
4. **File I/O**: Check disk performance for module loading

## Error Handling

### Graceful Degradation

Wayfinder handles reload errors gracefully:

```lua
-- If reload fails, the original module remains active
-- Error details are provided in the debug console
```

### Partial Success

Some reloads may partially succeed:

```lua
-- Successful function updates but failed state preservation
-- Wayfinder reports what was and wasn't preserved
```

### Recovery Strategies

Recovery options after failed reloads:

1. **Manual restart**: Restart the entire application
2. **Selective retry**: Retry specific parts of the reload
3. **State reset**: Reset problematic state manually
4. **Incremental changes**: Make smaller, more focused changes

## Next Steps

- Explore [multi-version support](multi-version-support.md)
- Learn about [IDE integration](ide-integration.md)
- Understand [configuration options](../reference/configuration.md)