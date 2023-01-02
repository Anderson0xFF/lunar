
#[derive(Debug, Clone, PartialEq)]
pub enum LuaType {
    Undefined,
    Nil,
    Bool,
    Function,
    Number,
    String,
    Userdata,
    LightUserdata,
    Table,
}

impl From<i32> for LuaType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Nil,
            1 => Self::Bool,
            2 => Self::LightUserdata,
            3 => Self::Number,
            4 => Self::String,
            5 => Self::Table,
            6 => Self::Function,
            7 => Self::Userdata,
            _ => Self::Undefined,
        }
    }
}


pub trait Type {
    fn type_of() -> LuaType;
}

impl Type for i8 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for i16 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for i32 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for i64 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for u8 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for u16 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for u32 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for u64 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for f32 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for f64 {
    fn type_of() -> LuaType {
        LuaType::Number
    }
}

impl Type for String {
    fn type_of() -> LuaType {
        LuaType::String
    }
}

impl Type for bool {
    fn type_of() -> LuaType {
        LuaType::Bool
    }
}


