//! Main integration test file
//!
//! This file runs all Phase 2 integration tests for the Wayfinder debugger

// Import all test modules
mod lua_version_tests;
mod dap_protocol_tests;
mod breakpoint_tests;
mod variable_inspection_tests;
mod expression_evaluation_tests;

// Re-export the modules so they get compiled and run
pub use lua_version_tests::*;
pub use dap_protocol_tests::*;
pub use breakpoint_tests::*;
pub use variable_inspection_tests::*;
pub use expression_evaluation_tests::*;

#[cfg(test)]
mod tests {
    // This will run all the tests from the imported modules
    use super::*;
    
    #[test]
    fn test_all_modules_compiled() {
        // This is just a placeholder test to ensure all modules are included
        assert!(true);
    }
}