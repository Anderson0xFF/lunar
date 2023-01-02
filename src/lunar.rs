#![allow(dead_code)]

use std::rc::Rc;

use crate::{
    context::{LunarContext, Value},
    lua::*,
    metatable::{MetaTable, MetaMethod},
    state::State,
    table::Table,
};

#[derive(Debug, Clone)]
pub struct Lunar {
    lua: State,
}

impl Lunar {
    #[inline]
    pub fn new() -> Self {
        Self { lua: State::new() }
    }

    #[inline]
    pub fn load_std_library(&self) {
        unsafe { luaL_openlibs(self.lua.L()) }
    }

    #[inline]
    pub fn exec(&self) -> Result<(), String> {
        pcall(self.lua.L(), 0, 0, 0)
    }

    #[inline]
    pub fn load_string(&self, string: String) {
        unsafe {
            luaL_loadstring(self.lua.L(), to_const_char(string));
        }
    }

    pub fn create_static_function(&self, name: &str, function: fn(ctx: LunarContext) -> i32) {
        unsafe {
            let function = function as *const ();
            push_function(self.lua.L(), function);
            lua_setglobal(self.lua.L(), to_const_char(name.to_string()));
        }
    }

    pub fn create_table(&self, name: &str, global: bool, table: fn(Table)) {
        let ctx = Rc::new(LunarContext::new(self.lua.L()));
        table(Table::new(ctx, name, global))
    }

    pub fn create_userdata(&self, name: &str, data: fn(&MetaTable)) {
        let ctx = Rc::new(LunarContext::new(self.lua.L()));
        let class = Table::new(ctx.clone(), name, true);
        let methods = MetaTable::new(ctx.clone(), "method");
        data(&methods);

        methods.add_meta_method(MetaMethod::NewIndex, Value::Function(|_|{
            println!("attempt to update a read-only table");
            return 0;
        }));

        class.push_table();
        methods.push_metatable();
        ctx.set_field("__index", 1);
        class.set_metatable(&methods);
    }
}
