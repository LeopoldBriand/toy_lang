use crate::grammar::{StatementBlock, Statement, PrintStatement,Term, Expression};
use crate::lexer::{Token, TokenType};
pub struct SyntaxAnalizer {
    tokens: Vec<Token>,
    pub ast: StatementBlock,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    position: usize,
}
impl SyntaxAnalizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut analizer =  SyntaxAnalizer{
            tokens,
            ast: StatementBlock {
                statements: vec![],
                symbol_table: vec![],
            },
            current_token: None,
            peek_token: None,
            position: 0,
        };
        analizer.next_token();
        analizer.next_token();
        analizer
        
    }
    pub fn parse(&mut self){
        self.ast = self.parse_statement_block();
    }
    fn check_token(&mut self, token_type: TokenType) -> bool {
        match self.current_token.clone() {
            Some(token) => token.token_type == token_type ,
            None => false,
        }
    }
    fn check_peek(&mut self, token_type: TokenType) -> bool {
        match self.peek_token.clone() {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }
    fn check_token_and_value(&mut self, token_type: TokenType, value: &str) -> bool {
        match self.current_token.clone() {
            Some(token) => token.token_type == token_type && token.value == value.to_string(),
            None => false,
        }
    }
    fn check_peek_and_value(&mut self, token_type: TokenType, value: &str) -> bool {
        match self.peek_token.clone() {
            Some(token) => token.token_type == token_type && token.value == value.to_string(),
            None => false,
        }
    }
    fn get_token_value(&mut self, token: Option<Token>) -> String {
        match token {
            Some(t) => t.value,
            None => panic!("No token to parse"),
        }
    }
    fn next_token(&mut self){
        self.position += 1;
        self.current_token = self.peek_token.clone();
        if self.position < self.tokens.len() {
            self.peek_token = Some(self.tokens[self.position].clone());
        } else {
            self.peek_token = None;
        }
    }
    fn parse_statement_block(&mut self) -> StatementBlock{
        let mut block = StatementBlock { 
            statements: vec![], 
            symbol_table: vec![] 
        };
        while !self.check_token(TokenType::EndOfBlock) {
            block.statements.push(self.parse_statement());
        }
        return block;
    }
    fn parse_statement(&mut self) -> Statement {
        // "print" (expression | string)
        if self.check_token_and_value(TokenType::Keyword, "print") {
            self.next_token();
            // Should start with brackets
            if self.check_token_and_value(TokenType::GroupDivider, "(") {
                self.next_token();
                // If there only string, save print statement
                if self.check_token(TokenType::Text) && self.check_peek_and_value(TokenType::GroupDivider, ")"){
                    let text = self.current_token.clone();
                    self.next_token();
                    self.next_token();
                    if self.check_token(TokenType::EndOfStatement){
                        self.next_token();
                        return Statement::Print(
                            PrintStatement::Term(Term::String(self.get_token_value(text)))
                        );
                    } else {
                        panic!("Missing end of statement")
                    }
                } else { // It's an expression
                    let expression = self.parse_expression();
                    // Check for closing bracket
                    if self.check_token_and_value(TokenType::GroupDivider, ")"){
                        self.next_token();
                        if self.check_token(TokenType::EndOfStatement){
                            self.next_token();
                            return Statement::Print(
                                PrintStatement::Expression(expression)
                            );
                        } else {
                            panic!("Missing end of statement")
                        }
                    } else {
                        panic!("Missing closing bracket");
                    }
                    
                }
            }
            
        }
        panic!("Syntax Error: Statement cannot be matched: {:?}", self.current_token)
    }
    fn parse_expression(&mut self) -> Expression {
        todo!()
    }
    
}