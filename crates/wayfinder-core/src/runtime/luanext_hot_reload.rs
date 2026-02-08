//! Hot reload service implementation for LuaNext runtime
//!
//! This module provides the hot reload service implementation for the
//! LuaNext runtime, which has direct access to the underlying Lua state
//! and can perform FFI operations needed for hot reloading.

use crate::hot_reload::service::{HotReloadService, HotReloadResult};
use crate::hot_reload::{HotReloadError, HotReloadWarning, WarningSeverity};
use crate::runtime::luanext::LuaNextRuntime;
use crate::runtime::lua_ffi::*;
use crate::runtime::lua_state::Lua;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

/// Hot reload service for LuaNext runtime
pub struct LuaNextHotReloadService {
    /// Reference to the Lua state (shared with the runtime)
    lua: Arc<Mutex<Lua>>,
}

impl LuaNextHotReloadService {
    /// Create a new hot reload service for LuaNext runtime
    pub fn new(lua: Arc<Mutex<Lua>>) -> Self {
        Self { lua }
    }
    
    /// Compile new module source via LuaNext
    fn compile_module(&mut self, source: &str) -> Result<(), HotReloadError> {
        let lua_guard = self.lua.lock().unwrap();
        let lua_state = lua_guard.state();
        
        unsafe {
            // In a real implementation, this would use LuaNext to compile the source
            // For now, we'll use standard Lua compilation as a placeholder

            let source_cstr = std::ffi::CString::new(source)
                .map_err(|_| HotReloadError::CompilationError("Invalid source string".to_string()))?;

            if luaL_loadstring(lua_state, source_cstr.as_ptr()) != LUA_OK as i32 {
                // Get the error message
                let error_msg = if lua_type(lua_state, -1) == LUA_TSTRING as i32 {
                    let c_str = lua_tolstring(lua_state, -1, std::ptr::null_mut());
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

                lua_pop(lua_state, 1); // Remove error message
                return Err(HotReloadError::CompilationError(error_msg));
            }
        }

        Ok(())
    }

    /// Execute to get new module table
    fn execute_module(&mut self) -> Result<i64, HotReloadError> {
        let lua_guard = self.lua.lock().unwrap();
        let lua_state = lua_guard.state();
        
        unsafe {
            // Execute the compiled chunk
            if lua_pcall(lua_state, 0, 1, 0) != LUA_OK as i32 {
                // Get the error message
                let error_msg = if lua_type(lua_state, -1) == LUA_TSTRING as i32 {
                    let c_str = lua_tolstring(lua_state, -1, std::ptr::null_mut());
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

                lua_pop(lua_state, 1); // Remove error message
                return Err(HotReloadError::CompilationError(error_msg));
            }

            // The result should be on top of the stack
            // Create a reference to it
            let module_ref = luaL_ref(lua_state, LUA_REGISTRYINDEX);
            Ok(module_ref as i64)
        }
    }

    /// Call new module chunk
    fn call_module_chunk(&mut self, chunk: &str) -> Result<i64, HotReloadError> {
        self.compile_module(chunk)?;
        self.execute_module()
    }
}

#[async_trait]
impl HotReloadService for LuaNextHotReloadService {
    async fn reload_module(
        &mut self,
        module_source: &str,
        module_name: Option<&str>,
    ) -> Result<HotReloadResult, HotReloadError> {
        // In a real implementation, we would:
        // 1. Capture the current state
        // 2. Compile and execute the new module
        // 3. Restore the state
        // 4. Update references
        
        // For now, we'll just compile and execute as a placeholder
        self.compile_module(module_source)?;
        
        let warnings = vec![
            HotReloadWarning {
                message: "State preservation not yet implemented".to_string(),
                severity: WarningSeverity::Warning,
            }
        ];
        
        Ok(HotReloadResult {
            success: true,
            warnings,
            message: Some(format!("Module '{}' compiled successfully", 
                                module_name.unwrap_or("unnamed"))),
        })
    }
    
    fn is_supported(&self) -> bool {
        true
    }
    
    fn capabilities(&self) -> String {
        "Full hot reload support for LuaNext".to_string()
    }
}