//! Safe Rust bindings for Lua C API

use super::lua_ffi::*;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;

#[cfg(feature = "dynamic-lua")]
use super::lua_loader::LuaLibrary;

#[derive(Clone)]
pub struct Lua {
    state: LuaState,
    #[cfg(feature = "dynamic-lua")]
    lib: LuaLibrary,
}

unsafe impl Send for Lua {}

// Method names follow Lua C API naming conventions
#[allow(non_snake_case)]
impl Lua {
    #[cfg(feature = "static-lua")]
    pub fn new() -> Self {
        unsafe {
            let state = luaL_newstate();
            if state.is_null() {
                panic!("Failed to create Lua state");
            }
            luaL_openlibs(state);
            Self { state }
        }
    }

    #[cfg(feature = "dynamic-lua")]
    pub fn new_with_library(lib: LuaLibrary) -> Self {
        unsafe {
            let state = lib.lual_newstate();
            if state.is_null() {
                panic!("Failed to create Lua state");
            }
            lib.lual_openlibs(state);
            Self { state, lib }
        }
    }

    pub fn state(&self) -> LuaState {
        self.state
    }

    pub fn get_stack(&self, level: c_int, ar: &mut lua_Debug) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getstack(self.state, level, ar);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getstack(self.state, level, ar);
        }
    }

    pub fn get_info(&self, what: &str, ar: &mut lua_Debug) -> c_int {
        unsafe {
            let what_cstr = CString::new(what).unwrap();
            #[cfg(feature = "static-lua")]
            return lua_getinfo(self.state, what_cstr.as_ptr(), ar);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getinfo(self.state, what_cstr.as_ptr(), ar);
        }
    }

    pub fn get_local(&self, ar: &mut lua_Debug, n: c_int) -> Option<String> {
        unsafe {
            #[cfg(feature = "static-lua")]
            let name_ptr = lua_getlocal(self.state, ar, n);

            #[cfg(feature = "dynamic-lua")]
            let name_ptr = self.lib.lua_getlocal(self.state, ar, n);

            if name_ptr.is_null() {
                None
            } else {
                let c_str = CStr::from_ptr(name_ptr);
                Some(c_str.to_string_lossy().to_string())
            }
        }
    }

    pub fn set_local(&self, ar: &mut lua_Debug, n: c_int) -> Option<String> {
        unsafe {
            #[cfg(feature = "static-lua")]
            let name_ptr = lua_setlocal(self.state, ar, n);

            #[cfg(feature = "dynamic-lua")]
            let name_ptr = self.lib.lua_setlocal(self.state, ar, n);

            if name_ptr.is_null() {
                None
            } else {
                let c_str = CStr::from_ptr(name_ptr);
                Some(c_str.to_string_lossy().to_string())
            }
        }
    }

    pub fn get_upvalue(&self, funcindex: c_int, n: c_int) -> Option<String> {
        unsafe {
            #[cfg(feature = "static-lua")]
            let name_ptr = lua_getupvalue(self.state, funcindex, n);

            #[cfg(feature = "dynamic-lua")]
            let name_ptr = self.lib.lua_getupvalue(self.state, funcindex, n);

            if name_ptr.is_null() {
                None
            } else {
                let c_str = CStr::from_ptr(name_ptr);
                Some(c_str.to_string_lossy().to_string())
            }
        }
    }

    pub fn set_upvalue(&self, funcindex: c_int, n: c_int) -> Option<String> {
        unsafe {
            #[cfg(feature = "static-lua")]
            let name_ptr = lua_setupvalue(self.state, funcindex, n);

            #[cfg(feature = "dynamic-lua")]
            let name_ptr = self.lib.lua_setupvalue(self.state, funcindex, n);

            if name_ptr.is_null() {
                None
            } else {
                let c_str = CStr::from_ptr(name_ptr);
                Some(c_str.to_string_lossy().to_string())
            }
        }
    }

    pub fn set_hook(&self, f: LuaHook, mask: c_int, count: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_sethook(self.state, f, mask, count);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_sethook(self.state, f, mask, count);
        }
    }

    pub fn get_hook(&self) -> LuaHook {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gethook(self.state);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gethook(self.state);
        }
    }

    pub fn get_hook_mask(&self) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gethookmask(self.state);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gethookmask(self.state);
        }
    }

    pub fn get_hook_count(&self) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gethookcount(self.state);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gethookcount(self.state);
        }
    }

    pub fn upvalue_id(&self, fidx: c_int, n: c_int) -> *mut c_void {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_upvalueid(self.state, fidx, n);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_upvalueid(self.state, fidx, n);
        }
    }

    pub fn get_metatable(&self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getmetatable(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getmetatable(self.state, idx);
        }
    }

    pub fn set_metatable(&self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_setmetatable(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_setmetatable(self.state, idx);
        }
    }

    pub fn new_metatable(&self, tname: &str) -> c_int {
        unsafe {
            let tname_cstr = CString::new(tname).unwrap();
            #[cfg(feature = "static-lua")]
            return luaL_newmetatable(self.state, tname_cstr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lual_newmetatable(self.state, tname_cstr.as_ptr());
        }
    }

    pub fn set_metatable_by_name(&self, tname: &str) {
        unsafe {
            let tname_cstr = CString::new(tname).unwrap();
            #[cfg(feature = "static-lua")]
            luaL_setmetatable(self.state, tname_cstr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            self.lib.lual_setmetatable(self.state, tname_cstr.as_ptr());
        }
    }

    pub fn close(&mut self) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_close(self.state);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_close(self.state);

            self.state = ptr::null_mut();
        }
    }

    pub fn load_string(&mut self, code: &str) -> Result<c_int, String> {
        unsafe {
            let code_ptr = CString::new(code.as_bytes()).unwrap();
            #[cfg(feature = "static-lua")]
            let result = luaL_loadstring(self.state, code_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            let result = self.lib.lual_loadstring(self.state, code_ptr.as_ptr());

            if result != LUA_OK {
                let error = self.pop_string();
                return Err(error);
            }
            Ok(result)
        }
    }

    pub fn load_file(&mut self, filename: &str) -> Result<c_int, String> {
        unsafe {
            let filename_ptr = CString::new(filename).unwrap();
            #[cfg(feature = "static-lua")]
            let result = luaL_loadfilex(self.state, filename_ptr.as_ptr(), ptr::null());

            #[cfg(feature = "dynamic-lua")]
            let result = self.lib.lual_loadfilex(self.state, filename_ptr.as_ptr(), ptr::null());

            if result != LUA_OK {
                let error = self.pop_string();
                return Err(error);
            }
            Ok(result)
        }
    }

    pub fn pcall(&mut self, nargs: c_int, nresults: c_int) -> Result<c_int, String> {
        unsafe {
            #[cfg(feature = "static-lua")]
            let result = lua_pcallk(self.state, nargs, nresults, 0, 0, None);

            #[cfg(feature = "dynamic-lua")]
            let result = self.lib.lua_pcall(self.state, nargs, nresults, 0);

            if result != LUA_OK {
                let error = self.pop_string();
                return Err(error);
            }
            Ok(result)
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<c_int, String> {
        self.load_string(code)?;
        self.pcall(0, LUA_MULTRET)
    }

    pub fn execute_file(&mut self, filename: &str) -> Result<c_int, String> {
        self.load_file(filename)?;
        self.pcall(0, LUA_MULTRET)
    }

    pub fn get_global(&mut self, name: &str) -> c_int {
        unsafe {
            let name_ptr = CString::new(name).unwrap();
            #[cfg(feature = "static-lua")]
            return lua_getglobal(self.state, name_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getglobal(self.state, name_ptr.as_ptr());
        }
    }

    pub fn set_global(&mut self, name: &str) {
        unsafe {
            let name_ptr = CString::new(name).unwrap();
            #[cfg(feature = "static-lua")]
            lua_setglobal(self.state, name_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_setglobal(self.state, name_ptr.as_ptr());
        }
    }

    pub fn push_nil(&mut self) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushnil(self.state);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushnil(self.state);
        }
    }

    pub fn push_number(&mut self, n: lua_Number) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushnumber(self.state, n);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushnumber(self.state, n);
        }
    }

    pub fn push_integer(&mut self, n: lua_Integer) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushinteger(self.state, n);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushinteger(self.state, n);
        }
    }

    pub fn push_string(&mut self, s: &str) {
        unsafe {
            let s_ptr = CString::new(s).unwrap();
            #[cfg(feature = "static-lua")]
            lua_pushstring(self.state, s_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushstring(self.state, s_ptr.as_ptr());
        }
    }

    pub fn push_boolean(&mut self, b: bool) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushboolean(self.state, if b { 1 } else { 0 });

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushboolean(self.state, if b { 1 } else { 0 });
        }
    }

    pub fn push_cfunction(&mut self, f: LuaCFunction, n: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushcclosure(self.state, f, n);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushcclosure(self.state, f, n);
        }
    }

    pub fn create_table(&mut self, narr: c_int, nrec: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_createtable(self.state, narr, nrec);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_createtable(self.state, narr, nrec);
        }
    }

    pub fn get_top(&self) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gettop(self.state);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gettop(self.state);
        }
    }

    pub fn set_top(&mut self, idx: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_settop(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_settop(self.state, idx);
        }
    }

    pub fn pop<T: LuaPop>(&mut self) -> T {
        T::pop(self)
    }

    pub fn pop_string(&mut self) -> String {
        unsafe {
            let mut len: usize = 0;
            #[cfg(feature = "static-lua")]
            let ptr = lua_tolstring(self.state, -1, &mut len);

            #[cfg(feature = "dynamic-lua")]
            let ptr = self.lib.lua_tolstring(self.state, -1, &mut len);

            if ptr.is_null() {
                String::new()
            } else {
                let slice = std::slice::from_raw_parts(ptr as *const u8, len);
                String::from_utf8_lossy(slice).to_string()
            }
        }
    }

    pub fn pop_integer(&mut self) -> lua_Integer {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_tointeger(self.state, -1);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_tointeger(self.state, -1);
        }
    }

    pub fn pop_number(&mut self) -> lua_Number {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_tonumber(self.state, -1);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_tonumber(self.state, -1);
        }
    }

    pub fn pop_boolean(&mut self) -> bool {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_toboolean(self.state, -1) != 0;

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_toboolean(self.state, -1) != 0;
        }
    }

    pub fn get_table(&mut self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gettable(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gettable(self.state, idx);
        }
    }

    pub fn get_field(&mut self, idx: c_int, key: &str) -> c_int {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            #[cfg(feature = "static-lua")]
            return lua_getfield(self.state, idx, key_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getfield(self.state, idx, key_ptr.as_ptr());
        }
    }

    pub fn set_field(&mut self, idx: c_int, key: &str) {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            #[cfg(feature = "static-lua")]
            lua_setfield(self.state, idx, key_ptr.as_ptr());

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_setfield(self.state, idx, key_ptr.as_ptr());
        }
    }

    pub fn raw_get_i(&mut self, idx: c_int, n: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_rawgeti(self.state, idx, n);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_rawgeti(self.state, idx, n);
        }
    }

    pub fn raw_set_i(&mut self, idx: c_int, n: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_rawseti(self.state, idx, n);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_rawseti(self.state, idx, n as i64);
        }
    }

    pub fn len(&mut self, _idx: c_int) -> i64 {
        unsafe {
            let mut len: usize = 0;
            #[cfg(feature = "static-lua")]
            let ptr = lua_tolstring(self.state, -1, &mut len);

            #[cfg(feature = "dynamic-lua")]
            let ptr = self.lib.lua_tolstring(self.state, -1, &mut len);

            if ptr.is_null() {
                0
            } else {
                len as i64
            }
        }
    }

    pub fn next(&mut self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_next(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_next(self.state, idx);
        }
    }

    pub fn type_of(&self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_type(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_type(self.state, idx);
        }
    }

    pub fn is_nil(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TNIL
    }

    pub fn is_number(&self, idx: c_int) -> bool {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_isnumber(self.state, idx) != 0;

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_isnumber(self.state, idx) != 0;
        }
    }

    pub fn is_string(&self, idx: c_int) -> bool {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_isstring(self.state, idx) != 0;

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_isstring(self.state, idx) != 0;
        }
    }

    pub fn is_function(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TFUNCTION
    }

    pub fn is_table(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TTABLE
    }

    pub fn type_name(&self, tp: c_int) -> &'static str {
        unsafe {
            #[cfg(feature = "static-lua")]
            let ptr = lua_typename(self.state, tp);

            #[cfg(feature = "dynamic-lua")]
            let ptr = self.lib.lua_typename(self.state, tp);

            CStr::from_ptr(ptr).to_str().unwrap_or("unknown")
        }
    }

    pub fn traceback(&mut self, msg: &str, level: c_int) {
        unsafe {
            let msg_ptr = CString::new(msg).unwrap();
            #[cfg(feature = "static-lua")]
            luaL_traceback(self.state, self.state, msg_ptr.as_ptr(), level);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lual_traceback(self.state, self.state, msg_ptr.as_ptr(), level);
        }
    }

    pub fn topointer(&mut self, idx: c_int) -> *const c_void {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_topointer(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_topointer(self.state, idx);
        }
    }

    pub fn error(&mut self, msg: &str) {
        unsafe {
            let _msg_ptr = CString::new(msg).unwrap();
            #[cfg(feature = "static-lua")]
            lua_error(self.state);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_error(self.state);
        }
    }

    pub fn new_userdata<T>(&mut self) -> *mut T {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_newuserdata(self.state, std::mem::size_of::<T>()) as *mut T;

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_newuserdata(self.state, std::mem::size_of::<T>()) as *mut T;
        }
    }

    pub fn check_stack(&mut self, extra: c_int) -> bool {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_checkstack(self.state, extra) != 0;

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_checkstack(self.state, extra) != 0;
        }
    }

    pub fn get_registry(&mut self) -> c_int {
        self.get_top() + 1
    }

    // Additional wrapper methods for hot_reload and other features
    pub fn lua_pop(&mut self, n: c_int) {
        self.set_top(-n - 1);
    }

    pub fn lua_type(&self, idx: c_int) -> c_int {
        self.type_of(idx)
    }

    pub fn lua_tolstring(&self, idx: c_int, len: *mut usize) -> *const c_char {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_tolstring(self.state, idx, len);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_tolstring(self.state, idx, len);
        }
    }

    pub fn lua_pcall(&mut self, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_pcallk(self.state, nargs, nresults, msgh, 0, None);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_pcall(self.state, nargs, nresults, msgh);
        }
    }

    pub fn luaL_ref(&mut self, t: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return luaL_ref(self.state, t);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lual_ref(self.state, t);
        }
    }

    pub fn lua_pushvalue(&mut self, idx: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushvalue(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushvalue(self.state, idx);
        }
    }

    pub fn lua_pushglobaltable(&mut self) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_pushglobaltable(self.state);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_pushglobaltable(self.state);
        }
    }

    pub fn luaL_unref(&mut self, t: c_int, r: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            luaL_unref(self.state, t, r);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lual_unref(self.state, t, r);
        }
    }

    pub fn luaL_loadstring(&mut self, s: *const c_char) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return luaL_loadstring(self.state, s);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lual_loadstring(self.state, s);
        }
    }

    pub fn lua_tointeger(&self, idx: c_int) -> lua_Integer {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_tointeger(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_tointeger(self.state, idx);
        }
    }

    pub fn lua_tonumber(&self, idx: c_int) -> lua_Number {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_tonumber(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_tonumber(self.state, idx);
        }
    }

    pub fn lua_toboolean(&self, idx: c_int) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_toboolean(self.state, idx);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_toboolean(self.state, idx);
        }
    }

    pub fn lua_sethook(&self, f: LuaHook, mask: c_int, count: c_int) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_sethook(self.state, f, mask, count);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_sethook(self.state, f, mask, count);
        }
    }

    pub fn lua_gethook(&self) -> LuaHook {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_gethook(self.state);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_gethook(self.state);
        }
    }

    pub fn lua_rawgeti(&mut self, idx: c_int, n: i64) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_rawgeti(self.state, idx, n as c_int);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_rawgeti(self.state, idx, n as c_int);
        }
    }

    pub fn lua_rawseti(&mut self, idx: c_int, n: i64) {
        unsafe {
            #[cfg(feature = "static-lua")]
            lua_rawseti(self.state, idx, n as c_int);

            #[cfg(feature = "dynamic-lua")]
            self.lib.lua_rawseti(self.state, idx, n);
        }
    }

    pub fn lua_getstack(&self, level: c_int, ar: *mut lua_Debug) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getstack(self.state, level, ar);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getstack(self.state, level, ar);
        }
    }

    pub fn lua_getlocal(&self, ar: *mut lua_Debug, n: c_int) -> *const c_char {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getlocal(self.state, ar, n);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getlocal(self.state, ar, n);
        }
    }

    pub fn lua_setlocal(&self, ar: *mut lua_Debug, n: c_int) -> *const c_char {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_setlocal(self.state, ar, n);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_setlocal(self.state, ar, n);
        }
    }

    // Additional public wrapper methods with lua_ prefix for hot_reload
    pub fn lua_pushnil(&mut self) {
        self.push_nil();
    }

    pub fn lua_pushboolean(&mut self, b: c_int) {
        self.push_boolean(b != 0);
    }

    pub fn lua_pushnumber(&mut self, n: lua_Number) {
        self.push_number(n);
    }

    pub fn lua_pushstring(&mut self, s: *const c_char) {
        unsafe {
            let c_str = CStr::from_ptr(s);
            if let Ok(s) = c_str.to_str() {
                self.push_string(s);
            }
        }
    }

    pub fn lua_setglobal(&mut self, name: *const c_char) {
        unsafe {
            let c_str = CStr::from_ptr(name);
            if let Ok(s) = c_str.to_str() {
                self.set_global(s);
            }
        }
    }

    pub fn lua_next(&mut self, idx: c_int) -> c_int {
        self.next(idx)
    }

    pub fn lua_getupvalue(&self, funcindex: c_int, n: c_int) -> *const c_char {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getupvalue(self.state, funcindex, n);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getupvalue(self.state, funcindex, n);
        }
    }

    pub fn lua_getmetatable(&self, idx: c_int) -> c_int {
        self.get_metatable(idx)
    }

    pub fn lua_getinfo(&self, what: *const c_char, ar: *mut lua_Debug) -> c_int {
        unsafe {
            #[cfg(feature = "static-lua")]
            return lua_getinfo(self.state, what, ar);

            #[cfg(feature = "dynamic-lua")]
            return self.lib.lua_getinfo(self.state, what, ar);
        }
    }

    pub fn lua_settop(&mut self, idx: c_int) {
        self.set_top(idx);
    }

    pub fn lua_getglobal(&mut self, name: *const c_char) -> c_int {
        unsafe {
            let c_str = CStr::from_ptr(name);
            if let Ok(s) = c_str.to_str() {
                return self.get_global(s);
            }
            LUA_TNIL
        }
    }

    pub fn lua_getfield(&mut self, idx: c_int, key: *const c_char) -> c_int {
        unsafe {
            let c_str = CStr::from_ptr(key);
            if let Ok(s) = c_str.to_str() {
                return self.get_field(idx, s);
            }
            LUA_TNIL
        }
    }

    pub fn lua_gettable(&mut self, idx: c_int) -> c_int {
        self.get_table(idx)
    }
}

