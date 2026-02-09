# Wayfinder VSCode Extension - Development Guide

## Project Structure

```
editors/vscode/
├── src/
│   ├── extension.ts              # Extension entry point and lifecycle
│   ├── configuration.ts          # Config loading, runtime detection
│   ├── debug-provider.ts         # VSCode debug configuration provider
│   ├── adapter.ts                # Debug adapter descriptor factory
│   ├── runtime-manager.ts        # Process and session management
│   └── commands.ts               # Command handlers
├── out/                          # Compiled output (generated)
├── node_modules/                 # Dependencies (generated)
├── package.json                  # Extension manifest and dependencies
├── tsconfig.json                 # TypeScript configuration
├── webpack.config.js             # Bundling configuration
├── .eslintrc.json                # Linting rules
├── .gitignore                    # Git exclusions
├── examples/                     # Example configurations
├── README.md                      # User documentation
├── DEVELOPMENT.md                # This file
├── TESTING.md                    # Testing guide
├── IMPLEMENTATION_SUMMARY.md     # Implementation status
└── IDE_EXTENSIONS_PLAN.md        # Original plan
```

## Development Setup

### 1. Install Dependencies

```bash
cd editors/vscode
npm install
```

This installs:
- `@vscode/debugadapter` - DAP protocol
- `@vscode/debugprotocol` - DAP type definitions
- TypeScript compiler
- ESLint for linting
- esbuild for bundling

### 2. Build

**Development build** (with source maps):
```bash
npm run esbuild
```

**Watch mode** (auto-rebuild on changes):
```bash
npm run esbuild-watch
```

**TypeScript compilation only**:
```bash
npm run compile
```

**TypeScript watch mode**:
```bash
npm run watch
```

### 3. Linting

```bash
npm run lint
```

Fix auto-fixable issues:
```bash
npm run lint -- --fix
```

### 4. Launch Extension in Development

**Option 1: Using VS Code Task** (requires `.vscode/launch.json`)
- Press F5 to start Extension Development Host

**Option 2: Command Line**
```bash
code --extensionDevelopmentPath=./editors/vscode ../..
```

This opens VSCode with the extension loaded in development mode.

## Architecture

### Extension Lifecycle

```
activate()
  ├── Initialize Configuration
  ├── Initialize RuntimeManager
  ├── Register DebugConfigurationProvider
  ├── Register DebugAdapterDescriptorFactory
  ├── Register Commands
  ├── Register Event Listeners
  └── Show Welcome Message

[User initiates debugging]
  ├── provideDebugConfigurations() or resolveDebugConfiguration()
  ├── createDebugAdapterDescriptor()
  ├── RuntimeManager.startSession()
  ├── Spawn Wayfinder process
  └── Connect VSCode to DAP server

[Debug session active]
  ├── VSCode sends DAP requests to Wayfinder
  ├── Wayfinder responds with debug events
  └── Display in VSCode UI

[Session terminates]
  ├── onDidTerminateDebugSession() event
  ├── RuntimeManager.stopSession()
  └── Cleanup processes

deactivate()
  ├── RuntimeManager.stopAllSessions()
  └── DebugAdapterDescriptorFactory.dispose()
```

### Data Flow

```
User Action (F5, Right-click, Command)
         ↓
Command Handler / Debug Configuration Provider
         ↓
Debug Adapter Descriptor Factory
         ↓
Runtime Manager (starts Wayfinder process)
         ↓
Wayfinder DAP Server (dap-server)
         ↓
Debug events ← → DAP requests
         ↓
VSCode Debug UI
```

### Key Components

#### 1. **extension.ts**
Main entry point for the extension.

**Responsibilities:**
- Register debug configuration provider
- Register debug adapter descriptor factory
- Register commands
- Handle activation/deactivation
- Display welcome message

**Key Methods:**
- `activate(context)` - Called when extension activates
- `deactivate()` - Called when extension deactivates

#### 2. **configuration.ts**
Manages all configuration and runtime detection.

**Responsibilities:**
- Load VSCode settings
- Auto-detect Wayfinder binary
- Detect Lua runtime from file type/config
- Verify runtime availability
- Handle configuration changes

**Key Methods:**
- `getInstance()` - Get singleton instance
- `getConfig()` - Get full configuration
- `detectRuntime(fileUri?)` - Detect appropriate runtime
- `getRuntimePath(runtime)` - Get path to runtime binary
- `verifyRuntimes()` - Check which runtimes are available
- `reload()` - Reload configuration from settings

#### 3. **debug-provider.ts**
Implements `vscode.DebugConfigurationProvider`.

**Responsibilities:**
- Provide default debug configurations
- Resolve dynamic configuration values
- Perform variable substitution
- Auto-detect runtime for launch configs
- Validate configuration

**Key Methods:**
- `provideDebugConfigurations()` - Suggest configurations
- `resolveDebugConfiguration()` - Resolve a configuration

#### 4. **adapter.ts**
Implements `vscode.DebugAdapterDescriptorFactory`.

