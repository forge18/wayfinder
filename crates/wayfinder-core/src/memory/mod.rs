use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Memory statistics from the Lua garbage collector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    /// Total memory in kilobytes
    pub total_kb: f64,
    /// Total memory in bytes
    pub total_bytes: usize,
    /// GC pause setting
    pub gc_pause: i32,
    /// GC step multiplier
    pub gc_step_mul: i32,
    /// Whether garbage collection is currently running
    pub gc_running: bool,
    /// Timestamp when the statistics were collected
    pub timestamp: SystemTime,
}

/// Information about a single Lua object in the heap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    /// Unique identifier for the object
    pub id: i64,
    /// Type name (table, function, userdata, etc)
    pub type_name: String,
    /// Estimated size in bytes
    pub size_estimate: usize,
    /// Memory address as a string
    pub address: String,
}

/// Count of objects by type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectCounts {
    /// Number of tables
    pub tables: usize,
    /// Number of functions
    pub functions: usize,
    /// Number of userdata objects
    pub userdata: usize,
    /// Number of threads
    pub threads: usize,
    /// Number of strings
    pub strings: usize,
}

/// A snapshot of the heap at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapSnapshot {
    /// Unique ID for this snapshot
    pub id: u64,
    /// When the snapshot was taken
    pub timestamp: SystemTime,
    /// Memory statistics at snapshot time
    pub statistics: MemoryStatistics,
    /// Count of objects by type
    pub object_counts: ObjectCounts,
    /// List of objects in the heap
    pub objects: Vec<ObjectInfo>,
}

/// Difference between two heap snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDiff {
    /// ID of the older snapshot
    pub from_id: u64,
    /// ID of the newer snapshot
    pub to_id: u64,
    /// Change in memory in kilobytes
    pub memory_delta_kb: f64,
    /// Changes in object counts by type
    pub object_count_deltas: HashMap<String, i64>,
    /// Objects that appeared in the newer snapshot
    pub new_objects: Vec<ObjectInfo>,
    /// Objects that disappeared
    pub deleted_objects: Vec<ObjectInfo>,
}