impl Drop for Lua {
    fn drop(&mut self) {
        if !self.state.is_null() {
            self.close();
        }
    }
}

pub trait LuaPop: Sized {
    fn pop(lua: &mut Lua) -> Self;
}

impl LuaPop for String {
    fn pop(lua: &mut Lua) -> Self {
        lua.pop_string()
    }
}

impl LuaPop for lua_Integer {
    fn pop(lua: &mut Lua) -> Self {
        lua.pop_integer()
    }
}

impl LuaPop for lua_Number {
    fn pop(lua: &mut Lua) -> Self {
        lua.pop_number()
    }
}

impl LuaPop for bool {
    fn pop(lua: &mut Lua) -> Self {
        lua.pop_boolean()
    }
}

impl LuaPop for () {
    fn pop(lua: &mut Lua) -> Self {
        lua.set_top(-1);
    }
}

pub struct LuaTable<'a> {
    lua: &'a mut Lua,
    index: c_int,
}

impl<'a> LuaTable<'a> {
    pub fn new(lua: &'a mut Lua, index: c_int) -> Self {
        Self { lua, index }
    }

    pub fn len(&mut self) -> i64 {
        self.lua.len(self.index)
    }

    pub fn get_i<T: LuaPop>(&mut self, i: i64) -> Option<T> {
        if self.lua.raw_get_i(self.index, i as c_int) == LUA_TNIL {
            self.lua.set_top(-2);
            None
        } else {
            let value = self.lua.pop();
            Some(value)
        }
    }

