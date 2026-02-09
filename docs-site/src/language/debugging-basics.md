# Debugging Basics

This section covers the fundamental concepts of debugging with Wayfinder.

## What is Debugging?

Debugging is the process of identifying and resolving defects or problems within a computer program that prevent correct operation. Wayfinder provides a comprehensive debugging experience for Lua applications through the Debug Adapter Protocol (DAP).

## Core Debugging Concepts

### Breakpoints

Breakpoints are markers that you set at specific lines in your code where you want the debugger to pause execution. This allows you to inspect the state of your program at that point.

### Stepping

Stepping refers to the ability to execute your program one statement at a time. Wayfinder supports several stepping modes:

- **Step Over**: Execute the next line, but don't step into function calls
- **Step Into**: Execute the next line and step into function calls
- **Step Out**: Continue execution until the current function returns

### Call Stack

The call stack shows the sequence of function calls that led to the current point of execution. Each function call creates a new frame on the stack.

### Variables and Scope

Variables can be inspected in different scopes:

- **Local variables**: Variables defined within the current function
- **Upvalues**: Variables from enclosing scopes
- **Global variables**: Variables in the global namespace

## Debugging Workflow

1. **Set breakpoints** at locations where you suspect issues
2. **Start debugging** by launching your program with Wayfinder
3. **Inspect state** when execution pauses at breakpoints
4. **Step through code** to understand execution flow
5. **Evaluate expressions** to test hypotheses
6. **Continue or stop** debugging as needed

## Next Steps

- Learn about [setting and managing breakpoints](breakpoints.md)
- Understand [variable inspection](variables.md)
- Explore [expression evaluation](expressions.md)
