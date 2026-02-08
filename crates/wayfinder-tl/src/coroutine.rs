//! Coroutine debugging support
//!
//! This module provides functionality for debugging Lua coroutines,
//! including enumeration, status tracking, and switching between coroutines.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during coroutine debugging
#[derive(Error, Debug)]
pub enum CoroutineError {
    #[error("Coroutine not found: {0}")]
    NotFound(String),

    #[error("Invalid coroutine state")]
    InvalidState,

    #[error("Coroutine operation failed: {0}")]
    OperationFailed(String),
}

/// Represents the status of a coroutine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoroutineStatus {
    /// Coroutine is currently running
    Running,

    /// Coroutine is suspended
    Suspended,

    /// Coroutine has completed execution
    Dead,

    /// Coroutine is in an error state
    Error,
}

/// Represents a coroutine in the debugged program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coroutine {
    /// Unique identifier for the coroutine
    pub id: String,

    /// Human-readable name for the coroutine (if available)
    pub name: Option<String>,

    /// Current status of the coroutine
    pub status: CoroutineStatus,

    /// Stack trace for the coroutine (if available)
    pub stack_trace: Option<Vec<StackFrame>>,
}

/// Represents a stack frame in a coroutine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    /// Name of the function in this frame
    pub function_name: String,

    /// Source file for this frame
    pub source_file: String,

    /// Line number in the source file
    pub line_number: u32,
}

/// Manages coroutine debugging functionality
pub struct CoroutineDebugger {
    /// Map of coroutines by ID
    coroutines: HashMap<String, Coroutine>,

    /// ID of the currently active coroutine
    current_coroutine: Option<String>,

    /// Whether to break on all coroutines or just the main one
    break_on_all: bool,
}

impl CoroutineDebugger {
    /// Create a new coroutine debugger
    pub fn new() -> Self {
        Self {
            coroutines: HashMap::new(),
            current_coroutine: None,
            break_on_all: false,
        }
    }

    /// Enumerate all active coroutines
    pub fn enumerate_coroutines(&self) -> Vec<&Coroutine> {
        self.coroutines.values().collect()
    }

    /// Get a specific coroutine by ID
    pub fn get_coroutine(&self, id: &str) -> Option<&Coroutine> {
        self.coroutines.get(id)
    }

    /// Add or update a coroutine
    pub fn update_coroutine(&mut self, coroutine: Coroutine) {
        self.coroutines.insert(coroutine.id.clone(), coroutine);
    }

    /// Remove a coroutine (when it's dead)
    pub fn remove_coroutine(&mut self, id: &str) {
        self.coroutines.remove(id);
    }

    /// Set the current active coroutine
    pub fn set_current_coroutine(&mut self, id: Option<String>) -> Result<(), CoroutineError> {
        if let Some(ref coroutine_id) = id {
            if !self.coroutines.contains_key(coroutine_id) {
                return Err(CoroutineError::NotFound(coroutine_id.clone()));
            }
        }
        self.current_coroutine = id;
        Ok(())
    }

    /// Get the current active coroutine
    pub fn get_current_coroutine(&self) -> Option<&Coroutine> {
        self.current_coroutine
            .as_ref()
            .and_then(|id| self.coroutines.get(id))
    }

    /// Set whether to break on all coroutines
    pub fn set_break_on_all(&mut self, enabled: bool) {
        self.break_on_all = enabled;
    }

    /// Check if we should break on the current coroutine
    pub fn should_break(&self) -> bool {
        if self.break_on_all {
            return true;
        }

        // Only break on the main coroutine by default
        // In a real implementation, we'd need to determine which is the main coroutine
        self.current_coroutine.is_none()
    }

    /// Use debug.setname for naming (Lua 5.2+)
    pub fn set_coroutine_name(&mut self, id: &str, name: String) -> Result<(), CoroutineError> {
        if let Some(coroutine) = self.coroutines.get_mut(id) {
            coroutine.name = Some(name);
            Ok(())
        } else {
            Err(CoroutineError::NotFound(id.to_string()))
        }
    }

    /// Display coroutine name in stack frames
    pub fn format_coroutine_frame(&self, coroutine_id: &str, frame: &StackFrame) -> String {
        if let Some(coroutine) = self.coroutines.get(coroutine_id) {
            if let Some(ref name) = coroutine.name {
                format!("{} ({})", frame.function_name, name)
            } else {
                format!("{} [{}]", frame.function_name, coroutine_id)
            }
        } else {
            frame.function_name.clone()
        }
    }

