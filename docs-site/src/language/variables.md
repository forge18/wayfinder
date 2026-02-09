# Variables

Understanding how to inspect and manipulate variables is crucial for effective debugging. Wayfinder provides comprehensive variable inspection capabilities for all Lua variable types.

## Variable Scopes

### Local Variables

Local variables are defined within a specific function or block scope:

```lua
function processData(data)
    local count = #data  -- ← Local variable
    local sum = 0        -- ← Local variable
    
    for i, value in ipairs(data) do
        sum = sum + value
    end
    
    return sum / count
end
```

### Upvalues (Non-Local Variables)

Upvalues are variables from enclosing scopes that are accessible to inner functions:

```lua
function createCounter()
    local count = 0  -- ← This becomes an upvalue
    
    return function()
        count = count + 1  -- ← Accessing upvalue
        return count
    end
end
```

### Global Variables

Global variables are accessible from anywhere in your program:

```lua
MY_GLOBAL_CONFIG = {
    debug = true,
    timeout = 30
}

function checkConfig()
    print(MY_GLOBAL_CONFIG.debug)  -- ← Accessing global variable
end
```

### Metatables

Lua tables can have metatables that define their behavior:

```lua
local Vector = {}
Vector.__index = Vector

function Vector.new(x, y)
    local v = {x = x, y = y}
    setmetatable(v, Vector)
    return v
end

function Vector:magnitude()
    return math.sqrt(self.x^2 + self.y^2)
end

local v = Vector.new(3, 4)  -- ← Table with metatable
```

## Variable Inspection Features

### Variable Expansion

Complex data structures can be expanded to view their contents:

```lua
local player = {
    name = "Alice",
    stats = {
        health = 100,
        mana = 50,
        level = 10
    },
    inventory = {
        "sword",
        "shield",
        "potion"
    }
}
```

When inspecting `player`, you can expand:

- `stats` to see health, mana, and level
- `inventory` to see the item list

### Table Navigation

Large tables can be navigated efficiently:

- **Pagination**: Large arrays are paginated (e.g., showing 100 elements at a time)
- **Filtering**: Search within table keys and values
- **Sorting**: Sort table entries alphabetically or by type

### Function Inspection

Functions can be inspected to view:

- **Definition location**: File and line where the function is defined
- **Upvalues**: Non-local variables the function captures
- **Environment**: The function's environment table

```lua
local multiplier = 5

local function multiply(x)
    return x * multiplier  -- ← multiplier is an upvalue
end
```

## Special Variable Types

### Coroutines

Coroutines have special debugging properties:

```lua
local co = coroutine.create(function()
    for i = 1, 10 do
        print("Co:", i)
        coroutine.yield(i)
    end
end)

coroutine.resume(co)  -- ← Coroutine state can be inspected
```

Each coroutine has its own stack and local variables.

### User Data

User data represents C objects in Lua:

```lua
local file = io.open("data.txt", "r")  -- ← User data object
local socket = require("socket").tcp() -- ← User data object
```

While the internal C data isn't directly inspectable, metadata about user data objects can be viewed.

## Variable Modification

### Safe Evaluation

By default, variable inspection is read-only for safety. To enable modification:

```yaml
# wayfinder.yaml
evaluate:
  mutate: true
```

### Expression Evaluation

Variables can be modified through expression evaluation:

```lua
-- In the debug console:
player.stats.health = 80
table.insert(player.inventory, "magic ring")
```

### Type Information

Wayfinder provides type information for variables:

- **Primitive types**: nil, boolean, number, string
- **Table types**: array, dictionary, mixed
- **Function types**: Lua function, C function
- **Special types**: thread, userdata

## Performance Considerations

### Memory Impact

Variable inspection has minimal performance impact:

- **Lazy loading**: Values are only retrieved when expanded
- **Caching**: Recently accessed values are cached
- **Limits**: Very large tables are truncated to prevent memory issues

### Refresh Behavior

Variables are refreshed:

- **Automatically** when pausing at breakpoints
- **On demand** when expanding nodes in the variables view
- **Periodically** for "watch" variables

## Debugging Complex Scenarios

### Circular References

Wayfinder handles circular references gracefully:

```lua
local parent = { name = "Parent" }
local child = { name = "Child" }

parent.child = child
child.parent = parent  -- ← Circular reference
```

Circular references are displayed with special notation to prevent infinite expansion.

### Weak Tables

Weak tables are properly handled during inspection:

```lua
local weakTable = setmetatable({}, { __mode = "k" })
```

The weak nature of tables is indicated during inspection.

### Metamethod Effects

Tables with metamethods are shown with their effective values:

```lua
local t = setmetatable({1, 2, 3}, {
    __len = function() return 10 end  -- ← Overrides # operator
})
```

Both the raw table contents and metamethod effects are visible.

## Best Practices

### Effective Variable Watching

1. **Watch key variables**: Monitor variables that are central to your logic
2. **Use conditional watches**: Set conditions on when to update watch values
3. **Group related variables**: Organize watches by functionality or scope

### Understanding Scope

1. **Know your current scope**: Be aware of which function's variables you're viewing
2. **Track upvalue changes**: Remember that upvalues can change between function calls
3. **Monitor global pollution**: Keep track of global variable creation

## Troubleshooting

### Missing Variables

If expected variables aren't visible:

1. **Check scope**: Ensure you're in the correct stack frame
2. **Verify execution**: Confirm the code defining the variable has run
3. **Check optimization**: Compiler optimizations might have eliminated variables

### Performance Issues

For slow variable loading:

1. **Collapse large structures**: Don't expand massive tables unnecessarily
2. **Limit watches**: Reduce the number of active watch expressions
3. **Use pagination**: Navigate large arrays page by page

## Next Steps

- Learn about [expression evaluation](expressions.md)
- Explore [call stack navigation](call-stack.md)
- Understand [advanced debugging techniques](../guides/)
