# Language Grammar

program ::= statement*
statement ::= declaration_statement | assignment_statement | if_statement | print_statement
statement_block ::= { statement* }
declaration_statement ::= var identifier = expression
assignment_statement ::= identifier = expression
operator ::= + | - | \* | / | % | == | != | < | <= | > | >=
expression ::= term | expression operator expression
term ::= integer_literal | string_literal | identifier
if_statement ::= if (expression) statement_block else statement_block
print_statement ::= (expression) | string_literal
