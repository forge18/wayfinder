//! Hot code reload functionality
//!
//! This module provides hot code reload capabilities for Lua modules,
//! allowing developers to modify and reload code without restarting
//! the debugging session.

pub mod hot_reload;
pub mod state_capture;

// Re-export the main types for convenience
pub use hot_reload::{HotReload, HotReloadError, HotReloadWarning, WarningSeverity};
pub use state_capture::{CapturedGlobal, CapturedTable, CapturedValue, StateCapture};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_structure() {
        // Simple test to ensure the module structure is correct
        assert!(true);
    }
}
