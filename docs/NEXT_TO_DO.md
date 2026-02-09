# Implementation Plan: Advanced Debugging Features for Wayfinder

## Context

Wayfinder is a Lua debugger with comprehensive DAP (Debug Adapter Protocol) support. The codebase currently has:

- Module-level hot reload with state capture/restore
- Extensive debugging capabilities (breakpoints, stepping, variable inspection)
- Strong infrastructure using Lua debug hooks and FFI
- Support for PUC Lua 5.1-5.4 and LuaNext runtimes

Three features are marked as TODO items that would significantly enhance the debugging experience:

1. **Function-level hot reload** - Currently, hot reload works at the module level but doesn't update existing function references
2. **Profiling integration** - No profiling capabilities exist despite having the necessary debug hook infrastructure
3. **Memory inspection** - Limited memory introspection beyond basic variable viewing

This plan details the implementation approach for all three features, prioritized by complexity and user value.

---

## Feature 1: Profiling Integration

### Overview

Add built-in profiling capabilities using existing debug hook infrastructure with minimal external dependencies.

### Technical Approach

**Recommended**: Hybrid Built-in + External Profiler Support

**Three Profiling Modes**:

1. **Sampling** - Low overhead (1-8%), statistical profiling via `LUA_MASKCOUNT`
2. **Call Trace** - Medium overhead (10-25%), full call/return tracking
3. **Line Level** - High overhead (50-200%), detailed line-by-line profiling

### Architecture Changes

**New Module**: `/crates/wayfinder-core/src/profiling/mod.rs`

Core types:

```rust
struct Profiler {
    data: ProfileData,
    enabled: bool,
    mode: ProfilingMode,
}

struct FunctionProfile {
    name: String,
    source: Option<String>,
    line_defined: u32,
    call_count: u64,
    total_time: Duration,
    self_time: Duration,
    children: HashMap<String, u64>,
}

enum ProfilingMode {
    Disabled,
    Sampling { interval_ms: u32 },
    CallTrace,
    LineLevel,
}
```

**Hook Integration** (`puc_lua.rs`):

Extend existing `lua_hook_callback` (lines 55-109):

- Add profiler state to static globals
- Handle `LUA_HOOKCALL`, `LUA_HOOKRET`, `LUA_HOOKCOUNT` events
- Use thread-local storage for lock-free data collection
- Track call stacks and timing

### DAP Protocol Extensions

New custom requests:

- `profiling/start` - Start profiling with mode (sampling/call/line)
- `profiling/stop` - Stop profiling, return summary
- `profiling/snapshot` - Get current profile data without stopping
- `profiling/export` - Export to JSON/flamegraph/callgrind format

Handler location: `/crates/wayfinder-core/src/session/mod.rs` (around line 413)

### Data Structures

**Profile Results**:

- Function call counts and durations
- Call graph (parent-child relationships)
- Flame graph data for visualization
- Top N hottest functions

**Export Formats**:

- JSON (native format)
- Flamegraph (for speedscope.app)
- Callgrind (for KCachegrind/QCachegrind)

### Performance Impact

| Mode | Hook Mask | Overhead | Use Case |
|------|-----------|----------|----------|
| Sampling (100ms) | COUNT | 1-3% | Long-running apps |
| Sampling (10ms) | COUNT | 3-8% | Moderate detail |
| Call Trace | CALL+RET | 10-25% | Function analysis |
| Line Level | CALL+RET+LINE | 50-200% | Detailed profiling |

### Implementation Phases

**Week 1**: Foundation

- Create profiling module structure
- Implement core data types
- Add profiler state management

**Week 2**: Hook Integration

- Extend `lua_hook_callback` for profiling
- Implement call/return tracking
- Add timing measurement

**Week 3**: DAP Protocol

- Add custom request handlers
- Implement start/stop/snapshot commands
- Create export functionality

**Week 4**: Testing & Polish

- Unit tests for profiling logic
- Integration tests for DAP protocol
- Performance benchmarks
- Documentation

### Critical Files

- `/crates/wayfinder-core/src/profiling/mod.rs` - NEW FILE
- `/crates/wayfinder-core/src/runtime/puc_lua.rs` - Hook integration (lines 55-109)
- `/crates/wayfinder-core/src/session/mod.rs` - DAP handlers (line 413)
- `/crates/wayfinder-core/src/runtime/mod.rs` - DebugRuntime trait extension (line 210)
- `/crates/wayfinder-core/tests/profiling_tests.rs` - NEW FILE

---

## Feature 2: Memory Inspection

### Overview

Provide comprehensive memory introspection beyond basic variable viewing, including heap statistics, object enumeration, and leak detection.

### Technical Approach

**Phased Implementation**:

**Phase 1 (MVP)**: Memory Statistics

- Heap size tracking using `lua_gc(L, LUA_GCCOUNT, 0)`
- GC status monitoring
- Force GC capability
- **Effort**: 2-3 days

**Phase 2**: Object Enumeration & Snapshots

