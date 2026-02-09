# LuaNext Debugging

Wayfinder provides first-class support for debugging LuaNext (formerly TypedLua) applications with source map translation, allowing you to debug high-level source code while running compiled output.

## What is LuaNext?

LuaNext is a typed superset of Lua that compiles to standard Lua code. It adds optional type annotations and advanced language features while maintaining compatibility with existing Lua code.

## Source Maps

### How Source Maps Work

When LuaNext compiles `.luax` files to `.lua` files, it generates source maps that map positions in the compiled code back to the original source:

```luax
-- math.luax (original source)
function add(a: number, b: number): number
    return a + b  -- ← You can set breakpoints here
end

local result = add(5, 3)  -- ← And here
```

Compiles to:
```lua
-- math.lua (compiled output)
function add(a, b)
    return a + b  -- ← Wayfinder maps this back to the original source
end

local result = add(5, 3)
```

### Source Map Formats

Wayfinder supports multiple source map formats:

1. **Separate files**: `.map` files alongside compiled Lua
2. **Inline source maps**: Base64-encoded source maps embedded in comments
3. **Bundle mode**: Single source map for multiple source files

## Configuration

### Source Map Behavior

Control how Wayfinder handles missing source maps:

```yaml
# wayfinder.yaml
sourceMapBehavior: "ask"  # Options: "ask", "lenient", "strict"
```

- **ask**: Prompt user when source map is missing
- **lenient**: Debug .lua files only if source map is missing
- **strict**: Error if source map is missing for .luax files

### Runtime Selection

Specify LuaNext as the runtime:

```bash
wayfinder launch --runtime luanext54 script.lua
```

Or in configuration:
```yaml
runtime: luanext54
```

## Debugging Workflow

### Setting Breakpoints

Breakpoints work seamlessly with LuaNext source maps:

```luax
-- app.luax
interface User {
    name: string
    age: number
}

function greet(user: User): string
    return `Hello, ${user.name}!`  -- ← Set breakpoint here
end

const alice: User = { name = "Alice", age = 30 }
print(greet(alice))  -- ← Or here
```

When debugging, breakpoints are placed in the original `.luax` files, but execution pauses at the corresponding positions in the compiled `.lua` code.

### Variable Inspection

Wayfinder shows variable information using original names and types:

```luax
-- Variables appear with their original typed names
const users: User[] = [
    { name = "Alice", age = 30 },
    { name = "Bob", age = 25 }
]

for const user of users
    print(user.name)  -- ← Inspect 'user' with type information
end
```

### Call Stack Navigation

The call stack shows original function names and locations:

```
▶ greet (app.luax:7)
▶ main (app.luax:12)
```

Even though the actual execution happens in compiled code, Wayfinder translates the stack back to the original source.

## Advanced Features

### Type Information

Wayfinder can display type information when available:

```luax
-- Hover or inspect to see type annotations
const config: {
    debug: boolean
    port: number
    hosts: string[]
} = {
    debug = true,
    port = 3000,
    hosts = ["localhost", "127.0.0.1"]
}
```

### Template String Debugging

Template strings are properly mapped:

```luax
const name = "Alice"
const age = 30
const message = `User ${name} is ${age} years old`  -- ← Debug this template
```

### Class and Interface Debugging

Object-oriented features are fully supported:

```luax
class Person {
    private name: string
    
    constructor(name: string)
        self.name = name
    end
    
    public greet(): string
        return `Hello, I'm ${self.name}`  -- ← Debug methods
    end
}

const person = Person("Alice")
print(person.greet())  -- ← Debug instance methods
```

## Bundle Mode Support

### Multi-File Source Maps

For projects that compile multiple source files into a single output:

```luax
// project.luax bundle
import { User } from "./user.luax"
import { Database } from "./database.luax"

// All source maps are combined into one
```

Wayfinder correctly maps breakpoints and stack traces across all original files.

### Source Map Sections

Large projects can use source map sections for better performance:

```json
{
  "version": 3,
  "sections": [
    {
      "offset": { "line": 0, "column": 0 },
      "map": { /* source map for first file */ }
    },
    {
      "offset": { "line": 100, "column": 0 },
      "map": { /* source map for second file */ }
    }
  ]
}
```

## Troubleshooting

### Missing Source Maps

If source maps aren't working:

1. **Verify generation**: Ensure LuaNext is generating source maps
2. **Check paths**: Confirm source map file paths are correct
3. **File extensions**: Make sure you're using `.luax` for source files
4. **Compiler flags**: Check that source map generation is enabled

### Incorrect Position Mapping

If breakpoints trigger at wrong locations:

1. **Source map accuracy**: Verify the source map generation is precise
2. **Compiler version**: Ensure compatible versions of LuaNext and Wayfinder
3. **Line endings**: Check for consistent line ending formats
4. **Character encoding**: Verify UTF-8 encoding consistency

### Performance Issues

For slow debugging with large source maps:

1. **Source map size**: Consider splitting large bundles
2. **Mapping complexity**: Simplify complex template strings
3. **Cache utilization**: Wayfinder caches parsed source maps
4. **Incremental compilation**: Use incremental builds when possible

## Best Practices

### Source Map Generation

1. **Always generate**: Enable source maps for all debug builds
2. **Preserve sources**: Include original source content in source maps
3. **Consistent naming**: Use predictable source map file names
4. **Version control**: Consider whether to commit source maps

### Debugging Experience

1. **Original source focus**: Work primarily with `.luax` files
2. **Type-aware debugging**: Leverage type information for better insights
3. **Cross-file breakpoints**: Set breakpoints across multiple source files
4. **Template debugging**: Debug complex template expressions

### Configuration Management

1. **Environment-specific**: Use different settings for development vs production
2. **Team consistency**: Share source map configuration across teams
3. **Build integration**: Integrate source map generation into build pipelines
4. **Fallback strategies**: Define behavior for missing source maps

## Comparison with Plain Lua Debugging

### Advantages

- **High-level debugging**: Debug original source instead of compiled output
- **Type information**: Access type annotations during debugging
- **Better error reporting**: Errors reference original source locations
- **Enhanced productivity**: Work in the language you're writing

### Considerations

- **Additional complexity**: Source map management adds overhead
- **Tool compatibility**: Requires source map-aware tools
- **Performance impact**: Slight overhead from position translation
- **Build requirements**: Need to generate source maps during compilation

## Next Steps

- Learn about [hot code reload](hot-code-reload.md)
- Explore [multi-version support](multi-version-support.md)
- Understand [configuration options](../reference/configuration.md)