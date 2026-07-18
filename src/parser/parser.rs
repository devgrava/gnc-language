use crate::token::Token;
use crate::ast::Program;
use crate::ast::{Stmt, Expr};
use crate::token::Keyword;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
    pub fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }
    pub fn advance(&mut self) {
        if self.current + 1 < self.tokens.len() {
            self.current += 1;
        }
    }
    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
    pub fn parse_program(&mut self) -> Program {
       let mut statements = Vec::new();

       while !matches!(self.current_token(), Token::EOF) {
           if let Some(stmt) = self.parse_statement() {
               statements.push(stmt);
           } else {
               break;
           }
       }

       Program { statements }
    }
    fn parse_statement(&mut self) -> Option<Stmt> {
      match self.current_token() {
        Token::Keyword(Keyword::Let) => self.parse_let_statement(),
        Token::Keyword(Keyword::Print) => self.parse_print_statement(),
        Token::Keyword(Keyword::If) => self.parse_if_statement(),
        Token::Keyword(Keyword::While) => self.parse_while_statement(),
        Token::Keyword(Keyword::For) => self.parse_for_statement(),
        Token::Keyword(Keyword::Break) => self.parse_break_statement(),
        Token::Keyword(Keyword::Continue) => self.parse_continue_statement(),
        Token::Keyword(Keyword::Fn) => self.parse_function_statement(),
        Token::Keyword(Keyword::Return) => self.parse_return_statement(),

        Token::Identifier(_) => {
          match self.peek_token() {
            Some(Token::Equal)
            | Some(Token::PlusPlus)
            | Some(Token::MinusMinus)
            | Some(Token::PlusEqual)
            | Some(Token::MinusEqual)
            | Some(Token::StarEqual)
            | Some(Token::SlashEqual)
            | Some(Token::PercentEqual) => {
                self.parse_assign_statement()
            }

            _ => {
              self.parse_expression_statement()
            }
          }
        }

        _ => None,
      }
    }
    fn parse_function_statement(&mut self) -> Option<Stmt> {
      // lewati keyword "fn"
      self.advance();

      // nama fungsi
      let name = match self.current_token() {
          Token::Identifier(name) => name.clone(),
          _ => return None,
      };

      // lewati nama fungsi
      self.advance();

      // harus ada '('
      match self.current_token() {
          Token::LeftParen => {}
          _ => return None,
      }

      // lewati '('
      self.advance();

      // baca parameter
      let mut params = Vec::new();

      while !matches!(self.current_token(), Token::RightParen) {
          match self.current_token() {
            Token::Identifier(name) => {
                params.push(name.clone());
            }
            _ => return None,
          }

          self.advance();

          if matches!(self.current_token(), Token::Comma) {
              self.advance();
          }
      }

      // lewati ')'
      self.advance();

      // harus ada '{'
      match self.current_token() {
          Token::LeftBrace => {}
          _ => return None,
      }

      // lewati '{'
      self.advance();

      // baca isi function
      let mut body = Vec::new();

      while !matches!(self.current_token(), Token::RightBrace | Token::EOF) {
          if let Some(stmt) = self.parse_statement() {
              body.push(stmt);
          } else {
              return None;
          }
      }

      // harus ada '}'
      match self.current_token() {
          Token::RightBrace => {}
          _ => return None,
      }

      // lewati '}'
      self.advance();

      Some(Stmt::Function {
        name,
        params,
        body,
      })
    }

    fn parse_return_statement(&mut self) -> Option<Stmt> {
       // lewati "return"
       self.advance();

       let value = self.parse_expression(Precedence::Lowest)?;

       match self.current_token() {
          Token::Semicolon => self.advance(),
           _ => return None,
       }

       Some(Stmt::Return { value })
    }
    fn parse_let_core(&mut self) -> Option<Stmt> {
       // lewati "let"
       self.advance();

       let name = match self.current_token() {
          Token::Identifier(name) => name.clone(),
          _ => return None,
       };

       // lewati identifier
       self.advance();

       match self.current_token() {
          Token::Equal => {}
          _ => return None,
       }

       // lewati '='
       self.advance();

       let value = self.parse_expression(Precedence::Lowest)?;

       Some(Stmt::Let {
          name,
          value,
       })
    }
    fn parse_let_statement(&mut self) -> Option<Stmt> {
       let stmt = self.parse_let_core()?;

       match self.current_token() {
          Token::Semicolon => {}
          _ => return None,
       }

       self.advance();

       Some(stmt)
    }
    fn parse_assign_core(&mut self) -> Option<Stmt> {
       let name = match self.current_token() {
           Token::Identifier(name) => name.clone(),
           _ => return None,
       };

       // lewati identifier
       self.advance();

       let value = match self.current_token() {
            Token::Equal => {
               // lewati '='
               self.advance();

               self.parse_expression(Precedence::Lowest)?
            }

            Token::PlusPlus => {
               // lewati '++'
               self.advance();

               Expr::Binary {
                  left: Box::new(Expr::Identifier(name.clone())),
                  operator: "+".to_string(),
                  right: Box::new(Expr::Number(1.0)),
               }
            }

            Token::MinusMinus => {
               // lewati '--'
               self.advance();

               Expr::Binary {
                  left: Box::new(Expr::Identifier(name.clone())),
                  operator: "-".to_string(),
                  right: Box::new(Expr::Number(1.0)),
               }
            }

            _ => return None,
         };

         Some(Stmt::Assign {
            name,
            value,
         })
    }   
    fn parse_assign_statement(&mut self) -> Option<Stmt> {
       let stmt = self.parse_assign_core()?;

       match self.current_token() {
          Token::Semicolon => {}
          _ => return None,
       }

       // lewati ';'
       self.advance();

       Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Stmt> {
       let expression = self.parse_expression(Precedence::Lowest)?;

       match self.current_token() {
          Token::Semicolon => {}
          _ => return None,
       }

       // lewati ';'
       self.advance();

       Some(Stmt::Expression {
          expression,
       })
    }

    fn parse_print_statement(&mut self) -> Option<Stmt> {
       // lewati 'print'
       self.advance();

       match self.current_token() {
           Token::LeftParen => {}
           _ => return None,
       }

       // lewati '('
       self.advance();

       let value = self.parse_expression(Precedence::Lowest)?;

       match self.current_token() {
           Token::RightParen => {}
           _ => return None,
       }

       // lewati ')'
       self.advance();

       match self.current_token() {
           Token::Semicolon => {}
           _ => return None,
       }

       // lewati ';'
       self.advance();

       Some(Stmt::Print { value })
    }
    fn parse_if_statement(&mut self) -> Option<Stmt> {
       // lewati 'if'
       self.advance();

       // harus '('
       match self.current_token() {
           Token::LeftParen => {}
            _ => return None,
       }

       // lewati '('
       self.advance();

       // kondisi
       let condition = self.parse_expression(Precedence::Lowest)?;

       // harus ')'
       match self.current_token() {
          Token::RightParen => {}
           _ => return None,
       }

       // lewati ')'
       self.advance();

       // harus '{'
       match self.current_token() {
          Token::LeftBrace => {}
           _ => return None,
       }

       // lewati '{'
       self.advance();

       let mut then_branch = Vec::new();

       while !matches!(self.current_token(), Token::RightBrace | Token::EOF) {
          if let Some(stmt) = self.parse_statement() {
               then_branch.push(stmt);
          } else {
               return None;
          }
       }

       // harus '}'
       match self.current_token() {
           Token::RightBrace => {}
           _ => return None,
       }

       // lewati '}'
       self.advance();
       let mut else_branch = None;

       if matches!(self.current_token(), Token::Keyword(Keyword::Else)) {
       // lewati 'else'
       self.advance();

       match self.current_token() {
          Token::LeftBrace => {}
           _ => return None,
       }

       // lewati '{'
       self.advance();

       let mut statements = Vec::new();

          while !matches!(self.current_token(), Token::RightBrace | Token::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                return None;
            }
          }

         match self.current_token() {
            Token::RightBrace => {}
             _ => return None,
         }

         // lewati '}'
         self.advance();

         else_branch = Some(statements);
       }
       Some(Stmt::If {
           condition,
           then_branch,
           else_branch,
       })
    }
    fn parse_while_statement(&mut self) -> Option<Stmt> {
       // lewati 'while'
       self.advance();

       match self.current_token() {
          Token::LeftParen => {}
           _ => return None,
       }

       self.advance();

       let condition = self.parse_expression(Precedence::Lowest)?;

       match self.current_token() {
          Token::RightParen => {}
           _ => return None,
       }

       self.advance();

       match self.current_token() {
          Token::LeftBrace => {}
           _ => return None,
          }

       self.advance();

       let mut body = Vec::new();

       while !matches!(self.current_token(), Token::RightBrace | Token::EOF) {
          if let Some(stmt) = self.parse_statement() {
              body.push(stmt);
          } else {
              return None;
          }
       }

       match self.current_token() {
           Token::RightBrace => {}
           _ => return None,
       }

       self.advance();

       Some(Stmt::While {
          condition,
          body,
       })
    }
    fn parse_for_statement(&mut self) -> Option<Stmt> {
       // lewati 'for'
       self.advance();

       // harus '('
       match self.current_token() {
          Token::LeftParen => {}
          _ => return None,
       }

       self.advance();

       // initializer
       let init = match self.current_token() {
          Token::Keyword(Keyword::Let) => self.parse_let_core()?,
          Token::Identifier(_) => self.parse_assign_core()?,
          _ => return None,
       };

       // harus ';'
       match self.current_token() {
          Token::Semicolon => {}
          _ => return None,
       }

       self.advance();

       // condition
       let condition = self.parse_expression(Precedence::Lowest)?;

       // harus ';'
       match self.current_token() {
          Token::Semicolon => {}
          _ => return None,
       }

       self.advance();

       // update
       let update = match self.current_token() {
          Token::Identifier(_) => self.parse_assign_core()?,
          _ => return None,
       };

       // harus ')'
       match self.current_token() {
          Token::RightParen => {}
          _ => return None,
       }

       self.advance();

       // harus '{'
       match self.current_token() {
          Token::LeftBrace => {}
          _ => return None,
       }

       self.advance();

       let mut body = Vec::new();

       while !matches!(self.current_token(), Token::RightBrace | Token::EOF) {
          if let Some(stmt) = self.parse_statement() {
              body.push(stmt);
          } else {
              return None;
          }
       }

       // harus '}'
       match self.current_token() {
          Token::RightBrace => {}
          _ => return None,
       }

       self.advance();

       Some(Stmt::For {
          init: Box::new(init),
          condition,
          update: Box::new(update),
          body,
       })
    }

    fn parse_break_statement(&mut self) -> Option<Stmt> {
       self.advance();

       match self.current_token() {
           Token::Semicolon => {}
           _ => return None,
       }

       self.advance();

       Some(Stmt::Break)
    }
    fn parse_continue_statement(&mut self) -> Option<Stmt> {
       self.advance();

       match self.current_token() {
          Token::Semicolon => {}
           _ => return None,
       }

       self.advance();

       Some(Stmt::Continue)
    }
  
    fn parse_primary(&mut self) -> Option<Expr> {
       match self.current_token() {
          Token::Plus | Token::Minus | Token::Bang => {
              let operator = match self.current_token() {
                 Token::Plus => "+",
                 Token::Minus => "-",
                 Token::Bang => "!",
                 _ => unreachable!(),
              }
              .to_string();

              // lewati operator
              self.advance();

              let right = self.parse_primary()?;

              return Some(Expr::Unary {
                  operator,
                  right: Box::new(right),
              });
          }

          _ => {}
       }

       match self.current_token() {
           Token::Number(value) => {
               let expr = Expr::Number(*value);
               self.advance();
               Some(expr)
           }

           Token::String(text) => {
               let expr = Expr::String(text.clone());
               self.advance();
               Some(expr)
           }

           Token::Identifier(name) => {
               let expr = Expr::Identifier(name.clone());

               self.advance();

               if matches!(self.current_token(), Token::LeftParen) {
                  return self.parse_call_expression(expr);
               }

               if matches!(self.current_token(), Token::LeftBracket) {
                  return self.parse_index_expression(expr);
               }

               Some(expr)
           }

           Token::Keyword(Keyword::True) => {
               self.advance();
               Some(Expr::Boolean(true))
           }

           Token::Keyword(Keyword::False) => {
               self.advance();
               Some(Expr::Boolean(false))
           }

           Token::Keyword(Keyword::Null) => {
               self.advance();
               Some(Expr::Null)
           }
           Token::LeftBracket => {
               self.parse_array_expression()
           }

           _ => None,
       }
    }
    fn parse_call_expression(&mut self, callee: Expr) -> Option<Expr> {
       // lewati '('
       self.advance();

       let mut arguments = Vec::new();

       while !matches!(self.current_token(), Token::RightParen) {
          let argument = self.parse_expression(Precedence::Lowest)?;
             arguments.push(argument);

            if matches!(self.current_token(), Token::Comma) {
               self.advance();
            }
          }

         // lewati ')'
         self.advance();

         Some(Expr::Call {
           callee: Box::new(callee),
           arguments,
         })
    }
    fn parse_index_expression(&mut self, object: Expr) -> Option<Expr> {
        // lewati '['
        self.advance();

        let index = self.parse_expression(Precedence::Lowest)?;

        match self.current_token() {
           Token::RightBracket => {}
           _ => return None,
        }

        // lewati ']'
        self.advance();

        Some(Expr::Index {
           object: Box::new(object),
           index: Box::new(index),
        })
    }
    fn parse_array_expression(&mut self) -> Option<Expr> {
        // lewati '['
        self.advance();

        let mut elements = Vec::new();

        while !matches!(self.current_token(), Token::RightBracket) {
            let expr = self.parse_expression(Precedence::Lowest)?;
            elements.push(expr);

            if matches!(self.current_token(), Token::Comma) {
               self.advance();
            }
        }

        // lewati ']'
        self.advance();

        Some(Expr::Array {
           elements,
        })
    } 
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expr> {
       let mut left = self.parse_primary()?;

       while self.current_precedence() > precedence {
          left = self.parse_infix(left)?;
       }

       Some(left)
    }

    fn parse_infix(&mut self, left: Expr) -> Option<Expr> {
      let operator = match self.current_token() {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Star => "*",
        Token::Slash => "/",
        Token::Percent => "%",
        Token::EqualEqual => "==",
        Token::BangEqual => "!=",
        Token::Less => "<",
        Token::LessEqual => "<=",
        Token::Greater => ">",
        Token::GreaterEqual => ">=",
        _ => return Some(left),
      }
      .to_string();

      let precedence = self.current_precedence();

      // lewati operator
      self.advance();

      let right = self.parse_expression(precedence)?;

      Some(Expr::Binary {
          left: Box::new(left),
          operator,
          right: Box::new(right),
      })
    }   
          
    fn current_precedence(&self) -> Precedence {
       match self.current_token() {
          Token::EqualEqual | Token::BangEqual => Precedence::Equality,

          Token::Less
          | Token::LessEqual
          | Token::Greater
          | Token::GreaterEqual => Precedence::Comparison,

            Token::Plus | Token::Minus => Precedence::Term,

            Token::Star | Token::Slash | Token::Percent => Precedence::Factor,

            _ => Precedence::Lowest,
       }
    }
    fn peek_precedence(&self) -> Precedence {
      match self.peek_token() {
        Some(Token::EqualEqual) | Some(Token::BangEqual) => Precedence::Equality,

        Some(Token::Less)
        | Some(Token::LessEqual)
        | Some(Token::Greater)
        | Some(Token::GreaterEqual) => Precedence::Comparison,

        Some(Token::Plus)
        | Some(Token::Minus) => Precedence::Term,

        Some(Token::Star)
        | Some(Token::Slash)
        | Some(Token::Percent) => Precedence::Factor,

        _ => Precedence::Lowest,
      }
    }
}
