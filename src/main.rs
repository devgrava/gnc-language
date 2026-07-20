use std::env;
use std::fs;

use gnc::interpreter::Interpreter;
use gnc::runner;

fn main() {
    // Ambil argumen command line
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gnc <file.gn>");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".gn") {
        eprintln!("Error: file must have a .gn extension");
        std::process::exit(1);
    }

    // Read source file
    let source = fs::read_to_string(filename)
        .expect("Failed to read source file");

    // Interpreter
    let mut interpreter = Interpreter::new();

    runner::run_source_with_file(source, filename, &mut interpreter);
}
