//! Lua runtime initialization
//!
//! This module handles loading Lua libraries and initializing the runtime environment.

use super::{LuaVersion, lua_loader::{LuaLibrary, LoaderError}};
use std::sync::OnceLock;

/// Global Lua library instance
/// This is initialized once at startup based on the requested Lua version
static LUA_LIBRARY: OnceLock<LuaLibrary> = OnceLock::new();

/// Initialize the Lua library for the given version
pub fn init_lua(version: LuaVersion) -> Result<(), LoaderError> {
    if LUA_LIBRARY.get().is_some() {
        // Already initialized
        return Ok(());
    }

    let lib = LuaLibrary::load(version)?;
    LUA_LIBRARY.set(lib).map_err(|_| LoaderError::LoadFailed("Lua library already initialized".to_string()))?;

    Ok(())
}

/// Get the initialized Lua library
pub fn get_lua() -> Option<&'static LuaLibrary> {
    LUA_LIBRARY.get()
}

/// Check if Lua has been initialized
pub fn is_initialized() -> bool {
    LUA_LIBRARY.get().is_some()
}
