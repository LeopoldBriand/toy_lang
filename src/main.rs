mod lexer;

fn main() {
    let mut lex = lexer::LexicalParser::new("
    var toto = 12 + 5;
    if (toto > 6){
        print('more than 6');
    } else {
        print('less than 6');
    }
    ".to_string());
    let result = lex.parse();
    println!("{:?}", result);
}
