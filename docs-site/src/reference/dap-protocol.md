# Debug Adapter Protocol (DAP)

Wayfinder implements the Debug Adapter Protocol (DAP), an open standard that defines the abstract protocol used between development tools and debug adapters. This document describes how Wayfinder implements and extends the DAP.

## Overview

The Debug Adapter Protocol allows debugger frontends (IDEs, editors) to communicate with debugger backends (debug adapters) through a standardized JSON-based protocol. This enables a single debug adapter to work with multiple development tools.

## Protocol Basics

### Message Format

All DAP messages are JSON objects sent over stdin/stdout or TCP:

```json
{
  "seq": 1,
  "type": "request",
  "command": "launch",
  "arguments": {
    "program": "main.lua"
  }
}
```

### Message Types

1. **Requests**: Sent from client to debug adapter
2. **Responses**: Sent from debug adapter to client in response to requests
3. **Events**: Sent from debug adapter to client to report state changes

## Implemented Requests

### Configuration Requests

#### `initialize`

Initializes the debug adapter.

**Client Request:**

```json
{
  "seq": 1,
  "type": "request",
  "command": "initialize",
  "arguments": {
    "clientID": "vscode",
    "clientName": "Visual Studio Code",
    "adapterID": "wayfinder",
    "pathFormat": "path",
    "linesStartAt1": true,
    "columnsStartAt1": true
  }
}
```

**Adapter Response:**

```json
{
  "seq": 1,
  "type": "response",
  "request_seq": 1,
  "success": true,
  "command": "initialize",
  "body": {
    "supportsConfigurationDoneRequest": true,
    "supportsEvaluateForHovers": true,
    "supportsStepBack": false,
    "supportsSetVariable": true
  }
}
```

#### `launch`

Starts the debuggee and begins debugging.

**Client Request:**

```json
{
  "seq": 2,
  "type": "request",
  "command": "launch",
  "arguments": {
    "name": "Debug Lua Script",
    "type": "lua",
    "request": "launch",
    "program": "main.lua",
    "runtime": "lua54",
    "stopOnEntry": false
  }
}
```

#### `attach`

Attaches to an already running debuggee.

**Client Request:**

```json
{
  "seq": 3,
  "type": "request",
  "command": "attach",
  "arguments": {
    "name": "Attach to Process",
    "type": "lua",
    "request": "attach",
    "port": 5678
  }
}
```

#### `disconnect`

Terminates the debug session.

**Client Request:**

```json
{
  "seq": 10,
  "type": "request",
  "command": "disconnect",
  "arguments": {
    "restart": false
  }
}
```

### Breakpoint Requests

#### `setBreakpoints`

Sets breakpoints for a source file.

**Client Request:**

```json
{
  "seq": 4,
  "type": "request",
  "command": "setBreakpoints",
  "arguments": {
    "source": {
      "name": "main.lua",
      "path": "/project/main.lua"
    },
    "breakpoints": [
      {
        "line": 10,
        "condition": "x > 5",
        "hitCondition": ">= 3"
      }
    ],
    "lines": [10]
  }
}
```

#### `setFunctionBreakpoints`

Sets breakpoints on function entry.

**Client Request:**

```json
{
  "seq": 5,
  "type": "request",
  "command": "setFunctionBreakpoints",
  "arguments": {
    "breakpoints": [
      {
        "name": "main"
      }
    ]
  }
}
```

#### `setExceptionBreakpoints`

Configures exception breakpoints.

**Client Request:**

```json
{
  "seq": 6,
  "type": "request",
  "command": "setExceptionBreakpoints",
  "arguments": {
    "filters": ["raised"]
  }
}
```

### Execution Requests

#### `configurationDone`

Signals the end of the configuration sequence.

**Client Request:**

```json
{
  "seq": 7,
  "type": "request",
  "command": "configurationDone"
}
```

#### `continue`

Resumes execution.

**Client Request:**

```json
{
  "seq": 8,
  "type": "request",
  "command": "continue",
  "arguments": {
    "threadId": 1
  }
}
```

#### `next`

Steps to the next line in the current frame.

**Client Request:**

```json
{
  "seq": 9,
  "type": "request",
  "command": "next",
  "arguments": {
    "threadId": 1
  }
}
```

#### `stepIn`

Steps into the next function call.

**Client Request:**

