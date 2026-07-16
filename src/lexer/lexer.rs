use crate::lexer::Scanner;
use crate::token::keyword::Keyword;
use crate::token::token::Token;

pub struct Lexer {
    scanner: Scanner,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
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

    fn identifier_or_keyword(&self, text: String) -> Token {
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
            "print" => Token::Keyword(Keyword::Print),

            _ => Token::Identifier(text),
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::new();
        number.push(first);

        while let Some(ch) = self.scanner.peek() {
            if ch.is_ascii_digit() {
                number.push(self.scanner.advance().unwrap());
            } else {
                break;
            }
        }

        if let Some('.') = self.scanner.peek() {
            if let Some(next) = self.scanner.peek_next() {
                if next.is_ascii_digit() {
                    number.push(self.scanner.advance().unwrap());

                    while let Some(ch) = self.scanner.peek() {
                        if ch.is_ascii_digit() {
                            number.push(self.scanner.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Token::Number(number.parse::<f64>().unwrap())
    }
    fn read_string(&mut self) -> Token {
       let mut value = String::new();

       while let Some(ch) = self.scanner.advance() {
           if ch == '"' {
              return Token::String(value);
           }

           if ch == '\\' {
              if let Some(next) = self.scanner.advance() {
                  match next {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    _ => {
                      value.push('\\');
                      value.push(next);
                   }
               }
           }
         } else {
           value.push(ch);
         }
       }

       Token::EOF
    }

    fn match_char(&mut self, expected: char) -> bool {
       if self.scanner.peek() == Some(expected) {
            self.scanner.advance();
            true
       } else {
            false
       }
    }
    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.scanner.peek() {
             if ch == '\n' {
                 break;
             }

             self.scanner.advance();
        }
    }
    fn skip_block_comment(&mut self) {
        while let Some(ch) = self.scanner.advance() {
           if ch == '*' && self.scanner.peek() == Some('/') {
               self.scanner.advance();
               break;
           }
        }
    }

    pub fn next_token(&mut self) -> Token {
        // Lewati whitespace
        while let Some(ch) = self.scanner.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.scanner.advance();
                }
                _ => break,
            }
        }

        // Akhir file
        if self.scanner.is_at_end() {
            return Token::EOF;
        }

        // Ambil karakter berikutnya
        let ch = self.scanner.advance().unwrap();

        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => {
              if self.match_char('/') {
                self.skip_line_comment();
                return self.next_token();
              }

              if self.match_char('*') {
                self.skip_block_comment();
                return self.next_token();
              }

              Token::Slash
            }
            '%' => Token::Percent,

            '=' => {
                if self.scanner.peek() == Some('=') {
                   self.scanner.advance();
                   Token::EqualEqual
                } else {
                   Token::Equal
                }
            }
            '!' => {
                if self.match_char('=') {
                   Token::BangEqual
                } else {
                   Token::Bang
                }
            }
            '<' => {
                if self.match_char('=') {
                   Token::LessEqual
                } else {
                   Token::Less
                }
            }
            '>' => {
               if self.match_char('=') {
                   Token::GreaterEqual
               } else {
                   Token::Greater
               }
            }
            '&' => {
               if self.match_char('&') {
                  Token::AndAnd
               } else {
                  Token::EOF
               }
             }

            '|' => {
                if self.match_char('|') {
                     Token::OrOr
                } else {
                     Token::EOF
                }
            }

            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            ':' => Token::Colon,

            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '"' => self.read_string(),

            '0'..='9' => self.read_number(ch),

            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_identifier(ch);
                self.identifier_or_keyword(ident)
            }

            _ => Token::Illegal(ch),
        }
    }
}
