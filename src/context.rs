#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::mem::size_of;

use libc::c_void;

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
    LightUserdata(*mut c_void),
    Table(Table),
}

#[derive(Debug, Clone)]
pub struct LunarContext(lua_State);

impl LunarContext {
    pub(crate) fn new(L: lua_State) -> LunarContext {
        Self(L)
    }

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
                Value::Userdata(name, data, size) => self.push_userdata(name, data, size),
                Value::LightUserdata(ptr) => lua_pushlightuserdata(self.0, ptr),
                Value::Table(table) => {
                    table.push_table();
                }
                Value::Uint(u) => lua_pushinteger(self.0, u.into()),
            }
        }
    }

    pub fn returns(&self, value: Value) -> i32 {
        self.push(value);
        return 1;
    }

    pub fn get_boolean(&self, arg: i32) -> Result<bool, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LuaType::Bool => Ok(lua_toboolean(self.0, arg)),
                _ => Err("[LUA]: Expected value of type 'bool'"),
            }
        }
    }

    pub fn get_int<T>(&self, arg: i32) -> Result<T, LunarError>
    where
        T: Type + From<i8> + From<i16> + From<i32>,
    {
        match T::type_of() {
            LuaType::Number => Ok(luaL_checkint(self.0, arg).into()),
            _ => Err("[LUA]: Expected value of type 'int'"),
        }
    }

    pub fn get_uint<T>(&self, arg: i32) -> Result<T, LunarError>
    where
        T: Type + From<u8> + From<u16> + From<u32>,
    {
        match T::type_of() {
            LuaType::Number => Ok(luaL_checkunsigned(self.0, arg).into()),
            _ => Err("[LUA]: Expected value of type 'uint'"),
        }
    }

    pub fn get_long(&self, arg: i32) -> Result<i64, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LuaType::Number => Ok(luaL_checkinteger(self.0, arg)),
                _ => Err("[LUA]: Expected value of type 'long'"),
            }
        }
    }

    pub fn get_userdata<T>(&self, arg: i32) -> Box<T> {
        unsafe {
            let ptr = lua_getuserdata(self.0, arg) as *mut T;
            let value: Box<T> = Box::from_raw(ptr);
            return value;
        }
    }

    pub fn get_float<T>(&self, arg: i32) -> Result<T, LunarError>
    where
        T: Type + From<f64>,
    {
        unsafe {
            match T::type_of() {
                LuaType::Number => Ok(luaL_checknumber(self.0, arg).into()),
                _ => Err("[LUA]: Expected value of type 'float'"),
            }
        }
    }

    pub fn get_string(&self, arg: i32) -> Result<String, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LuaType::String => {
                    let string = luaL_checklstring(self.0, arg, std::ptr::null_mut());
                    Ok(to_string(string).unwrap())
                }
                _ => Err("[LUA]: Expected value of type 'string'"),
            }
        }
    }

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
    pub(crate) fn get_type(&self, arg: i32) -> LuaType {
        unsafe {
            return lua_type(self.0, arg).into();
        }
    }

    #[inline]
    pub(crate) fn get_last(&self) -> i32 {
        unsafe {
            return lua_gettop(self.0);
        }
    }

    pub fn call_function(&self, arg: i32, args: Vec<Value>){
        let nargs = args.len() as i32;
        unsafe{
            lua_pushvalue(self.L(), arg);
            for v in args {
                self.push(v);
            }
            pcall(self.L(), nargs, 0, 0).unwrap();
        }
    }

    pub fn stacktrace(&self) {
        println!("\n--------------------------STACKTRACE--------------------------");

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
        let isstr = self.get_type(self.get_last()) == LuaType::String;
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
}

impl From<lua_State> for LunarContext {
    fn from(value: lua_State) -> Self {
        LunarContext(value)
    }
}
