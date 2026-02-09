//! Dynamic Lua library loader
//!
//! This module provides dynamic loading of Lua libraries at runtime,
//! allowing a single binary to support multiple Lua versions (5.1-5.4).

use super::LuaVersion;
use super::lua_ffi::{lua_Debug, luaL_Reg};
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use libloading::{Library, Symbol};

pub type LuaState = *mut c_void;
pub type LuaCFunction = extern "C" fn(*mut c_void) -> c_int;
pub type LuaHook = extern "C" fn(*mut c_void, *mut lua_Debug);
pub type lua_Integer = i64;
pub type lua_Number = f64;
pub type size_t = usize;

// Lua registry constants (consistent across versions)
const LUA_REGISTRYINDEX: c_int = -10000;
const LUA_RIDX_GLOBALS: c_int = 2;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Failed to load Lua library: {0}")]
    LoadFailed(String),

    #[error("Failed to find symbol {0}: {1}")]
    SymbolNotFound(String, String),

    #[error("Unsupported Lua version: {0}")]
    UnsupportedVersion(String),
}

/// Dynamically loaded Lua library
///
/// This struct holds function pointers to all Lua C API functions loaded at runtime.
/// It uses Arc internally so it can be cheaply cloned and shared across threads.
#[derive(Clone)]
pub struct LuaLibrary {
    inner: Arc<LuaLibraryInner>,
}

struct LuaLibraryInner {
    _lib: Library,
    version: LuaVersion,

    // Core API functions - required in all versions
    lua_close: Symbol<'static, unsafe extern "C" fn(LuaState)>,
    lua_newthread: Symbol<'static, unsafe extern "C" fn(LuaState) -> LuaState>,
    lua_gettop: Symbol<'static, unsafe extern "C" fn(LuaState) -> c_int>,
    lua_settop: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_pushvalue: Symbol<'static, unsafe extern "C" fn(LuaState, c_int)>,
    lua_type: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_typename: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> *const c_char>,
    lua_tonumber: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> lua_Number>,
    lua_tointeger: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> lua_Integer>,
    lua_toboolean: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_tolstring: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, *mut size_t) -> *const c_char>,
    lua_pushnil: Symbol<'static, unsafe extern "C" fn(LuaState)>,
    lua_pushnumber: Symbol<'static, unsafe extern "C" fn(LuaState, lua_Number)>,
    lua_pushinteger: Symbol<'static, unsafe extern "C" fn(LuaState, lua_Integer)>,
    lua_pushlstring: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char, size_t)>,
    lua_pushstring: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char)>,
    lua_pushboolean: Symbol<'static, unsafe extern "C" fn(LuaState, c_int)>,
    lua_getglobal: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char) -> c_int>,
    lua_setglobal: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char)>,
    lua_gettable: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_getfield: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, *const c_char) -> c_int>,
    lua_settable: Symbol<'static, unsafe extern "C" fn(LuaState, c_int)>,
    lua_setfield: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, *const c_char)>,
    lua_rawget: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_rawgeti: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int) -> c_int>,
    lua_rawset: Symbol<'static, unsafe extern "C" fn(LuaState, c_int)>,
    lua_createtable: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int)>,
    lua_getmetatable: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_setmetatable: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_next: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,

    // Debug API - required in all versions
    lua_sethook: Symbol<'static, unsafe extern "C" fn(LuaState, LuaHook, c_int, c_int)>,
    lua_getinfo: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char, *mut lua_Debug) -> c_int>,
    lua_getlocal: Symbol<'static, unsafe extern "C" fn(LuaState, *mut lua_Debug, c_int) -> *const c_char>,
    lua_setlocal: Symbol<'static, unsafe extern "C" fn(LuaState, *mut lua_Debug, c_int) -> *const c_char>,
    lua_getupvalue: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int) -> *const c_char>,
    lua_setupvalue: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int) -> *const c_char>,
    lua_getstack: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, *mut lua_Debug) -> c_int>,
    lua_gethook: Symbol<'static, unsafe extern "C" fn(LuaState) -> LuaHook>,
    lua_gethookmask: Symbol<'static, unsafe extern "C" fn(LuaState) -> c_int>,
    lua_gethookcount: Symbol<'static, unsafe extern "C" fn(LuaState) -> c_int>,

    // Auxiliary library - required in all versions
    lual_openlibs: Symbol<'static, unsafe extern "C" fn(LuaState)>,
    lual_newstate: Symbol<'static, unsafe extern "C" fn() -> LuaState>,
    lual_loadstring: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char) -> c_int>,
    lual_traceback: Symbol<'static, unsafe extern "C" fn(LuaState, LuaState, *const c_char, c_int)>,
    lual_newmetatable: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char) -> c_int>,
    lual_setmetatable: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char)>,
    lua_pushcclosure: Symbol<'static, unsafe extern "C" fn(LuaState, LuaCFunction, c_int)>,
    lua_isnumber: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_isstring: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_topointer: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> *const c_void>,
    lua_error: Symbol<'static, unsafe extern "C" fn(LuaState) -> !>,
    lua_newuserdata: Symbol<'static, unsafe extern "C" fn(LuaState, size_t) -> *mut c_void>,
    lua_checkstack: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lua_upvalueid: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int) -> *mut c_void>,
    lual_ref: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lual_unref: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int)>,

    // Version-specific optional functions (5.2+)
    lua_pcallk: Option<Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int, c_int, c_long, Option<unsafe extern "C" fn(*mut c_void, c_int)>) -> c_int>>,
    lua_pushglobaltable: Option<Symbol<'static, unsafe extern "C" fn(LuaState)>>,
    lual_loadbufferx: Option<Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char, size_t, *const c_char, *const c_char) -> c_int>>,

    // Lua 5.1-specific functions (deprecated in 5.2+)
    lua_pcall: Option<Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int, c_int) -> c_int>>,
    lua_objlen: Option<Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> size_t>>,
    lual_loadbuffer: Option<Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char, size_t, *const c_char) -> c_int>>,
}

