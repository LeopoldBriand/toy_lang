# Language Grammar
program ::= statement*
statement ::= assignment_statement | if_statement
statement_block ::= { statement* }
assignment_statement ::= identifier = expression
operator ::= + | - | * | / | % | == | != | < | <= | > | >=
expression ::= term | expression operator expression
term ::= integer_literal | string_literal | identifier
if_statement ::= if (expression) statement_block else statement_block
print_statement ::= (expression) | string_literal