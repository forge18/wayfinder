# Error Codes

Wayfinder uses standardized error codes to help you quickly identify and resolve issues. This reference documents all possible error codes and their meanings.

## Error Code Format

Wayfinder error codes follow the format `WFXXX` where:

- `WF` indicates a Wayfinder error
- `XXX` is a three-digit numeric code

## Debug Adapter Protocol Errors (WF000-WF099)

### WF001 - Failed to Start Debug Session

**Description**: Unable to initialize the debug session.
**Common Causes**:

- Invalid configuration
- Missing runtime dependencies
- Port already in use
**Solutions**:
- Check configuration file syntax
- Verify Lua installation
- Try a different port

### WF002 - Invalid Request

**Description**: Received an invalid or malformed DAP request.
**Common Causes**:

- Malformed JSON
- Missing required fields
- Unsupported request type
**Solutions**:
- Validate JSON syntax
- Check required parameters
- Update IDE/debugger

### WF003 - Runtime Not Found

**Description**: Specified Lua runtime is not available.
**Common Causes**:

- Runtime not installed
- Incorrect runtime name
- PATH issues
**Solutions**:
- Install the specified Lua version
- Verify runtime name in configuration
- Check system PATH

### WF004 - Script Not Found

**Description**: The specified script file could not be located.
**Common Causes**:

- Incorrect file path
- File permissions
- Working directory issues
**Solutions**:
- Verify file path
- Check file permissions
- Confirm working directory

### WF005 - Connection Timeout

**Description**: Failed to establish connection with the debug adapter.
**Common Causes**:

- Debug adapter not running
- Network issues
- Firewall blocking
**Solutions**:
- Ensure Wayfinder is running
- Check network connectivity
- Verify firewall settings

## Breakpoint Errors (WF100-WF199)

### WF101 - Breakpoint Not Validated

**Description**: Breakpoint could not be verified in the source code.
**Common Causes**:

- Breakpoint on non-executable line
- File path mismatch
- Source not available
**Solutions**:
- Move breakpoint to executable line
- Verify file paths
- Check source availability

### WF102 - Conditional Breakpoint Error

**Description**: Error evaluating conditional breakpoint expression.
**Common Causes**:

- Syntax error in condition
- Undefined variables in condition
- Runtime error in evaluation
**Solutions**:
- Check condition syntax
- Verify variable existence
- Test expression in debug console

### WF103 - Logpoint Evaluation Error

**Description**: Error evaluating logpoint message expression.
**Common Causes**:

- Invalid interpolation syntax
- Undefined variables in message
- Complex expression errors
**Solutions**:
- Verify interpolation syntax
- Check variable availability
- Simplify expressions

## Variable Inspection Errors (WF200-WF299)

### WF201 - Variable Not Available

**Description**: Requested variable is not accessible in current scope.
**Common Causes**:

- Variable out of scope
- Optimized away by compiler
- Not yet initialized
**Solutions**:
- Check current stack frame
- Verify execution point
- Confirm variable initialization

### WF202 - Property Not Found

**Description**: Requested property does not exist on the object.
**Common Causes**:

- Typo in property name
- Property not yet defined
- Nil table access
**Solutions**:
- Verify property name spelling
- Check object initialization
- Handle nil values appropriately

### WF203 - Index Out of Bounds

**Description**: Attempted to access array index that doesn't exist.
**Common Cases**:

- Negative indices (in some contexts)
- Indices beyond array length
- Non-integer indices for arrays
**Solutions**:
- Validate index bounds
- Check array length first
- Use proper indexing conventions

## Expression Evaluation Errors (WF300-WF399)

### WF301 - Syntax Error

**Description**: Expression contains syntax errors.
**Common Causes**:

- Missing parentheses or brackets
- Invalid operators
- Malformed function calls
**Solutions**:
- Check syntax carefully
- Validate parentheses matching
- Verify operator usage

### WF302 - Runtime Error

**Description**: Expression evaluation caused a runtime error.
**Common Causes**:

- Division by zero
- Type mismatches
- Calling nil values
**Solutions**:
- Add error checking
- Validate types before operations
- Handle nil values explicitly

### WF303 - Permission Denied

**Description**: Attempted to modify read-only context.
**Common Causes**:

- Safe evaluation mode enabled
- Attempting to modify globals in restricted context
- Security policies preventing changes
**Solutions**:
- Enable mutation in configuration
- Check evaluation context
- Review security settings

## Source Map Errors (WF400-WF499)

### WF401 - Source Map Not Found

**Description**: Required source map file is missing.
**Common Causes**:

- Source map not generated
- Incorrect file paths
- File permission issues
**Solutions**:
- Enable source map generation
- Verify file paths
- Check file permissions

### WF402 - Invalid Source Map Format

**Description**: Source map file is malformed or incompatible.
**Common Causes**:

