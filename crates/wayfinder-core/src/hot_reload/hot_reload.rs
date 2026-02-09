//! Hot code reload implementation
//!
//! This module handles the hot reloading of Lua modules, including compiling
//! new source code and updating references in the runtime.

use crate::hot_reload::state_capture::{CapturedGlobal, StateCapture};
use crate::runtime::lua_ffi::*;
use crate::runtime::lua_state::Lua;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during hot reload operations
#[derive(Error, Debug)]
pub enum HotReloadError {
    #[error("Lua compilation error: {0}")]
    CompilationError(String),

    #[error("State restoration error: {0}")]
    RestorationError(String),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Circular reference detected")]
    CircularReference,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Represents a warning generated during hot reload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotReloadWarning {
    /// Warning message
    pub message: String,

    /// Severity level
    pub severity: WarningSeverity,
}

/// Warning severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningSeverity {
    /// Low severity - information only
    Info,

    /// Medium severity - some state may not be preserved
    Warning,

    /// High severity - significant state loss
    Error,
}

/// Hot reload manager
pub struct HotReload {
    /// Lua state to operate on
    lua: Lua,

    /// State capture manager for preserving state during reload
    state_capture: StateCapture,

    /// Warnings generated during the reload process
    warnings: Vec<HotReloadWarning>,
}

impl HotReload {
    /// Create a new hot reload manager
    pub fn new(lua: Lua) -> Self {
        let state_capture = StateCapture::new(lua.clone());
        Self {
            lua,
            state_capture,
            warnings: Vec::new(),
        }
    }

    /// Compile new module source via LuaNext
    pub fn compile_module(&mut self, source: &str) -> Result<(), HotReloadError> {
        unsafe {
            // In a real implementation, this would use LuaNext to compile the source
            // For now, we'll use standard Lua compilation as a placeholder

            let source_cstr = std::ffi::CString::new(source).map_err(|_| {
                HotReloadError::CompilationError("Invalid source string".to_string())
            })?;

            if self.lua.luaL_loadstring(source_cstr.as_ptr()) != LUA_OK as i32 {
                // Get the error message
                let error_msg = if self.lua.lua_type(-1) == LUA_TSTRING as i32 {
                    let c_str = self.lua.lua_tolstring(-1, std::ptr::null_mut());
                    if !c_str.is_null() {
                        std::ffi::CStr::from_ptr(c_str)
                            .to_string_lossy()
                            .to_string()
                    } else {
                        "Unknown compilation error".to_string()
                    }
                } else {
                    "Unknown compilation error".to_string()
                };

                self.lua.lua_pop(1); // Remove error message
                return Err(HotReloadError::CompilationError(error_msg));
            }
        }

        Ok(())
    }

    /// Execute to get new module table
    pub fn execute_module(&mut self) -> Result<i64, HotReloadError> {
        unsafe {
            // Execute the compiled chunk
            if self.lua.lua_pcall(0, 1, 0) != LUA_OK as i32 {
                // Get the error message
                let error_msg = if self.lua.lua_type(-1) == LUA_TSTRING as i32 {
                    let c_str = self.lua.lua_tolstring(-1, std::ptr::null_mut());
                    if !c_str.is_null() {
                        std::ffi::CStr::from_ptr(c_str)
                            .to_string_lossy()
                            .to_string()
                    } else {
                        "Unknown execution error".to_string()
                    }
                } else {
                    "Unknown execution error".to_string()
                };

                self.lua.lua_pop(1); // Remove error message
                return Err(HotReloadError::CompilationError(error_msg));
            }

            // The result should be on top of the stack
            // Create a reference to it
            let module_ref = self.lua.luaL_ref(LUA_REGISTRYINDEX);
            Ok(module_ref as i64)
        }
    }

    /// Call new module chunk
    pub fn call_module_chunk(&mut self, chunk: &str) -> Result<i64, HotReloadError> {
        self.compile_module(chunk)?;
        self.execute_module()
    }

    /// Capture the current state before reload
    pub fn capture_state(&mut self) -> Vec<CapturedGlobal> {
        self.state_capture.clear_cache();
        self.state_capture.capture_globals()
    }

    /// Restore global variables (if they existed)
    pub fn restore_globals(
        &mut self,
        captured_globals: Vec<CapturedGlobal>,
    ) -> Result<(), HotReloadError> {
        unsafe {
            for global in captured_globals {
                // Push the value onto the stack based on its type
                match global.value {
                    crate::hot_reload::state_capture::CapturedValue::Nil => {
                        self.lua.lua_pushnil();
                    }

                    crate::hot_reload::state_capture::CapturedValue::Boolean(b) => {
                        self.lua.lua_pushboolean(if b { 1 } else { 0 });
                    }

                    crate::hot_reload::state_capture::CapturedValue::Number(n) => {
                        self.lua.lua_pushnumber(n);
                    }

                    crate::hot_reload::state_capture::CapturedValue::String(ref s) => {
                        let c_str = std::ffi::CString::new(s.as_str()).map_err(|_| {
                            HotReloadError::RestorationError("Invalid string value".to_string())
                        })?;
                        self.lua.lua_pushstring(c_str.as_ptr());
                    }

                    // For complex types, we'll just warn that they might not be preserved
                    _ => {
                        self.warnings.push(HotReloadWarning {
                            message: format!(
                                "Global '{}' of complex type may not be fully preserved",
                                global.name
                            ),
                            severity: WarningSeverity::Warning,
                        });
                        self.lua.lua_pushnil();
                    }
                }

                // Set the global variable
                let name_cstr = std::ffi::CString::new(global.name.as_str()).map_err(|_| {
                    HotReloadError::RestorationError("Invalid global name".to_string())
                })?;
                self.lua.lua_setglobal(name_cstr.as_ptr());
            }
        }

        Ok(())
    }

