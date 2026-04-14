mod compiler;
mod html;
mod lexer;
mod syntaxtree;
mod token;
mod traits;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use compiler::Lolcompiler;
use html::Htmlgenerator;
use traits::Compiler;
use webbrowser;



///  validate command line arguments
///  read the input lol file
///  run lexical analysis and syntax compilation
///  generate HTML output
///  write the output file and open it in the browser when possible
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Lolcompiler <input.lol>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".lol") {
        eprintln!("input file must have a .lol extension");
        process::exit(1);
    }

    // Read the entire source file into memory for compilation
    let source = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("could not read '{}': {}", filename, error);
            process::exit(1);
        }
    };

    // Compile the lol text into an AST
    let mut compiler = Lolcompiler::new();
    compiler.compile(&source);

    // Generate HTML from the resulting abstract syntax tree
    let mut html_generator = Htmlgenerator::new();
    let output_html = html_generator.generate(compiler.ast());
    let output_file = Path::new(filename).with_extension("html");

    
    match fs::write(&output_file, output_html) {
        Ok(_) => {
            println!("compiled successfully to {}", output_file.display());
            if let Err(e) = webbrowser::open(output_file.to_str().unwrap()) {
                eprintln!(" could not open browser: {}", e);
            }
        }
        Err(error) => {
            eprintln!("could not write output file: {}", error);
            process::exit(1);
        }
    }
}
