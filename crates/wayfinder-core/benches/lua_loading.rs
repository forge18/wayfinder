//! Benchmarks for Lua library loading and execution across versions
//!
//! These benchmarks measure:
//! 1. Library loading time (dynamic mode)
//! 2. State creation overhead
//! 3. Script execution performance
//! 4. Static vs dynamic mode comparison
//! 5. Version-specific performance differences

use std::time::Instant;

#[cfg(feature = "dynamic-lua")]
use wayfinder_core::runtime::lua_loader::LuaLibrary;
use wayfinder_core::runtime::LuaVersion;

/// Benchmark result for a single run
#[derive(Debug, Clone)]
struct BenchResult {
    name: String,
    duration_ns: u128,
    iterations: usize,
}

impl BenchResult {
    fn avg_duration_us(&self) -> f64 {
        (self.duration_ns as f64) / (self.iterations as f64 * 1000.0)
    }

    fn print(&self) {
        println!(
            "  {:<40} {:>12.2} μs/iter ({} iterations)",
            self.name,
            self.avg_duration_us(),
            self.iterations
        );
    }
}

/// Run a benchmark with warmup and multiple iterations
fn bench<F>(name: &str, warmup: usize, iterations: usize, mut f: F) -> BenchResult
where
    F: FnMut(),
{
    // Warmup
    for _ in 0..warmup {
        f();
    }

    // Actual benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();

    BenchResult {
        name: name.to_string(),
        duration_ns: duration.as_nanos(),
        iterations,
    }
}

#[cfg(feature = "dynamic-lua")]
fn bench_library_loading() {
    println!("\n=== Library Loading Benchmarks ===\n");

    let versions = [
        (LuaVersion::V51, "Lua 5.1"),
        (LuaVersion::V52, "Lua 5.2"),
        (LuaVersion::V53, "Lua 5.3"),
        (LuaVersion::V54, "Lua 5.4"),
    ];

    for (version, name) in versions {
        let result = bench(
            &format!("Load {}", name),
            5,    // warmup iterations
            100,  // benchmark iterations
            || {
                let _lib = LuaLibrary::load(version).expect("Failed to load library");
            },
        );
        result.print();
    }
}

#[cfg(feature = "dynamic-lua")]
fn bench_state_creation() {
    println!("\n=== State Creation Benchmarks ===\n");

    let versions = [
        (LuaVersion::V51, "Lua 5.1"),
        (LuaVersion::V52, "Lua 5.2"),
        (LuaVersion::V53, "Lua 5.3"),
        (LuaVersion::V54, "Lua 5.4"),
    ];

    for (version, name) in versions {
        if let Ok(lib) = LuaLibrary::load(version) {
            let result = bench(
                &format!("Create state ({})", name),
                10,
                1000,
                || unsafe {
                    let state = lib.lual_newstate();
                    lib.lua_close(state);
                },
            );
            result.print();
        } else {
            println!("  {:<40} SKIPPED (library not available)", format!("Create state ({})", name));
        }
    }
}

