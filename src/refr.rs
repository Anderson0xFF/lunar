#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::lua::{luaL_ref, lua_State, lua_geti, LUA_REGISTRYINDEX, lua_gettop};

#[derive(Debug, Clone)]
pub struct LuaRef {
    L: lua_State,
    id: i32,
}

impl LuaRef {

    #[inline]
    pub fn register_last_stack_value(L: lua_State) -> Self {
        Self {
            L,
            id: unsafe { luaL_ref(L, LUA_REGISTRYINDEX) },
        }
    }

    #[inline]
    pub fn id(&self) -> i32{
        self.id
    }

    #[inline]
    pub fn push_reference(&self) -> i32 {
        unsafe { 
            lua_geti(self.L, LUA_REGISTRYINDEX, self.id as i64);
            lua_gettop(self.L)
        }
    }

    pub(crate) fn from(L: lua_State, id : i32) -> Self{
        Self { L, id }
    }
}
