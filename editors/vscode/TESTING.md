# Wayfinder VSCode Extension - Testing Guide

## Setup for Testing

### Prerequisites
- VSCode 1.75.0 or later
- Node.js 16+
- Wayfinder CLI (`wayfinder` command available in PATH)
- Lua runtimes installed (lua5.4 minimum, others optional)

### Installation
```bash
cd editors/vscode
npm install
npm run esbuild
```

### Loading the Extension in Development

1. **Launch Extension Development Host**
   - Open VSCode in the wayfinder root directory
   - Press `F5` to launch the Extension Development Host (requires `.vscode/launch.json`)
   - Or: Run `code --extensionDevelopmentPath=./editors/vscode`

2. **Verify Extension Loads**
   - Check if "Wayfinder Lua Debugger is ready!" message appears in the Extension Development Host
   - Check VSCode Output panel (View > Output > Wayfinder)

## Manual Test Cases

### Test 1: Extension Activation
**Objective**: Verify extension loads without errors

**Steps:**
1. Launch Extension Development Host: `code --extensionDevelopmentPath=./editors/vscode`
2. Open any `.lua` or `.luax` file
3. Check the notification area

**Expected Result:**
- No error notifications
- "Wayfinder Lua Debugger is ready!" message appears
- Extension commands appear in command palette

**Verification:**
```bash
# In Extension Development Host, press Ctrl+Shift+P and search "Wayfinder"
# Should see: Debug File, Select Runtime, Attach to Process
```

### Test 2: Debug File Command
**Objective**: Verify "Debug File" command works

**Setup:**
1. Create test script: `/tmp/test.lua`
   ```lua
   print("Hello from Lua!")
   local x = 42
   print("x =", x)
   ```

2. Open the file in VSCode

**Steps:**
1. Press `Ctrl+Shift+P`
2. Type "Debug File"
3. Press Enter

**Expected Result:**
- Debug session starts
- Execution pauses at first line (if `stopOnEntry: true` is set)
- Debug sidebar shows variables and call stack
- Can set breakpoints and continue

### Test 3: Breakpoint Setting
**Objective**: Verify breakpoints work

**Setup:**
1. Create test script: `/tmp/breakpoint_test.lua`
   ```lua
   local function add(a, b)
     return a + b
   end

   print("Result:", add(5, 3))
   ```

**Steps:**
1. Open file in VSCode
2. Click on line number 5 to set breakpoint
3. Press F5 to debug
4. When breakpoint is hit, verify:
   - Execution pauses
   - Yellow highlight shows current line
   - Variables panel shows `a` and `b` values

**Expected Result:**
- Breakpoint is honored
- Execution pauses at the correct line
- Variables are accessible

### Test 4: Runtime Auto-Detection
**Objective**: Verify correct runtime is selected based on file type

**Steps:**
1. Create `.lua` file and debug it
2. Verify it uses `lua54` runtime (default)
3. Create `.luax` file and debug it
4. Verify it uses `luanext` runtime

**Expected Result:**
- `.lua` files default to lua54
- `.luax` files default to luanext

### Test 5: Manual Runtime Selection
**Objective**: Verify runtime can be manually selected

**Setup:**
1. Create launch configuration in `.vscode/launch.json`:
   ```json
   {
     "type": "wayfinder",
     "request": "launch",
     "name": "Launch with Lua 5.3",
     "program": "${workspaceFolder}/test.lua",
     "runtime": "lua53"
   }
   ```

**Steps:**
1. Press F5 to show debug configurations
2. Select "Launch with Lua 5.3"
3. Verify the debug session starts

**Expected Result:**
- Debug session uses Lua 5.3
- No errors in output

### Test 6: Step Control
**Objective**: Verify step over, step into, step out work

**Setup:**
1. Create test script:
   ```lua
   local function foo()
     print("In foo")
   end

   foo()
   print("Done")
   ```

**Steps:**
1. Set breakpoint at `foo()` call
2. Start debugging
3. Press F10 (Step Over) - should skip into foo
4. Press F11 (Step Into) - should go into function
5. Press Shift+F11 (Step Out) - should return

**Expected Result:**
- Step controls work as expected
- Execution follows the correct path
- Variables update correctly

### Test 7: Variable Inspection
**Objective**: Verify variable values are displayed correctly

**Setup:**
1. Create test script:
   ```lua
   local name = "Wayfinder"
   local count = 42
   local items = {1, 2, 3}

   print(name, count)
   ```

**Steps:**
1. Set breakpoint at print line
2. Start debugging
3. Inspect variables in sidebar

**Expected Result:**
- Variables panel shows all local variables
- Values are correct (name="Wayfinder", count=42, items={1,2,3})
- Can expand tables to see contents

### Test 8: Command: Select Runtime
**Objective**: Verify "Select Runtime" command shows available runtimes

**Steps:**
1. Press `Ctrl+Shift+P`
2. Type "Select Runtime"
3. Select a runtime from the list

**Expected Result:**
- QuickPick shows all available runtimes
- Selection works without error
- Runtime paths shown in descriptions

### Test 9: Command: Attach to Process
**Objective**: Verify "Attach to Process" command

**Setup:**
1. Have a Lua debug server running on port 5858

**Steps:**
1. Press `Ctrl+Shift+P`
2. Type "Attach to Process"
3. Enter port: 5858
4. Enter host: localhost

**Expected Result:**
- Attach configuration created
- Debugging session starts
- Connected to running process

### Test 10: Configuration Settings
**Objective**: Verify settings are applied correctly

