//! Watchpoint handling for data breakpoints
//!
//! This module provides functionality to monitor variable values
//! and trigger breakpoints when they change.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a data breakpoint (watchpoint)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataBreakpoint {
    /// Unique identifier for the watchpoint
    pub id: i64,
    /// The variable name or path to watch
    pub name: String,
    /// Optional condition that must be true for the watchpoint to trigger
    pub condition: Option<String>,
    /// Optional hit condition (e.g., "> 5" to trigger after 5 hits)
    pub hit_condition: Option<String>,
    /// Whether the watchpoint is verified (successfully set in the runtime)
    pub verified: bool,
    /// Optional message about the watchpoint status
    pub message: Option<String>,
    /// Number of times this watchpoint has been hit
    #[serde(skip)]
    pub hit_count: usize,
    /// The data type being watched
    pub data_type: DataType,
    /// Access type to monitor
    pub access_type: AccessType,
}

/// Types of data that can be watched
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// Watch a local variable
    Local,
    /// Watch a global variable
    Global,
    /// Watch an upvalue
    Upvalue,
    /// Watch a table field
    TableField { table_ref: i64, field: String },
}

/// Types of access to monitor
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessType {
    /// Monitor when the value is read
    Read,
    /// Monitor when the value is written
    Write,
    /// Monitor when the value is read or written
    ReadWrite,
}

/// Manages all watchpoints for a debugging session
#[derive(Debug, Clone)]
pub struct WatchpointManager {
    /// Data breakpoints organized by ID
    data_breakpoints: HashMap<i64, DataBreakpoint>,
    /// Next ID to assign to a watchpoint
    next_id: i64,
}

impl WatchpointManager {
    /// Creates a new watchpoint manager
    pub fn new() -> Self {
        Self {
            data_breakpoints: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds or updates data breakpoints
    pub fn set_data_breakpoints(
        &mut self,
        breakpoints: Vec<DataBreakpoint>,
    ) -> Vec<DataBreakpoint> {
        // Assign IDs to new breakpoints
        let mut breakpoints_with_ids = Vec::new();
        for mut bp in breakpoints {
            if bp.id == 0 {
                bp.id = self.next_id;
                self.next_id += 1;
            }
            // Initialize hit count to 0 for new breakpoints
            bp.hit_count = 0;
            breakpoints_with_ids.push(bp);
        }

        // Replace all data breakpoints
        self.data_breakpoints.clear();
        for bp in &breakpoints_with_ids {
            self.data_breakpoints.insert(bp.id, bp.clone());
        }

        breakpoints_with_ids
    }

    /// Gets all data breakpoints
    pub fn get_data_breakpoints(&self) -> Vec<&DataBreakpoint> {
        self.data_breakpoints.values().collect()
    }

    /// Finds a data breakpoint by ID
    pub fn find_data_breakpoint(&self, id: i64) -> Option<&DataBreakpoint> {
        self.data_breakpoints.get(&id)
    }

    /// Removes a data breakpoint by ID
    pub fn remove_data_breakpoint(&mut self, id: i64) -> bool {
        self.data_breakpoints.remove(&id).is_some()
    }

    /// Clears all data breakpoints
    pub fn clear_all_data_breakpoints(&mut self) {
        self.data_breakpoints.clear();
    }

    /// Gets the total count of all data breakpoints
    pub fn data_breakpoint_count(&self) -> usize {
        self.data_breakpoints.len()
    }

    /// Increments the hit count for a data breakpoint
    pub fn increment_data_breakpoint_hit_count(&mut self, id: i64) -> bool {
        if let Some(bp) = self.data_breakpoints.get_mut(&id) {
            bp.hit_count += 1;
            true
        } else {
            false
        }
    }

    /// Gets the hit count for a data breakpoint
    pub fn get_data_breakpoint_hit_count(&self, id: i64) -> Option<usize> {
        self.data_breakpoints.get(&id).map(|bp| bp.hit_count)
    }
}

impl Default for WatchpointManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchpoint_manager_creation() {
        let manager = WatchpointManager::new();
        assert_eq!(manager.data_breakpoint_count(), 0);
    }

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
        }];

        let result = manager.set_data_breakpoints(breakpoints);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1); // Should be assigned ID 1

        let retrieved = manager.get_data_breakpoints();
        assert_eq!(retrieved.len(), 1);

        let found = manager.find_data_breakpoint(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "x");
    }

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
        }];

        let result = manager.set_data_breakpoints(breakpoints);
        assert_eq!(manager.data_breakpoint_count(), 1);

        assert!(manager.remove_data_breakpoint(1));
        assert_eq!(manager.data_breakpoint_count(), 0);

        // Try to remove non-existent breakpoint
        assert!(!manager.remove_data_breakpoint(999));
    }

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
        }];

        manager.set_data_breakpoints(breakpoints);
        assert_eq!(manager.data_breakpoint_count(), 1);

        manager.clear_all_data_breakpoints();
        assert_eq!(manager.data_breakpoint_count(), 0);
    }

    #[test]
    fn test_hit_counting() {
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
}
