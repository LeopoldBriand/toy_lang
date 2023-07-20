use std::collections::HashMap;

use crate::grammar::{
    AssignmentStatement, Expression, Identifier, IfStatement, PrintStatement, Statement,
    StatementBlock, Term,
};
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
        let mut analizer = SyntaxAnalizer {
            tokens,
            ast: StatementBlock {
                statements: vec![],
                symbol_table: HashMap::new(),
            },
            current_token: None,
            peek_token: None,
            position: 0,
        };
        analizer.next_token();
        analizer.next_token();
        analizer
    }
    pub fn parse(&mut self) {
        self.ast = self.parse_statement_block();
    }
    fn check_token(&mut self, token_type: TokenType) -> bool {
        match self.current_token.clone() {
            Some(token) => token.token_type == token_type,
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
    fn next_token(&mut self) {
        self.position += 1;
        self.current_token = self.peek_token.clone();
        if self.position < self.tokens.len() {
            self.peek_token = Some(self.tokens[self.position].clone());
        } else {
            self.peek_token = None;
        }
    }
    fn parse_statement_block(&mut self) -> StatementBlock {
        let mut block = StatementBlock {
            statements: vec![],
            symbol_table: HashMap::new(),
        };
        if self.check_token(TokenType::StartOfBlock) {
            self.next_token();
            while !self.check_token(TokenType::EndOfBlock) {
                let statement = self.parse_statement(&mut block);
                block.statements.push(statement);
            }
        } else {
            panic!("Missing opening block");
        }
        return block;
    }
    fn parse_statement(&mut self, block: &mut StatementBlock) -> Statement {
        // print_statement ::= (expression) | string_literal
        if self.check_token_and_value(TokenType::Keyword, "print") {
            self.next_token();
            // Should start with brackets
            if self.check_token_and_value(TokenType::GroupDivider, "(") {
                self.next_token();
                // If there only string, save print statement
                if self.check_token(TokenType::Text)
                    && self.check_peek_and_value(TokenType::GroupDivider, ")")
                {
                    let text = self.current_token.clone();
                    self.next_token();
                    self.next_token();
                    if self.check_token(TokenType::EndOfStatement) {
                        self.next_token();
                        return Statement::Print(PrintStatement::Term(Term::String(
                            self.get_token_value(text),
                        )));
                    } else {
                        panic!("Missing end of statement")
                    }
                } else {
                    // It's an expression
                    let expression = self.parse_expression();
                    // Check for closing bracket
                    if self.check_token_and_value(TokenType::GroupDivider, ")") {
                        self.next_token();
                        if self.check_token(TokenType::EndOfStatement) {
                            self.next_token();
                            return Statement::Print(PrintStatement::Expression(expression));
                        } else {
                            panic!("Missing end of statement")
                        }
                    } else {
                        panic!("Missing closing bracket");
                    }
                }
            } else {
                panic!("Missing opening bracket");
            }
        // if_statement ::= if (expression) statement_block else statement_block
        } else if self.check_token_and_value(TokenType::Keyword, "if") {
            let expression: Expression;
            let then_statement_block: StatementBlock;
            let mut else_statement_block: Option<StatementBlock> = None;
            self.next_token();
            // Should start with brackets
            if self.check_token_and_value(TokenType::GroupDivider, "(") {
                self.next_token();
                expression = self.parse_expression();
                // Check for closing bracket
                if self.check_token_and_value(TokenType::GroupDivider, ")") {
                    self.next_token();
                    then_statement_block = self.parse_statement_block();
                    if self.check_token_and_value(TokenType::Keyword, "else") {
                        self.next_token();
                        else_statement_block = Some(self.parse_statement_block());
                    }
                } else {
                    panic!("Missing closing bracket");
                }
            } else {
                panic!("Missing opening bracket");
            }
            return Statement::If(IfStatement {
                expression,
                then_statement_block,
                else_statement_block,
            });
        // declaration_statement ::= var identifier = expression
        } else if self.check_token_and_value(TokenType::Keyword, "var") {
            self.next_token();
            if self.check_token(TokenType::Identifier) {
                // Check if identifier already exist in statement block
                let identifier_value = self.current_token.clone().unwrap().value;
                self.next_token();
                if !block.symbol_table.contains_key(&identifier_value) {
                    let identifier = Identifier {};
                    let expression: Expression;
                    if self.check_token_and_value(TokenType::Operator, "=") {
                        self.next_token();
                        expression = self.parse_expression();
                        if self.check_token(TokenType::EndOfStatement) {
                            self.next_token();
                        } else {
                            panic!("Missing end of statement")
                        }
                    } else {
                        panic!("Assignement without '=' sign");
                    }
                    block
                        .symbol_table
                        .insert(identifier_value, identifier.clone());
                    return Statement::Assignment(AssignmentStatement {
                        expression,
                        identifier,
                    });
                } else {
                    panic!("Identifier {} already used", identifier_value)
                }
            } else {
                panic!("Identifier needed after var keyword");
            }
        // assignment_statement ::= identifier = expression
        } else if self.check_token(TokenType::Identifier) {
            // Check if identifier already exist in statement block
            let identifier_value = self.current_token.clone().unwrap().value;
            self.next_token();
            if block.symbol_table.contains_key(&identifier_value) {
                let identifier = Identifier {};
                let expression: Expression;
                if self.check_token_and_value(TokenType::Operator, "=") {
                    self.next_token();
                    expression = self.parse_expression();
                    if self.check_token(TokenType::EndOfStatement) {
                        self.next_token();
                    } else {
                        panic!("Missing end of statement")
                    }
                } else {
                    panic!("Assignement without '=' sign");
                }
                return Statement::Assignment(AssignmentStatement {
                    expression,
                    identifier,
                });
            } else {
                panic!("Identifier {} not declared", identifier_value)
            }
        }
        panic!(
            "Syntax Error: Statement cannot be matched: {:?}",
            self.current_token
        )
    }
    fn parse_expression(&mut self) -> Expression {
        todo!()
    }
}
