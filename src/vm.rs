#![allow(dead_code)]

use crate::{
    context::LunarContext, lua::*, stack::LuaStack, state::LunarState, table::Table,
    typedef::LunarError,
};
use libc::c_void;
use std::{
    ffi::{CStr, CString},
    rc::Rc,
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

    pub(crate) fn get_state(&self) -> &LunarState {
        &self.state
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

    pub fn create_static_function(&self, name: &str, fcx: fn(ctx: LunarContext) -> i32) {
        unsafe {
            let ptr = fcx as *const ();
            let code: extern "C" fn(L: lua_State) = std::mem::transmute(ptr);
            lua_pushcclosure(self.state.get(), code as *mut c_void, 0);
            lua_setglobal(self.state.get(), Self::to_const_char(name));
        }
    }

    pub fn create_table(&self, name: &str, global: bool, tcx: fn(table: Table)) {
        let ctx = Rc::new(LunarContext::new(self.state.get()));
        let table = Table::new(ctx, name, global);
        tcx(table);
    }

    //pub fn create_userdate<T>(&self, name: &str, methods: fn(ctx: MetaTable)) {}

    #[inline]
    pub fn exec(&self) -> Result<(), String>{
        self.pcall(0, 0, 0)
    }

    fn pcall(&self, nargs: i32, nresults: i32, errfunc: i32) -> Result<(), String> {
        unsafe {
            if lua_pcallk(self.state.get(), nargs, nresults, errfunc, 0, 0) > 0 {
                let err = self.get_error();
                return Err(format!("Lunar: {err}."));
            }
        }

        return Ok(());
    }

    fn get_error(&self) -> String {
        match self.to_string(LuaStack::STACK1) {
            Ok(value) => value,
            Err(e) => panic!("[Lunar]: Unable to get error description.\n   {e}"),
        }
    }

    fn to_string(&self, stack: LuaStack) -> Result<String, LunarError> {
        unsafe {
            let str = lua_tolstring(self.state.get(), stack.into(), std::ptr::null_mut());
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