impl LuaLibrary {
    /// Load a Lua library for the specified version
    pub fn load(version: LuaVersion) -> Result<Self, LoaderError> {
        let lib_path = Self::find_library(version)?;

        unsafe {
            let lib = Library::new(&lib_path)
                .map_err(|e| LoaderError::LoadFailed(format!("{}: {}", lib_path.display(), e)))?;

            // Leak the library to get 'static lifetime
            let lib_static = Box::leak(Box::new(lib));

            // Load version-specific optional symbols
            let lua_pcallk_opt = Self::load_symbol_optional(lib_static, b"lua_pcallk\0");
            let lua_pcall_opt = Self::load_symbol_optional(lib_static, b"lua_pcall\0");
            let lua_pushglobaltable_opt = Self::load_symbol_optional(lib_static, b"lua_pushglobaltable\0");
            let lual_loadbufferx_opt = Self::load_symbol_optional(lib_static, b"luaL_loadbufferx\0");
            let lual_loadbuffer_opt = Self::load_symbol_optional(lib_static, b"luaL_loadbuffer\0");
            let lua_objlen_opt = Self::load_symbol_optional(lib_static, b"lua_objlen\0");

            let inner = LuaLibraryInner {
                _lib: std::ptr::read(lib_static as *const Library),
                version,

                // Load all required function pointers (available in all Lua versions 5.1-5.4)
                lua_close: Self::load_symbol(lib_static, b"lua_close\0")?,
                lua_newthread: Self::load_symbol(lib_static, b"lua_newthread\0")?,
                lua_gettop: Self::load_symbol(lib_static, b"lua_gettop\0")?,
                lua_settop: Self::load_symbol(lib_static, b"lua_settop\0")?,
                lua_pushvalue: Self::load_symbol(lib_static, b"lua_pushvalue\0")?,
                lua_type: Self::load_symbol(lib_static, b"lua_type\0")?,
                lua_typename: Self::load_symbol(lib_static, b"lua_typename\0")?,
                lua_tonumber: Self::load_symbol(lib_static, b"lua_tonumber\0")?,
                lua_tointeger: Self::load_symbol(lib_static, b"lua_tointeger\0")?,
                lua_toboolean: Self::load_symbol(lib_static, b"lua_toboolean\0")?,
                lua_tolstring: Self::load_symbol(lib_static, b"lua_tolstring\0")?,
                lua_pushnil: Self::load_symbol(lib_static, b"lua_pushnil\0")?,
                lua_pushnumber: Self::load_symbol(lib_static, b"lua_pushnumber\0")?,
                lua_pushinteger: Self::load_symbol(lib_static, b"lua_pushinteger\0")?,
                lua_pushlstring: Self::load_symbol(lib_static, b"lua_pushlstring\0")?,
                lua_pushstring: Self::load_symbol(lib_static, b"lua_pushstring\0")?,
                lua_pushboolean: Self::load_symbol(lib_static, b"lua_pushboolean\0")?,
                lua_getglobal: Self::load_symbol(lib_static, b"lua_getglobal\0")?,
                lua_setglobal: Self::load_symbol(lib_static, b"lua_setglobal\0")?,
                lua_gettable: Self::load_symbol(lib_static, b"lua_gettable\0")?,
                lua_getfield: Self::load_symbol(lib_static, b"lua_getfield\0")?,
                lua_settable: Self::load_symbol(lib_static, b"lua_settable\0")?,
                lua_setfield: Self::load_symbol(lib_static, b"lua_setfield\0")?,
                lua_rawget: Self::load_symbol(lib_static, b"lua_rawget\0")?,
                lua_rawgeti: Self::load_symbol(lib_static, b"lua_rawgeti\0")?,
                lua_rawset: Self::load_symbol(lib_static, b"lua_rawset\0")?,
                lua_createtable: Self::load_symbol(lib_static, b"lua_createtable\0")?,
                lua_getmetatable: Self::load_symbol(lib_static, b"lua_getmetatable\0")?,
                lua_setmetatable: Self::load_symbol(lib_static, b"lua_setmetatable\0")?,
                lua_next: Self::load_symbol(lib_static, b"lua_next\0")?,
                lua_sethook: Self::load_symbol(lib_static, b"lua_sethook\0")?,
                lua_getinfo: Self::load_symbol(lib_static, b"lua_getinfo\0")?,
                lua_getlocal: Self::load_symbol(lib_static, b"lua_getlocal\0")?,
                lua_setlocal: Self::load_symbol(lib_static, b"lua_setlocal\0")?,
                lua_getupvalue: Self::load_symbol(lib_static, b"lua_getupvalue\0")?,
                lua_setupvalue: Self::load_symbol(lib_static, b"lua_setupvalue\0")?,
                lua_getstack: Self::load_symbol(lib_static, b"lua_getstack\0")?,
                lua_gethook: Self::load_symbol(lib_static, b"lua_gethook\0")?,
                lua_gethookmask: Self::load_symbol(lib_static, b"lua_gethookmask\0")?,
                lua_gethookcount: Self::load_symbol(lib_static, b"lua_gethookcount\0")?,
                lual_openlibs: Self::load_symbol(lib_static, b"luaL_openlibs\0")?,
                lual_newstate: Self::load_symbol(lib_static, b"luaL_newstate\0")?,
                lual_loadstring: Self::load_symbol(lib_static, b"luaL_loadstring\0")?,
                lual_traceback: Self::load_symbol(lib_static, b"luaL_traceback\0")?,
                lual_newmetatable: Self::load_symbol(lib_static, b"luaL_newmetatable\0")?,
                lual_setmetatable: Self::load_symbol(lib_static, b"luaL_setmetatable\0")?,
                lua_pushcclosure: Self::load_symbol(lib_static, b"lua_pushcclosure\0")?,
                lua_isnumber: Self::load_symbol(lib_static, b"lua_isnumber\0")?,
                lua_isstring: Self::load_symbol(lib_static, b"lua_isstring\0")?,
                lua_topointer: Self::load_symbol(lib_static, b"lua_topointer\0")?,
                lua_error: Self::load_symbol(lib_static, b"lua_error\0")?,
                lua_newuserdata: Self::load_symbol(lib_static, b"lua_newuserdata\0")?,
                lua_checkstack: Self::load_symbol(lib_static, b"lua_checkstack\0")?,
                lua_upvalueid: Self::load_symbol(lib_static, b"lua_upvalueid\0")?,
                lual_ref: Self::load_symbol(lib_static, b"luaL_ref\0")?,
                lual_unref: Self::load_symbol(lib_static, b"luaL_unref\0")?,

                // Version-specific optional symbols
                lua_pcallk: lua_pcallk_opt,
                lua_pushglobaltable: lua_pushglobaltable_opt,
                lual_loadbufferx: lual_loadbufferx_opt,
                lua_pcall: lua_pcall_opt,
                lua_objlen: lua_objlen_opt,
                lual_loadbuffer: lual_loadbuffer_opt,
            };

            Ok(Self {
                inner: Arc::new(inner),
            })
        }
    }

