#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::c_void;
use std::mem::size_of;

use crate::{
    lua::*,
    metatable::MetaTable,
    table::Table,
    types::{LuaType, Type},
};

pub type Function = fn(LunarContext) -> i32;
const STACK_MAX: i32 = 8;

pub struct Userdata<T> {
    ptr: *mut T,
    size: usize,
}

impl<T> Userdata<T> {
    pub fn new(value: T) -> Userdata<T> {
        let data = Box::new(value);
        let ptr = Box::into_raw(data);
        let size = size_of::<T>();
        Userdata { ptr, size }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr as *mut c_void
    }

    pub fn as_box(&self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr) }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Function(Function),
    Int(i32),
    Uint(u32),
    Long(i64),
    Float(f64),
    String(String),
    Userdata(&'static str, *mut c_void, usize),
    LightUserdata(&'static str, *mut c_void),
    Table(Table),
}

#[derive(Debug, Clone)]
pub struct LunarContext(lua_State);

impl LunarContext {

    #[inline]
    pub(crate) fn new(L: lua_State) -> LunarContext {
        Self(L)
    }

    #[inline]
    pub(crate) fn L(&self) -> lua_State {
        self.0
    }

    pub(crate) fn push(&self, value: Value) {
        unsafe {
            match value {
                Value::Nil => lua_pushnil(self.0),
                Value::Bool(b) => lua_pushboolean(self.0, b),
                Value::Function(f) => push_function(self.0, f as *const ()),
                Value::Int(i) => lua_pushinteger(self.0, i.into()),
                Value::Long(i) => lua_pushinteger(self.0, i.into()),
                Value::Float(f) => lua_pushnumber(self.0, f),
                Value::String(s) => lua_pushstring(self.0, to_const_char(s)),
                Value::Userdata(name, ptr, size) => self.push_userdata(name, ptr, size),
                Value::LightUserdata(name, ptr) => self.push_light_userdata(name, ptr),
                Value::Table(table) => { table.push_table(); }
                Value::Uint(u) => lua_pushinteger(self.0, u.into()),
            }
        }
    }

    #[inline]
    pub fn returns(&self, value: Value) -> i32 {
        self.push(value);
        return 1;
    }

    pub fn get_boolean(&self, arg: i32) -> bool {
        unsafe {
            luaL_argexpected(self.0, self.get_type(arg) == LuaType::Bool, arg, "bool");
            lua_toboolean(self.0, arg)
        }
    }

    pub fn get_int<T>(&self, arg: i32) -> T
    where
        T: Type + From<i8> + From<i16> + From<i32>,
    {
        luaL_argexpected(self.0, self.get_type(arg) == LuaType::Number, arg, "int");
        luaL_checkint(self.0, arg).into()
    }

    pub fn get_uint<T>(&self, arg: i32) -> T
    where
        T: Type + From<u8> + From<u16> + From<u32>,
    {
        luaL_argexpected(self.0, self.get_type(arg) == LuaType::Number, arg, "uint");
        luaL_checkunsigned(self.0, arg).into()
    }

    pub fn get_float<T>(&self, arg: i32) -> T
    where
        T: Type + From<f64>,
    {
        unsafe {
            luaL_argexpected(self.0, self.get_type(arg) == LuaType::Number, arg, "float");
            luaL_checknumber(self.0, arg).into()
        }
    }

    pub fn get_long(&self, arg: i32) -> i64 {
        unsafe {
            luaL_argexpected(self.0, self.get_type(arg) == LuaType::Number, arg, "long");
            luaL_checkinteger(self.0, arg).into()
        }
        
    }

    pub fn get_userdata<T>(&self, arg: i32) -> Box<T> {
        unsafe {
            let ptr = lua_getuserdata(self.0, arg) as *mut T;
            let value: Box<T> = Box::from_raw(ptr);
            return value;
        }
    }

    pub fn get_light_userdata<T>(&self, arg: i32) -> Option<&mut T> {
        let ptr = lua_get_lightuserdata(self.0, arg) as *mut T;
        unsafe {
            return ptr.as_mut();
        }
    }

    pub fn check_userdata<T>(&self, arg: i32, tname: &str) -> Box<T> {
        unsafe {
            let ptr = lua_check_udata(self.0, arg, tname) as *mut T;
            let value: Box<T> = Box::from_raw(ptr);
            return value;
        }
    }

    pub fn get_string(&self, arg: i32) -> Result<String, LunarError> {
        unsafe {
            luaL_argexpected(self.0, self.get_type(arg) == LuaType::String, arg, "string");
            let string = luaL_checklstring(self.0, arg, std::ptr::null_mut());
            to_string(string)
        }
    }

    #[inline]
    pub(crate) fn get_global(&self, name: &str) {
        unsafe {
            lua_getglobal(self.0, to_const_char(name.to_string()));
        }
    }

    pub(crate) fn set_field(&self, field: &str, stack: i32) {
        unsafe {
            lua_setfield(self.0, stack, to_const_char(field.to_string()));
        }
    }

    pub(crate) fn get_field(&self, field: &str, stack: i32) -> i32 {
        unsafe {
            lua_getfield(self.0, stack, to_const_char(field.to_string()));
            lua_gettop(self.0)
        }
    }

    #[inline]
    pub fn get_type(&self, arg: i32) -> LuaType {
        unsafe {
            return lua_type(self.0, arg).into();
        }
    }

    #[inline]
    pub fn stack_size(&self) -> i32 {
        unsafe {
            return lua_gettop(self.0);
        }
    }

    pub fn call_function(&self, stack: i32, args: Vec<Value>, nresult: i32) {
        let nargs = args.len() as i32;
        unsafe {
            lua_pushvalue(self.L(), stack);
            for v in args {
                self.push(v);
            }
            pcall(self.L(), nargs, nresult, 0).unwrap();
        }
    }

    pub fn stackdump(&self) {
        println!("\n--------------------------STACKDUMP--------------------------");

        for stack in 1..STACK_MAX {
            let typename = self.get_type(stack);

            if (typename == LuaType::Table || typename == LuaType::Userdata)
                && self.is_string("__name", stack)
            {
                let field = self.get_field("__name", stack);
                let name = self.get_string(field).unwrap();
                println!(
                    "Stack [{}]  <->  Type [{:?}]  <->  Name[{}]",
                    stack, typename, name
                );

                self.pop_last();
            } else {
                println!("Stack [{}]  <->  Type [{:?}]", stack, typename);
            }
        }
        println!("--------------------------------------------------------------");
    }

    #[inline]
    pub(crate) fn pop_last(&self) {
        lua_pop(self.0, 1);
    }

    pub(crate) fn set_metatable(&self, table: &Table, metatable: &MetaTable) {
        let table = table.push_table();
        metatable.push_metatable();
        unsafe {
            lua_setmetatable(self.0, table);
        }
        self.pop_last();
    }

    fn is_string(&self, field: &str, stack: i32) -> bool {
        self.get_field(field, stack);
        let isstr = self.get_type(self.stack_size()) == LuaType::String;
        self.pop_last();
        return isstr;
    }

    fn push_userdata(&self, name: &str, ptr: *mut c_void, size: usize) {
        unsafe {
            lua_pushuserdata(self.0, ptr, size);
            let userdata = lua_gettop(self.0);

            if !name.is_empty() {
                lua_getglobal(self.0, to_const_char(name.to_string()));
                lua_setmetatable(self.0, userdata);
            }
        }
    }

    fn push_light_userdata(&self, name: &str, ptr: *mut c_void) {
        unsafe {
            lua_pushlightuserdata(self.0, ptr);
            let lightuserdata = lua_gettop(self.0);

            if !name.is_empty() {
                lua_getglobal(self.0, to_const_char(name.to_string()));
                lua_setmetatable(self.0, lightuserdata);
            }
        }
    }
}

impl From<lua_State> for LunarContext {
    fn from(value: lua_State) -> Self {
        LunarContext(value)
    }
}
