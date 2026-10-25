use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;
use std::path::PathBuf;

pub fn run_source(
    source: String,
    interpreter: &mut Interpreter,
) -> Result<(), crate::interpreter::signal::RuntimeSignal> {
    run_source_with_path(source, PathBuf::new(), interpreter)
}

pub fn run_source_with_path(
    source: String,
    path: PathBuf,
    interpreter: &mut Interpreter,
) -> Result<(), crate::interpreter::signal::RuntimeSignal> {
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
    match interpreter.run(&program) {
        Ok(()) => Ok(()),

        Err(crate::interpreter::signal::RuntimeSignal::Error(message)) => {
            eprintln!("Error: {}", message);
            Ok(())
        }

        Err(crate::interpreter::signal::RuntimeSignal::Break) => {
            eprintln!("Error: Unexpected 'break'");
            Ok(())
        }

        Err(crate::interpreter::signal::RuntimeSignal::Continue) => {
            eprintln!("Error: Unexpected 'continue'");
            Ok(())
        }

        Err(crate::interpreter::signal::RuntimeSignal::Return(_)) => {
            eprintln!("Error: Unexpected 'return'");
            Ok(())
        }
    }
}