    /// Find the Lua library path for the specified version
    fn find_library(version: LuaVersion) -> Result<PathBuf, LoaderError> {
        let version_str = match version {
            LuaVersion::V51 => "5.1",
            LuaVersion::V52 => "5.2",
            LuaVersion::V53 => "5.3",
            LuaVersion::V54 => "5.4",
        };

        // Get project root directory (where Cargo.toml is)
        let project_lua_libs = std::env::current_exe()
            .ok()
            .and_then(|exe| {
                exe.ancestors()
                    .find(|p| p.join("Cargo.toml").exists())
                    .map(|p| p.join("lua-libs"))
            });

        // Try different naming conventions and paths
        #[cfg(target_os = "macos")]
        let mut candidates = vec![];

        // First try project-local lua-libs directory
        #[cfg(target_os = "macos")]
        if let Some(ref lua_libs) = project_lua_libs {
            candidates.push(format!("{}/liblua{}.dylib", lua_libs.display(), version_str));
        }

        #[cfg(target_os = "macos")]
        candidates.extend(vec![
            format!("/opt/homebrew/lib/liblua{}.dylib", version_str),
            format!("/opt/homebrew/lib/liblua{}.so", version_str.replace(".", "")),
            format!("/usr/local/lib/liblua{}.dylib", version_str),
            format!("/usr/local/lib/liblua{}.so", version_str.replace(".", "")),
            format!("/usr/lib/liblua{}.dylib", version_str),
            format!("liblua{}.dylib", version_str),
        ]);

        #[cfg(target_os = "linux")]
        let mut candidates = vec![];

        // First try project-local lua-libs directory
        #[cfg(target_os = "linux")]
        if let Some(ref lua_libs) = project_lua_libs {
            candidates.push(format!("{}/liblua{}.so", lua_libs.display(), version_str));
        }

        #[cfg(target_os = "linux")]
        candidates.extend(vec![
            format!("/usr/lib/x86_64-linux-gnu/liblua{}.so", version_str),
            format!("/usr/lib/liblua{}.so", version_str),
            format!("/usr/local/lib/liblua{}.so", version_str),
            format!("liblua{}.so", version_str),
        ]);

        #[cfg(target_os = "windows")]
        let mut candidates = vec![];

        #[cfg(target_os = "windows")]
        if let Some(ref lua_libs) = project_lua_libs {
            candidates.push(format!("{}\\lua{}.dll", lua_libs.display(), version_str.replace(".", "")));
        }

        #[cfg(target_os = "windows")]
        candidates.extend(vec![
            format!("lua{}.dll", version_str.replace(".", "")),
            format!("lua{}.dll", version_str),
        ]);

        for candidate in &candidates {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return Ok(path);
            }
        }

