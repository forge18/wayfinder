//! Safe Rust bindings for Lua C API

use super::lua_ffi::*;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;

pub struct Lua {
    state: LuaState,
}

unsafe impl Send for Lua {}

impl Lua {
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

    pub fn state(&self) -> LuaState {
        self.state
    }

    pub fn close(&mut self) {
        unsafe {
            lua_close(self.state);
            self.state = ptr::null_mut();
        }
    }

    pub fn load_string(&mut self, code: &str) -> Result<c_int, String> {
        unsafe {
            let code_ptr = CString::new(code.as_bytes()).unwrap();
            let result = luaL_loadstring(self.state, code_ptr.as_ptr());
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
            let result = luaL_loadfilex(self.state, filename_ptr.as_ptr(), ptr::null());
            if result != LUA_OK {
                let error = self.pop_string();
                return Err(error);
            }
            Ok(result)
        }
    }

    pub fn pcall(&mut self, nargs: c_int, nresults: c_int) -> Result<c_int, String> {
        unsafe {
            let result = lua_pcallk(self.state, nargs, nresults, 0, 0, None);
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
            lua_getglobal(self.state, name_ptr.as_ptr())
        }
    }

    pub fn set_global(&mut self, name: &str) {
        unsafe {
            let name_ptr = CString::new(name).unwrap();
            lua_setglobal(self.state, name_ptr.as_ptr());
        }
    }

    pub fn push_nil(&mut self) {
        unsafe {
            lua_pushnil(self.state);
        }
    }

    pub fn push_number(&mut self, n: lua_Number) {
        unsafe {
            lua_pushnumber(self.state, n);
        }
    }

    pub fn push_integer(&mut self, n: lua_Integer) {
        unsafe {
            lua_pushinteger(self.state, n);
        }
    }

    pub fn push_string(&mut self, s: &str) {
        unsafe {
            let s_ptr = CString::new(s).unwrap();
            lua_pushstring(self.state, s_ptr.as_ptr());
        }
    }

    pub fn push_boolean(&mut self, b: bool) {
        unsafe {
            lua_pushboolean(self.state, if b { 1 } else { 0 });
        }
    }

    pub fn push_cfunction(&mut self, f: LuaCFunction, n: c_int) {
        unsafe {
            lua_pushcclosure(self.state, f, n);
        }
    }

    pub fn create_table(&mut self, narr: c_int, nrec: c_int) {
        unsafe {
            lua_createtable(self.state, narr, nrec);
        }
    }

    pub fn get_top(&self) -> c_int {
        unsafe { lua_gettop(self.state) }
    }

    pub fn set_top(&mut self, idx: c_int) {
        unsafe {
            lua_settop(self.state, idx);
        }
    }

    pub fn pop<T: LuaPop>(&mut self) -> T {
        T::pop(self)
    }

    pub fn pop_string(&mut self) -> String {
        unsafe {
            let mut len: usize = 0;
            let ptr = lua_tolstring(self.state, -1, &mut len);
            if ptr.is_null() {
                String::new()
            } else {
                let slice = std::slice::from_raw_parts(ptr as *const u8, len);
                String::from_utf8_lossy(slice).to_string()
            }
        }
    }

    pub fn pop_integer(&mut self) -> lua_Integer {
        unsafe { lua_tointeger(self.state, -1) }
    }

    pub fn pop_number(&mut self) -> lua_Number {
        unsafe { lua_tonumber(self.state, -1) }
    }

    pub fn pop_boolean(&mut self) -> bool {
        unsafe { lua_toboolean(self.state, -1) != 0 }
    }

    pub fn get_table(&mut self, idx: c_int) -> c_int {
        unsafe { lua_gettable(self.state, idx) }
    }

    pub fn get_field(&mut self, idx: c_int, key: &str) -> c_int {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            lua_getfield(self.state, idx, key_ptr.as_ptr())
        }
    }

    pub fn set_field(&mut self, idx: c_int, key: &str) {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            lua_setfield(self.state, idx, key_ptr.as_ptr());
        }
    }

    pub fn raw_get_i(&mut self, idx: c_int, n: c_int) -> c_int {
        unsafe { lua_rawgeti(self.state, idx, n) }
    }

    pub fn raw_set_i(&mut self, idx: c_int, n: c_int) {
        unsafe {
            lua_rawseti(self.state, idx, n);
        }
    }

    pub fn len(&mut self, _idx: c_int) -> i64 {
        unsafe {
            let mut len: usize = 0;
            let ptr = lua_tolstring(self.state, -1, &mut len);
            if ptr.is_null() {
                0
            } else {
                len as i64
            }
        }
    }

    pub fn next(&mut self, idx: c_int) -> c_int {
        unsafe { lua_next(self.state, idx) }
    }

    pub fn type_of(&self, idx: c_int) -> c_int {
        unsafe { lua_type(self.state, idx) }
    }

    pub fn is_nil(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TNIL
    }

    pub fn is_number(&self, idx: c_int) -> bool {
        unsafe { lua_isnumber(self.state, idx) != 0 }
    }

    pub fn is_string(&self, idx: c_int) -> bool {
        unsafe { lua_isstring(self.state, idx) != 0 }
    }

    pub fn is_function(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TFUNCTION
    }

    pub fn is_table(&self, idx: c_int) -> bool {
        self.type_of(idx) == LUA_TTABLE
    }

    pub fn type_name(&self, tp: c_int) -> &'static str {
        unsafe {
            let ptr = lua_typename(self.state, tp);
            CStr::from_ptr(ptr).to_str().unwrap_or("unknown")
        }
    }

    pub fn traceback(&mut self, msg: &str, level: c_int) {
        unsafe {
            let msg_ptr = CString::new(msg).unwrap();
            luaL_traceback(self.state, self.state, msg_ptr.as_ptr(), level);
        }
    }

    pub fn error(&mut self, msg: &str) {
        unsafe {
            let _msg_ptr = CString::new(msg).unwrap();
            lua_error(self.state);
        }
    }

    pub fn new_userdata<T>(&mut self) -> *mut T {
        unsafe { lua_newuserdata(self.state, std::mem::size_of::<T>()) as *mut T }
    }

    pub fn check_stack(&mut self, extra: c_int) -> bool {
        unsafe { lua_checkstack(self.state, extra) != 0 }
    }

    pub fn get_registry(&mut self) -> c_int {
        self.get_top() + 1
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
        unsafe {
            if lua_rawgeti(self.lua.state, self.index, i as c_int) == LUA_TNIL {
                self.lua.set_top(-2);
                None
            } else {
                let value = self.lua.pop();
                Some(value)
            }
        }
    }

    pub fn set_i(&mut self, i: i64) {
        unsafe {
            lua_rawseti(self.lua.state, self.index, i as c_int);
        }
    }

    pub fn get<T: LuaPop>(&mut self, key: &str) -> Option<T> {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            if lua_getfield(self.lua.state, self.index, key_ptr.as_ptr()) == LUA_TNIL {
                self.lua.set_top(-2);
                None
            } else {
                let value = self.lua.pop();
                Some(value)
            }
        }
    }

    pub fn set(&mut self, key: &str) {
        unsafe {
            let key_ptr = CString::new(key).unwrap();
            lua_setfield(self.lua.state, self.index, key_ptr.as_ptr());
        }
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
