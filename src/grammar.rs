#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
    pub symbol_table: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(AssignmentStatement),
    If(IfStatement),
    Print(PrintStatement),
}
#[derive(Debug, Clone)]
pub enum PrintStatement {
    Term(Term),
    Expression(Expression)
}
#[derive(Debug, Clone)]
pub struct IfStatement {
    expression: Expression,
    then_statement_block: StatementBlock,
    else_statement_block: StatementBlock,
}
#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    identifier: Identifier,
    expression: Expression,
}
#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiplication,
    Equal,
    NotEqual,
    Inferior,
    InfOrEqual,
    Superior,
    SupOrEqual
}
#[derive(Debug, Clone)]
pub struct Identifier {}
#[derive(Debug, Clone)]
pub enum Expression {
    Operation(Box<Operation>),
    Term(Term)
}
#[derive(Debug, Clone)]
pub struct Operation {
    left: Expression,
    operator: Operator,
    right: Expression
}
#[derive(Debug, Clone)]
pub enum Term {
    Integer(i64),
    String(String),
    Identifier(Identifier),
}