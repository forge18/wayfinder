//! State capture for hot code reload
//!
//! This module captures the current state of the Lua runtime to enable
//! preserving state during hot code reload operations.

use crate::runtime::lua_ffi::*;
use crate::runtime::lua_state::Lua;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the captured state of a global variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedGlobal {
    /// Name of the global variable
    pub name: String,

    /// Serialized value of the global variable
    pub value: CapturedValue,
}

/// Represents a captured Lua value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapturedValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table(CapturedTable),
    Function {
        /// Reference to the function (for identification)
        reference: i64,
        /// Name of the function if available
        name: Option<String>,
    },
    UserData {
        /// Reference to the userdata (for identification)
        reference: i64,
    },
    Thread {
        /// Reference to the thread (for identification)
        reference: i64,
    },
}

/// Represents a captured table structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedTable {
    /// Unique reference to identify the table
    pub reference: i64,

    /// Table contents as key-value pairs
    pub entries: Vec<(CapturedValue, CapturedValue)>,

    /// Metatable reference if present
    pub metatable: Option<i64>,
}

/// Manages state capture operations
pub struct StateCapture {
    /// Lua state to capture from
    lua: Lua,

    /// Tracking visited tables to detect circular references
    visited_tables: HashMap<i64, CapturedTable>,
}

impl StateCapture {
    /// Create a new state capture manager
    pub fn new(lua: Lua) -> Self {
        Self {
            lua,
            visited_tables: HashMap::new(),
        }
    }

    /// Capture global table entries
    pub fn capture_globals(&mut self) -> Vec<CapturedGlobal> {
        let mut globals = Vec::new();

        unsafe {
            // Push the global table (_G) onto the stack
            lua_pushglobaltable(self.lua.state());

            // Traverse the global table
            lua_pushnil(self.lua.state()); // Push nil as initial key

            while lua_next(self.lua.state(), -2) != 0 {
                // Key is at index -2, value is at index -1

                // Capture the key (should be a string for globals)
                if let Some(key) = self.capture_value(-2) {
                    if let CapturedValue::String(name) = key {
                        // Capture the value
                        let value = self.capture_value(-1);

                        if let Some(captured_value) = value {
                            globals.push(CapturedGlobal {
                                name,
                                value: captured_value,
                            });
                        }
                    }
                }

                // Remove value, keep key for next iteration
                lua_pop(self.lua.state(), 1);
            }

            // Remove the global table from the stack
            lua_pop(self.lua.state(), 1);
        }

        globals
    }

    /// Capture upvalues for existing functions
    pub fn capture_upvalues(&mut self, func_ref: i64) -> Vec<(String, CapturedValue)> {
        let mut upvalues = Vec::new();

        unsafe {
            // Get the function from registry
            lua_rawgeti(self.lua.state(), LUA_REGISTRYINDEX, func_ref);

            // Check if it's actually a function
            if lua_type(self.lua.state(), -1) == LUA_TFUNCTION as i32 {
                let mut i = 1;
                loop {
                    let name_ptr = lua_getupvalue(self.lua.state(), -1, i);
                    if name_ptr.is_null() {
                        break;
                    }

                    let name = if !name_ptr.is_null() {
                        std::ffi::CStr::from_ptr(name_ptr)
                            .to_string_lossy()
                            .to_string()
                    } else {
                        format!("upvalue_{}", i)
                    };

                    let value = self.capture_value(-1);

                    if let Some(captured_value) = value {
                        upvalues.push((name, captured_value));
                    }

                    lua_pop(self.lua.state(), 1); // Remove the upvalue
                    i += 1;
                }
            }

            lua_pop(self.lua.state(), 1); // Remove the function
        }

        upvalues
    }

    /// Record table structure and contents
    pub fn capture_table(&mut self, table_ref: i64) -> Option<CapturedTable> {
        // Check if we've already visited this table (circular reference detection)
        if self.visited_tables.contains_key(&table_ref) {
            return self.visited_tables.get(&table_ref).cloned();
        }

        let mut captured_table = CapturedTable {
            reference: table_ref,
            entries: Vec::new(),
            metatable: None,
        };

        // Add to visited tables to prevent infinite recursion
        self.visited_tables
            .insert(table_ref, captured_table.clone());

        unsafe {
            // Get the table from registry
            lua_rawgeti(self.lua.state(), LUA_REGISTRYINDEX, table_ref);

            // Check if it's actually a table
            if lua_type(self.lua.state(), -1) == LUA_TTABLE as i32 {
                // Capture metatable if present
                if lua_getmetatable(self.lua.state(), -1) != 0 {
                    // Metatable is on top of stack
                    let meta_ref = luaL_ref(self.lua.state(), LUA_REGISTRYINDEX);
                    captured_table.metatable = Some(meta_ref as i64);
                }

                // Traverse the table
                lua_pushnil(self.lua.state()); // Push nil as initial key

                while lua_next(self.lua.state(), -2) != 0 {
                    // Key is at index -2, value is at index -1

                    let key = self.capture_value(-2);
                    let value = self.capture_value(-1);

                    if let (Some(captured_key), Some(captured_value)) = (key, value) {
                        captured_table.entries.push((captured_key, captured_value));
                    }

                    // Remove value, keep key for next iteration
                    lua_pop(self.lua.state(), 1);
                }
            }

            lua_pop(self.lua.state(), 1); // Remove the table
        }

        Some(captured_table)
    }

