//! Phase 3 Watchpoint Tests
//!
//! These tests verify the watchpoint functionality implemented in Phase 3

use wayfinder_core::debug::hit_conditions::evaluate_hit_condition;
use wayfinder_core::debug::watchpoints::{AccessType, DataBreakpoint, DataType, WatchpointManager};

/// Test that we can create a watchpoint manager
#[test]
fn test_watchpoint_manager_creation() {
    let manager = WatchpointManager::new();
    assert_eq!(manager.data_breakpoint_count(), 0);
}

/// Test setting and getting data breakpoints
#[test]
fn test_data_breakpoints() {
    let mut manager = WatchpointManager::new();

    let breakpoints = vec![DataBreakpoint {
        id: 0,
        name: "x".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    }];

    let result = manager.set_data_breakpoints(breakpoints);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 1); // Should be assigned ID 1
    assert_eq!(result[0].previous_value, None); // Previous value should be None initially

    let retrieved = manager.get_data_breakpoints();
    assert_eq!(retrieved.len(), 1);

    let found = manager.find_data_breakpoint(1);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "x");
}

/// Test data breakpoint removal
#[test]
fn test_data_breakpoint_removal() {
    let mut manager = WatchpointManager::new();

    let breakpoints = vec![DataBreakpoint {
        id: 0,
        name: "x".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    }];

    let result = manager.set_data_breakpoints(breakpoints);
    assert_eq!(manager.data_breakpoint_count(), 1);

    assert!(manager.remove_data_breakpoint(1));
    assert_eq!(manager.data_breakpoint_count(), 0);

    // Try to remove non-existent breakpoint
    assert!(!manager.remove_data_breakpoint(999));
}

/// Test clearing all data breakpoints
#[test]
fn test_clear_data_breakpoints() {
    let mut manager = WatchpointManager::new();

    let breakpoints = vec![DataBreakpoint {
        id: 0,
        name: "x".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    }];

    manager.set_data_breakpoints(breakpoints);
    assert_eq!(manager.data_breakpoint_count(), 1);

    manager.clear_all_data_breakpoints();
    assert_eq!(manager.data_breakpoint_count(), 0);
}

/// Test hit counting for data breakpoints
#[test]
fn test_data_breakpoint_hit_counting() {
    let mut manager = WatchpointManager::new();

    let breakpoints = vec![DataBreakpoint {
        id: 0,
        name: "x".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    }];

    let result = manager.set_data_breakpoints(breakpoints);
    let bp_id = result[0].id;

    // Initially hit count should be 0
    assert_eq!(manager.get_data_breakpoint_hit_count(bp_id), Some(0));

    // Increment hit count
    assert!(manager.increment_data_breakpoint_hit_count(bp_id));
    assert_eq!(manager.get_data_breakpoint_hit_count(bp_id), Some(1));

    // Increment again
    assert!(manager.increment_data_breakpoint_hit_count(bp_id));
    assert_eq!(manager.get_data_breakpoint_hit_count(bp_id), Some(2));

    // Try to increment non-existent breakpoint
    assert!(!manager.increment_data_breakpoint_hit_count(999));
}

/// Test previous value tracking for data breakpoints
#[test]
fn test_data_breakpoint_value_tracking() {
    let mut manager = WatchpointManager::new();

    let breakpoints = vec![DataBreakpoint {
        id: 0,
        name: "x".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    }];

    let result = manager.set_data_breakpoints(breakpoints);
    let bp_id = result[0].id;

    // Initially no previous value
    assert_eq!(manager.get_data_breakpoint_previous_value(bp_id), None);

    // Update previous value
    assert!(manager.update_data_breakpoint_previous_value(bp_id, "10".to_string()));
    assert_eq!(
        manager.get_data_breakpoint_previous_value(bp_id),
        Some(&"10".to_string())
    );

    // Check value change detection
    assert!(manager.has_data_breakpoint_value_changed(bp_id, "20"));
    assert!(!manager.has_data_breakpoint_value_changed(bp_id, "10"));

    // Update to new value
    assert!(manager.update_data_breakpoint_previous_value(bp_id, "20".to_string()));
    assert!(!manager.has_data_breakpoint_value_changed(bp_id, "20"));
    assert!(manager.has_data_breakpoint_value_changed(bp_id, "10"));
}

/// Test different data types
#[test]
fn test_data_types() {
    // Test Local data type
    let local_bp = DataBreakpoint {
        id: 1,
        name: "local_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    assert!(matches!(local_bp.data_type, DataType::Local));

    // Test Global data type
    let global_bp = DataBreakpoint {
        id: 2,
        name: "global_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Global,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    assert!(matches!(global_bp.data_type, DataType::Global));

    // Test Upvalue data type
    let upvalue_bp = DataBreakpoint {
        id: 3,
        name: "upvalue_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Upvalue,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    assert!(matches!(upvalue_bp.data_type, DataType::Upvalue));

    // Test UpvalueId data type
    let upvalue_id_bp = DataBreakpoint {
        id: 4,
        name: "upvalue_id_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::UpvalueId {
            function_index: 1,
            upvalue_index: 2,
            upvalue_id: 12345,
        },
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    if let DataType::UpvalueId {
        function_index,
        upvalue_index,
        upvalue_id,
    } = upvalue_id_bp.data_type
    {
        assert_eq!(function_index, 1);
        assert_eq!(upvalue_index, 2);
        assert_eq!(upvalue_id, 12345);
    } else {
        panic!("Expected UpvalueId data type");
    }

    // Test TableField data type
    let table_field_bp = DataBreakpoint {
        id: 5,
        name: "table_field".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::TableField {
            table_ref: 100,
            field: "key".to_string(),
        },
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    if let DataType::TableField { table_ref, field } = table_field_bp.data_type {
        assert_eq!(table_ref, 100);
        assert_eq!(field, "key");
    } else {
        panic!("Expected TableField data type");
    }
}

/// Test access types
#[test]
fn test_access_types() {
    // Test Read access type
    let read_bp = DataBreakpoint {
        id: 1,
        name: "read_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::Read,
        previous_value: None,
    };
    assert!(matches!(read_bp.access_type, AccessType::Read));

    // Test Write access type
    let write_bp = DataBreakpoint {
        id: 2,
        name: "write_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::Write,
        previous_value: None,
    };
    assert!(matches!(write_bp.access_type, AccessType::Write));

    // Test ReadWrite access type
    let readwrite_bp = DataBreakpoint {
        id: 3,
        name: "readwrite_var".to_string(),
        condition: None,
        hit_condition: None,
        verified: true,
        message: None,
        hit_count: 0,
        data_type: DataType::Local,
        access_type: AccessType::ReadWrite,
        previous_value: None,
    };
    assert!(matches!(readwrite_bp.access_type, AccessType::ReadWrite));
}
