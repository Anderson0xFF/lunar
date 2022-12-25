#![allow(non_snake_case)]
use crate::lua::*;


#[derive(Debug, Clone)]
pub struct LunarState(lua_State);


impl LunarState {
    pub fn new() -> LunarState {
        unsafe {
            let ptr = luaL_newstate();
            if ptr.is_null(){
                panic!("[Lunar]: Unable to create a lua state.")
            }
            return LunarState(ptr);
        }
    }

    #[inline]
    pub fn get(&self) -> lua_State { self.0 }
}

impl Drop for LunarState {
    #[inline]
    fn drop(&mut self) {
        unsafe { lua_close(self.0); }
    }
}
