#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::*;
use std::ffi::{CStr, CString};

pub type void_ptr = *mut c_void;
pub type lua_State = void_ptr;
pub type lua_CFunction = void_ptr;
pub type const_char = *mut c_char;
pub type lua_Value = void_ptr;
pub type lua_Integer = i64;
pub type lua_Unsigned = u64;
pub type lua_Number = f64;
pub type LunarError = &'static str;
pub type lua_Writer = extern "C" fn(L: lua_State, void: void_ptr, __size: usize, ud: void_ptr);

pub const LUAI_MAXSTACK: i32 = 1000000;
pub const LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;

#[link(name = "lua", kind = "raw-dylib")]
extern "C" {
    pub fn luaL_newstate() -> lua_State;
    pub fn luaL_openlibs(L: lua_State);
    pub fn lua_close(L: lua_State);
    pub fn luaL_loadstring(L: lua_State, value: const_char);
    pub fn luaL_error(L: lua_State, fmt: const_char, ...) -> i32;
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
    pub fn lua_getglobal(L: lua_State, name: const_char) -> i32;
    pub fn lua_getfield(L: lua_State, stack: i32, key: const_char) -> i32;

    pub fn lua_touserdata(L: lua_State, idx: i32) -> *mut std::ffi::c_void;
    pub fn luaL_ref(L: lua_State, t: i32) -> i32;
    pub fn lua_pushnil(L: lua_State);
    pub fn lua_pushnumber(L: lua_State, number: lua_Number);
    pub fn lua_pushinteger(L: lua_State, n: lua_Integer);
    pub fn lua_pushlstring(L: lua_State, s: const_char, len: usize);
    pub fn lua_pushstring(L: lua_State, s: const_char);
    pub fn lua_pushboolean(L: lua_State, b: bool);
    pub fn lua_pushlightuserdata(L: lua_State, p: void_ptr);
    pub fn lua_newuserdatauv(L: lua_State, size: usize, nuvalue: i32) -> void_ptr;
    pub fn lua_pushcclosure(L: lua_State, function: lua_CFunction, n: i32);
    pub fn lua_pushvalue(L: lua_State, stack: i32);
    pub fn lua_createtable(L: lua_State, narr: i32, nrec: i32);
    pub fn lua_settop(L: lua_State, stack: i32);
    pub fn lua_setmetatable(L: lua_State, stack: i32) -> i32;
    pub fn lua_dump(L: lua_State, writer: lua_Writer, data: void_ptr, strip: i32) -> i32;
    pub fn luaL_typeerror(L: lua_State, arg: i32, tname: *const c_char) -> i32;
    pub fn luaL_argerror(L: lua_State, arg: i32, extramsg: const_char) -> i32;
    pub fn luaL_checkudata (L: lua_State, arg: i32, tname: const_char) -> void_ptr;
}

pub(crate) fn luaL_argexpected(L: lua_State, cond: bool, stack: i32, tname: &str) {
    unsafe {
        if !cond {
            let tname = CString::new(tname).unwrap().into_raw() as *const c_char;
            luaL_typeerror(L, stack, tname);
        }
    }
}

#[inline]
pub(crate) fn lua_argerror(L: lua_State, arg: i32, extramsg: &str) -> i32 {
    let extramsg = to_const_char(extramsg.to_string());
    unsafe { luaL_argerror(L, arg, extramsg) }
}

#[inline]
pub(crate) fn luaL_checkint(L: lua_State, stack: i32) -> i32 {
    unsafe { luaL_checkinteger(L, stack).try_into().unwrap() }
}

#[inline]
pub(crate) fn luaL_checkunsigned(L: lua_State, stack: i32) -> u32 {
    unsafe { luaL_checkinteger(L, stack).try_into().unwrap() }
}

#[inline]
pub(crate) fn lua_pop(L: lua_State, stack: i32) {
    unsafe { lua_settop(L, -(stack) - 1) }
}

#[inline]
fn lua_newuserdata(L: lua_State, size: usize) -> *mut c_void {
    unsafe { lua_newuserdatauv(L, size, 1) }
}


pub(crate) fn lua_check_udata(L: lua_State, idx: i32, tname: &str) -> *mut c_void {
    unsafe {
        let tname = to_const_char(tname.to_string());
        *(luaL_checkudata(L, idx, tname) as *mut *mut c_void)
    }
}

pub(crate) fn pcall(L: lua_State, nargs: i32, nresults: i32, errfunc: i32) -> Result<(), String> {
    unsafe {
        if lua_pcallk(L, nargs, nresults, errfunc, 0, 0) > 0 {
            return Err(error(L));
        }
    }
    return Ok(());
}

pub(crate) fn error(L: lua_State) -> String {
    let str = unsafe { lua_tolstring(L, 1, std::ptr::null_mut()) };
    match to_string(str) {
        Ok(value) => value,
        Err(e) => panic!("Unable to get error description.\n  {e}"),
    }
}

pub(crate) fn to_const_char(string: String) -> const_char {
    match CString::new(string) {
        Ok(c_string) => c_string,
        Err(_) => panic!("Unable to convert 'string' to 'const char*'."),
    }
    .into_raw()
}

pub(crate) fn to_string(c_str: *mut i8) -> Result<String, LunarError> {
    unsafe {
        match CStr::from_ptr(c_str).to_str() {
            Ok(str) => Ok(String::from(str)),
            Err(_) => Err("Unable to convert 'const char*' to 'string'."),
        }
    }
}

pub(crate) fn lua_pushuserdata(L: lua_State, ptr: *mut c_void, size: usize) {
    unsafe {
        let c_ptr = lua_newuserdata(L, size) as *mut *mut c_void;
        *c_ptr = ptr;
    }
}

#[inline]
pub(crate) fn lua_getuserdata(L: lua_State, idx: i32) -> *mut c_void {
    unsafe { *(lua_touserdata(L, idx) as *mut *mut c_void) }
}

#[inline]
pub(crate) fn lua_get_lightuserdata(L: lua_State, idx: i32) -> *mut c_void {
    unsafe { lua_touserdata(L, idx) }
}


pub(crate) fn push_function(L: lua_State, function: *const ()) {
    unsafe {
        let function: extern "C" fn(L: lua_State) = std::mem::transmute(function);
        lua_pushcclosure(L, function as *mut c_void, 0);
    }
}
