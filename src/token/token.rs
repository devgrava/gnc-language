use super::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