```json
{
  "seq": 10,
  "type": "request",
  "command": "stepIn",
  "arguments": {
    "threadId": 1
  }
}
```

#### `stepOut`

Steps out of the current function.

**Client Request:**

```json
{
  "seq": 11,
  "type": "request",
  "command": "stepOut",
  "arguments": {
    "threadId": 1
  }
}
```

#### `pause`

Pauses execution.

**Client Request:**

```json
{
  "seq": 12,
  "type": "request",
  "command": "pause",
  "arguments": {
    "threadId": 1
  }
}
```

### Data Requests

#### `threads`

Retrieves the list of threads.

**Client Request:**

```json
{
  "seq": 13,
  "type": "request",
  "command": "threads"
}
```

#### `stackTrace`

Retrieves the stack trace for a thread.

**Client Request:**

```json
{
  "seq": 14,
  "type": "request",
  "command": "stackTrace",
  "arguments": {
    "threadId": 1,
    "startFrame": 0,
    "levels": 20
  }
}
```

#### `scopes`

Retrieves the scopes of the current stack frame.

**Client Request:**

```json
{
  "seq": 15,
  "type": "request",
  "command": "scopes",
  "arguments": {
    "frameId": 1
  }
}
```

#### `variables`

Retrieves variables in a scope or from an evaluate request.

**Client Request:**

```json
{
  "seq": 16,
  "type": "request",
  "command": "variables",
  "arguments": {
    "variablesReference": 1000
  }
}
```

#### `evaluate`

Evaluates an expression in the context of a stack frame.

**Client Request:**

```json
{
  "seq": 17,
  "type": "request",
  "command": "evaluate",
  "arguments": {
    "expression": "x + y",
    "frameId": 1,
    "context": "repl"
  }
}
```

#### `setVariable`

Sets the value of a variable.

**Client Request:**

```json
{
  "seq": 18,
  "type": "request",
  "command": "setVariable",
  "arguments": {
    "variablesReference": 1000,
    "name": "x",
    "value": "42"
  }
}
```

## Implemented Events

### `initialized`

Sent after successful initialization.

```json
{
  "seq": 1,
  "type": "event",
  "event": "initialized"
}
```

### `stopped`

Sent when execution stops due to a breakpoint, exception, etc.

```json
{
  "seq": 2,
  "type": "event",
  "event": "stopped",
  "body": {
    "reason": "breakpoint",
    "threadId": 1,
    "hitBreakpointIds": [1]
  }
}
```

### `continued`

Sent when execution continues.

```json
{
  "seq": 3,
  "type": "event",
  "event": "continued",
  "body": {
    "threadId": 1
  }
}
```

### `exited`

Sent when the debuggee exits.

```json
{
  "seq": 4,
  "type": "event",
  "event": "exited",
  "body": {
    "exitCode": 0
  }
}
```

### `terminated`

Sent when the debug session terminates.

```json
{
  "seq": 5,
  "type": "event",
  "event": "terminated"
}
```

### `thread`

Sent when a thread state changes.

```json
{
  "seq": 6,
  "type": "event",
  "event": "thread",
  "body": {
    "reason": "started",
    "threadId": 2
  }
}
```

### `output`

Sent to report output from the debuggee.

```json
{
  "seq": 7,
  "type": "event",
  "event": "output",
  "body": {
    "category": "stdout",
    "output": "Hello, World!\n"
  }
}
```

### `breakpoint`

Sent when a breakpoint is added, removed, or modified.

```json
{
  "seq": 8,
  "type": "event",
  "event": "breakpoint",
  "body": {
    "reason": "changed",
    "breakpoint": {
      "id": 1,
      "verified": true,
      "line": 10
    }
  }
}
```

## Wayfinder Extensions

### Custom Requests

#### `hotReload`

Triggers hot code reload for specified modules.

**Client Request:**

```json
{
  "seq": 100,
  "type": "request",
  "command": "hotReload",
  "arguments": {
    "modules": ["mymodule"],
    "preserveState": true
  }
}
```

#### `getSourceMap`

Retrieves source map information for a file.

**Client Request:**

```json
{
  "seq": 101,
  "type": "request",
  "command": "getSourceMap",
  "arguments": {
    "source": "main.luax"
  }
}
```

### Custom Events

#### `hotReload`

Reports hot reload status.

```json
{
  "seq": 100,
  "type": "event",
  "event": "hotReload",
  "body": {
    "status": "completed",
    "modules": ["mymodule"],
    "warnings": []
  }
}
```