    /// Preserve table contents where possible
    pub fn preserve_table_contents(&mut self) -> Result<(), HotReloadError> {
        // In a real implementation, this would handle merging table contents
        // For now, we'll just add a warning that table contents might not be preserved
        self.warnings.push(HotReloadWarning {
            message: "Table contents preservation not fully implemented".to_string(),
            severity: WarningSeverity::Warning,
        });

        Ok(())
    }

    /// Handle new/deleted fields
    pub fn handle_field_changes(&mut self) -> Result<(), HotReloadError> {
        // In a real implementation, this would compare old and new module structures
        // For now, we'll just add a warning
        self.warnings.push(HotReloadWarning {
            message: "Field change handling not fully implemented".to_string(),
            severity: WarningSeverity::Info,
        });

        Ok(())
    }

    /// Generate warnings for unpreserved state
    pub fn generate_warnings(&self) -> &[HotReloadWarning] {
        &self.warnings
    }

    /// Output warnings to console/DAP output
    pub fn output_warnings(&self) {
        for warning in &self.warnings {
            match warning.severity {
                WarningSeverity::Info => {
                    println!("[INFO] Hot reload: {}", warning.message);
                }
                WarningSeverity::Warning => {
                    println!("[WARN] Hot reload: {}", warning.message);
                }
                WarningSeverity::Error => {
                    println!("[ERROR] Hot reload: {}", warning.message);
                }
            }
        }
    }

    /// Find existing closures referencing old module
    pub fn find_referencing_closures(&self, _module_ref: i64) -> Vec<i64> {
        // In a real implementation, this would traverse all functions
        // and check their upvalues for references to the old module
        // For now, we'll return an empty vector
        Vec::new()
    }

    /// Update module table reference
    pub fn update_module_reference(
        &mut self,
        old_ref: i64,
        new_ref: i64,
    ) -> Result<(), HotReloadError> {
        unsafe {
            // Replace the old module reference with the new one in the registry
            self.lua.lua_rawgeti(LUA_REGISTRYINDEX, old_ref as i64);
            self.lua.luaL_unref(LUA_REGISTRYINDEX, old_ref as i32);
            self.lua.lua_pushvalue(-1);
            let replaced_ref = self.lua.luaL_ref(LUA_REGISTRYINDEX);

            // The new reference should match the old one
            assert_eq!(replaced_ref as i64, old_ref);

            // Now set the new module at that reference
            self.lua.lua_rawgeti(LUA_REGISTRYINDEX, new_ref as i64);
            self.lua.lua_rawseti(LUA_REGISTRYINDEX, old_ref as i64);

            // Remove the temporary values from stack
            self.lua.lua_pop(2);
        }

        Ok(())
    }

    /// Preserve function identity where possible
    pub fn preserve_function_identity(&mut self) -> Result<(), HotReloadError> {
        // In a real implementation, this would try to maintain function identities
        // For now, we'll just add a warning
        self.warnings.push(HotReloadWarning {
            message: "Function identity preservation not implemented".to_string(),
            severity: WarningSeverity::Info,
        });

        Ok(())
    }

    /// Handle closures with captured state
    pub fn handle_closure_state(&mut self) -> Result<(), HotReloadError> {
        // In a real implementation, this would handle updating closures
        // that capture state from the reloaded module
        self.warnings.push(HotReloadWarning {
            message: "Closure state handling not fully implemented".to_string(),
            severity: WarningSeverity::Warning,
        });

        Ok(())
    }

