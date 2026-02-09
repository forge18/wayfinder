# Expression Evaluation

Expression evaluation is a powerful debugging feature that allows you to execute Lua code in the context of the current breakpoint, inspect values, and even modify program state.

## What is Expression Evaluation?

Expression evaluation lets you:

- **Inspect values**: Check the current value of variables and expressions
- **Execute code**: Run arbitrary Lua code in the debugged program's context
- **Modify state**: Change variable values (when enabled)
- **Test hypotheses**: Quickly verify assumptions about your code

## Basic Usage

### Simple Value Inspection

At any breakpoint, you can evaluate expressions in your IDE's debug console or through the Wayfinder CLI:

```lua
-- If you have these variables in scope:
local x = 10
local y = 20
local items = { "apple", "banana", "cherry" }

-- You can evaluate:
-- x + y          → 30
-- #items         → 3
-- items[1]       → "apple"
```

### Function Calls

You can call functions that are in scope:

```lua
local function calculateArea(width, height)
    return width * height
end

local rect = { width = 5, height = 3 }

-- You can evaluate:
-- calculateArea(rect.width, rect.height)  → 15
-- string.upper("hello")                   → "HELLO"
```

## Advanced Expression Features

### Table Manipulation

Create and manipulate tables during debugging:

```lua
-- Create a new table
local temp = { name = "debug", value = 42 }

-- Modify existing tables
table.insert(items, "date")
table.remove(items, 1)

-- Access nested values
player.stats.health = player.stats.health - 10
```

### String Operations

Perform string manipulations for testing:

```lua
local message = "Hello, World!"
-- string.len(message)     → 13
-- string.sub(message, 1, 5) → "Hello"
-- string.find(message, "World") → 8
```

### Mathematical Calculations

Perform calculations with current values:

```lua
-- If you have:
local prices = { 10.50, 15.75, 8.25 }
local tax_rate = 0.08

-- You can calculate:
-- prices[1] * (1 + tax_rate)  → 11.34
```

## Context Awareness

### Stack Frame Context

Expressions are evaluated in the context of the currently selected stack frame:

```lua
function outer()
    local x = 10
    
    function inner()
        local y = 20
        -- At this breakpoint, you can access:
        -- x  → 10 (upvalue from outer)
        -- y  → 20 (local to inner)
    end
    
    inner()
end
```

### Scope Resolution

Wayfinder follows Lua's standard scope resolution:

1. **Local variables** in the current function
2. **Upvalues** from enclosing scopes
3. **Global variables** in _G
4. **Metamethods** where applicable

## Safe Evaluation Mode

By default, expression evaluation is in safe mode to prevent accidental program modification:

```yaml
# wayfinder.yaml
evaluate:
  mutate: false  # Default - read-only evaluation
```

### Enabling Mutation

To allow variable modification:

```yaml
# wayfinder.yaml
evaluate:
  mutate: true  # Allow setting variables
```

With mutation enabled, you can:

```lua
-- Modify variables
x = x + 1
player.health = 100
table.insert(inventory, "potion")
```

## Expression Limitations

### Restricted Operations

For security and stability, some operations are restricted:

- **IO operations**: File system access is limited
- **Module loading**: require() may be restricted
- **System calls**: os.exit(), os.execute() may be blocked

### Performance Considerations

Complex expressions can impact debugging performance:

- **Large computations**: May slow down the debug session
- **Infinite loops**: Can hang the debugger
- **Memory allocation**: Large object creation affects program state

## Error Handling

### Syntax Errors

Malformed expressions show clear error messages:

```lua
-- Invalid syntax:
-- x + + y  → Syntax error: unexpected symbol near '+'

-- Undefined variables:
-- undefined_var  → nil
```

### Runtime Errors

Runtime errors in expressions are caught and reported:

```lua
local items = {1, 2, 3}
-- items[10] + "string"  → Error: attempt to perform arithmetic on string
```

## Practical Examples

### Debugging Loops

```lua
for i = 1, 1000 do
    -- Breakpoint here
    -- Evaluate: i, processed_items, error_count
end
```

### Validating Conditions

```lua
if user.age >= 18 and user.active then
    -- Breakpoint here
    -- Evaluate: user.name, user.permissions
end
```

### Testing Transformations

```lua
local processed = transform(data)
-- Breakpoint here
-- Evaluate: #processed, processed[1], validate(processed)
```

## IDE Integration

### Debug Console

Most IDEs provide a debug console where you can enter expressions:

```
> player.score
1500
> player.levelUp()
nil
> player.level
2
```

### Watch Expressions

Add expressions to the watch panel for continuous monitoring:

```
Watch: #inventory
Value: 5

Watch: player.health / player.maxHealth * 100
Value: 75.0
```

### Conditional Breakpoints

Use expressions in conditional breakpoints:

```lua
-- Break only when this expression is truthy:
-- player.health < 20

-- Break when a counter reaches a specific value:
-- request_count >= 100
```

## Best Practices

### Efficient Debugging

1. **Simple first**: Start with simple expressions to understand state
2. **Build complexity**: Gradually use more complex expressions
3. **Save useful expressions**: Keep a list of helpful debugging expressions

### Safe Modification

1. **Enable cautiously**: Only enable mutation when needed
2. **Backup state**: Note important values before modification
3. **Test changes**: Verify that modifications have expected effects

### Performance Mindset

1. **Avoid heavy computation**: Don't sort large arrays in expressions
2. **Limit side effects**: Prefer read-only inspection when possible
3. **Use watches wisely**: Don't watch expensive expressions

## Troubleshooting

### Expression Not Evaluating

If expressions aren't working:

1. **Check context**: Ensure you're paused at a breakpoint
2. **Verify syntax**: Check for Lua syntax errors
3. **Confirm scope**: Make sure variables are in scope

### Unexpected Results

For unexpected results:

1. **Check variable values**: Verify the actual values of components
2. **Consider metamethods**: Account for __index,__call, etc.
3. **Review scope**: Ensure you're in the right stack frame

## Next Steps

- Learn about [call stack navigation](call-stack.md)
- Explore [advanced debugging techniques](../guides/)
- Understand [IDE integration](../guides/ide-integration.md)
