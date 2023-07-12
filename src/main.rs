use std::{env, fs};
use std::fs::metadata;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let path = args[1].clone();
            let content: String;
            match metadata(&path) {
                Ok(p) => {
                    if p.is_dir() {
                        content = fs::read_to_string(format!("{path}/index.tl"))
                            .expect("Compiler is not able to read the file");
                    } else {
                        content = fs::read_to_string(path)
                            .expect("Compiler is not able to read the file");
                    }
                },
                Err(_) => panic!("Error while accessing the file")
            }
            let mut lex = lexer::LexicalParser::new(content);
            let result = lex.parse();
            println!("{:?}", result);
        },
        _ => panic!("Compiler need only one given argument")
    }
}