- Count objects by type (tables, functions, userdata)
- Heap snapshots (capture memory state at a point in time)
- Registry inspection for C objects
- **Effort**: 3-4 days

**Phase 3 (Advanced)**: Memory Diffing & Leak Detection

- Compare two snapshots
- Identify new/deleted objects
- Memory trend analysis
- **Effort**: 4-5 days

### Architecture Changes

**New Module**: `/crates/wayfinder-core/src/memory/`

Structure:

```
memory/
├── mod.rs           # Public API
├── statistics.rs    # Memory stats collection
├── snapshot.rs      # Heap snapshots
└── registry.rs      # Registry inspection
```

Core types:

```rust
struct MemoryStatistics {
    total_kb: f64,
    total_bytes: usize,
    gc_pause: i32,
    gc_step_mul: i32,
    gc_running: bool,
    timestamp: SystemTime,
}

struct ObjectCounts {
    tables: usize,
    functions: usize,
    userdata: usize,
    threads: usize,
    strings: usize,
}

struct HeapSnapshot {
    id: u64,
    timestamp: SystemTime,
    statistics: MemoryStatistics,
    object_counts: ObjectCounts,
    objects: Vec<ObjectInfo>,
}

struct SnapshotDiff {
    from_id: u64,
    to_id: u64,
    memory_delta_kb: f64,
    object_count_deltas: HashMap<String, i64>,
    new_objects: Vec<ObjectInfo>,
    deleted_objects: Vec<ObjectInfo>,
}
```

### Integration with Existing Code

**Leverage State Capture Infrastructure**:

- Reuse `StateCapture` from `hot_reload/state_capture.rs` for object graph traversal
- Reuse circular reference detection
- Follow same patterns as variable inspection in `puc_lua.rs:600-816`

**FFI Enhancements**:
Add GC constants to `lua_ffi.rs`:

```rust
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCISRUNNING: c_int = 9;
// etc.
```

### DAP Protocol Extensions

New custom requests:

- `memoryStatistics` - Get current memory usage
- `forceGC` - Trigger garbage collection
- `takeHeapSnapshot` - Capture memory state
- `getHeapSnapshot` - Retrieve snapshot by ID
- `listHeapSnapshots` - List all snapshots
- `diffSnapshots` - Compare two snapshots
- `inspectRegistry` - View registry contents

### Performance Considerations

**GC Safety**:

- Temporarily stop GC during snapshot: `lua_gc(L, LUA_GCSTOP, 0)`
- Restart after: `lua_gc(L, LUA_GCRESTART, 0)`
- Use registry references for stability

**Performance Mitigation**:

- Lazy loading (only enumerate on explicit request)
- Hard limits on object enumeration (max 10,000 objects)
- Sampling for large heaps (10% sample rate option)
- Progressive disclosure (summary first, details on demand)
- Background processing where safe

### Implementation Phases

**Week 1**: Foundation (MVP)

- Add GC constants to FFI
- Implement `MemoryStatistics` and `MemoryCollector`
- Extend `DebugRuntime` trait
- Add DAP protocol handlers
- Basic tests

**Week 2**: Object Enumeration & Snapshots

- Implement `ObjectEnumerator`
- Create `SnapshotManager`
- Integrate with `StateCapture`
- Snapshot DAP handlers

**Week 3**: Registry & Diffing

- Implement `RegistryInspector`
- Create snapshot diffing logic
- Add diff DAP handlers
- Comprehensive tests

**Week 4** (Optional): Advanced Features

- Memory pressure monitoring
- Allocation profiling (if feasible)
- Performance optimization
- Documentation

### Critical Files

- `/crates/wayfinder-core/src/runtime/lua_ffi.rs` - Add GC constants
- `/crates/wayfinder-core/src/runtime/mod.rs` - Extend DebugRuntime trait
- `/crates/wayfinder-core/src/runtime/puc_lua.rs` - Implementation
- `/crates/wayfinder-core/src/hot_reload/state_capture.rs` - Leverage for snapshots
- `/crates/wayfinder-core/src/session/mod.rs` - DAP handlers
- `/crates/wayfinder-core/src/memory/` - NEW MODULE

---

## Implementation Scope

**Selected Features** (based on risk/value assessment):

- ✅ Memory Inspection (low risk, high value)
- ✅ Profiling Integration (medium risk, high value)
- ❌ Function-Level Hot Reload (deferred - high complexity, edge case risks)

## Implementation Order

### Phase 1: Memory Inspection MVP (Week 1)

**Deliverable**: Basic memory statistics and GC control
**Timeline**: 2-3 days
**User Impact**: Immediate visibility into memory usage
**Risk**: Very low - uses standard Lua GC APIs

### Phase 2: Profiling Integration (Weeks 2-3)

**Deliverable**: Sampling and call-trace profiling with flame graphs
**Timeline**: 2-3 weeks
**User Impact**: Identify performance bottlenecks
**Risk**: Low-medium - well-understood debug hook approach

