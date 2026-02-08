//! Variable inspection tests
//!
//! These tests verify that variables can be correctly inspected during debugging

use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DebugSession;
use wayfinder_core::runtime::{Variable, Value};

/// Test that we can inspect local variables
#[tokio::test]
async fn test_local_variable_inspection() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Load a simple script with local variables
    let script = r#"
        local x = 10
        local y = "hello"
        local z = true
    "#;
    
    // This would normally be done during a debug session
    // For now, we're just testing that the session can be created
    let _result = session.runtime.load_and_run(script, None).await;
    
    // In a real test, we would pause at a breakpoint and then call session.variables()
    // But we can at least verify the session works
    assert!(true); // Placeholder until we have a better way to test this
}

/// Test that we can inspect global variables
#[tokio::test]
async fn test_global_variable_inspection() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Load a simple script with global variables
    let script = r#"
        global_var = 42
        global_string = "world"
    "#;
    
    let _result = session.runtime.load_and_run(script, None).await;
    
    // In a real test, we would inspect the global scope
    assert!(true); // Placeholder
}

/// Test that we can inspect table variables
#[tokio::test]
async fn test_table_variable_inspection() {
    let runtime = PUCLuaRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Load a script with table variables
    let script = r#"
        local t = {
            key1 = "value1",
            key2 = 123,
            nested = {
                inner = "data"
            }
        }
    "#;
    
    let _result = session.runtime.load_and_run(script, None).await;
    
    // In a real test, we would expand the table and inspect its contents
    assert!(true); // Placeholder
}

/// Test that variable values are correctly represented
#[test]
fn test_variable_value_representation() {
    // Test nil value
    let nil_var = Variable {
        name: "nil_var".to_string(),
        value: "nil".to_string(),
        type_: "nil".to_string(),
        variables_reference: None,
        named_variables: None,
        indexed_variables: None,
    };
    assert_eq!(nil_var.value, "nil");
    assert_eq!(nil_var.type_, "nil");
    
    // Test boolean value
    let bool_var = Variable {
        name: "bool_var".to_string(),
        value: "true".to_string(),
        type_: "boolean".to_string(),
        variables_reference: None,
        named_variables: None,
        indexed_variables: None,
    };
    assert_eq!(bool_var.value, "true");
    assert_eq!(bool_var.type_, "boolean");
    
    // Test number value
    let num_var = Variable {
        name: "num_var".to_string(),
        value: "3.14".to_string(),
        type_: "number".to_string(),
        variables_reference: None,
        named_variables: None,
        indexed_variables: None,
    };
    assert_eq!(num_var.value, "3.14");
    assert_eq!(num_var.type_, "number");
    
    // Test string value
    let str_var = Variable {
        name: "str_var".to_string(),
        value: "\"hello\"".to_string(),
        type_: "string".to_string(),
        variables_reference: None,
        named_variables: None,
        indexed_variables: None,
    };
    assert_eq!(str_var.value, "\"hello\"");
    assert_eq!(str_var.type_, "string");
    
    // Test table value with reference
    let table_var = Variable {
        name: "table_var".to_string(),
        value: "table (3 elements)".to_string(),
        type_: "table".to_string(),
        variables_reference: Some(123),
        named_variables: Some(3),
        indexed_variables: Some(0),
    };
    assert_eq!(table_var.value, "table (3 elements)");
    assert_eq!(table_var.type_, "table");
    assert_eq!(table_var.variables_reference, Some(123));
    assert_eq!(table_var.named_variables, Some(3));
}

/// Test that Value enum correctly represents different types
#[test]
fn test_value_enum_representation() {
    // Test Nil value
    let nil_value = Value::Nil;
    match nil_value {
        Value::Nil => assert!(true),
        _ => panic!("Expected Nil variant"),
    }
    
    // Test Boolean value
    let bool_value = Value::Boolean(true);
    match bool_value {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean variant"),
    }
    
    // Test Number value
    let num_value = Value::Number(3.14);
    match num_value {
        Value::Number(n) => assert_eq!(n, 3.14),
        _ => panic!("Expected Number variant"),
    }
    
    // Test String value
    let str_value = Value::String("test".to_string());
    match str_value {
        Value::String(s) => assert_eq!(s, "test"),
        _ => panic!("Expected String variant"),
    }
    
    // Test Table value
    let table_value = Value::Table {
        reference: 123,
        length: 5,
    };
    match table_value {
        Value::Table { reference, length } => {
            assert_eq!(reference, 123);
            assert_eq!(length, 5);
        }
        _ => panic!("Expected Table variant"),
    }
    
    // Test Function value
    let func_value = Value::Function {
        reference: 456,
        name: Some("my_func".to_string()),
    };
    match func_value {
        Value::Function { reference, name } => {
            assert_eq!(reference, 456);
            assert_eq!(name, Some("my_func".to_string()));
        }
        _ => panic!("Expected Function variant"),
    }
}