#### `sourceMapLoaded`

Reports source map loading status.

```json
{
  "seq": 101,
  "type": "event",
  "event": "sourceMapLoaded",
  "body": {
    "source": "main.luax",
    "mapped": true,
    "errors": []
  }
}
```

## Capabilities

Wayfinder supports these DAP capabilities:

### Standard Capabilities

- **Configuration Done Request**: Yes
- **Conditional Breakpoints**: Yes
- **Function Breakpoints**: Yes
- **Exception Breakpoints**: Yes
- **Hit Conditional Breakpoints**: Yes
- **Log Points**: Yes
- **Evaluate for Hovers**: Yes
- **Set Variable**: Yes
- **Completions Request**: Yes
- **Modules Request**: Yes
- **Restart Request**: Yes

### Extended Capabilities

- **Hot Reload**: Custom implementation
- **Source Map Support**: Enhanced positioning
- **Coroutine Debugging**: Multi-thread inspection
- **Typed Variable Display**: Type information for LuaNext

## Protocol Compliance

### Supported Features

Wayfinder fully implements these DAP features:

- Launch and attach debugging
- Breakpoint management
- Step execution control
- Variable inspection and modification
- Call stack navigation
- Thread management
- Expression evaluation
- Exception handling

### Partial Support

These features have partial support:

- **Step Back**: Not supported (forward stepping only)
- **Reverse Debugging**: Limited support
- **Data Breakpoints**: Basic support via watchpoints

### Future Enhancements

Planned DAP feature implementations:

- **Goto Targets**: Jump to specific locations
- **Step In Targets**: Choose specific function calls
- **Cancel Request**: Cancel long-running operations
- **Progress Reporting**: Detailed operation progress

## Message Sequences

### Typical Debug Session

1. **Initialization Sequence**

   ```
   Client → initialize → Adapter
   Adapter → initialized event → Client
   Client → launch/attach → Adapter
   Adapter → launch/attach response → Client
   Client → setBreakpoints → Adapter
   Client → setExceptionBreakpoints → Adapter
   Client → configurationDone → Adapter
   ```

2. **Execution Sequence**

   ```
   Adapter → stopped event → Client
   Client → stackTrace → Adapter
   Client → scopes → Adapter
   Client → variables → Adapter
   Client → continue/next/stepIn/stepOut → Adapter
   Adapter → continued event → Client
   ```

3. **Termination Sequence**

   ```
   Client → disconnect → Adapter
   Adapter → exited event → Client
   Adapter → terminated event → Client
   ```

## Error Handling

### Protocol Errors

Wayfinder handles protocol errors gracefully:

- **Malformed Messages**: Reports parsing errors
- **Unknown Requests**: Responds with error responses
- **Sequence Issues**: Validates message ordering
- **Timeout Handling**: Manages communication timeouts

### Error Response Format

```json
{
  "seq": 1,
  "type": "response",
  "request_seq": 1,
  "success": false,
  "command": "launch",
  "message": "Failed to start debug session",
  "body": {
    "error": {
      "id": 1001,
      "format": "Runtime '{runtime}' not found",
      "variables": {
        "runtime": "lua55"
      }
    }
  }
}
```

## Performance Considerations

### Message Processing

- **Asynchronous Handling**: Non-blocking message processing
- **Batch Operations**: Efficient handling of multiple requests
- **Caching**: Cached responses for repeated requests
- **Throttling**: Rate limiting for high-frequency events

### Memory Management

- **Object Pooling**: Reuse of common objects
- **Lazy Loading**: Deferred loading of expensive data
- **Garbage Collection**: Automatic cleanup of unused resources
- **Memory Limits**: Configurable memory usage caps

## Security Considerations

### Input Validation

- **Message Validation**: Strict JSON schema validation
- **Path Sanitization**: Secure handling of file paths
- **Expression Safety**: Safe evaluation of debug expressions
- **Network Security**: Secure network communication

### Access Control

- **Permission Checking**: Validate operation permissions
- **Resource Limits**: Prevent resource exhaustion
- **Isolation**: Isolate debug sessions
- **Audit Logging**: Log security-relevant events

## Next Steps

- Learn about [error codes](error-codes.md)
- Explore [CLI commands](cli-commands.md)
- Understand [configuration options](configuration.md)