    pub fn set_i(&mut self, i: i64) {
        self.lua.raw_set_i(self.index, i as c_int);
    }

    pub fn get<T: LuaPop>(&mut self, key: &str) -> Option<T> {
        if self.lua.get_field(self.index, key) == LUA_TNIL {
            self.lua.set_top(-2);
            None
        } else {
            let value = self.lua.pop();
            Some(value)
        }
    }

    pub fn set(&mut self, key: &str) {
        self.lua.set_field(self.index, key);
    }
}

pub struct DebugInfo<'a> {
    ar: lua_Debug,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> DebugInfo<'a> {
    pub unsafe fn new() -> Self {
        Self {
            ar: std::mem::zeroed(),
            _phantom: PhantomData,
        }
    }

    pub fn name(&self) -> Option<&str> {
        if self.ar.name.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(self.ar.name).to_str().ok()?) }
        }
    }

    pub fn namewhat(&self) -> &str {
        unsafe { CStr::from_ptr(self.ar.namewhat).to_str().unwrap_or("") }
    }

    pub fn what(&self) -> &str {
        unsafe { CStr::from_ptr(self.ar.what).to_str().unwrap_or("") }
    }

    pub fn source(&self) -> Option<&str> {
        if self.ar.source.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(self.ar.source).to_str().ok()?) }
        }
    }

    pub fn current_line(&self) -> c_int {
        self.ar.currentline
    }

    pub fn linedefined(&self) -> c_int {
        self.ar.linedefined
    }

    pub fn last_line_defined(&self) -> c_int {
        self.ar.lastlinedefined
    }

    pub fn nups(&self) -> c_int {
        self.ar.nups
    }

    pub fn nparams(&self) -> c_int {
        self.ar.nparams
    }

    pub fn is_vararg(&self) -> bool {
        self.ar.isvararg != 0
    }

    pub fn short_src(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.ar.short_src.as_ptr())
                .to_str()
                .unwrap_or("")
        }
    }

    pub unsafe fn ptr(&mut self) -> *mut lua_Debug {
        &mut self.ar as *mut lua_Debug
    }
}
