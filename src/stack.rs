#[repr(i32)]
#[derive(Debug, Clone)]
pub enum LuaStack {
    STACK1 = 1,
    STACK2 = 2,
    STACK3 = 3,
    STACK4 = 4,
    STACK5 = 5,
    STACK6 = 6
}


impl From<LuaStack> for i32 {
    fn from(value: LuaStack) -> Self {
        match value {
            LuaStack::STACK1 => 1,
            LuaStack::STACK2 => 2,
            LuaStack::STACK3 => 3,
            LuaStack::STACK4 => 4,
            LuaStack::STACK5 => 5,
            LuaStack::STACK6 => 6, 
        }
    }
}