    /// Implement coroutine switching
    pub fn switch_to_coroutine(&mut self, id: &str) -> Result<(), CoroutineError> {
        if self.coroutines.contains_key(id) {
            self.current_coroutine = Some(id.to_string());
            Ok(())
        } else {
            Err(CoroutineError::NotFound(id.to_string()))
        }
    }

    /// Add breakOnAll coroutine option
    pub fn enable_break_on_all(&mut self, enabled: bool) {
        self.break_on_all = enabled;
    }
}

impl Default for CoroutineDebugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coroutine_debugger_creation() {
        let debugger = CoroutineDebugger::new();
        assert!(debugger.coroutines.is_empty());
        assert!(debugger.current_coroutine.is_none());
        assert!(!debugger.break_on_all);
    }

    #[test]
    fn test_coroutine_status_enum() {
        let running = CoroutineStatus::Running;
        let suspended = CoroutineStatus::Suspended;
        let dead = CoroutineStatus::Dead;
        let error = CoroutineStatus::Error;

        assert_ne!(running, suspended);
        assert_ne!(suspended, dead);
        assert_ne!(dead, error);
    }

    #[test]
    fn test_coroutine_switching() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: Some("Test Coroutine".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);
        assert!(debugger.switch_to_coroutine("test-coroutine").is_ok());
        assert_eq!(
            debugger.current_coroutine,
            Some("test-coroutine".to_string())
        );
    }

    #[test]
    fn test_coroutine_enumeration() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine1 = Coroutine {
            id: "coroutine-1".to_string(),
            name: Some("Coroutine 1".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        let coroutine2 = Coroutine {
            id: "coroutine-2".to_string(),
            name: Some("Coroutine 2".to_string()),
            status: CoroutineStatus::Running,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine1);
        debugger.update_coroutine(coroutine2);

        let coroutines = debugger.enumerate_coroutines();
        assert_eq!(coroutines.len(), 2);
    }

    #[test]
    fn test_get_coroutine() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: Some("Test Coroutine".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);

        assert!(debugger.get_coroutine("test-coroutine").is_some());
        assert!(debugger.get_coroutine("non-existent").is_none());
    }

    #[test]
    fn test_remove_coroutine() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: Some("Test Coroutine".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);
        assert!(debugger.get_coroutine("test-coroutine").is_some());

        debugger.remove_coroutine("test-coroutine");
        assert!(debugger.get_coroutine("test-coroutine").is_none());
    }

    #[test]
    fn test_set_current_coroutine() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: Some("Test Coroutine".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);

        assert!(debugger
            .set_current_coroutine(Some("test-coroutine".to_string()))
            .is_ok());
        assert_eq!(
            debugger.current_coroutine,
            Some("test-coroutine".to_string())
        );

        assert!(debugger
            .set_current_coroutine(Some("non-existent".to_string()))
            .is_err());
    }

    #[test]
    fn test_should_break() {
        let mut debugger = CoroutineDebugger::new();

        // By default, should break only on main coroutine (None)
        assert!(debugger.should_break());

        // Enable break on all
        debugger.set_break_on_all(true);
        assert!(debugger.should_break());

        // Disable break on all
        debugger.set_break_on_all(false);
        assert!(debugger.should_break());
    }

    #[test]
    fn test_set_coroutine_name() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: None,
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);

        assert!(debugger
            .set_coroutine_name("test-coroutine", "New Name".to_string())
            .is_ok());
        assert_eq!(
            debugger.get_coroutine("test-coroutine").unwrap().name,
            Some("New Name".to_string())
        );

        assert!(debugger
            .set_coroutine_name("non-existent", "New Name".to_string())
            .is_err());
    }

    #[test]
    fn test_format_coroutine_frame() {
        let mut debugger = CoroutineDebugger::new();

        let coroutine = Coroutine {
            id: "test-coroutine".to_string(),
            name: Some("Test Coroutine".to_string()),
            status: CoroutineStatus::Suspended,
            stack_trace: None,
        };

        debugger.update_coroutine(coroutine);

        let frame = StackFrame {
            function_name: "test_function".to_string(),
            source_file: "test.lua".to_string(),
            line_number: 10,
        };

        let formatted = debugger.format_coroutine_frame("test-coroutine", &frame);
        assert_eq!(formatted, "test_function (Test Coroutine)");

        let formatted_unknown = debugger.format_coroutine_frame("unknown-coroutine", &frame);
        assert_eq!(formatted_unknown, "test_function");
    }

    #[test]
    fn test_enable_break_on_all() {
        let mut debugger = CoroutineDebugger::new();

        assert!(!debugger.break_on_all);
        debugger.enable_break_on_all(true);
        assert!(debugger.break_on_all);
        debugger.enable_break_on_all(false);
        assert!(!debugger.break_on_all);
    }
}
