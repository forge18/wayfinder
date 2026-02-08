//! Integration tests for different Lua versions
//!
//! These tests verify that Wayfinder works correctly with different Lua implementations:
//! - Lua 5.1 (PUC-Rio)
//! - Lua 5.2 (PUC-Rio)
//! - Lua 5.3 (PUC-Rio)
//! - Lua 5.4 (PUC-Rio)
//! - LuaNext (TypeScript-like Lua)

use wayfinder_core::runtime::{
    puc_lua::PUCLuaRuntime, 
    luanext::LuaNextRuntime,
    DebugRuntime, 
    RuntimeVersion
};
use wayfinder_core::session::DebugSession;

/// Test that we can create and initialize all supported Lua runtimes
#[tokio::test]
async fn test_lua_runtimes_initialization() {
    // Test PUC Lua 5.1
    let lua_51_result = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua51);
    assert!(lua_51_result.is_ok(), "Failed to create Lua 5.1 runtime");
    
    // Test PUC Lua 5.2
    let lua_52_result = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua52);
    assert!(lua_52_result.is_ok(), "Failed to create Lua 5.2 runtime");
    
    // Test PUC Lua 5.3
    let lua_53_result = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua53);
    assert!(lua_53_result.is_ok(), "Failed to create Lua 5.3 runtime");
    
    // Test PUC Lua 5.4
    let lua_54_result = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua54);
    assert!(lua_54_result.is_ok(), "Failed to create Lua 5.4 runtime");
    
    // Test LuaNext
    let luanext_result = LuaNextRuntime::new();
    assert!(luanext_result.is_ok(), "Failed to create LuaNext runtime");
}

/// Test that we can create debug sessions with all runtimes
#[tokio::test]
async fn test_debug_session_creation() {
    // Test with PUC Lua 5.4 (most recent stable)
    let lua_runtime = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua54).expect("Failed to create Lua runtime");
    let _session = DebugSession::new(lua_runtime);
    
    // Test with LuaNext
    let luanext_runtime = LuaNextRuntime::new().expect("Failed to create LuaNext runtime");
    let _session = DebugSession::new(luanext_runtime);
}

/// Test basic execution with different Lua versions
#[tokio::test]
async fn test_basic_execution() {
    let test_script = r#"
        local x = 10
        local y = 20
        local z = x + y
        return z
    "#;
    
    // Test with Lua 5.4
    let mut lua_runtime = PUCLuaRuntime::new_with_version(RuntimeVersion::Lua54)
        .expect("Failed to create Lua 5.4 runtime");
    
    // Load and run the script
    let result = lua_runtime.load_and_run(test_script, None).await;
    assert!(result.is_ok(), "Failed to execute script with Lua 5.4: {:?}", result.err());
    
    // Test with LuaNext
    let mut luanext_runtime = LuaNextRuntime::new()
        .expect("Failed to create LuaNext runtime");
    
    // Load and run the script
    let result = luanext_runtime.load_and_run(test_script, None).await;
    assert!(result.is_ok(), "Failed to execute script with LuaNext: {:?}", result.err());
}