//! Hot reload service trait and implementations
//!
//! This module defines the service layer approach for hot code reload functionality.
//! Each runtime implementation provides its own hot reload service that has direct
//! access to the underlying Lua state.

use crate::hot_reload::{HotReloadError, HotReloadWarning};
use async_trait::async_trait;

/// Result of a hot reload operation
#[derive(Debug, Clone)]
pub struct HotReloadResult {
    /// Whether the reload was successful
    pub success: bool,
    
    /// Warnings generated during the reload process
    pub warnings: Vec<HotReloadWarning>,
    
    /// Optional message describing the result
    pub message: Option<String>,
}

/// Hot reload service trait
///
/// This trait defines the interface for hot reload services that can be
/// implemented by each runtime type. Unlike the generic DebugRuntime trait,
/// hot reload services have direct access to the underlying Lua state and
/// can perform FFI operations needed for hot reloading.
#[async_trait]
pub trait HotReloadService: Send + Sync {
    /// Perform a hot reload of a module
    ///
    /// This method compiles and executes new module source code,
    /// preserves state where possible, and updates references.
    ///
    /// # Arguments
    /// * `module_source` - The new source code for the module
    /// * `module_name` - Optional name of the module being reloaded
    ///
    /// # Returns
    /// A result indicating success or failure of the reload operation
    async fn reload_module(
        &mut self,
        module_source: &str,
        module_name: Option<&str>,
    ) -> Result<HotReloadResult, HotReloadError>;
    
    /// Check if hot reload is supported by this runtime
    ///
    /// # Returns
    /// True if hot reload is supported, false otherwise
    fn is_supported(&self) -> bool {
        true
    }
    
    /// Get information about hot reload capabilities
    ///
    /// # Returns
    /// A string describing the hot reload capabilities of this service
    fn capabilities(&self) -> String {
        "Full hot reload support".to_string()
    }
}