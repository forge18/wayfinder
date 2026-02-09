# Multi-Version Support

Wayfinder supports debugging applications running on multiple versions of Lua, from Lua 5.1 through Lua 5.4, as well as LuaNext variants. This flexibility allows you to debug your code regardless of the target Lua environment.

## Supported Versions

### Standard Lua Versions

Wayfinder supports all major Lua versions:

- **Lua 5.1**: The long-term stable version
- **Lua 5.2**: Added yieldable pcall, goto statement, ephemeron tables
- **Lua 5.3**: Added integer subtype, bitwise operators, utf8 library
- **Lua 5.4**: Added to-be-closed variables, const attributes, warning system

### LuaNext Variants

Wayfinder also supports LuaNext versions with enhanced features:

- **LuaNext 5.1**: Lua 5.1 with LuaNext extensions
- **LuaNext 5.2**: Lua 5.2 with LuaNext extensions
- **LuaNext 5.3**: Lua 5.3 with LuaNext extensions
- **LuaNext 5.4**: Lua 5.4 with LuaNext extensions

## Version Selection

### Command Line

Specify the Lua version when launching:

```bash
# Lua 5.1
wayfinder launch --runtime lua51 script.lua

# Lua 5.2
wayfinder launch --runtime lua52 script.lua

# Lua 5.3
wayfinder launch --runtime lua53 script.lua

# Lua 5.4 (default)
wayfinder launch --runtime lua54 script.lua

# LuaNext 5.4
wayfinder launch --runtime luanext54 script.lua
```

### Configuration File

Set the runtime in your `wayfinder.yaml`:

```yaml
runtime: lua53  # Options: lua51, lua52, lua53, lua54, luanext51, etc.
```

### IDE Configuration

Configure the runtime in your IDE's launch configuration:

```json
{
  "name": "Debug with Lua 5.3",
  "type": "lua",
  "request": "launch",
  "program": "${file}",
  "runtime": "lua53"
}
```

## Version-Specific Features

### Lua 5.1 Features

Wayfinder properly handles Lua 5.1-specific constructs:

```lua
-- Module system (module() function)
module("mymodule", package.seeall)

-- Setfenv/getfenv functions
local env = getfenv()
setfenv(1, env)
```

### Lua 5.2 Features

Support for Lua 5.2 enhancements:

```lua
-- Yieldable pcall
local success, result = pcall(function()
    coroutine.yield("yielding from pcall")
end)

-- Goto statement
::continue::
if condition then
    goto continue
end

-- Ephemeron tables
local weak_table = setmetatable({}, { __mode = "k" })
```

### Lua 5.3 Features

Debugging support for Lua 5.3 additions:

```lua
-- Integer subtype
local int_value = 42  -- Stored as integer
local float_value = 42.0  -- Stored as float

-- Bitwise operators
local result = 5 & 3  -- Bitwise AND
local shifted = 8 << 2  -- Left shift

-- UTF-8 library
local len = utf8.len("Hello 🌍")  -- Unicode string length
```

### Lua 5.4 Features

Full support for Lua 5.4 capabilities:

```lua
-- To-be-closed variables
local f <close> = io.open("file.txt", "r")

-- Const attributes
local object <const> = { value = 42 }

-- Warning system
warn("@on")
warn("This is a warning message")
```

## Compatibility Considerations

### API Differences

Wayfinder adapts to version-specific API differences:

```lua
-- Lua 5.1: loadstring
local func = loadstring("return 42")

-- Lua 5.2+: load
local func = load("return 42")

-- Wayfinder handles these differences transparently
```

### Library Availability

Different Lua versions have different standard libraries:

```lua
-- Lua 5.1: has loadstring
if _VERSION == "Lua 5.1" then
    load_func = loadstring
else
    load_func = load
end

-- Lua 5.3+: has utf8 library
if utf8 then
    local len = utf8.len("test")
end
```

### Behavioral Changes

Some behaviors differ between versions:

```lua
-- Table length operator behavior
local t = { nil, 2, 3 }  -- Length varies by Lua version

-- Metamethod restrictions
-- Lua 5.2+ restricts certain metamethod calls
```

## Cross-Version Debugging

### Version Testing

Test your code across multiple Lua versions:

