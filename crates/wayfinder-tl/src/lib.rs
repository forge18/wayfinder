//! Wayfinder TypeScript-to-Lua Source Map Integration
//!
//! This crate provides source map translation capabilities for debugging
//! TypeScript-to-Lua compiled code, enabling seamless debugging experience
//! in the original TypeScript source files.

pub mod source_map;
pub mod translator;
pub mod dap_wrapper;
pub mod coroutine;
pub mod config;

// Re-export base64 for convenience
pub use base64;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}