#[cfg(feature = "dynamic-lua")]
fn bench_basic_operations() {
    println!("\n=== Basic Operations Benchmarks ===\n");

    let versions = [
        (LuaVersion::V51, "Lua 5.1"),
        (LuaVersion::V52, "Lua 5.2"),
        (LuaVersion::V53, "Lua 5.3"),
        (LuaVersion::V54, "Lua 5.4"),
    ];

    for (version, name) in versions {
        if let Ok(lib) = LuaLibrary::load(version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                // Benchmark: Push/pop number
                let result = bench(
                    &format!("Push/pop number ({})", name),
                    100,
                    10000,
                    || {
                        lib.lua_pushnumber(state, 42.0);
                        lib.lua_tonumber(state, -1);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                // Benchmark: Push/pop string
                let test_str = b"Hello, World!\0";
                let result = bench(
                    &format!("Push/pop string ({})", name),
                    100,
                    10000,
                    || {
                        lib.lua_pushstring(state, test_str.as_ptr() as *const i8);
                        lib.lua_tostring(state, -1);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                // Benchmark: Table operations
                let result = bench(
                    &format!("Create table ({})", name),
                    100,
                    10000,
                    || {
                        lib.lua_createtable(state, 0, 0);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                lib.lua_close(state);
            }
        } else {
            println!("  {:<40} SKIPPED (library not available)", format!("Basic ops ({})", name));
        }
    }
}

#[cfg(feature = "dynamic-lua")]
fn bench_script_execution() {
    println!("\n=== Script Execution Benchmarks ===\n");

    let versions = [
        (LuaVersion::V51, "Lua 5.1"),
        (LuaVersion::V52, "Lua 5.2"),
        (LuaVersion::V53, "Lua 5.3"),
        (LuaVersion::V54, "Lua 5.4"),
    ];

    // Simple script: factorial
    let factorial_script = b"
        local function factorial(n)
            if n <= 1 then return 1 end
            return n * factorial(n - 1)
        end
        return factorial(10)
    \0";

    // Complex script: fibonacci with memoization
    let fib_memo_script = b"
        local cache = {}
        local function fib(n)
            if n <= 1 then return n end
            if cache[n] then return cache[n] end
            local result = fib(n-1) + fib(n-2)
            cache[n] = result
            return result
        end
        return fib(30)
    \0";

    for (version, name) in versions {
        if let Ok(lib) = LuaLibrary::load(version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                // Benchmark: Simple factorial
                let result = bench(
                    &format!("Factorial(10) ({})", name),
                    10,
                    1000,
                    || {
                        lib.lual_loadstring(state, factorial_script.as_ptr() as *const i8);
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                // Benchmark: Fibonacci with memoization
                let result = bench(
                    &format!("Fib(30) memoized ({})", name),
                    5,
                    100,
                    || {
                        lib.lual_loadstring(state, fib_memo_script.as_ptr() as *const i8);
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                lib.lua_close(state);
            }
        } else {
            println!("  {:<40} SKIPPED (library not available)", format!("Scripts ({})", name));
        }
    }
}

#[cfg(feature = "dynamic-lua")]
fn bench_compatibility_shims() {
    println!("\n=== Compatibility Shim Benchmarks ===\n");

    let versions = [
        (LuaVersion::V51, "Lua 5.1"),
        (LuaVersion::V52, "Lua 5.2"),
        (LuaVersion::V53, "Lua 5.3"),
        (LuaVersion::V54, "Lua 5.4"),
    ];

    for (version, name) in versions {
        if let Ok(lib) = LuaLibrary::load(version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                // Benchmark: lua_pushglobaltable (native in 5.2+, shimmed in 5.1)
                let result = bench(
                    &format!("lua_pushglobaltable ({})", name),
                    100,
                    10000,
                    || {
                        lib.lua_pushglobaltable(state);
                        lib.lua_settop(state, 0);
                    },
                );
                result.print();

                // Benchmark: lua_pcall (native in 5.1, shimmed in 5.2+)
                let simple_script = b"return 2 + 2\0";
                lib.lual_loadstring(state, simple_script.as_ptr() as *const i8);
                let result = bench(
                    &format!("lua_pcall ({})", name),
                    100,
                    10000,
                    || {
                        lib.lua_pushvalue(state, -1); // Duplicate the function
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, -2); // Remove result, keep function
                    },
                );
                result.print();

                lib.lua_close(state);
            }
        } else {
            println!("  {:<40} SKIPPED (library not available)", format!("Shims ({})", name));
        }
    }
}

#[cfg(feature = "static-lua")]
fn bench_static_mode() {
    println!("\n=== Static Mode Benchmarks (Lua 5.4) ===\n");

    use wayfinder_core::runtime::lua_ffi::*;

    unsafe {
        // Benchmark: State creation
        let result = bench("Create state (static)", 10, 1000, || {
            let state = luaL_newstate();
            lua_close(state);
        });
        result.print();

        // Benchmark: Basic operations
        let state = luaL_newstate();
        luaL_openlibs(state);

        let result = bench("Push/pop number (static)", 100, 10000, || {
            lua_pushnumber(state, 42.0);
            lua_tonumber(state, -1);
            lua_settop(state, 0);
        });
        result.print();

        // Benchmark: Script execution
        let script = b"return 2 + 2\0";
        let result = bench("Simple script (static)", 100, 10000, || {
            luaL_loadstring(state, script.as_ptr() as *const i8);
            lua_pcall(state, 0, 1, 0);
            lua_settop(state, 0);
        });
        result.print();

        lua_close(state);
    }
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║        Wayfinder Lua Loading Benchmark Suite              ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    #[cfg(feature = "dynamic-lua")]
    {
        println!("\nRunning in DYNAMIC mode (runtime library loading)\n");

        bench_library_loading();
        bench_state_creation();
        bench_basic_operations();
        bench_script_execution();
        bench_compatibility_shims();
    }

    #[cfg(feature = "static-lua")]
    {
        println!("\nRunning in STATIC mode (compile-time Lua 5.4 linking)\n");

        bench_static_mode();
    }

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    Benchmark Complete                     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}
