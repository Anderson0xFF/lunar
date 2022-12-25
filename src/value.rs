use crate::{lua::*, context::LunarContext, table::Table};


#[derive(Debug, Clone)]
pub enum LunarValue{
    NIL,
    BOOLEAN(bool),
    LIGHTUSERDATA(void_ptr),
    INTEGER(i64),
    NUMBER(f64),
    STRING(String),
    TABLE(Table),
    FUNCTION(fn(L: LunarContext)-> i32),
    USERDATA(void_ptr),
}


impl From<i8> for LunarValue {
    fn from(value: i8) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<i16> for LunarValue {
    fn from(value: i16) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<i32> for LunarValue {
    fn from(value: i32) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<i64> for LunarValue {
    fn from(value: i64) -> Self {
        LunarValue::INTEGER(value)
    }
}

impl From<u8> for LunarValue {
    fn from(value: u8) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<u16> for LunarValue {
    fn from(value: u16) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<u32> for LunarValue {
    fn from(value: u32) -> Self {
        LunarValue::INTEGER(value.into())
    }
}

impl From<f64> for LunarValue {
    fn from(value: f64) -> Self {
        LunarValue::NUMBER(value)
    }
}

impl From<String> for LunarValue {
    fn from(value: String) -> Self {
        LunarValue::STRING(value)
    }
}