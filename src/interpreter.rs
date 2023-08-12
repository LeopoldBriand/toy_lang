use std::collections::HashMap;

use crate::grammar::{StatementBlock, Statement, PrintStatement, Term, Expression, Value, Identifier};

pub fn interpret(ast: StatementBlock) {
    let mut var_table = ast.symbol_table;
    for statement in ast.statements {
        match statement {
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
            _ => panic!("Unknown statement {:?}", statement)
        }
    }
}

fn interpret_expression(context: &HashMap<String, Identifier>, expression: Expression) -> Value {
    match expression {
        Expression::Operation(op) => {
            todo!("operations")
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