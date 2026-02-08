//! Configuration for the Wayfinder debugger
//!
//! This module provides configuration options for the debugger,
//! including evaluate mutation settings.

use serde::{Deserialize, Serialize};

/// Configuration for the Wayfinder debugger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerConfig {
    /// Whether to allow mutation during expression evaluation
    #[serde(default)]
    pub evaluate_mutation: bool,

    /// Whether to show modifications made during evaluation
    #[serde(default)]
    pub show_modifications: bool,

    /// Safety level for evaluation
    #[serde(default)]
    pub eval_safety: EvalSafety,
}

/// Safety levels for expression evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvalSafety {
    /// No safety checks - allow all operations
    None,
    /// Basic safety - prevent dangerous operations
    Basic,
    /// Strict safety - only allow read operations
    Strict,
}

impl Default for EvalSafety {
    fn default() -> Self {
        EvalSafety::Basic
    }
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            evaluate_mutation: false,
            show_modifications: true,
            eval_safety: EvalSafety::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DebuggerConfig::default();
        assert!(!config.evaluate_mutation);
        assert!(config.show_modifications);
        assert_eq!(config.eval_safety, EvalSafety::Basic);
    }

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
}
