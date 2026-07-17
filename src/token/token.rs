use super::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),

    Keyword(Keyword),

    Identifier(String),

    Number(f64),

    String(String),

    Plus,
    PlusPlus,
    PlusEqual,

    Minus,
    MinusMinus,
    MinusEqual,

    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,

    Equal,
    EqualEqual,

    Bang,
    BangEqual,

    Less,
    LessEqual,

    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    Comma,
    Dot,
    Colon,
    Semicolon,

    EOF,
}
