use std::fs;
mod grammar;
mod lexer;
mod parser;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap();
    let content: String;
    match fs::metadata(&path) {
        Ok(p) => {
            if p.is_dir() {
                content = fs::read_to_string(format!("{path}/index.toy"))
                    .expect("Compiler is not able to read the file");
            } else {
                content = fs::read_to_string(path)
                    .expect("Compiler is not able to read the file");
            }
        }
        Err(_) => panic!("Error while accessing the file"),
    }
    let mut lex = lexer::LexicalParser::new(content);
    match lex.parse() {
        Ok(lexicon) => {
            let mut parser = parser::SyntaxAnalizer::new(lexicon);
            parser.parse();
            let result = parser.ast;
            println!("{:?}", result);
        }
        Err(error) => panic!("Lexical error: {}", error.as_ref().to_string()),
    }
    
}
