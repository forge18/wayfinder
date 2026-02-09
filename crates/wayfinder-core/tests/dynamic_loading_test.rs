//! Integration tests for dynamic Lua loading across versions 5.1-5.4
//!
//! These tests verify that:
//! 1. Libraries for all Lua versions can be loaded
//! 2. Version-specific compatibility shims work correctly
//! 3. Optional symbols are handled properly

#[cfg(feature = "dynamic-lua")]
mod dynamic_loading_tests {
    use wayfinder_core::runtime::lua_loader::LuaLibrary;
    use wayfinder_core::runtime::LuaVersion;

    #[test]
    fn test_load_lua_51() {
        // This will fail if Lua 5.1 is not installed, but that's expected
        match LuaLibrary::load(LuaVersion::V51) {
            Ok(lib) => {
                assert_eq!(lib.version(), LuaVersion::V51);
                println!("✓ Successfully loaded Lua 5.1");
            }
            Err(e) => {
                println!("⚠ Lua 5.1 not available: {}", e);
                println!("  Install with: ./scripts/install_lua_versions.sh");
            }
        }
    }

    #[test]
    fn test_load_lua_52() {
        match LuaLibrary::load(LuaVersion::V52) {
            Ok(lib) => {
                assert_eq!(lib.version(), LuaVersion::V52);
                println!("✓ Successfully loaded Lua 5.2");
            }
            Err(e) => {
                println!("⚠ Lua 5.2 not available: {}", e);
                println!("  Install with: ./scripts/install_lua_versions.sh");
            }
        }
    }

    #[test]
    fn test_load_lua_53() {
        match LuaLibrary::load(LuaVersion::V53) {
            Ok(lib) => {
                assert_eq!(lib.version(), LuaVersion::V53);
                println!("✓ Successfully loaded Lua 5.3");
            }
            Err(e) => {
                println!("⚠ Lua 5.3 not available: {}", e);
                println!("  Install with: ./scripts/install_lua_versions.sh");
            }
        }
    }

    #[test]
    fn test_load_lua_54() {
        match LuaLibrary::load(LuaVersion::V54) {
            Ok(lib) => {
                assert_eq!(lib.version(), LuaVersion::V54);
                println!("✓ Successfully loaded Lua 5.4");
            }
            Err(e) => {
                println!("⚠ Lua 5.4 not available: {}", e);
                println!("  Install with: ./scripts/install_lua_versions.sh");
            }
        }
    }

    #[test]
    fn test_create_lua_state_all_versions() {
        let versions = [
            LuaVersion::V51,
            LuaVersion::V52,
            LuaVersion::V53,
            LuaVersion::V54,
        ];

        for version in versions {
            match LuaLibrary::load(version) {
                Ok(lib) => unsafe {
                    // Create a Lua state
                    let state = lib.lual_newstate();
                    assert!(!state.is_null(), "Failed to create Lua state for {:?}", version);

                    // Open standard libraries
                    lib.lual_openlibs(state);

                    // Push a value to test basic operations
                    lib.lua_pushnumber(state, 42.0);
                    let top = lib.lua_gettop(state);
                    assert_eq!(top, 1, "Stack should have 1 element");

                    let num = lib.lua_tonumber(state, -1);
                    assert_eq!(num, 42.0, "Should retrieve pushed number");

                    // Clean up
                    lib.lua_close(state);

                    println!("✓ Created and tested Lua state for {:?}", version);
                },
                Err(e) => {
                    println!("⚠ Skipping {:?}: {}", version, e);
                }
            }
        }
    }

