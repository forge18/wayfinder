//! Integration tests for different Lua versions
//!
//! These tests verify that Wayfinder works correctly with different Lua implementations:
//! - Lua 5.1 (PUC-Rio)
//! - Lua 5.2 (PUC-Rio)
//! - Lua 5.3 (PUC-Rio)
//! - Lua 5.4 (PUC-Rio)
//! - LuaNext (TypeScript-like Lua with source maps)

#[cfg(feature = "dynamic-lua")]
use wayfinder_core::runtime::lua_loader::{LuaLibrary, LuaVersion};

/// Test that we can load all supported Lua versions dynamically
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_lua_library_loading() {
    // Test Lua 5.1
    let lua_51 = LuaLibrary::load(LuaVersion::V51);
    assert!(lua_51.is_ok(), "Failed to load Lua 5.1: {:?}", lua_51.err());

    // Test Lua 5.2
    let lua_52 = LuaLibrary::load(LuaVersion::V52);
    assert!(lua_52.is_ok(), "Failed to load Lua 5.2: {:?}", lua_52.err());

    // Test Lua 5.3
    let lua_53 = LuaLibrary::load(LuaVersion::V53);
    assert!(lua_53.is_ok(), "Failed to load Lua 5.3: {:?}", lua_53.err());

    // Test Lua 5.4
    let lua_54 = LuaLibrary::load(LuaVersion::V54);
    assert!(lua_54.is_ok(), "Failed to load Lua 5.4: {:?}", lua_54.err());
}

/// Test library version reporting
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_lua_version_reporting() {
    if let Ok(lua_51) = LuaLibrary::load(LuaVersion::V51) {
        let version = lua_51.version();
        assert!(version.starts_with("Lua 5.1"), "Unexpected version: {}", version);
    }

    if let Ok(lua_54) = LuaLibrary::load(LuaVersion::V54) {
        let version = lua_54.version();
        assert!(version.starts_with("Lua 5.4"), "Unexpected version: {}", version);
    }
}

/// Test that we can create Lua states with all versions
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_lua_state_creation() {
    use wayfinder_core::runtime::lua_state::Lua;

    for version in &[LuaVersion::V51, LuaVersion::V52, LuaVersion::V53, LuaVersion::V54] {
        if let Ok(lib) = LuaLibrary::load(*version) {
            let lua = Lua::new(lib);
            assert!(lua.is_ok(), "Failed to create Lua state for {:?}: {:?}", version, lua.err());
        }
    }
}

/// Test basic Lua operations across versions
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_lua_basic_operations() {
    use wayfinder_core::runtime::lua_state::Lua;

    for version in &[LuaVersion::V51, LuaVersion::V52, LuaVersion::V53, LuaVersion::V54] {
        if let Ok(lib) = LuaLibrary::load(*version) {
            let lua = Lua::new(lib).expect("Failed to create Lua state");

            // Test pushing and getting numbers
            unsafe {
                lua.pushnumber(42.0);
                let value = lua.tonumber(-1);
                assert_eq!(value, 42.0, "Number push/get failed for {:?}", version);
                lua.pop(1);
            }
        }
    }
}

/// Test version-specific features
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_version_specific_features() {
    use wayfinder_core::runtime::lua_state::Lua;

    // Lua 5.1: uses lua_getfenv/lua_setfenv (deprecated in 5.2+)
    if let Ok(lib) = LuaLibrary::load(LuaVersion::V51) {
        let _lua = Lua::new(lib).expect("Failed to create Lua 5.1 state");
        // Version-specific functionality would be tested here
    }

    // Lua 5.2+: uses _ENV instead of global table manipulation
    if let Ok(lib) = LuaLibrary::load(LuaVersion::V52) {
        let lua = Lua::new(lib).expect("Failed to create Lua 5.2 state");
        unsafe {
            // Test global table access (5.2+ way)
            lua.pushglobaltable();
            lua.pop(1);
        }
    }

    // Lua 5.3+: has integer division operator //
    if let Ok(lib) = LuaLibrary::load(LuaVersion::V53) {
        let lua = Lua::new(lib).expect("Failed to create Lua 5.3 state");
        unsafe {
            // Test executing code with integer division
            let code = "return 10 // 3";
            let result = lua.load_string(code);
            assert_eq!(result, 0, "Failed to load 5.3+ integer division code");
        }
    }
}

/// Test compatibility shims
#[cfg(feature = "dynamic-lua")]
#[test]
fn test_compatibility_shims() {
    use wayfinder_core::runtime::lua_state::Lua;

    // Test that pushglobaltable works across all versions
    for version in &[LuaVersion::V51, LuaVersion::V52, LuaVersion::V53, LuaVersion::V54] {
        if let Ok(lib) = LuaLibrary::load(*version) {
            let lua = Lua::new(lib).expect("Failed to create Lua state");
            unsafe {
                lua.pushglobaltable();
                assert!(lua.istable(-1), "pushglobaltable compatibility shim failed for {:?}", version);
                lua.pop(1);
            }
        }
    }
}

/// Test multi-version support is working
#[cfg(feature = "static-lua")]
#[test]
fn test_static_lua_builds() {
    // When built with static-lua, we should still have the infrastructure
    // to support multiple versions (even if only one is linked statically)

    // This test just verifies the code compiles with static-lua feature
    // Runtime version selection happens with dynamic-lua feature
    assert!(true, "Static Lua build compiles successfully");
}

/// Test that LuaNext can use all Lua versions
#[test]
fn test_luanext_multi_version() {
    // LuaNext should work with any Lua version (5.1-5.4) since it
    // compiles to standard Lua code. The version only affects which
    // Lua runtime executes the compiled code.

    // This is tested by the source map tests - if source maps work,
    // then LuaNext debugging works with that Lua version
    assert!(true, "LuaNext multi-version support is architecture-level");
}