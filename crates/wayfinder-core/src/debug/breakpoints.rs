//! Breakpoint management for the Wayfinder debugger
//!
//! This module handles different types of breakpoints:
//! - Line breakpoints
//! - Function breakpoints
//! - Exception breakpoints
//! - Conditional breakpoints
//! - Logpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a line breakpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineBreakpoint {
    /// Unique identifier for the breakpoint
    pub id: i64,
    /// The source file path
    pub source: String,
    /// The line number in the source file
    pub line: u32,
    /// Optional condition that must be true for the breakpoint to trigger
    pub condition: Option<String>,
    /// Optional log message to output instead of pausing execution
    pub log_message: Option<String>,
    /// Optional hit condition (e.g., "> 5" to trigger after 5 hits)
    pub hit_condition: Option<String>,
    /// Whether the breakpoint is verified (successfully set in the runtime)
    pub verified: bool,
    /// Optional message about the breakpoint status
    pub message: Option<String>,
}

/// Represents a function breakpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionBreakpoint {
    /// Unique identifier for the breakpoint
    pub id: i64,
    /// The name of the function to break on
    pub name: String,
    /// Optional condition that must be true for the breakpoint to trigger
    pub condition: Option<String>,
    /// Whether the breakpoint is verified (successfully set in the runtime)
    pub verified: bool,
    /// Optional message about the breakpoint status
    pub message: Option<String>,
}

/// Represents an exception breakpoint filter
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExceptionBreakpointFilter {
    /// The filter name (e.g., "all", "uncaught")
    pub filter: String,
    /// Human-readable label for the filter
    pub label: String,
    /// Description of what this filter does
    pub description: String,
    /// Whether this filter supports conditions
    pub supports_condition: bool,
    /// Whether this filter supports hit conditions
    pub supports_hit_condition: bool,
}

/// Manages all breakpoints for a debugging session
#[derive(Debug, Clone)]
pub struct BreakpointManager {
    /// Line breakpoints organized by source file
    line_breakpoints: HashMap<String, Vec<LineBreakpoint>>,
    /// Function breakpoints
    function_breakpoints: Vec<FunctionBreakpoint>,
    /// Active exception breakpoint filters
    exception_filters: Vec<String>,
    /// Next ID to assign to a breakpoint
    next_id: i64,
}

impl BreakpointManager {
    /// Creates a new breakpoint manager
    pub fn new() -> Self {
        Self {
            line_breakpoints: HashMap::new(),
            function_breakpoints: Vec::new(),
            exception_filters: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds or updates line breakpoints for a source file
    pub fn set_line_breakpoints(
        &mut self,
        source: String,
        breakpoints: Vec<LineBreakpoint>,
    ) -> Vec<LineBreakpoint> {
        // Assign IDs to new breakpoints
        let mut breakpoints_with_ids = Vec::new();
        for mut bp in breakpoints {
            if bp.id == 0 {
                bp.id = self.next_id;
                self.next_id += 1;
            }
            breakpoints_with_ids.push(bp);
        }

        // Replace all breakpoints for this source
        self.line_breakpoints
            .insert(source, breakpoints_with_ids.clone());

        breakpoints_with_ids
    }

    /// Gets all line breakpoints for a source file
    pub fn get_line_breakpoints(&self, source: &str) -> Option<&Vec<LineBreakpoint>> {
        self.line_breakpoints.get(source)
    }

    /// Gets all line breakpoints across all sources
    pub fn get_all_line_breakpoints(&self) -> Vec<&LineBreakpoint> {
        self.line_breakpoints.values().flatten().collect()
    }

    /// Removes all line breakpoints for a source file
    pub fn clear_line_breakpoints(&mut self, source: &str) {
        self.line_breakpoints.remove(source);
    }

    /// Adds or updates function breakpoints
    pub fn set_function_breakpoints(
        &mut self,
        breakpoints: Vec<FunctionBreakpoint>,
    ) -> Vec<FunctionBreakpoint> {
        // Assign IDs to new breakpoints
        let mut breakpoints_with_ids = Vec::new();
        for mut bp in breakpoints {
            if bp.id == 0 {
                bp.id = self.next_id;
                self.next_id += 1;
            }
            breakpoints_with_ids.push(bp);
        }

        // Replace all function breakpoints
        self.function_breakpoints = breakpoints_with_ids.clone();

        breakpoints_with_ids
    }

    /// Gets all function breakpoints
    pub fn get_function_breakpoints(&self) -> &Vec<FunctionBreakpoint> {
        &self.function_breakpoints
    }

    /// Sets the active exception breakpoint filters
    pub fn set_exception_breakpoints(&mut self, filters: Vec<String>) {
        self.exception_filters = filters;
    }

    /// Gets the active exception breakpoint filters
    pub fn get_exception_breakpoints(&self) -> &Vec<String> {
        &self.exception_filters
    }

    /// Checks if a line breakpoint exists at the specified source and line
    pub fn has_line_breakpoint(&self, source: &str, line: u32) -> bool {
        if let Some(breakpoints) = self.line_breakpoints.get(source) {
            breakpoints.iter().any(|bp| bp.line == line)
        } else {
            false
        }
    }

    /// Finds a line breakpoint at the specified source and line
    pub fn find_line_breakpoint(&self, source: &str, line: u32) -> Option<&LineBreakpoint> {
        if let Some(breakpoints) = self.line_breakpoints.get(source) {
            breakpoints.iter().find(|bp| bp.line == line)
        } else {
            None
        }
    }

    /// Finds a function breakpoint by name
    pub fn find_function_breakpoint(&self, name: &str) -> Option<&FunctionBreakpoint> {
        self.function_breakpoints.iter().find(|bp| bp.name == name)
    }

    /// Removes a breakpoint by ID
    pub fn remove_breakpoint(&mut self, id: i64) -> bool {
        // Try to remove from line breakpoints
        for breakpoints in self.line_breakpoints.values_mut() {
            if let Some(pos) = breakpoints.iter().position(|bp| bp.id == id) {
                breakpoints.remove(pos);
                return true;
            }
        }

        // Try to remove from function breakpoints
        if let Some(pos) = self.function_breakpoints.iter().position(|bp| bp.id == id) {
            self.function_breakpoints.remove(pos);
            return true;
        }

        false
    }

    /// Clears all breakpoints
    pub fn clear_all_breakpoints(&mut self) {
        self.line_breakpoints.clear();
        self.function_breakpoints.clear();
        self.exception_filters.clear();
    }

    /// Gets the total count of all breakpoints
    pub fn breakpoint_count(&self) -> usize {
        self.line_breakpoints
            .values()
            .map(|v| v.len())
            .sum::<usize>()
            + self.function_breakpoints.len()
    }
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_manager_creation() {
        let manager = BreakpointManager::new();
        assert_eq!(manager.breakpoint_count(), 0);
    }

    #[test]
    fn test_line_breakpoints() {
        let mut manager = BreakpointManager::new();

        let breakpoints = vec![LineBreakpoint {
            id: 0,
            source: "test.lua".to_string(),
            line: 10,
            condition: None,
            log_message: None,
            hit_condition: None,
            verified: true,
            message: None,
        }];

        let result = manager.set_line_breakpoints("test.lua".to_string(), breakpoints);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1); // Should be assigned ID 1

        let retrieved = manager.get_line_breakpoints("test.lua");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().len(), 1);

        assert!(manager.has_line_breakpoint("test.lua", 10));
        assert!(!manager.has_line_breakpoint("test.lua", 15));
        assert!(!manager.has_line_breakpoint("other.lua", 10));
    }

