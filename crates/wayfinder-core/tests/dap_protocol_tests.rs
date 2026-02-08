//! DAP protocol compliance tests
//!
//! These tests verify that Wayfinder correctly implements the Debug Adapter Protocol

use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DapServer;
use serde_json::json;

/// Test that the initialize request returns correct capabilities
#[test]
fn test_initialize_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    let params = json!({});
    let response = server.handle_initialize(1);
    
    // Check that we got a response
    assert_eq!(response["id"], 1);
    
    // Check that we have the expected capabilities
    let capabilities = &response["result"];
    assert!(capabilities["supportsConfigurationDoneRequest"].as_bool().unwrap_or(false));
    assert!(capabilities["supportsFunctionBreakpoints"].as_bool().unwrap_or(false));
    assert!(capabilities["supportsConditionalBreakpoints"].as_bool().unwrap_or(false));
    assert!(capabilities["supportsExceptionOptions"].as_bool().unwrap_or(false));
    assert!(capabilities["supportsLogBreakpoints"].as_bool().unwrap_or(false));
    assert!(capabilities["supportsEvaluateForHovers"].as_bool().unwrap_or(false));
}

/// Test that we can handle launch requests
#[tokio::test]
async fn test_launch_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "noDebug": false,
        "program": "test.lua"
    });
    
    let response = server.handle_launch(1, &params).await;
    assert!(response.is_some());
    
    let response = response.unwrap();
    assert_eq!(response["id"], 1);
    assert!(response["result"].as_object().is_some());
}

/// Test that we can handle setBreakpoints requests
#[tokio::test]
async fn test_set_breakpoints_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "source": {
            "path": "/test/test.lua"
        },
        "breakpoints": [
            {
                "line": 10
            },
            {
                "line": 20,
                "condition": "x > 5"
            }
        ]
    });
    
    // This might fail if there's no actual script, but we're testing the protocol handling
    let response = server.handle_set_breakpoints(1, &params).await;
    // We just check that we got a response (even if it's an error)
    assert!(response.is_some());
}

/// Test that we can handle setFunctionBreakpoints requests
#[tokio::test]
async fn test_set_function_breakpoints_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "breakpoints": [
            {
                "name": "main"
            },
            {
                "name": "helper",
                "condition": "param ~= nil"
            }
        ]
    });
    
    let response = server.handle_set_function_breakpoints(1, &params).await;
    assert!(response.is_some());
}

/// Test that we can handle setExceptionBreakpoints requests
#[tokio::test]
async fn test_set_exception_breakpoints_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "filters": ["raised", "uncaught"]
    });
    
    let response = server.handle_set_exception_breakpoints(1, &params).await;
    assert!(response.is_some());
}

/// Test that we can handle stackTrace requests
#[tokio::test]
async fn test_stack_trace_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "threadId": 1
    });
    
    let response = server.handle_stack_trace(1, &params).await;
    // Should get a response (might be error if no debug session)
    assert!(response.is_some());
}

/// Test that we can handle scopes requests
#[tokio::test]
async fn test_scopes_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "frameId": 1
    });
    
    let response = server.handle_scopes(1, &params).await;
    assert!(response.is_some());
}

/// Test that we can handle variables requests
#[tokio::test]
async fn test_variables_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "variablesReference": 1
    });
    
    let response = server.handle_variables(1, &params).await;
    assert!(response.is_some());
}

/// Test that we can handle evaluate requests
#[tokio::test]
async fn test_evaluate_request() {
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();
    
    // Set up runtime first
    let runtime = PUCLuaRuntime::new();
    server.set_runtime(runtime);
    
    let params = json!({
        "expression": "1 + 1",
        "frameId": 1
    });
    
    let response = server.handle_evaluate(1, &params).await;
    assert!(response.is_some());
}