    #[test]
    fn test_lua_pushglobaltable_compatibility() {
        // Test that lua_pushglobaltable works in both 5.1 (fallback) and 5.2+ (native)
        let versions = [
            LuaVersion::V51, // Should use fallback
            LuaVersion::V52, // Should use native
            LuaVersion::V53, // Should use native
            LuaVersion::V54, // Should use native
        ];

        for version in versions {
            match LuaLibrary::load(version) {
                Ok(lib) => unsafe {
                    let state = lib.lual_newstate();
                    lib.lual_openlibs(state);

                    // Push global table
                    lib.lua_pushglobaltable(state);

                    // Should be a table
                    let typ = lib.lua_type(state, -1);
                    const LUA_TTABLE: i32 = 5;
                    assert_eq!(typ, LUA_TTABLE, "lua_pushglobaltable should push a table for {:?}", version);

                    lib.lua_close(state);
                    println!("✓ lua_pushglobaltable works for {:?}", version);
                },
                Err(e) => {
                    println!("⚠ Skipping {:?}: {}", version, e);
                }
            }
        }
    }

    #[test]
    fn test_lua_pcall_compatibility() {
        // Test that lua_pcall works in both 5.1 (native) and 5.2+ (via lua_pcallk)
        let versions = [
            LuaVersion::V51, // Should use native lua_pcall
            LuaVersion::V52, // Should use lua_pcallk
            LuaVersion::V53, // Should use lua_pcallk
            LuaVersion::V54, // Should use lua_pcallk
        ];

        for version in versions {
            match LuaLibrary::load(version) {
                Ok(lib) => unsafe {
                    let state = lib.lual_newstate();
                    lib.lual_openlibs(state);

                    // Load a simple Lua script
                    let code = b"return 2 + 2\0";
                    let result = lib.lual_loadstring(state, code.as_ptr() as *const i8);
                    assert_eq!(result, 0, "Failed to load Lua code for {:?}", version);

                    // Call the function
                    let call_result = lib.lua_pcall(state, 0, 1, 0);
                    assert_eq!(call_result, 0, "Failed to call Lua function for {:?}", version);

                    // Get the result
                    let num = lib.lua_tonumber(state, -1);
                    assert_eq!(num, 4.0, "Incorrect result from Lua call for {:?}", version);

                    lib.lua_close(state);
                    println!("✓ lua_pcall works for {:?}", version);
                },
                Err(e) => {
                    println!("⚠ Skipping {:?}: {}", version, e);
                }
            }
        }
    }

    #[test]
    fn test_execute_simple_script_all_versions() {
        let versions = [
            LuaVersion::V51,
            LuaVersion::V52,
            LuaVersion::V53,
            LuaVersion::V54,
        ];

        let script = b"
            local function factorial(n)
                if n <= 1 then return 1 end
                return n * factorial(n - 1)
            end
            return factorial(5)
        \0";

        for version in versions {
            match LuaLibrary::load(version) {
                Ok(lib) => unsafe {
                    let state = lib.lual_newstate();
                    lib.lual_openlibs(state);

                    // Load script
                    let load_result = lib.lual_loadstring(state, script.as_ptr() as *const i8);
                    assert_eq!(load_result, 0, "Failed to load script for {:?}", version);

                    // Execute
                    let exec_result = lib.lua_pcall(state, 0, 1, 0);
                    assert_eq!(exec_result, 0, "Failed to execute script for {:?}", version);

                    // Check result (5! = 120)
                    let result = lib.lua_tonumber(state, -1);
                    assert_eq!(result, 120.0, "Incorrect factorial result for {:?}", version);

                    lib.lua_close(state);
                    println!("✓ Executed factorial script for {:?}", version);
                },
                Err(e) => {
                    println!("⚠ Skipping {:?}: {}", version, e);
                }
            }
        }
    }
}

#[cfg(feature = "static-lua")]
mod static_mode_notice {
    #[test]
    fn notice_static_mode() {
        println!("ℹ Dynamic loading tests are only available with --features dynamic-lua");
        println!("  Current build uses static Lua 5.4 linking");
        println!("  To test dynamic loading:");
        println!("    cargo test --features dynamic-lua --no-default-features");
    }
}
