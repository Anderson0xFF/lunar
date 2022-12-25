use std::rc::Rc;

use crate::{context::LunarContext, lua::*};

#[derive(Debug, Clone)]
pub struct LunarRef {
    ctx: Rc<LunarContext>,
    ptr: i32,
}

impl LunarRef {

    #[inline]
    pub fn new(ctx: Rc<LunarContext>) -> Self {
        let ptr = unsafe { luaL_ref(ctx.ptr(), LUA_REGISTRYINDEX) };
        Self { ctx, ptr }
    }

    #[inline]
    pub fn get(&self) {
        unsafe { lua_geti(self.ctx.ptr(), LUA_REGISTRYINDEX, self.ptr as i64) };
    }
}
