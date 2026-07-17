use std::env;
use std::fs;

use gnc::interpreter::Interpreter;
use gnc::lexer::Lexer;
use gnc::parser::Parser;
use gnc::token::Token;

fn main() {
    // Ambil argumen command line
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gnc <file.gn>");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".gn") {
        eprintln!("Error: file harus berekstensi .gn");
        std::process::exit(1);
    }

    // Baca isi file
    let source = fs::read_to_string(filename)
        .expect("Tidak dapat membaca file");

    // Lexer
    let mut lexer = Lexer::new(source);

    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        let is_eof = token == Token::EOF;

        tokens.push(token);
       // println!("{:?}", tokens.last().unwrap());

        if is_eof {
            break;
        }
    }

    // Parser
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Interpreter
    let mut interpreter = Interpreter::new();
    interpreter.run(&program);
}
