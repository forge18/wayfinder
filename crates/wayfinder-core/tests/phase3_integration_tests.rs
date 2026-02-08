//! Phase 3 Integration Tests
//!
//! These tests verify the integration of all Phase 3 features

use wayfinder_core::debug::breakpoints::BreakpointManager;
use wayfinder_core::debug::watchpoints::WatchpointManager;
use wayfinder_core::debug::conditions::ConditionEvaluator;
use wayfinder_core::debug::hit_conditions::evaluate_hit_condition;
use wayfinder_core::debug::logpoints::LogpointEvaluator;
use wayfinder_core::config::{DebuggerConfig, EvalSafety};
use wayfinder_core::runtime::mock::MockRuntime;
use wayfinder_core::session::DebugSession;

/// Test integration of breakpoint manager with all Phase 3 features
#[tokio::test]
async fn test_breakpoint_manager_phase3_features() {
    let mut manager = BreakpointManager::new();
    
    // Test that manager can handle all breakpoint types
    assert_eq!(manager.line_breakpoint_count(), 0);
    assert_eq!(manager.function_breakpoint_count(), 0);
    assert_eq!(manager.exception_filter_count(), 0);
    
    // Test basic functionality still works
    assert!(manager.get_line_breakpoints("test.lua").is_empty());
    assert!(manager.get_function_breakpoints().is_empty());
    assert!(manager.get_exception_breakpoints().is_empty());
}

/// Test integration of watchpoint manager with all Phase 3 features
#[tokio::test]
async fn test_watchpoint_manager_phase3_features() {
    let mut manager = WatchpointManager::new();
    
    // Test that manager can handle data breakpoints
    assert_eq!(manager.data_breakpoint_count(), 0);
    
    // Test basic functionality works
    assert!(manager.get_data_breakpoints().is_empty());
}

/// Test condition evaluator integration
#[tokio::test]
async fn test_condition_evaluator_integration() {
    let mut runtime = MockRuntime::new();
    
    // Test that condition evaluator works with mock runtime
    let result = ConditionEvaluator::should_break(&mut runtime, 0, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Test with empty condition
    let result = ConditionEvaluator::should_break(&mut runtime, 0, Some(&"".to_string())).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

/// Test hit condition evaluation integration
#[test]
fn test_hit_condition_evaluation_integration() {
    // Test various hit conditions
    assert_eq!(evaluate_hit_condition("> 5", 6).unwrap(), true);
    assert_eq!(evaluate_hit_condition("> 5", 5).unwrap(), false);
    assert_eq!(evaluate_hit_condition(">= 5", 5).unwrap(), true);
    assert_eq!(evaluate_hit_condition("< 5", 4).unwrap(), true);
    assert_eq!(evaluate_hit_condition("== 5", 5).unwrap(), true);
    assert_eq!(evaluate_hit_condition("!= 5", 4).unwrap(), true);
    assert_eq!(evaluate_hit_condition("% 3", 6).unwrap(), true);
}

/// Test logpoint evaluator integration
#[tokio::test]
async fn test_logpoint_evaluator_integration() {
    let mut runtime = MockRuntime::new();
    
    // Test basic logpoint evaluation
    let result = LogpointEvaluator::evaluate_log_message(&mut runtime, 0, "Hello, world!").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, world!");
    
    // Test with variable substitution (basic)
    let result = LogpointEvaluator::evaluate_log_message(&mut runtime, 0, "Value is {x}").await;
    assert!(result.is_ok());
    // Note: In a real implementation, this would substitute variables
    // For now, it just returns the template
}

/// Test configuration integration
#[test]
fn test_configuration_integration() {
    // Test that all configuration options work together
    let config = DebuggerConfig {
        evaluate_mutation: true,
        show_modifications: true,
        eval_safety: EvalSafety::Basic,
    };
    
    assert!(config.evaluate_mutation);
    assert!(config.show_modifications);
    assert!(matches!(config.eval_safety, EvalSafety::Basic));
}

/// Test session integration with all Phase 3 features
#[tokio::test]
async fn test_session_phase3_integration() {
    let runtime = MockRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Test that session can access all managers
    let _bp_manager = session.breakpoint_manager();
    let _wp_manager = session.watchpoint_manager();
    
    // Test that session can check watchpoints
    let result = session.check_watchpoints(0).await;
    assert!(result.is_ok());
    
    // Test that session can access configuration
    let config = session.config();
    assert!(!config.evaluate_mutation);
    
    // Test that session can set configuration
    let new_config = DebuggerConfig {
        evaluate_mutation: true,
        show_modifications: false,
        eval_safety: EvalSafety::Strict,
    };
    session.set_config(new_config);
    let config = session.config();
    assert!(config.evaluate_mutation);
}

/// Test comprehensive Phase 3 feature integration
#[tokio::test]
async fn test_comprehensive_phase3_integration() {
    // Create all the components
    let runtime = MockRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Test that all Phase 3 features are integrated
    // 1. Breakpoint manager
    let _bp_manager = session.breakpoint_manager();
    
    // 2. Watchpoint manager
    let _wp_manager = session.watchpoint_manager();
    
    // 3. Configuration
    let config = session.config();
    assert_eq!(config.eval_safety, EvalSafety::Basic);
    
    // 4. Watchpoint checking
    let watchpoint_result = session.check_watchpoints(0).await;
    assert!(watchpoint_result.is_ok());
    
    // 5. All managers are accessible and functional
    assert_eq!(session.breakpoint_manager().line_breakpoint_count(), 0);
    assert_eq!(session.watchpoint_manager().data_breakpoint_count(), 0);
}

/// Test error handling integration
#[tokio::test]
async fn test_error_handling_integration() {
    let runtime = MockRuntime::new();
    let mut session = DebugSession::new(runtime);
    
    // Test that error handling works for watchpoint checking
    let result = session.check_watchpoints(0).await;
    assert!(result.is_ok()); // Should not error even if no watchpoints
    
    // Test that configuration can be updated without errors
    let config = DebuggerConfig::default();
    session.set_config(config);
    
    // Test that all managers handle edge cases gracefully
    let bp_manager = session.breakpoint_manager();
    assert!(bp_manager.get_line_breakpoints("nonexistent.lua").is_empty());
    
    let wp_manager = session.watchpoint_manager();
    assert!(wp_manager.get_data_breakpoints().is_empty());
}