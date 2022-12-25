use std::{
    convert::From,
    ffi::{CStr, CString},
};

use crate::{
    lua::*,
    vm::Lunar,
    state::LunarState,
    typedef::{LunarError, LunarType, Type},
    value::LunarValue,
};

#[derive(Debug, Clone)]
pub struct LunarContext(lua_State);

impl LunarContext {
    pub(crate) fn new(state: lua_State)-> LunarContext{
        Self(state)
    }

    pub(crate) fn ptr(&self) -> lua_State {
        self.0
    }

    pub fn get_string(&self, arg: i32) -> Result<String, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LunarType::STRING => {
                    let string = luaL_checklstring(self.0, arg, std::ptr::null_mut());
                    match CStr::from_ptr(string).to_str() {
                        Ok(str) => Ok(String::from(str)),
                        Err(_) => {
                            Err("[Lunar Error]: Unable to convert 'const char*' to 'String'.")
                        }
                    }
                }
                _ => Err("[Lunar Error]: Expected value of type 'String'"),
            }
        }
    }

    pub fn get_integer<T>(&self, arg: i32) -> Result<T, LunarError>
    where
        T: Type + From<i8> + From<i16> + From<i32>,
    {
        match T::type_of() {
            LunarType::INTEGER => Ok(self.get_i32(arg).unwrap().into()),
            _ => Err("[Lunar Error]: Expected value of type 'Integer'"),
        }
    }

    pub fn get_floating<T>(&self, arg: i32) -> Result<T, LunarError>
    where
        T: Type + From<f64>,
    {
        match T::type_of() {
            LunarType::NUMBER => Ok(self.get_f64(arg).unwrap().into()),
            _ => Err("[Lunar Error]: Expected value of type 'Floating'"),
        }
    }

    pub fn get_boolean(&self, arg: i32) -> Result<bool, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LunarType::BOOLEAN => Ok(lua_toboolean(self.0, arg)),
                _ => Err("[Lunar Error]: Expected value of type 'bool'"),
            }
        }
    }

    pub(crate) fn get_i32(&self, arg: i32) -> Result<i32, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LunarType::NUMBER => Ok(luaL_checkinteger(self.0, arg) as i32),
                _ => Err("[Lunar Error]: Expected value of type 'i32'"),
            }
        }
    }

    pub(crate) fn get_f64(&self, arg: i32) -> Result<f64, LunarError> {
        unsafe {
            match self.get_type(arg) {
                LunarType::NUMBER => Ok(luaL_checknumber(self.0, arg)),
                _ => Err("[Lunar Error]: Expected value of type 'f64'"),
            }
        }
    }

    pub(crate) fn push(&self, value: LunarValue) {
        unsafe {
            match value {
                LunarValue::NIL => lua_pushnil(self.0),
                LunarValue::BOOLEAN(b) => lua_pushboolean(self.0, b),
                LunarValue::LIGHTUSERDATA(ptr) => lua_pushlightuserdata(self.0, ptr),
                LunarValue::INTEGER(i) => lua_pushinteger(self.0, i as lua_Integer),
                LunarValue::NUMBER(f) => lua_pushnumber(self.0, f as lua_Number),
                LunarValue::STRING(s) => {
                    let c_str = match CString::new(s) {
                        Ok(c_str) => c_str,
                        Err(_) => panic!("[Lunar]: Unable to convert 'String' to 'const char*'."),
                    }
                    .into_raw();
                    lua_pushstring(self.0, c_str);
                }
                LunarValue::TABLE(t) => t.push_table(),
                LunarValue::FUNCTION(function) => {
                    let ptr = function as *const ();
                    let code: extern "C" fn(L: lua_State) = std::mem::transmute(ptr);
                    lua_pushcclosure(self.0, code as lua_CFunction, 0);
                }
                LunarValue::USERDATA(_ptr) => todo!(),
            }
        }
    }

    pub(crate) fn set_field(&self, field: &str, stack: i32) {
        unsafe {
            lua_setfield(self.0, stack, Lunar::to_const_char(field));
        }
    }

    pub fn returns(&self, value: LunarValue) -> i32 {
        self.push(value);
        return 1;
    }

    #[inline]
    fn get_type(&self, arg: i32) -> LunarType {
        unsafe {
            return lua_type(self.0, arg).into();
        }
    }
}

impl From<lua_State> for LunarContext {
    fn from(value: lua_State) -> Self {
        LunarContext(value)
    }
}

impl From<LunarState> for LunarContext {
    fn from(value: LunarState) -> Self {
        LunarContext(value.get())
    }
}
