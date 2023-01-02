use std::rc::Rc;

use crate::{
    context::{Function, LunarContext, Value},
    table::Table,
};

pub struct MetaTable {
    table: Table,
}

impl MetaTable {
    pub(crate) fn new(ctx: Rc<LunarContext>, name: &str) -> Self {
        Self {
            table: Table::new(ctx, name, false),
        }
    }

    pub fn constructor(&self, func: Function) {
        self.add_meta_method(MetaMethod::Call, Value::Function(func));
    }

    pub fn add_method(&self, name: &str, method: Function) {
        self.table.set(name, Value::Function(method));
    }

    pub fn add_meta_method(&self, metamethod: MetaMethod, value: Value) {
        self.table.set(metamethod.name(), value)
    }

    pub(crate) fn push_metatable(&self) -> i32 {
        self.table.push_table()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MetaMethod {
    /// The `+` operator.
    Add,
    /// The `-` operator.
    Sub,
    /// The `*` operator.
    Mul,
    /// The `/` operator.
    Div,
    /// The `%` operator.
    Mod,
    /// The `^` operator.
    Pow,
    /// The unary minus (`-`) operator.
    Unm,
    /// The floor division (//) operator.
    IDiv,
    /// The bitwise AND (&) operator.
    BAnd,
    /// The bitwise OR (|) operator.
    BOr,
    /// The bitwise XOR (binary ~) operator.
    BXor,
    /// The bitwise NOT (unary ~) operator.
    BNot,
    /// The bitwise left shift (<<) operator.
    Shl,
    /// The bitwise right shift (>>) operator.
    Shr,
    /// The string concatenation operator `..`.
    Concat,
    /// The length operator `#`.
    Len,
    /// The `==` operator.
    Eq,
    /// The `<` operator.
    Lt,
    /// The `<=` operator.
    Le,
    /// Index access `obj[key]`.
    Index,
    /// Index write access `obj[key] = value`.
    NewIndex,
    /// The call "operator" `obj(arg1, args2, ...)`.
    Call,
    /// The `__tostring` metamethod.
    ///
    /// This is not an operator, but will be called by methods such as `tostring` and `print`.
    ToString,
    /// The `__pairs` metamethod.
    ///
    /// This is not an operator, but it will be called by the built-in `pairs` function.
    Pairs,
}

impl MetaMethod {
    pub(crate) fn name(self) -> &'static str {
        match self {
            MetaMethod::Add => "__add",
            MetaMethod::Sub => "__sub",
            MetaMethod::Mul => "__mul",
            MetaMethod::Div => "__div",
            MetaMethod::Mod => "__mod",
            MetaMethod::Pow => "__pow",
            MetaMethod::Unm => "__unm",
            MetaMethod::IDiv => "__idiv",
            MetaMethod::BAnd => "__band",
            MetaMethod::BOr => "__bor",
            MetaMethod::BXor => "__bxor",
            MetaMethod::BNot => "__bnot",
            MetaMethod::Shl => "__shl",
            MetaMethod::Shr => "__shr",
            MetaMethod::Concat => "__concat",
            MetaMethod::Len => "__len",
            MetaMethod::Eq => "__eq",
            MetaMethod::Lt => "__lt",
            MetaMethod::Le => "__le",
            MetaMethod::Index => "__index",
            MetaMethod::NewIndex => "__newindex",
            MetaMethod::Call => "__call",
            MetaMethod::ToString => "__tostring",
            MetaMethod::Pairs => "__pairs",
        }
    }
}
