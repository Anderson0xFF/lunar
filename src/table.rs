use std::rc::Rc;

use crate::{lua::*, vm::Lunar, lunar_ref::LunarRef, value::LunarValue, context::LunarContext};

#[derive(Debug, Clone)]
pub struct Table {
    ctx: Rc<LunarContext>,
    lunar_ref: LunarRef,
}

impl Table {

    pub fn new(ctx: Rc<LunarContext>, name: &str, global: bool) -> Table{
        unsafe {
            lua_createtable(ctx.ptr(), 0, 0);
            let table = lua_gettop(ctx.ptr());

            if global {
                lua_pushvalue(ctx.ptr(), table);
                lua_setglobal(ctx.ptr(), Lunar::to_const_char(name));
            }

            if !name.is_empty() {
                lua_pushstring(ctx.ptr(), Lunar::to_const_char(name));
                lua_setfield(ctx.ptr(), table, Lunar::to_const_char("__name"));
            }
        }
        Table{ctx : ctx.clone(), lunar_ref: LunarRef::new(ctx)}
    }

    pub fn set(&self, field: &str, value: LunarValue) {
        self.lunar_ref.get();
        self.ctx.push(value);
        self.ctx.set_field(field, 1);
    }

    #[inline]
    pub fn push_table(&self){
        self.lunar_ref.get();
    }

}

pub struct MetaTable {
    lunar_ref: LunarRef,
}
