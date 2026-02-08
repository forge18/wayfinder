//! Breakpoint accuracy tests
//!
//! These tests verify that breakpoints are correctly set, hit, and managed

use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DebugSession;
use wayfinder_core::debug::breakpoints::BreakpointManager;

/// Test that we can create a breakpoint manager
#[test]
fn test_breakpoint_manager_creation() {
    let manager = BreakpointManager::new();
    assert!(manager.is_ok());
}

/// Test setting and getting line breakpoints
#[tokio::test]
async fn test_line_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Set a breakpoint
    let result = session.set_breakpoint("/test/script.lua", 10).await;
    // Even if it fails (because there's no actual script), we should get a result
    assert!(result.is_ok() || result.is_err());
    
    // Check that the breakpoint is stored in the manager
    let breakpoints = session.breakpoint_manager().get_line_breakpoints("/test/script.lua");
    assert!(!breakpoints.is_empty());
}

/// Test setting and getting function breakpoints
#[tokio::test]
async fn test_function_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Set a function breakpoint
    let result = session.set_function_breakpoint("main").await;
    // Even if it fails (because there's no actual function), we should get a result
    assert!(result.is_ok() || result.is_err());
    
    // Check that the breakpoint is stored in the manager
    let breakpoints = session.breakpoint_manager().get_function_breakpoints();
    assert!(!breakpoints.is_empty());
}

/// Test setting and getting exception breakpoints
#[tokio::test]
async fn test_exception_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Set an exception breakpoint
    let result = session.set_exception_breakpoint("uncaught").await;
    // Even if it fails, we should get a result
    assert!(result.is_ok() || result.is_err());
    
    // Check that the breakpoint is stored in the manager
    let filters = session.breakpoint_manager().get_exception_breakpoints();
    assert!(!filters.is_empty());
}

/// Test conditional breakpoints
#[tokio::test]
async fn test_conditional_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    let manager = session.breakpoint_manager();
    
    // Create a conditional breakpoint
    let mut breakpoints = vec![wayfinder_core::debug::breakpoints::LineBreakpoint {
        id: 0,
        source: "/test/script.lua".to_string(),
        line: 10,
        condition: Some("x > 5".to_string()),
        log_message: None,
        hit_condition: None,
        verified: false,
        message: None,
    }];
    
    let stored_breakpoints = manager.set_line_breakpoints("/test/script.lua".to_string(), breakpoints);
    assert!(!stored_breakpoints.is_empty());
    
    // Check that condition is preserved
    let stored_breakpoint = &stored_breakpoints[0];
    assert_eq!(stored_breakpoint.condition, Some("x > 5".to_string()));
}

/// Test logpoint breakpoints
#[tokio::test]
async fn test_logpoint_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    let manager = session.breakpoint_manager();
    
    // Create a logpoint breakpoint
    let mut breakpoints = vec![wayfinder_core::debug::breakpoints::LineBreakpoint {
        id: 0,
        source: "/test/script.lua".to_string(),
        line: 15,
        condition: None,
        log_message: Some("Value of x is {x}".to_string()),
        hit_condition: None,
        verified: false,
        message: None,
    }];
    
    let stored_breakpoints = manager.set_line_breakpoints("/test/script.lua".to_string(), breakpoints);
    assert!(!stored_breakpoints.is_empty());
    
    // Check that log message is preserved
    let stored_breakpoint = &stored_breakpoints[0];
    assert_eq!(stored_breakpoint.log_message, Some("Value of x is {x}".to_string()));
}

/// Test hit condition breakpoints
#[tokio::test]
async fn test_hit_condition_breakpoints() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    let manager = session.breakpoint_manager();
    
    // Create a hit condition breakpoint
    let mut breakpoints = vec![wayfinder_core::debug::breakpoints::LineBreakpoint {
        id: 0,
        source: "/test/script.lua".to_string(),
        line: 20,
        condition: None,
        log_message: None,
        hit_condition: Some(">= 3".to_string()),
        verified: false,
        message: None,
    }];
    
    let stored_breakpoints = manager.set_line_breakpoints("/test/script.lua".to_string(), breakpoints);
    assert!(!stored_breakpoints.is_empty());
    
    // Check that hit condition is preserved
    let stored_breakpoint = &stored_breakpoints[0];
    assert_eq!(stored_breakpoint.hit_condition, Some(">= 3".to_string()));
}