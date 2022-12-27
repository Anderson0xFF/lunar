#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use libc::*;

pub type void_ptr = *mut c_void;
pub type lua_State = void_ptr;
pub type lua_CFunction = void_ptr;
pub type const_char = *mut c_char;
pub type lua_Value = void_ptr;
pub type lua_Integer = i64;
pub type lua_Unsigned = u64;
pub type lua_Number = f64;

pub const LUAI_MAXSTACK: i32 = 1000000;
pub const LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;

#[link(name = "lua", kind = "raw-dylib")]
extern "C" {
    pub fn luaL_newstate() -> lua_State;
    pub fn luaL_openlibs(L: lua_State);
    pub fn lua_close(L: lua_State);
    pub fn luaL_loadstring(L: lua_State, value: const_char);
    pub fn lua_pcallk(
        L: lua_State,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
        ctx: i32,
        k: i32,
    ) -> i32;
    pub fn lua_tolstring(L: lua_State, stack: i32, len: *mut size_t) -> const_char;
    pub fn lua_tostring(L: lua_State, stack: i32);
    pub fn lua_type(L: lua_State, stack: i32) -> i32;

    pub fn lua_setglobal(L: lua_State, key: const_char);
    pub fn lua_setfield(L: lua_State, stack: i32, key: const_char);

    pub fn luaL_checkinteger(L: lua_State, stack: i32) -> i64;
    pub fn lua_toboolean(L: lua_State, stack: i32) -> bool;
    pub fn luaL_checknumber(L: lua_State, stack: i32) -> f64;
    pub fn luaL_checklstring(L: lua_State, stack: i32, len: *mut size_t) -> const_char;
    pub fn lua_tocfunction(L: lua_State, stack: i32) -> lua_CFunction;
    pub fn lua_gettop(L: lua_State) -> i32;
    pub fn lua_geti(L: lua_State, stack: i32, idx: lua_Integer) -> i32;

    pub fn luaL_ref(L: lua_State, t: i32) -> i32;
    pub fn lua_pushnil(L: lua_State);
    pub fn lua_pushnumber(L: lua_State, number: lua_Number);
    pub fn lua_pushinteger(L: lua_State, n: lua_Integer);
    pub fn lua_pushlstring(L: lua_State, s: const_char, len: usize);
    pub fn lua_pushstring(L: lua_State, s: const_char);
    pub fn lua_pushboolean(L: lua_State, b: bool);
    pub fn lua_pushlightuserdata(L: lua_State, p: void_ptr);
    pub fn lua_pushcclosure(L: lua_State, function: lua_CFunction, n: i32);
    pub fn lua_pushvalue(L: lua_State, stack: i32);
    pub fn lua_createtable(L: lua_State, narr: i32, nrec: i32);
    pub fn lua_settop(L: lua_State, stack: i32);

}

pub fn lua_pop(L: lua_State, stack: i32) {
    unsafe { lua_settop(L, stack - 1) }
}
