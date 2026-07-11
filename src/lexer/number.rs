use crate::lexer::Lexer;
use crate::token::token::Token;

impl Lexer {
    pub(crate) fn read_number(&mut self, first: char) -> Token {
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
}