- Corrupted source map
- Unsupported version
- Encoding issues
**Solutions**:
- Regenerate source map
- Check source map version
- Verify file encoding

### WF403 - Position Mapping Failed

**Description**: Unable to map position between source and generated code.
**Common Causes**:

- Incomplete source map
- Position outside mapped range
- Mapping conflicts
**Solutions**:
- Validate source map completeness
- Check position ranges
- Resolve mapping conflicts

## Hot Reload Errors (WF500-WF599)

### WF501 - Module Not Found

**Description**: Specified module cannot be located for reload.
**Common Causes**:

- Incorrect module name
- Module not loaded
- Path resolution issues
**Solutions**:
- Verify module name
- Check module loading
- Review path configuration

### WF502 - State Preservation Failed

**Description**: Unable to preserve module state during reload.
**Common Causes**:

- Complex state structures
- Circular references
- Registry conflicts
**Solutions**:
- Simplify state structures
- Break circular references
- Resolve registry conflicts

### WF503 - Reload Conflict

**Description**: Reload operation conflicts with current execution.
**Common Causes**:

- Active function references
- Pending callbacks
- Thread interference
**Solutions**:
- Wait for safe reload point
- Cancel pending operations
- Synchronize threads

## Configuration Errors (WF600-WF699)

### WF601 - Invalid Configuration

**Description**: Configuration file contains errors.
**Common Causes**:

- YAML syntax errors
- Invalid option values
- Missing required options
**Solutions**:
- Validate YAML syntax
- Check option values
- Provide required options

### WF602 - Configuration Conflict

**Description**: Conflicting configuration options detected.
**Common Causes**:

- Mutually exclusive options
- Override conflicts
- Environment mismatches
**Solutions**:
- Resolve option conflicts
- Review override hierarchy
- Match environment settings

## Runtime Errors (WF700-WF799)

### WF701 - Lua Error

**Description**: Error occurred in the Lua runtime.
**Common Causes**:

- Lua syntax errors
- Runtime exceptions
- Library loading failures
**Solutions**:
- Fix Lua syntax
- Handle exceptions appropriately
- Verify library availability

### WF702 - Memory Error

**Description**: Insufficient memory or allocation failure.
**Common Causes**:

- Memory leaks
- Large data structures
- System resource limits
**Solutions**:
- Fix memory leaks
- Optimize data structures
- Increase system resources

### WF703 - Thread Error

**Description**: Error in coroutine or threading operations.
**Common Causes**:

- Invalid thread operations
- Deadlock conditions
- Resource contention
**Solutions**:
- Validate thread operations
- Prevent deadlocks
- Manage resource access

## Network Errors (WF800-WF899)

### WF801 - Connection Failed

**Description**: Unable to establish network connection.
**Common Causes**:

- Host unreachable
- Port blocked
- Network configuration issues
**Solutions**:
- Check host accessibility
- Verify port availability
- Review network settings

### WF802 - Connection Lost

**Description**: Established connection was terminated unexpectedly.
**Common Causes**:

- Network interruption
- Process termination
- Timeout exceeded
**Solutions**:
- Check network stability
- Verify process status
- Increase timeout values

## System Errors (WF900-WF999)

### WF901 - File System Error

**Description**: Error accessing or manipulating files.
**Common Causes**:

- Permission denied
- Disk full
- File locked
**Solutions**:
- Check file permissions
- Free disk space
- Unlock files

### WF902 - System Resource Error

**Description**: Insufficient system resources.
**Common Causes**:

- Low memory
- Process limits exceeded
- File descriptor exhaustion
**Solutions**:
- Free system memory
- Adjust process limits
- Close unnecessary files

### WF903 - Internal Error

**Description**: Unexpected internal error in Wayfinder.
**Common Causes**:

- Software bugs
- Inconsistent state
- Unhandled edge cases
**Solutions**:
- Report bug to developers
- Restart Wayfinder
- Check for updates

## Troubleshooting Tips

### Reading Error Messages

Wayfinder error messages follow this format:

```
[WFXXX] Brief description: Detailed explanation
Context information (if available)
```

### Error Resolution Strategy

1. **Identify the error code**: Note the WFXXX code
2. **Consult this reference**: Find the specific error description
3. **Check common causes**: Review typical reasons for the error
4. **Apply suggested solutions**: Try the recommended fixes
5. **Seek additional help**: Use logs and context for deeper analysis

### Logging and Diagnostics

Enable detailed logging for troubleshooting:

```bash
WAYFINDER_LOG_LEVEL=debug wayfinder launch script.lua
```

### Reporting Issues

When reporting bugs, include:

- Error code and message
- Wayfinder version
- Lua version
- Operating system
- Steps to reproduce
- Relevant log output

## Next Steps

- Learn about [CLI commands](cli-commands.md)
- Explore [configuration options](configuration.md)
- Understand [IDE integration](../guides/ide-integration.md)
