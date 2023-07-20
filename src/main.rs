use std::fs::metadata;
use std::{env, fs};
mod grammar;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let path = args[1].clone();
            let content: String;
            match metadata(&path) {
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
        _ => panic!("Compiler need only one given argument"),
    }
}
