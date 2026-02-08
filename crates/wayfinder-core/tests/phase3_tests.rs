//! Main Phase 3 Test File
//!
//! This file runs all Phase 3 integration tests for the Wayfinder debugger

// Import all Phase 3 test modules
mod phase3_watchpoint_tests;
mod phase3_evaluate_tests;
mod phase3_integration_tests;

// Re-export the modules so they get compiled and run
pub use phase3_watchpoint_tests::*;
pub use phase3_evaluate_tests::*;
pub use phase3_integration_tests::*;

#[cfg(test)]
mod tests {
    // This will run all the tests from the imported modules
    use super::*;
    
    #[test]
    fn test_all_phase3_modules_compiled() {
        // This is just a placeholder test to ensure all modules are included
        assert!(true);
    }
}