# Call Stack

The call stack is a fundamental concept in debugging that shows the sequence of function calls leading to the current execution point. Understanding how to navigate and interpret the call stack is essential for effective debugging.

## What is the Call Stack?

The call stack (also known as the execution stack or function call stack) is a data structure that stores information about the active subroutines or functions in a program. Each time a function is called, a new frame is pushed onto the stack, and when a function returns, its frame is popped off the stack.

## Stack Frames

### Frame Structure

Each stack frame contains:

- **Function name**: The name of the executing function
- **File and line**: Location in the source code
- **Arguments**: Parameters passed to the function
- **Local variables**: Variables defined within the function
- **Upvalues**: Non-local variables accessible to the function

### Example Call Stack

```lua
function main()
    local config = loadConfig()
    processData(config.data)  -- ← Breakpoint here
end

function processData(items)
    for _, item in ipairs(items) do
        validateItem(item)    -- ← Execution paused here
    end
end

function validateItem(item)
    if not item.id then
        error("Missing ID")   -- ← Stack trace originates here
    end
end
```

When paused at the error, the call stack would show:

1. `validateItem` (current frame)
2. `processData` (calling function)
3. `main` (top-level function)

## Navigating the Call Stack

### Viewing Stack Frames

In your IDE's debug view, you'll typically see something like:

```
▶ validateItem (item.lua:15)
▶ processData (processor.lua:8)
▶ main (main.lua:3)
```

Each line represents a stack frame, usually showing:

- Function name
- Source file
- Line number

### Selecting Frames

You can click on any frame to view its context:

- **Variables**: Local variables, upvalues, and globals in that scope
- **Code**: Source code around the execution point
- **Arguments**: Parameters passed to the function

### Frame Information

When selecting a frame, you can inspect:

```lua
-- Frame 0: validateItem
-- Arguments: item = { id = nil, name = "test" }
-- Locals: none

-- Frame 1: processData
-- Arguments: items = { {...}, {...} }  -- Array of 10 items
-- Locals: item = { id = nil, name = "test" }

-- Frame 2: main
-- Arguments: none
-- Locals: config = { data = {...}, debug = true }
```

## Stack Manipulation

### Stepping Through Frames

You can navigate the stack using stepping commands:

- **Step Into**: Move to the next line, entering function calls
- **Step Over**: Execute the current line, staying in the same frame
- **Step Out**: Continue until returning from the current function

### Async Stack Traces

For programs using coroutines or callbacks, Wayfinder can track asynchronous call stacks:

```lua
function asyncOperation(callback)
    -- Simulate async work
    timer.setTimeout(1000, function()
        local result = performWork()
        callback(result)  -- ← Asynchronous callback
    end)
end
```

Wayfinder maintains context to show the logical call sequence even across async boundaries.

## Advanced Stack Features

### Tail Call Optimization

Lua performs tail call optimization, which can affect stack traces:

```lua
function factorial(n, acc)
    if n == 0 then
        return acc
    else
        return factorial(n - 1, n * acc)  -- ← Tail call
    end
end
```

In optimized tail calls, intermediate frames may not appear in the stack trace.

### C Function Frames

Calls to C functions appear in the stack:

```lua
local file = io.open("data.txt", "r")
local content = file:read("*a")  -- ← C function call
file:close()
```

C function frames show the C function name and may have limited variable inspection.

### Error Stack Traces

When errors occur, Lua provides stack traces:

```lua
function level1()
    level2()
end

function level2()
    level3()
end

function level3()
    error("Something went wrong!")
end

level1()
```

Error message:

```
lua: script.lua:11: Something went wrong!
stack traceback:
    [C]: in function 'error'
    script.lua:11: in function 'level3'
    script.lua:7: in function 'level2'
    script.lua:3: in function 'level1'
    script.lua:14: in main chunk
    [C]: ?
```

## Debugging with the Call Stack

### Identifying Issues

The call stack helps you:

1. **Trace execution path**: See how you got to the current point
2. **Identify recursion**: Spot unintended recursive calls
3. **Find data sources**: Trace where problematic data originated
4. **Understand context**: See the broader program flow

### Common Patterns

#### Deep Call Stacks

```lua
function a() b() end
function b() c() end
function c() d() end
-- ... many more levels
function z() -- breakpoint here -- end
```

Deep stacks often indicate:

- Complex architectures
- Potential performance issues
- Difficult debugging scenarios

#### Recursion Detection

```lua
function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)  -- ← Repeated frames
    end
end
```

Repeated function names in the stack indicate recursion.

### Stack Analysis Techniques

#### Bottom-Up Analysis

Start from the main entry point and work down to understand the overall flow.

#### Top-Down Analysis

Start from the current frame and work up to understand the immediate context.

#### Cross-Frame Inspection

Compare variable values across different stack frames to trace data flow.

## Performance Considerations

### Stack Size Limits

Lua has limits on stack size:

- **C stack**: Limited by system resources
- **Lua stack**: Limited by available memory
- **Debug overhead**: Additional frames for debug hooks

### Memory Impact

Large call stacks consume memory:

- Each frame stores variable information
- Deep recursion can lead to stack overflow
- Debug information increases memory usage

## Best Practices

### Effective Stack Usage

1. **Understand the flow**: Use the stack to comprehend program execution
2. **Trace data origins**: Follow variables back through the call stack
3. **Identify hot paths**: Recognize frequently called code paths
4. **Detect anomalies**: Spot unusual stack patterns

### Stack Reading Tips

1. **Read bottom to top**: The bottom frame is usually your entry point
2. **Look for patterns**: Repeated function names often indicate loops or recursion
3. **Check arguments**: Verify that functions are receiving expected parameters
4. **Monitor depth**: Keep an eye on stack depth to avoid overflow

## Troubleshooting

### Missing Stack Frames

If frames are missing from the stack:

1. **Tail call optimization**: Check for optimized tail calls
2. **Compiler optimizations**: Some builds optimize aggressively
3. **Debug information**: Ensure debug symbols are available

### Confusing Stack Traces

For unclear stack traces:

1. **Add debug prints**: Temporary print statements can clarify flow
2. **Simplify code**: Reduce complexity to isolate issues
3. **Check async flows**: Consider asynchronous execution paths

### Performance Issues

For stack-related performance problems:

1. **Reduce recursion depth**: Convert deep recursion to iteration
2. **Limit stack inspection**: Don't expand all frames automatically
3. **Optimize breakpoints**: Place breakpoints strategically to reduce stack operations

## Next Steps

- Explore [IDE integration](../guides/ide-integration.md)
- Learn about [LuaNext debugging](../guides/luanext-debugging.md)
- Understand [hot code reload](../guides/hot-code-reload.md)