        Err(LoaderError::LoadFailed(format!(
            "Could not find Lua {} library. Tried: {:?}",
            version_str,
            candidates
        )))
    }

    /// Load a required symbol from the library
    unsafe fn load_symbol<T>(lib: &'static Library, name: &[u8]) -> Result<Symbol<'static, T>, LoaderError> {
        lib.get(name)
            .map_err(|e| LoaderError::SymbolNotFound(
                String::from_utf8_lossy(name).to_string(),
                e.to_string()
            ))
    }

    /// Load an optional symbol from the library (returns None if not found)
    unsafe fn load_symbol_optional<T>(lib: &'static Library, name: &[u8]) -> Option<Symbol<'static, T>> {
        lib.get(name).ok()
    }

    pub fn version(&self) -> LuaVersion {
        self.inner.version
    }

    // Provide safe wrappers for all Lua C API functions
    pub unsafe fn lua_close(&self, l: LuaState) {
        (self.inner.lua_close)(l)
    }

    pub unsafe fn luaL_newstate(&self) -> LuaState {
        (self.inner.lual_newstate)()
    }

    pub unsafe fn luaL_openlibs(&self, l: LuaState) {
        (self.inner.lual_openlibs)(l)
    }

    pub unsafe fn lua_getstack(&self, l: LuaState, level: c_int, ar: *mut lua_Debug) -> c_int {
        (self.inner.lua_getstack)(l, level, ar)
    }

    pub unsafe fn lua_getinfo(&self, l: LuaState, what: *const c_char, ar: *mut lua_Debug) -> c_int {
        (self.inner.lua_getinfo)(l, what, ar)
    }

    pub unsafe fn lua_getlocal(&self, l: LuaState, ar: *mut lua_Debug, n: c_int) -> *const c_char {
        (self.inner.lua_getlocal)(l, ar, n)
    }

    pub unsafe fn lua_setlocal(&self, l: LuaState, ar: *mut lua_Debug, n: c_int) -> *const c_char {
        (self.inner.lua_setlocal)(l, ar, n)
    }

    pub unsafe fn lua_getupvalue(&self, l: LuaState, funcindex: c_int, n: c_int) -> *const c_char {
        (self.inner.lua_getupvalue)(l, funcindex, n)
    }

    pub unsafe fn lua_setupvalue(&self, l: LuaState, funcindex: c_int, n: c_int) -> *const c_char {
        (self.inner.lua_setupvalue)(l, funcindex, n)
    }

    pub unsafe fn lua_sethook(&self, l: LuaState, f: LuaHook, mask: c_int, count: c_int) {
        (self.inner.lua_sethook)(l, f, mask, count)
    }

    pub unsafe fn lua_gethook(&self, l: LuaState) -> LuaHook {
        (self.inner.lua_gethook)(l)
    }

    pub unsafe fn lua_gethookmask(&self, l: LuaState) -> c_int {
        (self.inner.lua_gethookmask)(l)
    }

    pub unsafe fn lua_gethookcount(&self, l: LuaState) -> c_int {
        (self.inner.lua_gethookcount)(l)
    }

    pub unsafe fn lua_gettop(&self, l: LuaState) -> c_int {
        (self.inner.lua_gettop)(l)
    }

    pub unsafe fn lua_settop(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_settop)(l, idx)
    }

    pub unsafe fn lua_type(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_type)(l, idx)
    }

    pub unsafe fn lua_tolstring(&self, l: LuaState, idx: c_int, len: *mut size_t) -> *const c_char {
        (self.inner.lua_tolstring)(l, idx, len)
    }

    pub unsafe fn lua_pushnil(&self, l: LuaState) {
        (self.inner.lua_pushnil)(l)
    }

    pub unsafe fn lua_pushnumber(&self, l: LuaState, n: lua_Number) {
        (self.inner.lua_pushnumber)(l, n)
    }

    pub unsafe fn lua_pushstring(&self, l: LuaState, s: *const c_char) {
        (self.inner.lua_pushstring)(l, s)
    }

    pub unsafe fn lua_pushboolean(&self, l: LuaState, b: c_int) {
        (self.inner.lua_pushboolean)(l, b)
    }

    pub unsafe fn lua_getglobal(&self, l: LuaState, name: *const c_char) -> c_int {
        (self.inner.lua_getglobal)(l, name)
    }

    pub unsafe fn lua_setglobal(&self, l: LuaState, name: *const c_char) {
        (self.inner.lua_setglobal)(l, name)
    }

    pub unsafe fn lua_getfield(&self, l: LuaState, idx: c_int, k: *const c_char) -> c_int {
        (self.inner.lua_getfield)(l, idx, k)
    }

    pub unsafe fn lua_setfield(&self, l: LuaState, idx: c_int, k: *const c_char) {
        (self.inner.lua_setfield)(l, idx, k)
    }

    pub unsafe fn lua_rawgeti(&self, l: LuaState, idx: c_int, n: c_int) -> c_int {
        (self.inner.lua_rawgeti)(l, idx, n)
    }

    pub unsafe fn lua_createtable(&self, l: LuaState, narr: c_int, nrec: c_int) {
        (self.inner.lua_createtable)(l, narr, nrec)
    }

    pub unsafe fn lua_next(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_next)(l, idx)
    }

    pub unsafe fn lua_pcallk(&self, l: LuaState, nargs: c_int, nresults: c_int, msgh: c_int, ctx: c_long, k: Option<unsafe extern "C" fn(*mut c_void, c_int)>) -> c_int {
        if let Some(ref f) = self.inner.lua_pcallk {
            // Lua 5.2+: use native lua_pcallk with continuation support
            f(l, nargs, nresults, msgh, ctx, k)
        } else if let Some(ref f) = self.inner.lua_pcall {
            // Lua 5.1: continuations not supported, ignore ctx and k parameters
            if k.is_some() {
                eprintln!("Warning: Continuation functions are not supported in Lua 5.1");
            }
            f(l, nargs, nresults, msgh)
        } else {
            panic!("Neither lua_pcallk nor lua_pcall available in Lua library")
        }
    }

    pub unsafe fn luaL_loadstring(&self, l: LuaState, s: *const c_char) -> c_int {
        (self.inner.lual_loadstring)(l, s)
    }

    pub unsafe fn lua_tonumber(&self, l: LuaState, idx: c_int) -> f64 {
        (self.inner.lua_tonumber)(l, idx)
    }

    pub unsafe fn lua_toboolean(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_toboolean)(l, idx)
    }

    pub unsafe fn lua_pcall(&self, l: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int {
        if let Some(ref f) = self.inner.lua_pcallk {
            // Lua 5.2+: use lua_pcallk with no continuation
            f(l, nargs, nresults, msgh, 0, None)
        } else if let Some(ref f) = self.inner.lua_pcall {
            // Lua 5.1: use native lua_pcall
            f(l, nargs, nresults, msgh)
        } else {
            panic!("Neither lua_pcallk nor lua_pcall available in Lua library")
        }
    }

    pub unsafe fn lua_gettable(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_gettable)(l, idx)
    }

    pub unsafe fn lua_getmetatable(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_getmetatable)(l, idx)
    }

    pub unsafe fn lua_rawseti(&self, l: LuaState, idx: c_int, n: i64) {
        (self.inner.lua_rawset)(l, idx);
        (self.inner.lua_pushnumber)(l, n as f64);
    }

    pub unsafe fn lua_pushvalue(&self, l: LuaState, idx: c_int) {
        (self.inner.lua_pushvalue)(l, idx)
    }

    pub unsafe fn lua_pushglobaltable(&self, l: LuaState) {
        if let Some(ref f) = self.inner.lua_pushglobaltable {
            // Lua 5.2+ has native lua_pushglobaltable
            f(l)
        } else {
            // Lua 5.1 compatibility: push globals table from registry
            (self.inner.lua_rawgeti)(l, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
        }
    }

    pub unsafe fn luaL_ref(&self, l: LuaState, t: c_int) -> c_int {
        (self.inner.lual_ref)(l, t)
    }

    pub unsafe fn luaL_unref(&self, l: LuaState, t: c_int, ref_: c_int) {
        (self.inner.lual_unref)(l, t, ref_)
    }

    pub unsafe fn lual_unref(&self, l: LuaState, t: c_int, ref_: c_int) {
        (self.inner.lual_unref)(l, t, ref_)
    }

    pub unsafe fn lual_newstate(&self) -> LuaState {
        (self.inner.lual_newstate)()
    }

    pub unsafe fn lual_openlibs(&self, l: LuaState) {
        (self.inner.lual_openlibs)(l)
    }

    pub unsafe fn lual_loadstring(&self, l: LuaState, s: *const c_char) -> c_int {
        (self.inner.lual_loadstring)(l, s)
    }

    pub unsafe fn lual_loadfilex(&self, l: LuaState, filename: *const c_char, mode: *const c_char) -> c_int {
        if let Some(ref f) = self.inner.lual_loadbufferx {
            // Lua 5.2+: use luaL_loadbufferx with mode parameter
            f(l, filename, 0, filename, mode)
        } else if let Some(ref f) = self.inner.lual_loadbuffer {
            // Lua 5.1: use luaL_loadbuffer (ignores mode parameter)
            if !mode.is_null() {
                eprintln!("Warning: Mode parameter ignored in Lua 5.1");
            }
            f(l, filename, 0, filename)
        } else {
            panic!("Neither luaL_loadbufferx nor luaL_loadbuffer available in Lua library")
        }
    }

    pub unsafe fn lua_pushinteger(&self, l: LuaState, n: lua_Integer) {
        (self.inner.lua_pushinteger)(l, n)
    }

    pub unsafe fn lua_pushcclosure(&self, l: LuaState, f: LuaCFunction, n: c_int) {
        (self.inner.lua_pushcclosure)(l, f, n)
    }

    pub unsafe fn lua_tointeger(&self, l: LuaState, idx: c_int) -> lua_Integer {
        (self.inner.lua_tointeger)(l, idx)
    }

    pub unsafe fn lua_isnumber(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_isnumber)(l, idx)
    }

    pub unsafe fn lua_isstring(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_isstring)(l, idx)
    }

    pub unsafe fn lua_typename(&self, l: LuaState, tp: c_int) -> *const c_char {
        (self.inner.lua_typename)(l, tp)
    }

    pub unsafe fn lual_traceback(&self, l: LuaState, l1: LuaState, msg: *const c_char, level: c_int) {
        (self.inner.lual_traceback)(l, l1, msg, level)
    }

    pub unsafe fn lua_topointer(&self, l: LuaState, idx: c_int) -> *const c_void {
        (self.inner.lua_topointer)(l, idx)
    }

    pub unsafe fn lua_error(&self, l: LuaState) -> ! {
        (self.inner.lua_error)(l)
    }

    pub unsafe fn lua_newuserdata(&self, l: LuaState, size: size_t) -> *mut c_void {
        (self.inner.lua_newuserdata)(l, size)
    }

    pub unsafe fn lua_checkstack(&self, l: LuaState, extra: c_int) -> c_int {
        (self.inner.lua_checkstack)(l, extra)
    }

    pub unsafe fn lual_ref(&self, l: LuaState, t: c_int) -> c_int {
        (self.inner.lual_ref)(l, t)
    }

    pub unsafe fn lua_upvalueid(&self, l: LuaState, fidx: c_int, n: c_int) -> *mut c_void {
        (self.inner.lua_upvalueid)(l, fidx, n)
    }

    pub unsafe fn lua_setmetatable(&self, l: LuaState, idx: c_int) -> c_int {
        (self.inner.lua_setmetatable)(l, idx)
    }

    pub unsafe fn lual_newmetatable(&self, l: LuaState, tname: *const c_char) -> c_int {
        (self.inner.lual_newmetatable)(l, tname)
    }

    pub unsafe fn lual_setmetatable(&self, l: LuaState, tname: *const c_char) {
        (self.inner.lual_setmetatable)(l, tname)
    }
}

// For backwards compatibility with static FFI code
pub use super::lua_ffi::*;
