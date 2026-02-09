# Wayfinder Lua Loading Benchmarks

This document describes the benchmark suite for Wayfinder's multi-version Lua support and how to interpret the results.

## Overview

Wayfinder includes comprehensive benchmarks to measure:

1. **Library loading overhead** - Time to dynamically load Lua libraries
2. **State creation performance** - Time to create new Lua states
3. **Basic operations** - Push/pop operations for numbers, strings, tables
4. **Script execution** - Performance of actual Lua code execution
5. **Compatibility shims** - Overhead of version-specific compatibility layers
6. **Static vs Dynamic** - Performance comparison between build modes

## Benchmark Types

### 1. Simple Benchmarks (`lua_loading.rs`)

Basic benchmarks with custom timing code. Good for quick sanity checks.

**Run:**
```bash
# Dynamic mode
cargo bench --features dynamic-lua --no-default-features --bench lua_loading

# Static mode
cargo bench --bench lua_loading
```

**Output:**
```
=== Library Loading Benchmarks ===
  Load Lua 5.1                                      1.23 μs/iter (100 iterations)
  Load Lua 5.2                                      1.25 μs/iter (100 iterations)
  ...
```

### 2. Criterion Benchmarks (`lua_loading_criterion.rs`)

Advanced benchmarks using [Criterion](https://github.com/bheisler/criterion.rs) for:
- Statistical analysis (mean, median, std deviation)
- Outlier detection
- Regression detection
- HTML reports with graphs

**Run:**
```bash
# Dynamic mode
cargo bench --features dynamic-lua --no-default-features --bench lua_loading_criterion

# Static mode
cargo bench --bench lua_loading_criterion
```

**View results:**
```bash
# Open HTML reports
open target/criterion/report/index.html
```

## Benchmark Categories

### Library Loading (`bench_library_loading`)

Measures the overhead of dynamically loading Lua libraries at runtime.

**What it tests:**
- `LuaLibrary::load()` for each Lua version (5.1-5.4)
- Symbol resolution time
- Library verification

**Expected results:**
- ~1-2ms per load (cold)
- ~0.1-0.5ms per load (cached)
- Similar performance across all versions

**Sample output:**
```
library_loading/load/5.1  time: [1.234 ms 1.245 ms 1.256 ms]
library_loading/load/5.2  time: [1.240 ms 1.251 ms 1.262 ms]
library_loading/load/5.3  time: [1.238 ms 1.249 ms 1.260 ms]
library_loading/load/5.4  time: [1.242 ms 1.253 ms 1.264 ms]
```

### State Creation (`bench_state_creation`)

Measures Lua state (`lua_State*`) creation and destruction.

**What it tests:**
- `luaL_newstate()` + `lua_close()` cycle
- Memory allocation overhead
- Internal Lua initialization

**Expected results:**
- ~5-15μs per state creation
- Minimal version differences
- Static mode slightly faster (no function pointer indirection)

**Sample output:**
```
state_creation/create_state/5.1  time: [8.234 μs 8.345 μs 8.456 μs]
state_creation/create_state/5.2  time: [8.340 μs 8.451 μs 8.562 μs]
```

### Basic Operations (`bench_basic_operations`)

Measures low-level Lua C API operations.

**What it tests:**
- Push/pop numbers: `lua_pushnumber()` + `lua_tonumber()`
- Push/pop strings: `lua_pushstring()` + `lua_tostring()`
- Table creation: `lua_createtable()`

**Expected results:**
- ~50-200ns per push/pop operation
- String operations slower than numbers (heap allocation)
- Table creation ~500-1000ns
- Version 5.3+ may be faster for integers

**Sample output:**
```
push_pop_number/number/5.1  time: [123.45 ns 124.56 ns 125.67 ns]
push_pop_string/string/5.1  time: [234.56 ns 235.67 ns 236.78 ns]
table_operations/create_table/5.1  time: [789.01 ns 790.12 ns 791.23 ns]
```

### Script Execution (`bench_script_execution`)

Measures actual Lua code execution performance.

**What it tests:**
- **Factorial(10)**: Recursive function performance
- **Fibonacci(30) with memoization**: Table operations + recursion
- **Table manipulation**: Loop performance + table access

**Expected results:**
- Factorial(10): ~5-15μs
- Fibonacci(30): ~20-50μs (benefits from memoization)
- Table manipulation: ~10-30μs
- Version 5.3+ may have optimizations for integer operations

**Sample output:**
```
factorial/factorial_10/5.1  time: [8.234 μs 8.345 μs 8.456 μs]
fibonacci/fib_30_memo/5.1   time: [34.567 μs 35.678 μs 36.789 μs]
```

### Compatibility Shims (`bench_compatibility_shims`)

Measures overhead of version-specific compatibility layers.

**What it tests:**
- **`lua_pushglobaltable`**: Native in 5.2+, shimmed in 5.1
- **`lua_pcall`**: Native in 5.1, shimmed in 5.2+

**Expected results:**
- Negligible overhead (~1-5ns)
- Shims should be nearly identical to native implementations
- Lua 5.1 `lua_pushglobaltable` may be slightly slower (uses `lua_rawgeti`)

**Sample output:**
```
compatibility_shims/pushglobaltable/5.1  time: [234.56 ns 235.67 ns 236.78 ns]
compatibility_shims/pushglobaltable/5.2  time: [230.12 ns 231.23 ns 232.34 ns]
compatibility_shims/pcall/5.1            time: [345.67 ns 346.78 ns 347.89 ns]
compatibility_shims/pcall/5.2            time: [348.90 ns 350.01 ns 351.12 ns]
```

### Static vs Dynamic Mode

Compares performance between compile-time static linking and runtime dynamic loading.

**What it tests:**
- Same operations in both modes
- Function call overhead (static: direct, dynamic: function pointer)

**Expected results:**
- Dynamic mode: ~0-2% slower (negligible in practice)
- Startup: Dynamic mode adds 1-2ms for library loading
- Runtime: Virtually identical after library is loaded

**Sample output:**
```
static_mode/create_state     time: [8.123 μs 8.234 μs 8.345 μs]
state_creation/create_state/5.4  time: [8.234 μs 8.345 μs 8.456 μs]
                                        ↑ dynamic mode (0.1μs difference)
```

## Running Benchmarks

### Prerequisites

1. **Build Lua libraries locally:**
   ```bash
   ./scripts/install_lua_versions.sh
   ```

2. **Verify libraries exist:**
   ```bash
   ls -l lua-libs/
   # Should show: liblua5.1.dylib, liblua5.2.dylib, liblua5.3.dylib, liblua5.4.dylib
   ```

### Run All Benchmarks

```bash
# Dynamic mode - all Lua versions
cargo bench --features dynamic-lua --no-default-features

# Static mode - Lua 5.4 only
cargo bench
```

### Run Specific Benchmarks

```bash
# Library loading only
cargo bench --features dynamic-lua --no-default-features -- library_loading

# Script execution only
cargo bench --features dynamic-lua --no-default-features -- script_execution

# Compatibility shims only
cargo bench --features dynamic-lua --no-default-features -- compatibility_shims

# Specific Lua version (via regex)
cargo bench --features dynamic-lua --no-default-features -- '/5.2'
```

### Compare Static vs Dynamic

```bash
# Run static mode
cargo bench --bench lua_loading_criterion > static_results.txt

# Run dynamic mode
cargo bench --features dynamic-lua --no-default-features \
  --bench lua_loading_criterion > dynamic_results.txt

# Compare
diff static_results.txt dynamic_results.txt
```

## Interpreting Results

### Understanding Criterion Output

```
library_loading/load/5.1
                        time:   [1.234 ms 1.245 ms 1.256 ms]
                        ↑ lower ↑ mean  ↑ upper
                        bound          bound
```

- **Lower bound**: 95% confidence interval lower bound
- **Mean**: Average time across iterations
- **Upper bound**: 95% confidence interval upper bound

### Detecting Regressions

Criterion automatically detects performance regressions:

```
library_loading/load/5.1
                        time:   [1.234 ms 1.245 ms 1.256 ms]
                        change: [-5.2% -3.1% -1.0%] (p = 0.00 < 0.05)
                        Performance has improved.
```

- **Positive change**: Performance regression (slower)
- **Negative change**: Performance improvement (faster)
- **p < 0.05**: Statistically significant change

### HTML Reports

Criterion generates detailed HTML reports:

```bash
open target/criterion/report/index.html
```

Reports include:
- Line charts showing performance over time
- Violin plots showing distribution
- Outlier detection
- Comparison with previous runs

## Expected Performance Characteristics

### Lua Version Differences

**5.1 vs 5.2+:**
- 5.1: Older, may be slightly slower on modern systems
- 5.2+: Generally similar performance, some optimizations

**5.3 vs 5.4:**
- 5.3: Integer optimizations (faster integer math)
- 5.4: Minor improvements, constant tables

**Real-world impact:** Differences are typically <5% and workload-dependent.

### Static vs Dynamic Mode

**Static mode advantages:**
- No library loading overhead at startup
- Direct function calls (no pointer indirection)
- Slightly smaller binary size

**Dynamic mode advantages:**
- Single binary supports all Lua versions
- No build-time Lua dependency
- Runtime version selection

**Performance difference:** <2% in practice, negligible for most applications.

## Performance Goals

Based on these benchmarks, Wayfinder aims for:

1. **Library loading**: <2ms per version
2. **State creation**: <20μs per state
3. **Basic operations**: <200ns per operation
4. **Script execution**: Matches native Lua performance (±5%)
5. **Compatibility shims**: <5ns overhead
6. **Static vs dynamic**: <2% difference

## Continuous Monitoring

### In CI/CD

Add benchmark tests to CI:

```yaml
- name: Run benchmarks
  run: |
    ./scripts/install_lua_versions.sh
    cargo bench --features dynamic-lua --no-default-features -- --output-format bencher | tee output.txt
```

### Regression Detection

Use Criterion's baseline feature:

```bash
# Save current performance as baseline
cargo bench --features dynamic-lua --no-default-features -- --save-baseline main

# Compare against baseline after changes
cargo bench --features dynamic-lua --no-default-features -- --baseline main
```

## Troubleshooting

### "Library not available" Messages

**Cause:** Lua libraries not installed locally

**Fix:**
```bash
./scripts/install_lua_versions.sh
ls -l lua-libs/  # Verify libraries exist
```

### Inconsistent Results

**Cause:** System load, thermal throttling, background processes

**Fix:**
- Close unnecessary applications
- Run benchmarks multiple times
- Use `--sample-size 200` for more samples
- Check CPU frequency scaling (disable turbo boost for consistency)

### Benchmark Crashes

**Cause:** Invalid Lua library, ABI mismatch

**Fix:**
1. Rebuild Lua libraries: `./scripts/install_lua_versions.sh`
2. Verify symbols: `nm -D lua-libs/liblua5.4.dylib | grep lua_`
3. Check for library conflicts: `lsof | grep liblua`

## Future Enhancements

- [ ] Memory allocation benchmarks
- [ ] Debugger hook overhead benchmarks
- [ ] Multi-threaded state creation benchmarks
- [ ] Hot reload performance benchmarks
- [ ] Large script compilation benchmarks
- [ ] Compare with other Lua embedders (rlua, mlua)

## References

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Lua Performance Tips](https://www.lua.org/gems/sample.pdf)
- [Lua C API Manual](https://www.lua.org/manual/5.4/manual.html#4)
