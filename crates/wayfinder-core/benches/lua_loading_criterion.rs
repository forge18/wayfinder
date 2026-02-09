//! Criterion-based benchmarks for Lua loading and execution
//!
//! Provides detailed statistical analysis and HTML reports

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[cfg(feature = "dynamic-lua")]
use wayfinder_core::runtime::lua_loader::LuaLibrary;
use wayfinder_core::runtime::LuaVersion;

#[cfg(feature = "dynamic-lua")]
fn bench_library_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("library_loading");

    let versions = [
        (LuaVersion::V51, "5.1"),
        (LuaVersion::V52, "5.2"),
        (LuaVersion::V53, "5.3"),
        (LuaVersion::V54, "5.4"),
    ];

    for (version, name) in versions {
        group.bench_with_input(BenchmarkId::new("load", name), &version, |b, &v| {
            b.iter(|| {
                let _ = black_box(LuaLibrary::load(v));
            });
        });
    }

    group.finish();
}

#[cfg(feature = "dynamic-lua")]
fn bench_state_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_creation");

    let versions = [
        (LuaVersion::V51, "5.1"),
        (LuaVersion::V52, "5.2"),
        (LuaVersion::V53, "5.3"),
        (LuaVersion::V54, "5.4"),
    ];

    for (version, name) in versions {
        if let Ok(lib) = LuaLibrary::load(version) {
            group.bench_with_input(BenchmarkId::new("create_state", name), &lib, |b, lib| {
                b.iter(|| unsafe {
                    let state = black_box(lib.lual_newstate());
                    lib.lua_close(state);
                });
            });
        }
    }

    group.finish();
}

#[cfg(feature = "dynamic-lua")]
fn bench_basic_operations(c: &mut Criterion) {
    let versions = [
        (LuaVersion::V51, "5.1"),
        (LuaVersion::V52, "5.2"),
        (LuaVersion::V53, "5.3"),
        (LuaVersion::V54, "5.4"),
    ];

    // Number operations
    let mut group = c.benchmark_group("push_pop_number");
    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(BenchmarkId::new("number", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lua_pushnumber(state, black_box(42.0));
                        let _n = black_box(lib.lua_tonumber(state, -1));
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();

    // String operations
    let mut group = c.benchmark_group("push_pop_string");
    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                let test_str = b"Hello, World!\0";
                group.bench_with_input(BenchmarkId::new("string", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lua_pushstring(state, black_box(test_str.as_ptr() as *const i8));
                        let _s = black_box(lib.lua_tostring(state, -1));
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();

    // Table operations
    let mut group = c.benchmark_group("table_operations");
    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(BenchmarkId::new("create_table", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lua_createtable(state, black_box(0), black_box(0));
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();
}

#[cfg(feature = "dynamic-lua")]
fn bench_script_execution(c: &mut Criterion) {
    let versions = [
        (LuaVersion::V51, "5.1"),
        (LuaVersion::V52, "5.2"),
        (LuaVersion::V53, "5.3"),
        (LuaVersion::V54, "5.4"),
    ];

    // Factorial benchmark
    let mut group = c.benchmark_group("factorial");
    let factorial_script = b"
        local function factorial(n)
            if n <= 1 then return 1 end
            return n * factorial(n - 1)
        end
        return factorial(10)
    \0";

    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(BenchmarkId::new("factorial_10", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lual_loadstring(state, black_box(factorial_script.as_ptr() as *const i8));
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();

    // Fibonacci with memoization
    let mut group = c.benchmark_group("fibonacci");
    group.throughput(Throughput::Elements(1));
    let fib_script = b"
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

    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(BenchmarkId::new("fib_30_memo", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lual_loadstring(state, black_box(fib_script.as_ptr() as *const i8));
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();

    // Table manipulation
    let mut group = c.benchmark_group("table_manipulation");
    let table_script = b"
        local t = {}
        for i = 1, 100 do
            t[i] = i * 2
        end
        local sum = 0
        for i = 1, 100 do
            sum = sum + t[i]
        end
        return sum
    \0";

    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(BenchmarkId::new("table_ops", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lual_loadstring(state, black_box(table_script.as_ptr() as *const i8));
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, 0);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();
}

#[cfg(feature = "dynamic-lua")]
fn bench_compatibility_shims(c: &mut Criterion) {
    let versions = [
        (LuaVersion::V51, "5.1"),
        (LuaVersion::V52, "5.2"),
        (LuaVersion::V53, "5.3"),
        (LuaVersion::V54, "5.4"),
    ];

    // lua_pushglobaltable benchmark
    let mut group = c.benchmark_group("compatibility_shims");
    for (version, name) in &versions {
        if let Ok(lib) = LuaLibrary::load(*version) {
            unsafe {
                let state = lib.lual_newstate();
                lib.lual_openlibs(state);

                group.bench_with_input(
                    BenchmarkId::new("pushglobaltable", name),
                    &lib,
                    |b, lib| {
                        b.iter(|| {
                            lib.lua_pushglobaltable(state);
                            lib.lua_settop(state, 0);
                        });
                    },
                );

                // lua_pcall benchmark
                let simple_script = b"return 2 + 2\0";
                lib.lual_loadstring(state, simple_script.as_ptr() as *const i8);

                group.bench_with_input(BenchmarkId::new("pcall", name), &lib, |b, lib| {
                    b.iter(|| {
                        lib.lua_pushvalue(state, -1);
                        lib.lua_pcall(state, 0, 1, 0);
                        lib.lua_settop(state, -2);
                    });
                });

                lib.lua_close(state);
            }
        }
    }
    group.finish();
}

#[cfg(feature = "static-lua")]
fn bench_static_mode(c: &mut Criterion) {
    use wayfinder_core::runtime::lua_ffi::*;

    let mut group = c.benchmark_group("static_mode");

    // State creation
    group.bench_function("create_state", |b| {
        b.iter(|| unsafe {
            let state = black_box(luaL_newstate());
            lua_close(state);
        });
    });

    // Basic operations
    unsafe {
        let state = luaL_newstate();
        luaL_openlibs(state);

        group.bench_function("push_pop_number", |b| {
            b.iter(|| {
                lua_pushnumber(state, black_box(42.0));
                let _n = black_box(lua_tonumber(state, -1));
                lua_settop(state, 0);
            });
        });

        // Script execution
        let script = b"return 2 + 2\0";
        group.bench_function("simple_script", |b| {
            b.iter(|| {
                luaL_loadstring(state, black_box(script.as_ptr() as *const i8));
                lua_pcall(state, 0, 1, 0);
                lua_settop(state, 0);
            });
        });

        lua_close(state);
    }

    group.finish();
}

#[cfg(feature = "dynamic-lua")]
criterion_group!(
    benches,
    bench_library_loading,
    bench_state_creation,
    bench_basic_operations,
    bench_script_execution,
    bench_compatibility_shims
);

#[cfg(feature = "static-lua")]
criterion_group!(benches, bench_static_mode);

criterion_main!(benches);