```bash
# Test with Lua 5.1
wayfinder launch --runtime lua51 test.lua

# Test with Lua 5.2
wayfinder launch --runtime lua52 test.lua

# Test with Lua 5.3
wayfinder launch --runtime lua53 test.lua

# Test with Lua 5.4
wayfinder launch --runtime lua54 test.lua
```

### Conditional Debugging

Use version-specific debugging approaches:

```lua
-- Version-specific code paths
if _VERSION == "Lua 5.1" then
    -- Lua 5.1 specific debugging
    debug_info = debug.getinfo(1, "Sl")
elseif _VERSION == "Lua 5.2" then
    -- Lua 5.2 specific debugging
    debug_info = debug.getinfo(1, "Sl")
else
    -- Lua 5.3+ debugging
    debug_info = debug.getinfo(1, "Sl")
end
```

## Dynamic Loading (Experimental)

Wayfinder supports dynamic Lua loading, allowing a single binary to work with multiple Lua versions:

### Enabling Dynamic Loading

```bash
# Build with dynamic loading support
cargo build --features dynamic-lua --no-default-features
```

### Runtime Detection

The dynamic version detects and loads the appropriate Lua library at runtime:

```bash
# Automatically detect Lua version
wayfinder launch script.lua

# Force specific version
WAYFINDER_LUA_VERSION=5.3 wayfinder launch script.lua
```

### Environment Variables

Control dynamic loading behavior:

```bash
# Specify Lua library path
WAYFINDER_LUA_LIBRARY=/usr/lib/lua5.3/liblua.so wayfinder launch script.lua

# Set search paths
WAYFINDER_LUA_PATH=/usr/lib/lua/5.3/?.so wayfinder launch script.lua
```

## Version-Specific Limitations

### Lua 5.1 Limitations

- **Module system**: Uses older `module()` approach
- **Environment handling**: Different `setfenv`/`getfenv` behavior
- **Debug API**: Limited compared to newer versions

### Lua 5.2 Limitations

- **String packing**: No `string.pack` family of functions
- **Integer type**: No distinct integer subtype
- **Bitwise operations**: No native bitwise operators

### Lua 5.3 Limitations

- **UTF-8 support**: Limited compared to external libraries
- **Warning system**: No built-in warning mechanism
- **Close attributes**: No to-be-closed variables

### Lua 5.4 Limitations

- **Adoption**: May not be available on all systems
- **Library maturity**: Some libraries may not support it yet
- **Compatibility**: Older code may need updates

## Best Practices

### Version-Aware Coding

Write code that works across versions:

```lua
-- Compatible approach to loading code
local load_func = load or loadstring
local func = load_func("return 42")

-- Version detection
local lua_version = _VERSION:match("%d+%.%d+") or "5.1"

-- Conditional features
if utf8 then
    -- Use UTF-8 functions
else
    -- Fallback for older versions
end
```

### Testing Strategy

Test across target versions:

```lua
-- Test matrix approach
local versions = {"lua51", "lua52", "lua53", "lua54"}

for _, version in ipairs(versions) do
    print("Testing with " .. version)
    -- Run tests with specified version
end
```

### Documentation

Document version requirements:

```lua
--[[
Supported Lua Versions:
- Lua 5.1: Basic functionality
- Lua 5.2: Enhanced error handling
- Lua 5.3: Integer and bitwise support
- Lua 5.4: Close attributes and warnings
]]
```

## Troubleshooting

### Version Detection Issues

If Wayfinder can't detect the correct version:

1. **Check installation**: Ensure the specified Lua version is installed
2. **Verify paths**: Confirm Lua libraries are in standard locations
3. **Environment variables**: Check `LUA_PATH` and `LUA_CPATH`
4. **Permissions**: Ensure read access to Lua libraries

### Feature Compatibility

For version-specific feature issues:

1. **Feature detection**: Check if features exist before using them
2. **Graceful degradation**: Provide fallbacks for missing features
3. **Version checking**: Use `_VERSION` to adapt behavior
4. **Error handling**: Catch and handle version-specific errors

### Performance Differences

Addressing performance variations:

1. **Benchmark versions**: Measure performance across versions
2. **Optimize for targets**: Focus optimization on target versions
3. **Memory usage**: Monitor memory patterns per version
4. **GC behavior**: Understand garbage collector differences

## Next Steps

- Learn about [configuration options](../reference/configuration.md)
- Explore [CLI commands](../reference/cli-commands.md)
- Understand [IDE integration](ide-integration.md)