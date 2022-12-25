
pub type LunarError = &'static str;

#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LunarType  {
    UNDEFINED,
    NIL,
    BOOLEAN,
    LIGHTUSERDATA,
    NUMBER,
    STRING,
    TABLE,
    FUNCTION,
    USERDATA,
    INTEGER,
}



impl From<i32> for LunarType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::NIL,
            1 => Self::BOOLEAN,
            2 => Self::LIGHTUSERDATA,
            3 => Self::NUMBER,
            4 => Self::STRING,
            5 => Self::TABLE,
            6 => Self::FUNCTION,
            7 => Self::USERDATA,
            _=> Self::UNDEFINED
        }
    }
}

pub trait Type {
    fn type_of() -> LunarType;
}

impl Type for i8 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for i16 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for i32 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for i64 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for u8 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for u16 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for u32 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for u64 {
    fn type_of() -> LunarType {
        LunarType::INTEGER
    }
}

impl Type for f32 {
    fn type_of() -> LunarType {
        LunarType::NUMBER
    }
}

impl Type for f64 {
    fn type_of() -> LunarType {
        LunarType::NUMBER
    }
}

impl Type for String {
    fn type_of() -> LunarType {
        LunarType::STRING
    }
}

impl Type for bool {
    fn type_of() -> LunarType {
        LunarType::BOOLEAN
    }
}


