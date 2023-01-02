#![allow(non_snake_case)]
use crate::lua::*;

#[derive(Debug, Clone)]
pub struct State(lua_State);

impl State {
    pub(crate) fn new() -> State {
        unsafe {
            let ptr = luaL_newstate();
            if ptr.is_null() {
                panic!("[LUA]: Unable to create a lua state.")
            }
            return State(ptr);
        }
    }

    pub(super) fn destroy(&mut self) {
        unsafe {
            lua_close(self.0);
            self.0 = std::ptr::null_mut();
        }
    }

    #[inline]
    pub(crate) fn L(&self) -> lua_State {
        self.0
    }
}

impl Drop for State {
    #[inline]
    fn drop(&mut self) {
        if !self.0.is_null() { self.destroy(); }
    }
}
