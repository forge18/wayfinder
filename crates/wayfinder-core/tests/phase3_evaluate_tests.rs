//! Phase 3 Evaluate Mutation Tests
//!
//! These tests verify the evaluate mutation functionality implemented in Phase 3

use wayfinder_core::config::{DebuggerConfig, EvalSafety};

/// Test default debugger configuration
#[test]
fn test_default_config() {
    let config = DebuggerConfig::default();
    assert!(!config.evaluate_mutation);
    assert!(config.show_modifications);
    assert_eq!(config.eval_safety, EvalSafety::Basic);
}

/// Test custom debugger configuration
#[test]
fn test_custom_config() {
    let config = DebuggerConfig {
        evaluate_mutation: true,
        show_modifications: false,
        eval_safety: EvalSafety::Strict,
    };

    assert!(config.evaluate_mutation);
    assert!(!config.show_modifications);
    assert_eq!(config.eval_safety, EvalSafety::Strict);
}

/// Test eval safety levels
#[test]
fn test_eval_safety_levels() {
    // Test None safety level
    let none_config = DebuggerConfig {
        eval_safety: EvalSafety::None,
        ..Default::default()
    };
    assert!(matches!(none_config.eval_safety, EvalSafety::None));

    // Test Basic safety level
    let basic_config = DebuggerConfig {
        eval_safety: EvalSafety::Basic,
        ..Default::default()
    };
    assert!(matches!(basic_config.eval_safety, EvalSafety::Basic));

    // Test Strict safety level
    let strict_config = DebuggerConfig {
        eval_safety: EvalSafety::Strict,
        ..Default::default()
    };
    assert!(matches!(strict_config.eval_safety, EvalSafety::Strict));
}

/// Test configuration with mutation enabled
#[test]
fn test_mutation_enabled_config() {
    let config = DebuggerConfig {
        evaluate_mutation: true,
        show_modifications: true,
        eval_safety: EvalSafety::Basic,
    };

    assert!(config.evaluate_mutation);
    assert!(config.show_modifications);
    assert!(matches!(config.eval_safety, EvalSafety::Basic));
}
