#![allow(dead_code)]

use crate::{
    context::LunarContext,
    lua::*,
    stack::LuaStack,
    state::LunarState,
    table::{MetaTable, Table},
    typedef::LunarError,
};
use libc::c_void;
use std::{
    ffi::{CStr, CString}, rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Lunar {
    state: LunarState,
}

impl Lunar {
    #[inline]
    pub fn new() -> Self {
        Self {
            state: LunarState::new(),
        }
    }

    #[inline]
    pub fn load_std_library(&self) {
        unsafe { luaL_openlibs(self.state.get()) }
    }

    pub fn load_str(&self, string: &str) {
        let c_string = match CString::new(string) {
            Ok(c_string) => c_string,
            Err(_) => panic!("[Lunar]: Failed to load '&str'."),
        }
        .into_raw();

        unsafe {
            luaL_loadstring(self.state.get(), c_string);
        }
    }

    pub fn load_string(&self, string: String) {
        let c_string = match CString::new(string) {
            Ok(c_string) => c_string,
            Err(_) => panic!("[Lunar]: Failed to load 'string'."),
        }
        .into_raw();

        unsafe {
            luaL_loadstring(self.state.get(), c_string);
        }
    }

    pub fn create_static_function(&self, name: &str, function: fn(ctx: LunarContext) -> i32) {
        unsafe {
            let ptr = function as *const ();
            let code: extern "C" fn(L: lua_State) = std::mem::transmute(ptr);
            lua_pushcclosure(self.state.get(), code as *mut c_void, 0);
            lua_setglobal(self.state.get(), Self::to_const_char(name));
        }
    }

    pub fn create_table(&self, name: &str, global: bool, callback: fn(Table)) {
        let ctx = Rc::new(LunarContext::new(self.state.get()));
        let table = Table::new(ctx, name, global);
        callback(table);
    }

    pub fn create_userdate<T>(&self, name: &str, methods: fn(ctx: MetaTable)) {}

    #[inline]
    pub fn exec(&self) {
        self.pcall(0, 0, 0);
    }

    fn pcall(&self, nargs: i32, nresults: i32, errfunc: i32) {
        unsafe {
            if lua_pcallk(self.state.get(), nargs, nresults, errfunc, 0, 0) > 0 {
                println!("Lunar: {}.", self.get_error());
            }
        }
    }

    fn get_error(&self) -> String {
        match self.to_string(LuaStack::STACK1) {
            Ok(value) => value,
            Err(e) => panic!("[Lunar]: Unable to get error description.\n   {e}"),
        }
    }

    fn to_string(&self, stack: LuaStack) -> Result<String, LunarError> {
        unsafe {
            let str = lua_tolstring(self.state.get(), 1, std::ptr::null_mut());
            match CStr::from_ptr(str).to_str() {
                Ok(str) => Ok(String::from(str)),
                Err(_) => Err("[Lunar]: Unable to convert 'const char*' to 'String'."),
            }
        }
    }

    pub(crate) fn to_const_char(string: &str) -> const_char {
        match CString::new(string) {
            Ok(c_string) => c_string,
            Err(_) => panic!("[Lunar]: Failed convert to 'string'."),
        }
        .into_raw()
    }
}
