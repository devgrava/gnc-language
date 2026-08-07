#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Fn,
    Struct,

    If,
    Else,
    While,
    For,

    Return,
    Break,
    Continue,

    True,
    False,
    Null,

    Import,
    Print,
}