    #[test]
    fn test_function_breakpoints() {
        let mut manager = BreakpointManager::new();

        let breakpoints = vec![FunctionBreakpoint {
            id: 0,
            name: "main".to_string(),
            condition: None,
            verified: true,
            message: None,
        }];

        let result = manager.set_function_breakpoints(breakpoints);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1); // Should be assigned ID 1

        let retrieved = manager.get_function_breakpoints();
        assert_eq!(retrieved.len(), 1);

        let found = manager.find_function_breakpoint("main");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "main");
    }

    #[test]
    fn test_exception_breakpoints() {
        let mut manager = BreakpointManager::new();

        let filters = vec!["all".to_string(), "uncaught".to_string()];
        manager.set_exception_breakpoints(filters.clone());

        let retrieved = manager.get_exception_breakpoints();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0], "all");
        assert_eq!(retrieved[1], "uncaught");
    }

    #[test]
    fn test_breakpoint_removal() {
        let mut manager = BreakpointManager::new();

        // Add line breakpoint
        let line_breakpoints = vec![LineBreakpoint {
            id: 0,
            source: "test.lua".to_string(),
            line: 10,
            condition: None,
            log_message: None,
            hit_condition: None,
            verified: true,
            message: None,
        }];
        manager.set_line_breakpoints("test.lua".to_string(), line_breakpoints);

        // Add function breakpoint
        let func_breakpoints = vec![FunctionBreakpoint {
            id: 0,
            name: "main".to_string(),
            condition: None,
            verified: true,
            message: None,
        }];
        manager.set_function_breakpoints(func_breakpoints);

        assert_eq!(manager.breakpoint_count(), 2);

        // Remove line breakpoint
        assert!(manager.remove_breakpoint(1));
        assert_eq!(manager.breakpoint_count(), 1);

        // Remove function breakpoint
        assert!(manager.remove_breakpoint(2));
        assert_eq!(manager.breakpoint_count(), 0);

        // Try to remove non-existent breakpoint
        assert!(!manager.remove_breakpoint(999));
    }

    #[test]
    fn test_clear_breakpoints() {
        let mut manager = BreakpointManager::new();

        // Add line breakpoint
        let line_breakpoints = vec![LineBreakpoint {
            id: 0,
            source: "test.lua".to_string(),
            line: 10,
            condition: None,
            log_message: None,
            hit_condition: None,
            verified: true,
            message: None,
        }];
        manager.set_line_breakpoints("test.lua".to_string(), line_breakpoints);

        // Add function breakpoint
        let func_breakpoints = vec![FunctionBreakpoint {
            id: 0,
            name: "main".to_string(),
            condition: None,
            verified: true,
            message: None,
        }];
        manager.set_function_breakpoints(func_breakpoints);

        // Set exception breakpoints
        manager.set_exception_breakpoints(vec!["all".to_string()]);

        assert_eq!(manager.breakpoint_count(), 2);
        assert_eq!(manager.get_exception_breakpoints().len(), 1);

        // Clear all
        manager.clear_all_breakpoints();

        assert_eq!(manager.breakpoint_count(), 0);
        assert_eq!(manager.get_exception_breakpoints().len(), 0);
    }
}
