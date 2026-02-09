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

    // Core API functions - load all of them dynamically
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
    lua_pcallk: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int, c_int, c_long, Option<unsafe extern "C" fn(*mut c_void, c_int)>) -> c_int>,
    lua_next: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,

    // Debug API
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

    // Auxiliary library
    lual_openlibs: Symbol<'static, unsafe extern "C" fn(LuaState)>,
    lual_newstate: Symbol<'static, unsafe extern "C" fn() -> LuaState>,
    lual_loadbufferx: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char, size_t, *const c_char, *const c_char) -> c_int>,
    lual_loadstring: Symbol<'static, unsafe extern "C" fn(LuaState, *const c_char) -> c_int>,
    lua_pushglobaltable: Symbol<'static, unsafe extern "C" fn(LuaState)>,
    lual_ref: Symbol<'static, unsafe extern "C" fn(LuaState, c_int) -> c_int>,
    lual_unref: Symbol<'static, unsafe extern "C" fn(LuaState, c_int, c_int)>,
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

            let inner = LuaLibraryInner {
                _lib: std::ptr::read(lib_static as *const Library),
                version,

                // Load all function pointers
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
                lua_pcallk: Self::load_symbol(lib_static, b"lua_pcallk\0")?,
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
                lual_loadbufferx: Self::load_symbol(lib_static, b"luaL_loadbufferx\0")?,
                lual_loadstring: Self::load_symbol(lib_static, b"luaL_loadstring\0")?,
                lua_pushglobaltable: Self::load_symbol(lib_static, b"lua_pushglobaltable\0")?,
                lual_ref: Self::load_symbol(lib_static, b"luaL_ref\0")?,
                lual_unref: Self::load_symbol(lib_static, b"luaL_unref\0")?,
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

        // Try different naming conventions and paths
        #[cfg(target_os = "macos")]
        let candidates = vec![
            format!("/opt/homebrew/lib/liblua{}.dylib", version_str),
            format!("/opt/homebrew/lib/liblua{}.so", version_str.replace(".", "")),
            format!("/usr/local/lib/liblua{}.dylib", version_str),
            format!("/usr/local/lib/liblua{}.so", version_str.replace(".", "")),
            format!("/usr/lib/liblua{}.dylib", version_str),
            format!("liblua{}.dylib", version_str),
        ];

        #[cfg(target_os = "linux")]
        let candidates = vec![
            format!("/usr/lib/x86_64-linux-gnu/liblua{}.so", version_str),
            format!("/usr/lib/liblua{}.so", version_str),
            format!("/usr/local/lib/liblua{}.so", version_str),
            format!("liblua{}.so", version_str),
        ];

        #[cfg(target_os = "windows")]
        let candidates = vec![
            format!("lua{}.dll", version_str.replace(".", "")),
            format!("lua{}.dll", version_str),
        ];

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

    /// Load a symbol from the library
    unsafe fn load_symbol<T>(lib: &'static Library, name: &[u8]) -> Result<Symbol<'static, T>, LoaderError> {
        lib.get(name)
            .map_err(|e| LoaderError::SymbolNotFound(
                String::from_utf8_lossy(name).to_string(),
                e.to_string()
            ))
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
        (self.inner.lua_pcallk)(l, nargs, nresults, msgh, ctx, k)
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
        (self.inner.lua_pcallk)(l, nargs, nresults, msgh, 0, None)
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
        (self.inner.lua_pushglobaltable)(l)
    }

    pub unsafe fn luaL_ref(&self, l: LuaState, t: c_int) -> c_int {
        (self.inner.lual_ref)(l, t)
    }

    pub unsafe fn luaL_unref(&self, l: LuaState, t: c_int, ref_: c_int) {
        (self.inner.lual_unref)(l, t, ref_)
    }
}

// For backwards compatibility with static FFI code
pub use super::lua_ffi::*;
