use std::{
    fs::{File, metadata},
    io::{prelude::*, BufReader},
};
mod grammar;
mod lexer;
mod parser;
mod interpreter;
mod errors;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run interpreter instead of compiler
    #[arg(short, long)]
    interpreter: bool,
    /// Path of toy lang file to compile or interpret
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap();
    let content: Vec<String>;
    match metadata(&path) {
        Ok(p) => {
            if p.is_dir() {
                content = lines_from_file(format!("{path}/index.toy"));
            } else {
                content = lines_from_file(path);
            }
        }
        Err(_) => panic!("Error while accessing the file"),
    }
    let mut lex = lexer::LexicalParser::new(content);
    match lex.parse() {
        Ok(lexicon) => {
            let mut parser = parser::SyntaxAnalizer::new(lexicon);
            match parser.parse() {
                Ok(ast) => {
                    if cli.interpreter {
                        interpreter::interpret(ast);
                    } else {
                        println!("{:?}", ast);
                    }
                },
                Err(error) => println!("{}", error.to_string()),
            }
            
        }
        Err(error) => println!("{}", error.to_string()),
    }
    
}

fn lines_from_file(filename: String) -> Vec<String> {
    let file = File::open(filename.clone()).expect(&format!("Compiler is not able to read the file {}", filename));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}