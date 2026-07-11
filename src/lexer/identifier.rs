use crate::lexer::Lexer;
use crate::token::keyword::Keyword;
use crate::token::token::Token;

impl Lexer {
    pub(crate) fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while let Some(ch) = self.scanner.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(self.scanner.advance().unwrap());
            } else {
                break;
            }
        }

        ident
    }

    pub(crate) fn identifier_or_keyword(&self, text: String) -> Token {
        match text.as_str() {
            "let" => Token::Keyword(Keyword::Let),
            "fn" => Token::Keyword(Keyword::Fn),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "while" => Token::Keyword(Keyword::While),
            "for" => Token::Keyword(Keyword::For),
            "return" => Token::Keyword(Keyword::Return),
            "break" => Token::Keyword(Keyword::Break),
            "continue" => Token::Keyword(Keyword::Continue),
            "true" => Token::Keyword(Keyword::True),
            "false" => Token::Keyword(Keyword::False),
            "null" => Token::Keyword(Keyword::Null),
            "import" => Token::Keyword(Keyword::Import),
            _ => Token::Identifier(text),
        }
    }
}
