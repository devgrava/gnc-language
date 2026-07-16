use super::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),

    Keyword(Keyword),

    Identifier(String),

    Number(f64),

    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

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