**Responsibilities:**
- Create debug adapter descriptors
- Start Wayfinder processes
- Return DAP server descriptors
- Handle cleanup

**Key Methods:**
- `createDebugAdapterDescriptor()` - Create descriptor and start process
- `dispose()` - Stop all sessions

#### 5. **runtime-manager.ts**
Manages debug session lifecycle.

**Responsibilities:**
- Track active debug sessions
- Manage port allocation
- Spawn and terminate Wayfinder processes
- Store session metadata

**Key Methods:**
- `getNextPort()` - Get next available port
- `startSession()` - Start a new debug session
- `stopSession(sessionId)` - Stop specific session
- `stopAllSessions()` - Stop all sessions
- `getSession(sessionId)` - Get session info

#### 6. **commands.ts**
Handles user commands.

**Responsibilities:**
- Debug file command (F5, context menu)
- Select runtime command
- Attach to process command
- Verify runtime availability

**Key Methods:**
- `registerCommands()` - Register all commands
- `debugFile(fileUri?)` - Debug current or specified file
- `selectRuntime()` - Runtime selection UI
- `attachProcess()` - Attach to running process

## Configuration Loading

Configuration is loaded from multiple sources with fallback chain:

```
1. VSCode settings (wayfinder.*)
   ↓
2. Environment variables (optional)
   ↓
3. wayfinder.yaml in workspace
   ↓
4. Default values
```

### Configuration Structure

```typescript
interface WayfinderConfig {
  wayfinderPath: string;
  runtimePaths: {
    lua51: string;
    lua52: string;
    lua53: string;
    lua54: string;
    luanext: string;
  };
  defaultPort: number;
  autoDetectRuntime: boolean;
  sourceMapBehavior: 'ask' | 'lenient' | 'strict';
}
```

## Runtime Detection

### Detection Algorithm

1. **File extension check**
   - `.luax` → LuaNext
   - `.lua` → continue to step 2

2. **Workspace configuration**
   - Parse `wayfinder.yaml` for `runtime:` field
   - If found and valid, use it

3. **VSCode settings**
   - Check `wayfinder.debug.*` settings
   - Apply based on file type

4. **Default**
   - Lua 5.4 for `.lua` files
   - LuaNext for `.luax` files

### Example: wayfinder.yaml

```yaml
runtime: lua53
port: 5858
```

## Debug Configuration

### Launch Request

```json
{
  "type": "wayfinder",
  "request": "launch",
  "name": "Launch Script",
  "program": "${workspaceFolder}/main.lua",
  "cwd": "${workspaceFolder}",
  "args": ["arg1", "arg2"],
  "runtime": "lua54",
  "port": 5858,
  "stopOnEntry": false,
  "console": "integratedTerminal"
}
```

**Processing:**
1. `resolveDebugConfiguration()` validates config
2. Variables are substituted
3. Runtime is auto-detected if not specified
4. `createDebugAdapterDescriptor()` starts process
5. Wayfinder spawned with: `wayfinder dap-server --port 5858 --runtime lua54 --script /path/to/main.lua --cwd /workspace --args arg1 arg2`

### Attach Request

```json
{
  "type": "wayfinder",
  "request": "attach",
  "port": 5858,
  "host": "localhost"
}
```

**Processing:**
1. No process spawning needed
2. VSCode connects directly to port 5858
3. Wayfinder must already be running DAP server

## Port Management

### Auto-increment Strategy

When starting multiple debug sessions:

```
Session 1: port 5858
Session 2: port 5859 (incremented)
Session 3: port 5860 (incremented)
...
```

The `RuntimeManager` tracks used ports and increments automatically.

### Configuration

Default port can be changed in settings:

```json
{
  "wayfinder.debug.port": 6000
}
```

New sessions will start from 6000 and increment from there.

## Variable Substitution

Supported variables in debug configurations:

| Variable | Expands To |
|----------|-----------|
| `${workspaceFolder}` | Root workspace directory |
| `${workspaceFolderBasename}` | Workspace folder name |
| `${file}` | Current editor file |
| `${fileDirname}` | Directory of current file |
| `${fileBasename}` | Filename only |
| `${userHome}` | User home directory |

## Error Handling

### Wayfinder Binary Not Found

**Detection:** `execSync('which wayfinder')` fails

**Resolution:**
1. Try default PATH
2. Try `~/.cargo/bin/wayfinder`
3. Fall back to config setting
4. Error message guides user to configure

### Runtime Not Found

**Detection:** Runtime binary not executable

**Resolution:**
1. Check configuration
2. Show verification status
3. Suggest configuration update

### Invalid Configuration

**Detection:** During `resolveDebugConfiguration()`

**Resolution:**
1. Show error message
2. Suggest fix (e.g., program path)
3. Abort debug session

## Adding New Features

### Adding a Command

1. **Define in package.json:**
```json
"commands": [
  {
    "command": "wayfinder.myCommand",
    "title": "My Command",
    "category": "Wayfinder"
  }
]
```

2. **Implement in commands.ts:**
```typescript
private async myCommand(): Promise<void> {
  // Implementation
}
```

