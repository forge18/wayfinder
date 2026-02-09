# Breakpoints

Breakpoints are one of the most fundamental debugging tools. They allow you to pause program execution at specific points to inspect the state of your application.

## Types of Breakpoints

### Line Breakpoints

Line breakpoints pause execution when a specific line of code is about to be executed.

```lua
function calculateTotal(items)
    local total = 0  -- ← You can set a breakpoint here
    for _, item in ipairs(items) do
        total = total + item.price * item.quantity
    end
    return total
end
```

### Function Breakpoints

Function breakpoints pause execution when a specific function is called, regardless of where it's defined in the code.

### Exception Breakpoints

Exception breakpoints pause execution when an error occurs in your Lua code.

### Conditional Breakpoints

Conditional breakpoints only pause execution when a specific condition evaluates to true.

```lua
-- This breakpoint will only trigger when i equals 5
for i = 1, 10 do
    print(i)  -- ← Conditional breakpoint: i == 5
end
```

### Logpoints

Logpoints print a message to the debug console without pausing execution.

## Setting Breakpoints

### In Your IDE

Most IDEs allow you to set breakpoints by clicking in the margin next to line numbers:

1. Click in the left margin next to a line number
2. A red dot typically appears to indicate a breakpoint
3. Click again to remove the breakpoint

### Via Configuration

You can also define breakpoints in your `wayfinder.yaml` configuration file:

```yaml
breakpoints:
  - file: "src/main.lua"
    line: 15
    condition: "variable > 10"
  - file: "src/utils.lua"
    function: "calculateTotal"
```

## Managing Breakpoints

### Enabling/Disabling Breakpoints

Breakpoints can be temporarily disabled without removing them:

- **Disable**: The breakpoint exists but won't trigger
- **Enable**: The breakpoint will trigger when reached

### Hit Count Filtering

You can configure breakpoints to trigger only after a certain number of hits:

- **Greater than or equal to N**: Trigger on the Nth hit and onwards
- **Equal to N**: Trigger only on the Nth hit
- **Modulo N**: Trigger every Nth hit

### Removing Breakpoints

Breakpoints can be removed individually or all at once:

```bash
# Remove all breakpoints via CLI (when connected to a debug session)
wayfinder remove-breakpoints --all
```

## Advanced Breakpoint Features

### Column Breakpoints

Some debuggers support setting breakpoints at specific columns within a line, useful for multi-statement lines.

### Event-Based Breakpoints

Wayfinder supports breakpoints that trigger on specific events:

- **Entry breakpoints**: Trigger when entering a function
- **Exit breakpoints**: Trigger when exiting a function
- **Throw breakpoints**: Trigger when an exception is thrown

## Best Practices

### Strategic Placement

Place breakpoints at:
1. **Entry points** of functions to observe inputs
2. **Loop boundaries** to monitor iteration behavior
3. **Conditional branches** to verify logic flow
4. **Return statements** to check output values

### Conditional Logic

Use conditional breakpoints to:
- Reduce noise from frequently hit breakpoints
- Focus on specific edge cases
- Debug intermittent issues

### Performance Considerations

Keep in mind that:
- Each breakpoint adds slight overhead
- Complex conditions can significantly impact performance
- Logpoints are generally less disruptive than regular breakpoints

## Troubleshooting Breakpoints

### Breakpoints Not Hitting

If a breakpoint isn't triggering:

1. **Verify the code is executing**: Ensure the line is actually reached
2. **Check breakpoint placement**: Make sure it's on an executable line
3. **Verify breakpoint is enabled**: Check that it hasn't been disabled
4. **Confirm correct file**: Ensure you're debugging the right file
5. **Check conditions**: Verify conditional breakpoints have correct syntax

### Debugging Optimized Code

In some cases with heavily optimized code:
- Line numbers may not correspond to actual execution
- Variables may be optimized away
- Control flow may be rearranged

## Next Steps

- Learn about [variable inspection](variables.md)
- Explore [expression evaluation](expressions.md)
- Understand [call stack navigation](call-stack.md)