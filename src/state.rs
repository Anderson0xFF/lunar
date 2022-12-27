#![allow(non_snake_case)]
use crate::lua::*;

#[derive(Debug, Clone)]
pub struct LunarState(lua_State);

impl LunarState {
    pub(crate) fn new() -> LunarState {
        unsafe {
            let ptr = luaL_newstate();
            if ptr.is_null() {
                panic!("[Lunar]: Unable to create a lua state.")
            }
            return LunarState(ptr);
        }
    }

    pub(super) fn destroy(&mut self) {
        unsafe {
            lua_close(self.0);
            self.0 = std::ptr::null_mut();
        }
    }

    #[inline]
    pub(crate) fn get(&self) -> lua_State {
        self.0
    }
}

impl Drop for LunarState {
    #[inline]
    fn drop(&mut self) {
        if !self.0.is_null() { self.destroy(); }
    }
}
