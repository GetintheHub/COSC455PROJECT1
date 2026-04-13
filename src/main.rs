mod syntaxtree;
mod lexer;
mod token;

use std::env;
use std::fs;
use std::process;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("lolcompiler <input.lol>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".lol") {
        eprintln!(" input file must have a .lol extension");
        process::exit(1);
    }

    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not read '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.get_all_tokens();

    for token in tokens {
        println!("{:?} => {:?}", token.kind, token.lexeme);
    }
}