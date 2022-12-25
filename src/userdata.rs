


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
    pub(crate) fn name(self) -> &'static [u8] {
        match self {
            MetaMethod::Add => b"__add",
            MetaMethod::Sub => b"__sub",
            MetaMethod::Mul => b"__mul",
            MetaMethod::Div => b"__div",
            MetaMethod::Mod => b"__mod",
            MetaMethod::Pow => b"__pow",
            MetaMethod::Unm => b"__unm",
            MetaMethod::IDiv => b"__idiv",
            MetaMethod::BAnd => b"__band",
            MetaMethod::BOr => b"__bor",
            MetaMethod::BXor => b"__bxor",
            MetaMethod::BNot => b"__bnot",
            MetaMethod::Shl => b"__shl",
            MetaMethod::Shr => b"__shr",
            MetaMethod::Concat => b"__concat",
            MetaMethod::Len => b"__len",
            MetaMethod::Eq => b"__eq",
            MetaMethod::Lt => b"__lt",
            MetaMethod::Le => b"__le",
            MetaMethod::Index => b"__index",
            MetaMethod::NewIndex => b"__newindex",
            MetaMethod::Call => b"__call",
            MetaMethod::ToString => b"__tostring",
            MetaMethod::Pairs => b"__pairs",
        }
    }
}