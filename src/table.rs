use std::rc::Rc;

use crate::{
    context::{LunarContext, Value},
    lua::*,
    metatable::MetaTable,
    refr::LuaRef,
    types::Type,
};

#[derive(Debug, Clone)]
pub struct Table {
    ctx: Rc<LunarContext>,
    luaref: LuaRef,
}

impl Table {
    pub(crate) fn new(ctx: Rc<LunarContext>, name: &str, global: bool) -> Table {
        unsafe {
            lua_createtable(ctx.L(), 0, 0);
            let mut table = lua_gettop(ctx.L());

            if !name.is_empty() {
                lua_pushstring(ctx.L(), to_const_char(name.to_string()));
                lua_setfield(ctx.L(), table, to_const_char("__name".to_string()));

                if global {
                    lua_pushvalue(ctx.L(), table);
                    lua_setglobal(ctx.L(), to_const_char(name.to_string()));
                }
            }
            let luaref = LuaRef::register_last_stack_value(ctx.L());
            luaref.push_reference();
            table = lua_gettop(ctx.L());

            ctx.push(Value::Int(luaref.id()));
            lua_setfield(ctx.L(), table, to_const_char("__ref".to_string()));

            ctx.pop_last();
            Table {
                ctx: ctx.clone(),
                luaref,
            }
        }
    }

    pub fn set(&self, field: &str, value: Value) {
        let stack = self.luaref.push_reference();
        self.ctx.push(value);
        self.ctx.set_field(field, stack);
        self.ctx.pop_last();
    }

    pub fn set_function(&self, name: &str, function: fn(ctx: LunarContext) -> i32) {
        self.set(name, Value::Function(function));
    }

    pub fn set_metatable(&self, metatable: &MetaTable) {
        self.ctx.set_metatable(self, metatable);
    }

    pub fn get_boolean(&self, field: &str) -> Result<bool, LunarError> {
        let stack = self.luaref.push_reference();
        let field = self.ctx.get_field(field, stack);
        self.ctx.get_boolean(field)
    }

    pub fn get_int<T>(&self, field: &str) -> Result<T, LunarError>
    where
        T: Type + From<i8> + From<i16> + From<i32>,
    {
        let stack = self.luaref.push_reference();
        let field = self.ctx.get_field(field, stack);
        self.ctx.get_int::<T>(field)
    }

    pub fn get_float<T>(&self, field: &str) -> Result<T, LunarError>
    where
        T: Type + From<f64>,
    {
        let stack = self.luaref.push_reference();
        let field = self.ctx.get_field(field, stack);
        self.ctx.get_float::<T>(field)
    }

    pub fn get_uint(&self, field: &str) -> Result<u32, LunarError> {
        let stack = self.luaref.push_reference();
        let field = self.ctx.get_field(field, stack);
        self.ctx.get_uint(field)
    }

    pub fn get_long(&self, field: &str) -> Result<i64, LunarError> {
        let stack = self.luaref.push_reference();
        let field = self.ctx.get_field(field, stack);
        self.ctx.get_long(field)
    }

    #[inline]
    pub(crate) fn push_table(&self) -> i32 {
        self.luaref.push_reference()
    }

    pub fn from(ctx: Rc<LunarContext>, name: &str) -> Self {
        unsafe {
            ctx.get_global(name);
            let table = lua_gettop(ctx.L());
            ctx.get_field("__ref", table);

            let stack = lua_gettop(ctx.L());
            let id = luaL_checkinteger(ctx.L(), stack) as i32;
            let luaref = LuaRef::from(ctx.L(), id);

            Self { ctx, luaref }
        }
    }
}
