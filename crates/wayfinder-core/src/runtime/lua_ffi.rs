//! Lua C API FFI bindings
//! Uses libc for C types and functions

pub use libc::{c_char, c_int, c_long, c_void, size_t};

pub type LuaState = *mut c_void;
pub type LuaCFunction = extern "C" fn(*mut c_void) -> c_int;
pub type LuaHook = extern "C" fn(*mut c_void, *mut lua_Debug);
pub type lua_Integer = i64;
pub type lua_Number = f64;

#[repr(C)]
pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub nups: c_int,
    pub nparams: c_int,
    pub isvararg: c_int,
    pub istailcall: c_char,
    pub short_src: [c_char; 60],
    pub i_ci: *mut c_void,
}

// Static linking mode: link against Lua library at build time
#[cfg(feature = "static-lua")]
#[link(name = "lua5.4")]
extern "C" {
    pub fn lua_close(L: LuaState);
    pub fn lua_newthread(L: LuaState) -> LuaState;
    pub fn lua_resetthread(L: LuaState) -> c_int;

    pub fn lua_gettop(L: LuaState) -> c_int;
    pub fn lua_settop(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_pushvalue(L: LuaState, idx: c_int);
    pub fn lua_rotate(L: LuaState, idx: c_int, n: c_int);
    pub fn lua_copy(L: LuaState, fromidx: c_int, toidx: c_int);
    pub fn lua_checkstack(L: LuaState, n: c_int) -> c_int;

    pub fn lua_insert(L: LuaState, idx: c_int);
    pub fn lua_remove(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_replace(L: LuaState, idx: c_int);
    pub fn lua_xmove(from: LuaState, to: LuaState, n: c_int);

    pub fn lua_isnumber(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_isstring(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_type(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_typename(L: LuaState, tp: c_int) -> *const c_char;

    pub fn lua_tonumber(L: LuaState, idx: c_int) -> lua_Number;
    pub fn lua_tointeger(L: LuaState, idx: c_int) -> lua_Integer;
    pub fn lua_toboolean(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: LuaState, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn lua_tocfunction(L: LuaState, idx: c_int) -> LuaCFunction;
    pub fn lua_touserdata(L: LuaState, idx: c_int) -> *mut c_void;
    pub fn lua_tothread(L: LuaState, idx: c_int) -> LuaState;

    pub fn lua_pushnil(L: LuaState);
    pub fn lua_pushnumber(L: LuaState, n: lua_Number);
    pub fn lua_pushinteger(L: LuaState, n: lua_Integer);
    pub fn lua_pushlstring(L: LuaState, s: *const c_char, len: size_t);
    pub fn lua_pushstring(L: LuaState, s: *const c_char);
    pub fn lua_pushcclosure(L: LuaState, f: LuaCFunction, n: c_int);
    pub fn lua_pushboolean(L: LuaState, b: c_int);
    pub fn lua_pushlightuserdata(L: LuaState, p: *mut c_void);

    pub fn lua_arith(L: LuaState, op: c_int);
    pub fn lua_len(L: LuaState, idx: c_int);
    pub fn lua_concat(L: LuaState, n: c_int);
    pub fn lua_rawequal(L: LuaState, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_compare(L: LuaState, idx1: c_int, idx2: c_int, op: c_int) -> c_int;

    pub fn lua_getglobal(L: LuaState, name: *const c_char) -> c_int;
    pub fn lua_setglobal(L: LuaState, name: *const c_char);
    pub fn lua_gettable(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_getfield(L: LuaState, idx: c_int, k: *const c_char) -> c_int;
    pub fn lua_settable(L: LuaState, idx: c_int);
    pub fn lua_setfield(L: LuaState, idx: c_int, k: *const c_char);
    pub fn lua_rawget(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_rawgeti(L: LuaState, idx: c_int, n: c_int) -> c_int;
    pub fn lua_rawset(L: LuaState, idx: c_int);
    pub fn lua_rawseti(L: LuaState, idx: c_int, n: c_int);

    pub fn lua_createtable(L: LuaState, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(L: LuaState, size: size_t) -> *mut c_void;
    pub fn lua_getmetatable(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_setmetatable(L: LuaState, idx: c_int) -> c_int;

    pub fn lua_callk(
        L: LuaState,
        nargs: c_int,
        nresults: c_int,
        ctx: c_long,
        k: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    );
    pub fn lua_pcallk(
        L: LuaState,
        nargs: c_int,
        nresults: c_int,
        msgh: c_int,
        ctx: c_long,
        k: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    ) -> c_int;
    pub fn lua_load(
        L: LuaState,
        reader: Option<
            unsafe extern "C" fn(
                *mut c_void,
                *mut *const c_char,
                *mut *mut c_char,
                *mut size_t,
                *mut c_void,
            ),
        >,
        dt: *mut c_void,
        chunkname: *const c_char,
        mode: *const c_char,
    ) -> c_int;
    pub fn lua_dump(
        L: LuaState,
        writer: Option<
            unsafe extern "C" fn(*mut c_void, *const *const c_char, size_t, *mut c_void),
        >,
        data: *mut c_void,
        strip: c_int,
    ) -> c_int;

    pub fn lua_error(L: LuaState) -> c_int;
    pub fn lua_next(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_gc(L: LuaState, what: c_int, data: c_long, arg: c_int) -> c_int;
    pub fn lua_status(L: LuaState) -> c_int;
    pub fn lua_isyieldable(L: LuaState, idx: c_int) -> c_int;
    pub fn lua_resume(L: LuaState, from: LuaState, narg: c_int) -> c_int;
    pub fn lua_yieldk(
        L: LuaState,
        nresults: c_int,
        ctx: c_long,
        k: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    );

    pub fn lua_sethook(L: LuaState, f: LuaHook, mask: c_int, count: c_int);
    pub fn lua_gethook(L: LuaState) -> LuaHook;
    pub fn lua_gethookmask(L: LuaState) -> c_int;
    pub fn lua_gethookcount(L: LuaState) -> c_int;

    pub fn lua_getinfo(L: LuaState, what: *const c_char, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getlocal(L: LuaState, ar: *mut lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_setlocal(L: LuaState, ar: *mut lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_getupvalue(L: LuaState, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_setupvalue(L: LuaState, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_getstack(L: LuaState, level: c_int, ar: *mut lua_Debug) -> c_int;
    pub fn lua_topointer(L: LuaState, idx: c_int) -> *const c_void;

    pub fn lua_upvalueid(L: LuaState, fidx: c_int, n: c_int) -> *mut c_void;
    pub fn lua_upvaluejoin(L: LuaState, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int);
}

// Dynamic mode: Provide stub implementations for functions used in C callbacks
// These will panic if called - C callbacks are not supported in dynamic mode yet
#[cfg(feature = "dynamic-lua")]
pub unsafe fn lua_getinfo(_L: LuaState, _what: *const c_char, _ar: *mut lua_Debug) -> c_int {
    panic!("lua_getinfo not available in dynamic mode - C callbacks not yet supported");
}

#[cfg(feature = "dynamic-lua")]
pub unsafe fn lua_getlocal(_L: LuaState, _ar: *mut lua_Debug, _n: c_int) -> *const c_char {
    panic!("lua_getlocal not available in dynamic mode - C callbacks not yet supported");
}

#[cfg(feature = "dynamic-lua")]
pub unsafe fn lua_getupvalue(_L: LuaState, _funcindex: c_int, _n: c_int) -> *const c_char {
    panic!("lua_getupvalue not available in dynamic mode - C callbacks not yet supported");
}

// Auxiliary library functions
#[cfg(feature = "static-lua")]
#[link(name = "lua5.4")]
extern "C" {
    pub fn luaL_openlibs(L: LuaState);
    pub fn luaL_newstate() -> LuaState;
    pub fn luaL_checkversion(L: LuaState);
    pub fn luaL_traceback(L: LuaState, L1: LuaState, msg: *const c_char, level: c_int);
    pub fn luaL_argerror(L: LuaState, arg: c_int, extramsg: *const c_char) -> c_int;
    pub fn luaL_typeerror(L: LuaState, arg: c_int, tname: *const c_char) -> c_int;
    pub fn luaL_where(L: LuaState, level: c_int);
    pub fn luaL_error(L: LuaState, fmt: *const c_char, ...) -> c_int;
    pub fn luaL_checkoption(
        L: LuaState,
        arg: c_int,
        def: *const c_char,
        lst: *const *const c_char,
    ) -> c_int;
    pub fn luaL_checkinteger(L: LuaState, arg: c_int) -> lua_Integer;
    pub fn luaL_optinteger(L: LuaState, arg: c_int, def: lua_Integer) -> lua_Integer;
    pub fn luaL_checknumber(L: LuaState, arg: c_int) -> lua_Number;
    pub fn luaL_optnumber(L: LuaState, arg: c_int, def: lua_Number) -> lua_Number;
    pub fn luaL_checklstring(L: LuaState, arg: c_int, len: *mut size_t) -> *const c_char;
    pub fn luaL_optlstring(
        L: LuaState,
        arg: c_int,
        def: *const c_char,
        len: *mut size_t,
    ) -> *const c_char;
    pub fn luaL_checkstring(L: LuaState, arg: c_int) -> *const c_char;
    pub fn luaL_optstring(L: LuaState, arg: c_int, def: *const c_char) -> *const c_char;
    pub fn luaL_checkfunction(L: LuaState, arg: c_int) -> LuaCFunction;
    pub fn luaL_checkudata(L: LuaState, arg: c_int, tname: *const c_char) -> *mut c_void;
    pub fn luaL_checkstack(L: LuaState, sz: c_int, msg: *const c_char);
    pub fn luaL_checkany(L: LuaState, arg: c_int);
    pub fn luaL_getmetafield(L: LuaState, obj: c_int, event: *const c_char) -> c_int;
    pub fn luaL_callmeta(L: LuaState, obj: c_int, event: *const c_char) -> c_int;
    pub fn luaL_len(L: LuaState, idx: c_int) -> i64;
    pub fn luaL_gsub(
        L: LuaState,
        s: *const c_char,
        p: *const c_char,
        r: *const c_char,
    ) -> *const c_char;
    pub fn luaL_setfuncs(L: LuaState, l: *const luaL_Reg, nup: c_int);
    pub fn luaL_getsubtable(L: LuaState, idx: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_fileresult(L: LuaState, stat: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_execresult(L: LuaState, stat: c_int) -> c_int;

    pub fn luaL_loadbufferx(
        L: LuaState,
        buff: *const c_char,
        sz: size_t,
        name: *const c_char,
        mode: *const c_char,
    ) -> c_int;

    // Additional FFI functions needed for hot reload
    pub fn lua_pop(L: LuaState, n: c_int);
    pub fn lua_pcall(L: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
    pub fn luaL_ref(L: LuaState, t: c_int) -> c_int;
    pub fn luaL_unref(L: LuaState, t: c_int, ref_: c_int);
    pub fn lua_pushglobaltable(L: LuaState);
    pub fn luaL_loadstring(L: LuaState, s: *const c_char) -> c_int;
    pub fn luaL_loadfilex(L: LuaState, filename: *const c_char, mode: *const c_char) -> c_int;

    pub fn luaL_newmetatable(L: LuaState, tname: *const c_char) -> c_int;
    pub fn luaL_setmetatable(L: LuaState, tname: *const c_char);
    pub fn luaL_testudata(L: LuaState, arg: c_int, tname: *const c_char) -> *mut c_void;
}

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: LuaCFunction,
}

pub const LUA_TNONE: c_int = -1;
pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

pub const LUA_REGISTRYINDEX: c_int = -1000000;

pub const LUA_HOOKCALL: c_int = 0;
pub const LUA_HOOKRET: c_int = 1;
pub const LUA_HOOKLINE: c_int = 2;
pub const LUA_HOOKCOUNT: c_int = 3;
pub const LUA_HOOKTAILCALL: c_int = 4;

pub const LUA_MASKCALL: c_int = 1 << LUA_HOOKCALL;
pub const LUA_MASKRET: c_int = 1 << LUA_HOOKRET;
pub const LUA_MASKLINE: c_int = 1 << LUA_HOOKLINE;
pub const LUA_MASKCOUNT: c_int = 1 << LUA_HOOKCOUNT;

pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRERR: c_int = 5;

pub const LUA_MULTRET: c_int = -1;

pub const LUA_OPADD: c_int = 0;
pub const LUA_OPSUB: c_int = 1;
pub const LUA_OPMUL: c_int = 2;
pub const LUA_OPDIV: c_int = 3;
pub const LUA_OPMOD: c_int = 4;
pub const LUA_OPPOW: c_int = 5;
pub const LUA_OPUNM: c_int = 6;
pub const LUA_OPEQ: c_int = 0;
pub const LUA_OPLT: c_int = 1;
pub const LUA_OPLE: c_int = 2;