**Setup:**
1. Open VSCode settings (Ctrl+,)
2. Search for "wayfinder"

**Steps:**
1. Change `wayfinder.debug.port` to 6000
2. Create new debug session
3. Verify it uses port 6000

**Expected Result:**
- Settings are applied
- Custom port is used
- No errors

### Test 11: Context Menu Integration
**Objective**: Verify right-click context menu works

**Steps:**
1. Open a `.lua` or `.luax` file
2. Right-click in the editor
3. Look for "Debug File" option

**Expected Result:**
- Context menu appears
- "Debug File" option is available for .lua/.luax files
- Hidden for other file types

### Test 12: Multiple Debug Sessions
**Objective**: Verify multiple debug sessions can run simultaneously

**Setup:**
1. Create two test scripts

**Steps:**
1. Start debugging first script with F5
2. While first is running, open second script
3. Start second debug session
4. Verify both sessions are visible in debug sidebar

**Expected Result:**
- Multiple sessions can run
- Each has its own port (auto-incremented)
- Can switch between sessions
- No conflicts or errors

### Test 13: Debug Console Evaluation
**Objective**: Verify debug console works

**Setup:**
1. Create and debug test script with breakpoint hit

**Steps:**
1. Open Debug Console (View > Debug Console)
2. Type Lua expression: `2 + 2`
3. Press Enter

**Expected Result:**
- Expression is evaluated
- Result appears in console
- Works with variables in scope

### Test 14: Error Handling - Missing Binary
**Objective**: Verify graceful handling when Wayfinder binary not found

**Setup:**
1. Temporarily rename wayfinder binary or remove from PATH

**Steps:**
1. Try to start debug session
2. Check error message

**Expected Result:**
- Clear error message about missing binary
- Suggests configuration option
- Doesn't crash VSCode

### Test 15: Error Handling - Invalid Script Path
**Objective**: Verify error when script file doesn't exist

**Setup:**
1. Create launch config with non-existent file path

**Steps:**
1. Try to start debug session

**Expected Result:**
- Error message indicating file not found
- Debug session doesn't start
- Can correct and retry

## Automated Testing (Future)

### Unit Tests
```bash
npm test
```

Tests should cover:
- Configuration loading and validation
- Runtime detection logic
- Variable substitution
- Port management

### Integration Tests
- Extension activation
- Debug session lifecycle
- Command execution
- Configuration application

## Test Results Template

```markdown
## Test Execution: [Date]

| Test Case | Status | Notes |
|-----------|--------|-------|
| 1. Extension Activation | ✅/❌ | |
| 2. Debug File Command | ✅/❌ | |
| 3. Breakpoint Setting | ✅/❌ | |
| 4. Runtime Auto-Detection | ✅/❌ | |
| 5. Manual Runtime Selection | ✅/❌ | |
| 6. Step Control | ✅/❌ | |
| 7. Variable Inspection | ✅/❌ | |
| 8. Select Runtime Command | ✅/❌ | |
| 9. Attach to Process | ✅/❌ | |
| 10. Configuration Settings | ✅/❌ | |
| 11. Context Menu | ✅/❌ | |
| 12. Multiple Sessions | ✅/❌ | |
| 13. Debug Console | ✅/❌ | |
| 14. Error: Missing Binary | ✅/❌ | |
| 15. Error: Invalid Path | ✅/❌ | |

**Overall Status**: ✅ All Passed / ❌ Some Failed / ⚠️ Needs Review

**Environment:**
- VSCode Version:
- Node Version:
- Platform:
- Wayfinder Version:

**Issues Found:**
- (List any issues)

**Notes:**
- (Any additional notes)
```

## Debugging the Extension Itself

### Enable Debug Logging
1. In Extension Development Host, press Ctrl+Shift+P
2. Type "Developer: Toggle Developer Tools"
3. Watch Console tab for extension logs

### Common Issues

**Issue: Extension doesn't load**
- Check VSCode console for error messages
- Verify `npm install` completed successfully
- Try `npm run esbuild` to rebuild

**Issue: Commands don't appear**
- Restart Extension Development Host (Ctrl+Shift+F5)
- Check package.json for command registration

**Issue: Debug session fails to start**
- Check Wayfinder binary is in PATH: `which wayfinder`
- Check configuration settings in VSCode
- Review Debug Console output for errors

**Issue: Runtime not found**
- Verify Lua binary is installed: `which lua5.4`
- Update runtime path in settings
- Check configuration settings

## Performance Testing

### Startup Time
Measure extension activation time:
```bash
time code --extensionDevelopmentPath=./editors/vscode .
```

Expected: < 2 seconds

### Debug Session Startup
Measure time from F5 to first breakpoint:
- Expected: < 1 second

### Memory Usage
Monitor VSCode memory with extensions panel:
- Should not increase significantly during debugging

## Platform Testing

Test on all supported platforms:
- [ ] macOS (Intel/Apple Silicon)
- [ ] Linux (Ubuntu, Fedora, etc.)
- [ ] Windows (WSL2, native)

## Compatibility Testing

### VSCode Versions
- [ ] VSCode 1.75.0 (minimum)
- [ ] VSCode latest stable
- [ ] VSCode Insiders

### Lua Versions
- [ ] Lua 5.1
- [ ] Lua 5.2
- [ ] Lua 5.3
- [ ] Lua 5.4
- [ ] LuaNext

## Next Steps

1. Implement automated tests
2. Set up CI/CD pipeline
3. Create test fixtures and examples
4. Document known limitations
5. Prepare for VSCode Marketplace