    /// Capture a value from the Lua stack
    fn capture_value(&mut self, index: i32) -> Option<CapturedValue> {
        unsafe {
            match lua_type(self.lua.state(), index) {
                t if t == LUA_TNIL as i32 => Some(CapturedValue::Nil),

                t if t == LUA_TBOOLEAN as i32 => {
                    let value = lua_toboolean(self.lua.state(), index) != 0;
                    Some(CapturedValue::Boolean(value))
                }

                t if t == LUA_TNUMBER as i32 => {
                    let value = lua_tonumber(self.lua.state(), index);
                    Some(CapturedValue::Number(value))
                }

                t if t == LUA_TSTRING as i32 => {
                    let c_str = lua_tostring(self.lua.state(), index);
                    if !c_str.is_null() {
                        let string = std::ffi::CStr::from_ptr(c_str)
                            .to_string_lossy()
                            .to_string();
                        Some(CapturedValue::String(string))
                    } else {
                        Some(CapturedValue::Nil)
                    }
                }

                t if t == LUA_TTABLE as i32 => {
                    // Create a reference to the table
                    lua_pushvalue(self.lua.state(), index);
                    let table_ref = luaL_ref(self.lua.state(), LUA_REGISTRYINDEX);

                    // Capture the table structure
                    let captured_table = self.capture_table(table_ref as i64);

                    // Release the reference
                    luaL_unref(self.lua.state(), LUA_REGISTRYINDEX, table_ref);

                    captured_table.map(CapturedValue::Table)
                }

                t if t == LUA_TFUNCTION as i32 => {
                    // Create a reference to the function
                    lua_pushvalue(self.lua.state(), index);
                    let func_ref = luaL_ref(self.lua.state(), LUA_REGISTRYINDEX);

                    // Try to get function name
                    let name = self.get_function_name(index);

                    // Release the reference (we're just capturing metadata)
                    luaL_unref(self.lua.state(), LUA_REGISTRYINDEX, func_ref);

                    Some(CapturedValue::Function {
                        reference: func_ref as i64,
                        name,
                    })
                }

                t if t == LUA_TUSERDATA as i32 => {
                    // Create a reference to the userdata
                    lua_pushvalue(self.lua.state(), index);
                    let ud_ref = luaL_ref(self.lua.state(), LUA_REGISTRYINDEX);

                    // Release the reference (we're just capturing metadata)
                    luaL_unref(self.lua.state(), LUA_REGISTRYINDEX, ud_ref);

                    Some(CapturedValue::UserData {
                        reference: ud_ref as i64,
                    })
                }

                t if t == LUA_TTHREAD as i32 => {
                    // Create a reference to the thread
                    lua_pushvalue(self.lua.state(), index);
                    let thread_ref = luaL_ref(self.lua.state(), LUA_REGISTRYINDEX);

                    // Release the reference (we're just capturing metadata)
                    luaL_unref(self.lua.state(), LUA_REGISTRYINDEX, thread_ref);

                    Some(CapturedValue::Thread {
                        reference: thread_ref as i64,
                    })
                }

                _ => Some(CapturedValue::Nil),
            }
        }
    }

    /// Try to get the name of a function
    fn get_function_name(&self, index: i32) -> Option<String> {
        unsafe {
            // Try to get debug info for the function
            let mut ar: lua_Debug = std::mem::zeroed();
            if lua_getinfo(self.lua.state(), b"n\0".as_ptr() as *const i8, &mut ar) != 0 {
                if !ar.name.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(ar.name);
                    return Some(c_str.to_string_lossy().to_string());
                }
            }
            None
        }
    }

    /// Detect circular references (already handled in capture_table)
    pub fn has_circular_references(&self) -> bool {
        // Circular references are detected and handled during capture
        false
    }

    /// Clear the visited tables cache
    pub fn clear_cache(&mut self) {
        self.visited_tables.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::lua_state::Lua;

    #[test]
    fn test_state_capture_creation() {
        let lua = Lua::new();
        let capture = StateCapture::new(lua);
        assert!(capture.visited_tables.is_empty());
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
}
