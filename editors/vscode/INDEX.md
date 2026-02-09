# Wayfinder VSCode Extension - Documentation Index

Welcome to the Wayfinder VSCode Debug Extension! This index helps you navigate the documentation.

## Quick Links by Role

### 👤 I'm a User
- **Just want to use it?** → Read [QUICK_START.md](./QUICK_START.md)
- **Need detailed help?** → Read [README.md](./README.md)
- **Something not working?** → Check [README.md Troubleshooting](./README.md#troubleshooting)

### 👨‍💻 I'm a Developer
- **New to the project?** → Read [DEVELOPMENT.md](./DEVELOPMENT.md)
- **Want to understand architecture?** → See [DEVELOPMENT.md Architecture](./DEVELOPMENT.md#architecture)
- **Need to add features?** → See [DEVELOPMENT.md Adding New Features](./DEVELOPMENT.md#adding-new-features)
- **Want to debug the extension?** → See [DEVELOPMENT.md Debugging the Extension](./DEVELOPMENT.md#debugging-the-extension)

### 🧪 I'm a Tester
- **Need test cases?** → Read [TESTING.md](./TESTING.md)
- **Want to check features?** → See [TESTING.md Test Cases](./TESTING.md#manual-test-cases)
- **Need performance data?** → See [TESTING.md Performance Testing](./TESTING.md#performance-testing)

### 📊 I'm Project Manager
- **What's been done?** → Read [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md)
- **What features exist?** → See [IMPLEMENTATION_SUMMARY.md Features](./IMPLEMENTATION_SUMMARY.md#features-implemented)
- **What's the timeline?** → See [IMPLEMENTATION_SUMMARY.md Future Phases](./IMPLEMENTATION_SUMMARY.md#future-phases-not-yet-implemented)

## Documentation by Purpose

### Installation & Setup
| Document | Purpose | Audience |
|----------|---------|----------|
| [QUICK_START.md](./QUICK_START.md) | Get started in 5 minutes | Everyone |
| [README.md - Installation](./README.md#installation) | Detailed installation | Users |
| [DEVELOPMENT.md - Setup](./DEVELOPMENT.md#development-setup) | Development setup | Developers |

### Usage & Features
| Document | Purpose | Audience |
|----------|---------|----------|
| [QUICK_START.md - Examples](./QUICK_START.md#example-lua-script) | Quick examples | Users |
| [README.md - Quick Start](./README.md#quick-start) | Multiple debugging methods | Users |
| [README.md - Configuration](./README.md#configuration) | All configuration options | Users |
| [README.md - Commands](./README.md#commands) | Available commands | Users |

### Testing
| Document | Purpose | Audience |
|----------|---------|----------|
| [TESTING.md](./TESTING.md) | 15 test cases with steps | Testers |
| [TESTING.md - Automated Testing](./TESTING.md#automated-testing-future) | Future test automation | Developers |

### Development & Architecture
| Document | Purpose | Audience |
|----------|---------|----------|
| [DEVELOPMENT.md - Structure](./DEVELOPMENT.md#architecture) | Component architecture | Developers |
| [DEVELOPMENT.md - Components](./DEVELOPMENT.md#key-components) | What each file does | Developers |
| [DEVELOPMENT.md - Configuration](./DEVELOPMENT.md#configuration-loading) | How config works | Developers |
| [DEVELOPMENT.md - Debug](./DEVELOPMENT.md#debugging-the-extension) | Debug the extension | Developers |

### Status & Planning
| Document | Purpose | Audience |
|----------|---------|----------|
| [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md) | What's implemented | Everyone |
| [IMPLEMENTATION_SUMMARY.md - Future](./IMPLEMENTATION_SUMMARY.md#future-phases-not-yet-implemented) | What's planned | Managers |

### Examples
| Document | Purpose | Audience |
|----------|---------|----------|
| [examples/.vscode-launch.json](./examples/.vscode-launch.json) | 5 launch configs | Users |
| [examples/.vscode-settings.json](./examples/.vscode-settings.json) | Example settings | Users |

## File Guide

### Source Code (`src/`)
```
extension.ts           Main entry point
├─ Registers providers
├─ Registers commands
└─ Handles lifecycle

configuration.ts       Configuration management
├─ Loads VSCode settings
├─ Detects runtime
├─ Auto-discovers Wayfinder
└─ Verifies runtimes

debug-provider.ts      VSCode DebugConfigurationProvider
├─ Provides configurations
├─ Resolves configurations
├─ Substitutes variables
└─ Validates settings

adapter.ts             DebugAdapterDescriptorFactory
├─ Creates DAP descriptors
├─ Spawns Wayfinder
└─ Manages servers

runtime-manager.ts     Runtime/Process Manager
├─ Manages sessions
├─ Allocates ports
├─ Spawns processes
└─ Tracks metadata

commands.ts            Command Handlers
├─ Debug File command
├─ Select Runtime command
└─ Attach Process command
```

### Configuration Files
```
package.json           Extension manifest & dependencies
tsconfig.json          TypeScript configuration
webpack.config.js      Bundling configuration
.eslintrc.json         Linting rules
.gitignore             Git exclusions
```

### Documentation
```
README.md              Complete user guide (~400 lines)
DEVELOPMENT.md         Developer guide (~500 lines)
TESTING.md             Test plan (~450 lines)
QUICK_START.md         Fast introduction (~250 lines)
IMPLEMENTATION_SUMMARY Feature checklist (~400 lines)
QUICK_START.md         This file
```

## Feature Map

| Feature | File | Docs |
|---------|------|------|
| Launch debugging | adapter.ts, commands.ts | README: Quick Start |
| Attach debugging | debug-provider.ts, adapter.ts | README: Attach |
| Runtime detection | configuration.ts | DEVELOPMENT: Detection |
| Settings | configuration.ts, package.json | README: Configuration |
| Commands | commands.ts | README: Commands |
| Breakpoints | extension.ts (DAP pass-through) | README: Debug Workflow |
| Variables | extension.ts (DAP pass-through) | README: Debug Workflow |

## Common Tasks

### "I want to debug a Lua script"
→ [QUICK_START.md](./QUICK_START.md#first-debug-session)

### "I want to understand the code"
→ [DEVELOPMENT.md - Architecture](./DEVELOPMENT.md#architecture)

### "I want to test the extension"
→ [TESTING.md](./TESTING.md#manual-test-cases)

### "I want to configure the extension"
→ [README.md - Configuration](./README.md#configuration)

### "Something is broken"
→ [README.md - Troubleshooting](./README.md#troubleshooting)

### "I want to add a feature"
→ [DEVELOPMENT.md - Adding Features](./DEVELOPMENT.md#adding-new-features)

### "I want to deploy this"
→ [DEVELOPMENT.md - Release](./DEVELOPMENT.md#release-process)

## Version & Status

- **Version**: 0.1.0
- **Status**: ✅ Phases 1-3 Complete
- **Last Updated**: 2026-02-08
- **Commit**: 6fd6119

## Quick Reference

### Default Settings
- DAP Port: `5858` (auto-increments)
- Default Runtime: `lua54`
- Auto-detect: Enabled
- Source Map Behavior: Ask

### Supported Runtimes
- Lua 5.1 (`lua51`)
- Lua 5.2 (`lua52`)
- Lua 5.3 (`lua53`)
- Lua 5.4 (`lua54`) ← Default
- LuaNext (`luanext`)

### Commands
- `wayfinder.debugFile` - Debug current file
- `wayfinder.selectRuntime` - Choose runtime
- `wayfinder.attachProcess` - Attach to process

### Keyboard Shortcuts
- F5 - Start/continue debugging
- F10 - Step over
- F11 - Step into
- Shift+F11 - Step out
- Ctrl+Shift+D - Debug panel

## Getting Help

1. **Check relevant documentation** (see table above)
2. **Search for keywords** in all .md files
3. **See examples** in examples/ directory
4. **Review TESTING.md** for common scenarios
5. **Check DEVELOPMENT.md** for architecture questions

## Contributing to Documentation

When adding documentation:
1. Choose appropriate file or create new
2. Update this INDEX.md
3. Use clear headings and structure
4. Include examples where helpful
5. Link to related docs

## Next Steps

**I'm Ready to...**
- 🚀 **Get started** → [QUICK_START.md](./QUICK_START.md)
- 📖 **Learn more** → [README.md](./README.md)
- 🏗️ **Understand architecture** → [DEVELOPMENT.md](./DEVELOPMENT.md)
- 🧪 **Test features** → [TESTING.md](./TESTING.md)
- 📊 **Check status** → [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md)

---

**Happy debugging!** 🐛✨
