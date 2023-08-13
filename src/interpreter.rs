use std::collections::HashMap;

use crate::grammar::{StatementBlock, Statement, PrintStatement, Term, Expression, Value, Identifier, Operator};

pub fn interpret(ast: StatementBlock) {
    let mut var_table = ast.symbol_table;
    for statement in ast.statements {
        match statement {
            Statement::If(if_statement) => {
                let value = interpret_expression(&var_table, if_statement.expression);
                match value {
                    Value::Bool(b) => {
                        if b {
                            interpret(if_statement.then_statement_block)
                        } else {
                            match if_statement.else_statement_block {
                                Some(block) => interpret(block),
                                None => {}
                            } 
                        }
                    },
                    _ => panic!("If condition must return a boolean")
                }
            },
            Statement::Assignment(assignement) => {
                let id_name = assignement.identifier.name;
                let id = Identifier {
                    name: id_name.clone(),
                    value: Some(interpret_expression(&var_table, assignement.expression)),
                };
                var_table.insert(id_name, id);

            }
            Statement::Print(print) => {
                match print {
                    PrintStatement::Term(term) => {
                        match term {
                            Term::Integer(int) => println!("{}",int),
                            Term::String(string) => println!("{}",string),
                            Term::Bool(b) => println!("{}",b),
                            Term::Identifier(identifier) => {
                                let id = &var_table.get(&identifier.name).unwrap();
                                match &id.value {
                                    Some(value) => println!("{:?}",value),
                                    None => println!("null"),
                                }
                            }
                        }
                    },
                    PrintStatement::Expression(expression) => {
                        let value = interpret_expression(&var_table, expression);
                        match value {
                            Value::Integer(int) => println!("{}", int),
                            Value::String(str) => println!("{}", str),
                            Value::Bool(b) => println!("{}", b),                        
                        }
                    }
                }
            }
        }
    }
}

fn interpret_expression(context: &HashMap<String, Identifier>, expression: Expression) -> Value {
    match expression {
        Expression::Operation(op) => {
            let left = interpret_expression(context, op.left);
            let right = interpret_expression(context, op.right);
            match left {
                Value::Bool(left_b) => {
                    if let Value::Bool(right_b) = right {
                        match op.operator {
                            Operator::Equal => return Value::Bool(left_b == right_b),
                            Operator::NotEqual => return Value::Bool(left_b != right_b),
                            Operator::And => return Value::Bool(left_b && right_b),
                            Operator::Or => return Value::Bool(left_b || right_b),
                            _ => panic!("Operation not permitted on boolean values")
                        }
                    } else {
                        panic!("Cannot operand differents types");
                    }
                }
                Value::Integer(left_i) => {
                    if let Value::Integer(right_i) = right {
                        match op.operator {
                            Operator::Equal => return Value::Bool(left_i == right_i),
                            Operator::NotEqual => return Value::Bool(left_i != right_i),
                            Operator::Plus => return Value::Integer(left_i + right_i),
                            Operator::Minus => return Value::Integer(left_i - right_i),
                            Operator::Division => {
                                if right_i == 0 {
                                    panic!("Cannot divide by 0");
                                }
                                return Value::Integer(left_i / right_i);
                            },
                            Operator::Modulo => return Value::Integer(left_i % right_i),
                            Operator::Multiplication => return Value::Integer(left_i * right_i),
                            Operator::Inferior => Value::Bool(left_i < right_i),
                            Operator::InfOrEqual => Value::Bool(left_i <= right_i),
                            Operator::Superior => Value::Bool(left_i > right_i),
                            Operator::SupOrEqual => Value::Bool(left_i >= right_i),
                            _ => panic!("Operation not permitted on integer values")
                        }
                    } else {
                        panic!("Cannot operand differents types");
                    }
                }
                Value::String(left_s) => {
                    if let Value::String(right_s) = right {
                        match op.operator {
                            Operator::Equal => return Value::Bool(left_s == right_s),
                            Operator::NotEqual => return Value::Bool(left_s != right_s),
                            Operator::Plus => return Value::String(left_s + &right_s),
                            _ => panic!("Operation not permitted on string values")
                        }
                    } else {
                        panic!("Cannot operand differents types");
                    }
                },
            }
        },
        Expression::Term(term) => {
            match term {
                Term::Integer(int) => return Value::Integer(int),
                Term::String(string) => return Value::String(string),
                Term::Bool(b) => return Value::Bool(b),
                Term::Identifier(id) => {
                    let scoped_id = context.get(&id.name).unwrap().clone();
                    return scoped_id.value.unwrap()
                },
            }
        }
    }
}