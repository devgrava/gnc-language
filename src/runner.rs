use crate::interpreter::Interpreter;
use std::path::PathBuf;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;

pub fn run_source(
    source: String,
    interpreter: &mut Interpreter,
) {
    run_source_with_path(
        source,
        PathBuf::new(),
        interpreter,
    );
}

pub fn run_source_with_path(
    source: String,
    path: PathBuf,
    interpreter: &mut Interpreter,
) {

    interpreter.set_current_file(path);

    // Lexer
    let mut lexer = Lexer::new(source);

    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        let is_eof = token == Token::EOF;

        tokens.push(token);

        if is_eof {
            break;
        }
    }

    // Parser
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Interpreter
    interpreter.run(&program);
}