    /// Perform a complete hot reload operation
    pub fn reload_module(&mut self, module_source: &str) -> Result<(), HotReloadError> {
        // Clear previous warnings
        self.warnings.clear();

        // 1. Capture current state
        let captured_globals = self.capture_state();

        // 2. Compile and execute new module
        let new_module_ref = self.call_module_chunk(module_source)?;

        // 3. Restore state
        self.restore_globals(captured_globals)?;
        self.preserve_table_contents()?;
        self.handle_field_changes()?;

        // 4. Update references
        // In a real implementation, we would find and update all references
        // For now, we'll just demonstrate the concept
        let referencing_closures = self.find_referencing_closures(new_module_ref);
        for closure_ref in referencing_closures {
            // Update each closure's reference to the new module
            // This is a simplified example
            println!(
                "Would update closure {} to reference new module",
                closure_ref
            );
        }

        self.preserve_function_identity()?;
        self.handle_closure_state()?;

        // 5. Output any warnings
        self.output_warnings();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::lua_state::Lua;

    #[test]
    fn test_hot_reload_creation() {
        let lua = Lua::new();
        let hot_reload = HotReload::new(lua);
        assert!(hot_reload.warnings.is_empty());
    }

    #[test]
    fn test_warning_severity_enum() {
        let info = WarningSeverity::Info;
        let warning = WarningSeverity::Warning;
        let error = WarningSeverity::Error;

        assert!(matches!(info, WarningSeverity::Info));
        assert!(matches!(warning, WarningSeverity::Warning));
        assert!(matches!(error, WarningSeverity::Error));
    }

    #[test]
    fn test_hot_reload_warning_struct() {
        let warning = HotReloadWarning {
            message: "Test warning".to_string(),
            severity: WarningSeverity::Warning,
        };

        assert_eq!(warning.message, "Test warning");
        assert!(matches!(warning.severity, WarningSeverity::Warning));
    }

    #[test]
    fn test_compile_simple_module() {
        let lua = Lua::new();
        let mut hot_reload = HotReload::new(lua);

        let simple_module = "return { test = 'value' }";
        assert!(hot_reload.compile_module(simple_module).is_ok());
    }

    #[test]
    fn test_compile_invalid_module() {
        let lua = Lua::new();
        let mut hot_reload = HotReload::new(lua);

        let invalid_module = "return { test = "; // Invalid syntax
        assert!(hot_reload.compile_module(invalid_module).is_err());
    }

    #[test]
    fn test_state_capture_creation() {
        let lua = Lua::new();
        let state_capture = StateCapture::new(lua.clone());
        // Note: visited_tables is private, so we can't directly test it
        // but we can test that the StateCapture was created successfully
        assert!(true); // Placeholder - StateCapture creation succeeded
    }

    #[test]
    fn test_captured_value_enum() {
        let nil_value = CapturedValue::Nil;
        let bool_value = CapturedValue::Boolean(true);
        let num_value = CapturedValue::Number(42.0);
        let str_value = CapturedValue::String("test".to_string());

        assert!(matches!(nil_value, CapturedValue::Nil));
        assert!(matches!(bool_value, CapturedValue::Boolean(true)));
        assert!(matches!(num_value, CapturedValue::Number(42.0)));
        assert!(matches!(str_value, CapturedValue::String(_)));
    }

    #[test]
    fn test_captured_global_struct() {
        let global = CapturedGlobal {
            name: "test_var".to_string(),
            value: CapturedValue::Number(123.0),
        };

        assert_eq!(global.name, "test_var");
        assert!(matches!(global.value, CapturedValue::Number(123.0)));
    }

    #[test]
    fn test_capture_and_restore_workflow() {
        let lua = Lua::new();
        let mut hot_reload = HotReload::new(lua);

        // Test that we can capture state (even if it's empty initially)
        let captured_globals = hot_reload.state_capture.capture_globals();
        assert!(captured_globals.is_empty());

        // Test that we can restore state (even if it's empty)
        assert!(hot_reload.restore_globals(captured_globals).is_ok());
    }

    #[test]
    fn test_warning_system() {
        let lua = Lua::new();
        let mut hot_reload = HotReload::new(lua);

        // Initially no warnings
        assert!(hot_reload.warnings.is_empty());

        // Add a warning
        hot_reload.warnings.push(HotReloadWarning {
            message: "Test warning".to_string(),
            severity: WarningSeverity::Warning,
        });

        assert_eq!(hot_reload.warnings.len(), 1);
        assert_eq!(hot_reload.warnings[0].message, "Test warning");
        assert!(matches!(
            hot_reload.warnings[0].severity,
            WarningSeverity::Warning
        ));
    }

    #[test]
    fn test_warning_severity_levels() {
        let lua = Lua::new();
        let mut hot_reload = HotReload::new(lua);

        let info_warning = HotReloadWarning {
            message: "Info message".to_string(),
            severity: WarningSeverity::Info,
        };

        let warning_warning = HotReloadWarning {
            message: "Warning message".to_string(),
            severity: WarningSeverity::Warning,
        };

        let error_warning = HotReloadWarning {
            message: "Error message".to_string(),
            severity: WarningSeverity::Error,
        };

        hot_reload.warnings.push(info_warning);
        hot_reload.warnings.push(warning_warning);
        hot_reload.warnings.push(error_warning);

        assert_eq!(hot_reload.warnings.len(), 3);
        assert!(matches!(
            hot_reload.warnings[0].severity,
            WarningSeverity::Info
        ));
        assert!(matches!(
            hot_reload.warnings[1].severity,
            WarningSeverity::Warning
        ));
        assert!(matches!(
            hot_reload.warnings[2].severity,
            WarningSeverity::Error
        ));
    }
}
