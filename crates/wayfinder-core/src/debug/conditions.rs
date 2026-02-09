//! Condition evaluation for breakpoints
//!
//! This module provides functionality to evaluate breakpoint conditions
//! expressed as Lua expressions.

use crate::runtime::{DebugRuntime, Value};

/// Evaluates a condition expression in the context of the current runtime
pub struct ConditionEvaluator;

impl ConditionEvaluator {
    /// Evaluates a condition expression and returns whether it's truthy
    pub async fn evaluate_condition<R: DebugRuntime>(
        runtime: &mut R,
        frame_id: i64,
        condition: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Evaluate the condition expression
        match runtime.evaluate(frame_id, condition).await {
            Ok(value) => {
                // Convert the result to a boolean following Lua truthiness rules
                // In Lua, only nil and false are falsy, everything else is truthy
                let is_truthy = match value {
                    Value::Nil => false,
                    Value::Boolean(false) => false,
                    _ => true,
                };
                Ok(is_truthy)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Checks if a condition should cause a breakpoint to be hit
    pub async fn should_break<R: DebugRuntime>(
        runtime: &mut R,
        frame_id: i64,
        condition: Option<&String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // If there's no condition, we should break
        let condition_str = match condition {
            Some(c) => c,
            None => return Ok(true),
        };

        // If the condition is empty, we should break
        if condition_str.trim().is_empty() {
            return Ok(true);
        }

        // Evaluate the condition
        match Self::evaluate_condition(runtime, frame_id, condition_str).await {
            Ok(result) => Ok(result),
            Err(e) => {
                // If condition evaluation fails, we still break but log the error
                eprintln!("Warning: Condition evaluation failed: {}", e);
                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{mock::MockRuntime, Value};

    #[tokio::test]
    async fn test_should_break_without_condition() {
        let mut runtime = MockRuntime::new();
        let result = ConditionEvaluator::should_break(&mut runtime, 0, None).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_should_break_with_empty_condition() {
        let mut runtime = MockRuntime::new();
        let result = ConditionEvaluator::should_break(&mut runtime, 0, Some(&"".to_string())).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_evaluate_condition_true() {
        let mut runtime = MockRuntime::new();
        runtime.set_evaluation_result(Value::Boolean(true));
        let result = ConditionEvaluator::evaluate_condition(&mut runtime, 0, "x > 5").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_evaluate_condition_false() {
        let mut runtime = MockRuntime::new();
        runtime.set_evaluation_result(Value::Boolean(false));
        let result = ConditionEvaluator::evaluate_condition(&mut runtime, 0, "x < 5").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_evaluate_condition_nil() {
        let mut runtime = MockRuntime::new();
        runtime.set_evaluation_result(Value::Nil);
        let result = ConditionEvaluator::evaluate_condition(&mut runtime, 0, "undefined_var").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_evaluate_condition_number() {
        let mut runtime = MockRuntime::new();
        runtime.set_evaluation_result(Value::Number(42.0));
        let result = ConditionEvaluator::evaluate_condition(&mut runtime, 0, "42").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_evaluate_condition_string() {
        let mut runtime = MockRuntime::new();
        runtime.set_evaluation_result(Value::String("hello".to_string()));
        let result = ConditionEvaluator::evaluate_condition(&mut runtime, 0, "'hello'").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}