3. **Register in extension.ts:**
```typescript
context.subscriptions.push(
  vscode.commands.registerCommand(
    'wayfinder.myCommand',
    commandHandler.myCommand.bind(commandHandler)
  )
);
```

### Adding a Configuration Option

1. **Define in package.json:**
```json
"configuration": {
  "properties": {
    "wayfinder.myOption": {
      "type": "string",
      "default": "value",
      "description": "Description"
    }
  }
}
```

2. **Load in configuration.ts:**
```typescript
const myOption = vscodeConfig.get<string>('myOption') || 'default';
```

3. **Use in extension:**
```typescript
const value = config.getConfig().myOption;
```

## Testing

### Unit Tests
```bash
npm test
```

Should cover:
- Configuration loading
- Runtime detection
- Port management
- Variable substitution

### Integration Tests
- Extension loads correctly
- Commands work as expected
- Debug sessions start/stop properly
- Configuration is applied

### Manual Testing
See [TESTING.md](./TESTING.md)

## Building for Distribution

### Create .vsix Package

```bash
npm run vscode:prepublish  # Minified build
npm run package            # Create .vsix
```

Output: `wayfinder-debugger-0.1.0.vsix`

### Publish to VSCode Marketplace

```bash
npm install -g @vscode/vsce
vsce publish
```

Requires personal access token.

## Debugging the Extension

### Enable Debug Logging

In extension, add:
```typescript
console.log('Debug message');
```

View in Extension Development Host console.

### Set Breakpoints

In Extension Development Host:
1. Press Ctrl+Shift+D (Open Debug)
2. Select "Attach to Extension Host"
3. F5 to start debugging
4. Set breakpoints in TypeScript editor
5. Breakpoints are hit when code executes

### Common Debugging Scenarios

**Debug: "Command not working"**
- Check command registration in activate()
- Verify command in package.json
- Check console for errors

**Debug: "Debug session won't start"**
- Check Wayfinder binary is found
- Verify configuration is correct
- Check RuntimeManager output
- Review Wayfinder startup command

**Debug: "Variables not showing"**
- Verify Wayfinder sends variable data
- Check DAP protocol messages
- Review runtime-manager logs

## Performance Optimization

### Bundle Size

Current bundle: 17.5 KB (gzipped)

To reduce:
- Remove unused dependencies
- Tree-shake unused code
- Minify with esbuild

### Startup Performance

Check extension activation time:
- Load configuration: ~5ms
- Register providers: ~2ms
- Register commands: ~1ms
- Total: <10ms

### Debug Session Performance

- Session start: <1000ms
- Breakpoint hit: <100ms
- Variable fetch: <50ms

## Code Style

### TypeScript Conventions

- Use `const` and `let`, avoid `var`
- Use strict mode (enabled in tsconfig.json)
- Use explicit type annotations for public APIs
- Use interfaces for data structures
- Document public methods with JSDoc

### Formatting

```bash
npm run lint -- --fix
```

Uses ESLint with @typescript-eslint plugins.

## Dependency Management

### Current Dependencies

- `@vscode/debugadapter`: DAP protocol
- `@vscode/debugprotocol`: DAP types

### Updating Dependencies

```bash
npm update
npm audit fix
```

### Adding Dependencies

Only add if:
1. Significantly simplifies code
2. Actively maintained
3. Small size
4. Minimal vulnerabilities

## Release Process

1. **Update version** in package.json
2. **Update CHANGELOG** (if you create one)
3. **Run tests**: `npm test`
4. **Build**: `npm run vscode:prepublish`
5. **Package**: `npm run package`
6. **Test in clean environment**
7. **Publish**: `vsce publish`

## Troubleshooting Development

| Problem | Solution |
|---------|----------|
| Dependencies not installing | Delete `node_modules/`, run `npm install` again |
| TypeScript errors | Run `npm run compile` to see all errors |
| Linting errors | Run `npm run lint -- --fix` |
| Extension doesn't load | Check console, restart Extension Host (Ctrl+Shift+F5) |
| Source maps not working | Rebuild with `npm run esbuild` |
| Commands not registered | Verify package.json and reload |

## Next Development Phases

### Phase 4: Source Maps
- Detect source maps in bundled code
- Translate breakpoints to bundle positions
- Handle multi-file bundles

### Phase 5: Advanced Features
- CodeLens for test blocks
- Source map preference UI
- Bundle debugging workflow

### Phase 6: Neovim Plugin
- Separate Neovim implementation
- nvim-dap adapter
- Telescope integration

## Resources

- [VSCode Extension API](https://code.visualstudio.com/api)
- [Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Wayfinder Docs](../../README.md)
- [LuaNext](https://github.com/forge18/luanext)

## Getting Help

- Check [TESTING.md](./TESTING.md) for test cases
- Review [README.md](./README.md) for user docs
- See [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md) for status
- Check [IDE_EXTENSIONS_PLAN.md](./IDE_EXTENSIONS_PLAN.md) for architecture