### Phase 3: Memory Inspection Advanced (Week 4)

**Deliverable**: Heap snapshots, object enumeration, snapshot diffing
**Timeline**: 1 week
**User Impact**: Find memory leaks and optimize usage
**Risk**: Low - leverages existing state capture infrastructure

### Phase 4: Testing & Polish (Week 5)

**Deliverable**: Comprehensive tests, documentation, IDE integration examples
**Timeline**: 1 week
**User Impact**: Production-ready features
**Risk**: Very low

---

## Testing Strategy

### Unit Tests

- Each module gets comprehensive unit tests
- Test normal cases, edge cases, error conditions
- Mock Lua state where possible

### Integration Tests

- DAP protocol end-to-end tests
- Test all custom requests
- Verify correct responses and error handling

### Performance Benchmarks

- Measure overhead of each feature
- Compare with/without features enabled
- Benchmark large-scale scenarios (1000+ functions, 10MB+ heap)

### Manual Testing

- IDE integration tests (VSCode, Neovim)
- Real-world application debugging
- User experience validation

---

## Configuration & User Experience

### Configuration File (`wayfinder.yaml`)

```yaml
hotReload:
  functionTracking:
    enabled: true
    updateReferences: true
    migrateUpvalues: true
    strictMatching: false

profiling:
  defaultMode: sampling
  samplingInterval: 10
  autoExport: false

memory:
  maxSnapshotsInMemory: 10
  snapshotDirectory: ~/.wayfinder/snapshots
  objectLimit: 10000
```

### User Workflows

**Performance Profiling**:

1. Start profiling (sampling mode)
2. Run slow code section
3. Stop profiling
4. View flame graph or call tree
5. Export to external tool if needed

**Memory Leak Detection**:

1. Take snapshot #1
2. Perform operation suspected of leaking
3. Take snapshot #2
4. Diff snapshots
5. Identify leaked objects
6. Force GC and verify

---

## Documentation Updates

### Files to Update

- `README.md` - Feature overview
- `docs/TODO.md` - Mark items complete
- `docs-site/src/` - User guides
- `docs/hot-reload.md` - Function-level docs

### New Documentation

- `docs/profiling.md` - Profiling guide
- `docs/memory-inspection.md` - Memory debugging guide
- `docs-site/src/guides/debugging-memory-leaks.md` - Tutorial
- `docs-site/src/guides/profiling-performance.md` - Tutorial

---

## Verification

After implementation, verify with:

### Profiling Integration

- [ ] Start sampling profiling, run code, stop profiling
- [ ] Verify profile data shows function call counts
- [ ] Export to flamegraph format and open in speedscope.app
- [ ] Compare overhead vs non-profiled execution
- [ ] Test call trace mode on recursive function

### Memory Inspection

- [ ] Get memory statistics during debugging
- [ ] Take heap snapshot, verify object counts
- [ ] Create intentional leak, take two snapshots
- [ ] Diff snapshots, verify leak detected
- [ ] Force GC, verify memory decreases
- [ ] Inspect registry for C objects

---

## Risk Mitigation

### Profiling Risks

- **Risk**: Excessive overhead impacting debugging experience
- **Mitigation**: Multiple modes with different overhead, default to low-overhead sampling

### Memory Inspection Risks

- **Risk**: Large heaps causing UI freezes or memory pressure
- **Mitigation**: Hard limits, sampling, progressive loading, background processing

---

## Success Criteria

### Profiling Integration

- ✅ Sampling mode <5% overhead
- ✅ Accurate call counts and timing
- ✅ Flamegraph export works with standard tools
- ✅ Works seamlessly during debugging

### Memory Inspection

- ✅ Accurate memory statistics
- ✅ Snapshot/diff workflow finds leaks
- ✅ <2 seconds for snapshot on 10MB heap
- ✅ Registry inspection reveals C objects

---

## Total Estimated Timeline

**Selected Scope**:

- **Memory Inspection (MVP)**: 2-3 days
- **Profiling Integration**: 2-3 weeks
- **Memory Inspection (Full)**: 1 week
- **Testing & Polish**: 1 week

**Total**: ~5-6 weeks for both features complete

**Deferred**:

- Function-Level Hot Reload: Postponed due to complexity and edge case risks. Can be revisited after proving out the architecture with lower-risk features.

## Future Considerations

### Function-Level Hot Reload (Deferred)

While technically feasible, this feature has been deferred because:

- **High Complexity**: 6-7 weeks of development time
- **Edge Case Risks**: Event handlers, coroutines, FFI callbacks require special handling
- **Uncertain ROI**: Module-level hot reload may be sufficient for most use cases

**Recommendation**: Gather user feedback on memory inspection and profiling features first. If users frequently request more granular hot reload, revisit this feature with a focused prototype to validate edge case handling.

**Alternative Approaches**:

1. Start with limited scope (only update functions in module table, document limitations)
2. Build incrementally based on user feedback
3. Consider opt-in beta feature flag for early adopters
