use std::collections::HashMap;

use crate::errors::SyntaxError;
use crate::grammar::{
    AssignmentStatement, Expression, Identifier, IfStatement, Operation, Operator, PrintStatement,
    Statement, StatementBlock, Term,
};
use crate::lexer::{Token, TokenType};
pub struct SyntaxAnalizer {
    tokens: Vec<Token>,
    pub ast: StatementBlock,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    token_pos: usize,
    file_pos: (i32, i32),
}
impl SyntaxAnalizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut analizer = SyntaxAnalizer {
            tokens: tokens.clone(),
            ast: StatementBlock {
                statements: vec![],
                symbol_table: HashMap::new(),
            },
            current_token: None,
            peek_token: None,
            token_pos: 1,
            file_pos: (1,1),
        };
        if tokens.len() > 2 {
            analizer.current_token = Some(tokens[0].clone());
            analizer.peek_token = Some(tokens[1].clone());
        } else {
            panic!("Empty file");
        }
        analizer
    }
    pub fn parse(&mut self) -> Result<StatementBlock, SyntaxError>{
        self.parse_statement_block()
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
        self.token_pos += 1;
        self.current_token = self.peek_token.clone();
        if let Some(token) = self.current_token.clone() {
            self.file_pos = token.pos;
        }
        if self.token_pos < self.tokens.len() {
            self.peek_token = Some(self.tokens[self.token_pos].clone());
        } else {
            self.peek_token = None;
        }
    }
    fn parse_statement_block(&mut self) -> Result<StatementBlock, SyntaxError> {
        let mut block = StatementBlock {
            statements: vec![],
            symbol_table: HashMap::new(),
        };
        if self.check_token(TokenType::StartOfBlock) {
            self.next_token();
            while !self.check_token(TokenType::EndOfBlock) {
                match self.parse_statement(&mut block){
                    Ok(statement) => block.statements.push(statement),
                    Err(error) => return Err(error),
                }
            }
            self.next_token();
        } else {
            return Err(self.get_error("Missing opening block"));
        }
        return Ok(block);
    }
    fn parse_statement(&mut self, block: &mut StatementBlock) -> Result<Statement, SyntaxError> {
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
                        return Ok(Statement::Print(PrintStatement::Term(Term::String(
                            self.get_token_value(text),
                        ))));
                    } else {
                        return Err(self.get_error("Missing end of statement"));
                    }
                } else {
                    // It's an expression
                    let expression = self.parse_expression(block);
                    // Check for closing bracket
                    if self.check_token_and_value(TokenType::GroupDivider, ")") {
                        self.next_token();
                        if self.check_token(TokenType::EndOfStatement) {
                            self.next_token();
                            return Ok(Statement::Print(PrintStatement::Expression(expression)));
                        } else {
                            return Err(self.get_error("Missing end of statement"));
                        }
                    } else {
                        return Err(self.get_error("Missing closing bracket"));
                    }
                }
            } else {
                return Err(self.get_error("Missing opening bracket"));
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
                expression = self.parse_expression(block);
                // Check for closing bracket
                if self.check_token_and_value(TokenType::GroupDivider, ")") {
                    self.next_token();
                    match self.parse_statement_block() {
                        Ok(block) => {
                            then_statement_block = block;
                            if self.check_token_and_value(TokenType::Keyword, "else") {
                                self.next_token();
                                match self.parse_statement_block() {
                                    Ok(block) => else_statement_block = Some(block),
                                    Err(error) => return Err(error),
                                }
                            }
                        },
                        Err(error) => return Err(error),
                    }
                } else {
                    return Err(self.get_error("Missing closing bracket"));
                }
            } else {
                return Err(self.get_error("Missing opening bracket"));
            }
            return Ok(Statement::If(IfStatement {
                expression,
                then_statement_block,
                else_statement_block,
            }));
        // declaration_statement ::= var identifier = expression
        } else if self.check_token_and_value(TokenType::Keyword, "var") {
            self.next_token();
            if self.check_token(TokenType::Identifier) {
                // Check if identifier already exist in statement block
                let identifier_value = self.current_token.clone().unwrap().value;
                self.next_token();
                if !block.symbol_table.contains_key(&identifier_value) {
                    let identifier = Identifier {name: identifier_value.clone(), value: None};
                    let expression: Expression;
                    if self.check_token_and_value(TokenType::Operator, "=") {
                        self.next_token();
                        expression = self.parse_expression(block);
                        if self.check_token(TokenType::EndOfStatement) {
                            self.next_token();
                        } else {
                            return Err(self.get_error("Missing end of statement"));
                        }
                    } else {
                        return Err(self.get_error("Assignement without '=' sign"));
                    }
                    block
                        .symbol_table
                        .insert(identifier_value, identifier.clone());
                    return Ok(Statement::Assignment(AssignmentStatement {
                        expression,
                        identifier,
                    }));
                } else {
                    return Err(self.get_error(&format!("Identifier {} already used", identifier_value)));
                }
            } else {
                return Err(self.get_error("Identifier needed after var keyword"));
            }
        // assignment_statement ::= identifier = expression
        } else if self.check_token(TokenType::Identifier) {
            // Check if identifier already exist in statement block
            let identifier_value = self.current_token.clone().unwrap().value;
            self.next_token();
            if block.symbol_table.contains_key(&identifier_value) {
                let identifier = Identifier {name: identifier_value, value: None};
                let expression: Expression;
                if self.check_token_and_value(TokenType::Operator, "=") {
                    self.next_token();
                    expression = self.parse_expression(block);
                    if self.check_token(TokenType::EndOfStatement) {
                        self.next_token();
                    } else {
                        return Err(self.get_error("Missing end of statement"));
                    }
                } else {
                    return Err(self.get_error("Assignement without '=' sign"));
                }
                return Ok(Statement::Assignment(AssignmentStatement {
                    expression,
                    identifier,
                }));
            } else {
                return Err(self.get_error(&format!("Identifier {} not declared", identifier_value)))
            }
        }
        return Err(self.get_error(&format!(
            "Syntax Error: Statement cannot be matched: {:?}",
            self.current_token
        )))
    }
    fn parse_expression(&mut self, block: &mut StatementBlock) -> Expression {
        if self.check_peek(TokenType::Operator) {
            let value = self.get_token_value(self.peek_token.clone());
            let operator: Operator = self.parse_operator(value);
            let left_expression = self.parse_term(block);
            self.next_token();
            let right_expresion = self.parse_expression(block);
            return Expression::Operation(Box::new(Operation {
                left: Expression::Term(left_expression),
                operator,
                right: right_expresion,
            }));
        } else {
            return Expression::Term(self.parse_term(block));
        }
    }
    fn parse_operator(&mut self, value: String) -> Operator {
        match value.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "%" => Operator::Modulo,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            "<=" => Operator::InfOrEqual,
            ">=" => Operator::SupOrEqual,
            ">" => Operator::Superior,
            "<" => Operator::Inferior,
            "&&" => Operator::And,
            "||" => Operator::Or,
            _ => panic!("Unknown operator"),
        }
    }
    fn parse_term(&mut self, block: &mut StatementBlock) -> Term {
        if self.check_token(TokenType::Identifier) {
            let identifier_value = self.current_token.clone().unwrap().value;
            self.next_token();
            if block.symbol_table.contains_key(&identifier_value) {
                let identifier = Identifier {name: identifier_value, value: None};
                return Term::Identifier(identifier);
            } else {
                panic!("Identifier {} not declared", identifier_value);
            }
        } else if self.check_token(TokenType::Numeric) {
            let value = self
                .get_token_value(self.current_token.clone())
                .parse::<i64>();
            self.next_token();
            match value {
                Ok(integer) => return Term::Integer(integer),
                Err(_) => panic!("Parsing error"),
            }
        } else if self.check_token(TokenType::Logical) {
            let value = self
                .get_token_value(self.current_token.clone())
                .parse::<bool>();
            self.next_token();
            match value {
                Ok(b) => return Term::Bool(b),
                Err(_) => panic!("Parsing error"),
            }
        } else if self.check_token(TokenType::Text) {
            let text = self.get_token_value(self.current_token.clone());
            self.next_token();
            return Term::String(text);
        }
        panic!(
            "Syntax Error: Term cannot be matched: {:?}",
            self.current_token
        )
    }
    fn get_error(&mut self, message: &str) -> SyntaxError {
        SyntaxError { line: self.file_pos.0, col: self.file_pos.1, message: message.to_owned()}
    }
}
