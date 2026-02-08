//! Logpoint handling for breakpoints
//!
//! This module provides functionality to evaluate logpoint messages
//! and output them without pausing execution.

use crate::runtime::{DebugRuntime, Value};
use regex::Regex;

/// Evaluates a logpoint message template with variable substitution
pub struct LogpointEvaluator;

impl LogpointEvaluator {
    /// Evaluates a logpoint message template and substitutes variables
    pub async fn evaluate_log_message<R: DebugRuntime>(
        runtime: &mut R,
        frame_id: i64,
        template: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Parse the template for {expression} placeholders
        let re = Regex::new(r"\{([^}]+)\}").unwrap();
        
        let mut result = template.to_string();
        let mut last_match = 0;
        
        // Find all placeholders and evaluate them
        for cap in re.captures_iter(template) {
            let full_match = &cap[0]; // e.g., "{x}"
            let expression = &cap[1]; // e.g., "x"
            
            // Evaluate the expression
            match runtime.evaluate(frame_id, expression).await {
                Ok(value) => {
                    // Convert the value to a string representation
                    let value_str = match value {
                        Value::Nil => "nil".to_string(),
                        Value::Boolean(b) => b.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::String(s) => s,
                        Value::Table { reference, .. } => format!("table:0x{:x}", reference as usize),
                        Value::Function { reference, name } => {
                            if let Some(n) = name {
                                format!("function:{}:0x{:x}", n, reference as usize)
                            } else {
                                format!("function:0x{:x}", reference as usize)
                            }
                        },
                        Value::UserData => "userdata".to_string(),
                        Value::Thread => "thread".to_string(),
                    };
                    
                    // Replace the placeholder with the evaluated value
                    result = result.replacen(full_match, &value_str, 1);
                }
                Err(e) => {
                    // If evaluation fails, leave the placeholder and log an error
                    eprintln!("Warning: Failed to evaluate logpoint expression '{}': {}", expression, e);
                }
            }
        }
        
        Ok(result)
    }

    /// Processes a logpoint by evaluating its message and returning it
    pub async fn process_logpoint<R: DebugRuntime>(
        runtime: &mut R,
        frame_id: i64,
        log_message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Self::evaluate_log_message(runtime, frame_id, log_message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::mock::MockRuntime;

    #[tokio::test]
    async fn test_evaluate_log_message() {
        let mut runtime = MockRuntime::new();
        let result = LogpointEvaluator::evaluate_log_message(&mut runtime, 0, "Hello, world!").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn test_process_logpoint() {
        let mut runtime = MockRuntime::new();
        let result = LogpointEvaluator::process_logpoint(&mut runtime, 0, "Value is {x}").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value is {x}");
    }
}